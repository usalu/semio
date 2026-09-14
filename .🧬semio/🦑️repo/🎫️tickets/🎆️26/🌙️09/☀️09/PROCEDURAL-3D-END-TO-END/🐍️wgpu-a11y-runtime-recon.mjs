/** ♿️🔎️ wgpu ARIA-mirror RECON — why `#semio-wgpu-accessibility` reports `nodeCount 0` on 6118.
 *
 * Separates the four candidate causes the end-to-end lane could not tell apart, because the probe's
 * `Number(root.dataset.nodeCount ?? 0)` reads `0` for ALL of them:
 *   1. the mirror element was never created (no `onReady`) — `present:false`;
 *   2. it was created but never PAINTED (no UI turn ever reached `refresh`) — `present:true` with
 *      NO `data-node-count` attribute at all (`attr:null`), which is not the same as `"0"`;
 *   3. it painted an EMPTY projection — `attr:"0"`;
 *   4. the projection is non-empty per window but the dump ANSWERS for only one of them — the
 *      "largest viewport" selection `dumpStructure` uses, which is right for a diagnostic and wrong
 *      for the production accessibility path.
 *
 * Usage: cd <ticket> && bun 🐍️wgpu-a11y-runtime-recon.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const baseUrl = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-a11y-runtime/recon");
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT ?? 180);
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 60);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const note = (text) => {
  lines.push(`${at()} PROBE ${text}`);
  console.log(`[DEBUG] ${text}`);
};

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 6000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

const introspect = (kind, windowId) =>
  page
    .evaluate(
      async ([kind, id]) => {
        const beacon = globalThis.semioWgpuIntrospection;
        if (typeof beacon?.[kind] !== "function") return null;
        try {
          const raw = await beacon[kind](id);
          return raw ? JSON.parse(raw) : { empty: true };
        } catch (error) {
          return { error: String(error) };
        }
      },
      [kind, windowId],
    )
    .catch(() => null);

/** ♿️ The raw mirror state — distinguishing "never painted" from "painted empty". */
const mirrorState = () =>
  page
    .evaluate(() => {
      const root = document.getElementById("semio-wgpu-accessibility");
      const canvas = document.getElementById("semio-wgpu-canvas");
      return {
        present: root !== null,
        role: root?.getAttribute("role") ?? null,
        label: root?.getAttribute("aria-label") ?? null,
        attr: root?.getAttribute("data-node-count") ?? null,
        children: root?.children.length ?? -1,
        parentId: root?.parentElement?.id ?? null,
        canvasPresent: canvas !== null,
        uiTurn: canvas?.dataset.uiTurn ?? null,
      };
    })
    .catch(() => null);

const pump = async (ms) => {
  const deadline = Date.now() + ms;
  let flip = 0;
  while (Date.now() < deadline) {
    await page.waitForTimeout(180);
    flip = 1 - flip;
    await page.mouse.move(3 + flip, 3).catch(() => {});
  }
};

await page.goto(baseUrl, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));
let windowIds = [];
for (let second = 0; second < bootSeconds; second += 1) {
  await pump(1000);
  windowIds = (await introspect("dumpStructure", undefined))?.windowIds ?? [];
  if (windowIds.length > 0) break;
}
note(`booted windowIds=${JSON.stringify(windowIds)} at ${Math.round(at() / 1000)}s`);

const samples = [];
for (let sample = 0; sample < Math.ceil(settleSeconds / 10); sample += 1) {
  await pump(10000);
  const state = await mirrorState();
  const defaultDump = await introspect("dumpAccessibility", undefined);
  const structure = await introspect("dumpStructure", undefined);
  const ids = structure?.windowIds ?? windowIds;
  const perWindow = {};
  for (const id of ids) {
    const dump = await introspect("dumpAccessibility", id);
    const struct = await introspect("dumpStructure", id);
    perWindow[id] = { a11y: (dump?.windows ?? []).reduce((sum, entry) => sum + (entry.nodes?.length ?? 0), 0), structure: struct?.nodes?.length ?? 0, viewport: struct?.viewport ?? null, windowId: dump?.windowId ?? null };
  }
  samples.push({ t: at(), mirror: state, defaultA11yWindows: (defaultDump?.windows ?? []).map((entry) => `${entry.windowId}:${entry.nodes?.length ?? 0}`), defaultA11yCount: (defaultDump?.windows ?? []).reduce((sum, entry) => sum + (entry.nodes?.length ?? 0), 0), defaultStructureWindow: structure?.windowId ?? null, perWindow });
  note(`sample ${sample}: mirror=${JSON.stringify(state)} defaultA11y=${JSON.stringify((defaultDump?.windows ?? []).map((entry) => `${entry.windowId}:${entry.nodes?.length ?? 0}`))} perWindow=${JSON.stringify(perWindow)}`);
}

const last = samples.at(-1);
const fullDefault = await introspect("dumpAccessibility", undefined);
const fullPerWindow = {};
for (const id of last?.perWindow ? Object.keys(last.perWindow) : []) fullPerWindow[id] = await introspect("dumpAccessibility", id);
writeFileSync(join(outDir, "recon.json"), JSON.stringify({ url: baseUrl, windowIds, samples, fullDefault, fullPerWindow }, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
note("done");
await browser.close();
