/** 🎟️ wgpu RESIDENT-BUDGET + EVENT-DRIVEN-SETTLE probe.
 *
 * Reads three things the `📓️wgpu-dock-layout-world3d-2026-09-12.md` §6.1/§6.2 hop needs proven:
 *   1. the process-wide resident census the shell now prints on every render leave,
 *   2. every `Capacity` refusal (now a typed diagnostic naming what was asked for), and
 *   3. the preview window's own `meshes_json` / status — `extrude@solid`, `facesDone`, `ratio`.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-settle/run-1 bun 🐍️wgpu-settle-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 120);
const shotEvery = Number(process.env.SEMIO_PROBE_SHOT_EVERY ?? 30);
const windows = (process.env.SEMIO_PROBE_WINDOWS ?? "procedural-main,procedural-preview").split(",").filter(Boolean);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-settle/run");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 6000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));

const samples = [];
for (let second = 1; second <= seconds; second += 1) {
  await page.waitForTimeout(1000);
  if (process.env.SEMIO_PROBE_CLICK_AT && second === Number(process.env.SEMIO_PROBE_CLICK_SECOND ?? 20)) {
    // 🖱️ One real pointer click, where the deliverable asks for a mutation (generate mode's
    // "Add Generation") rather than a boot-only chain.
    const [cx, cy] = process.env.SEMIO_PROBE_CLICK_AT.split(",").map(Number);
    await page.mouse.click(cx, cy).catch((error) => lines.push(`${at()} clickerror ${String(error).slice(0, 200)}`));
    lines.push(`${at()} PROBE clicked ${cx},${cy}`);
  }
  if (process.env.SEMIO_PROBE_NUDGE === "1") {
    // 🖱️ Forces a frame each second: the wgpu shell is event-driven, so a surface whose own cursor
    // wake never reaches the scheduler makes no progress at all while nothing else moves.
    await page.mouse.move(200 + (second % 5), 400 + (second % 5)).catch(() => {});
  }
  const sample = await page
    .evaluate(async (ids) => {
      const beacon = globalThis.semioWgpuIntrospection;
      const parse = async (call) => {
        try {
          const raw = await call();
          return raw ? JSON.parse(raw) : null;
        } catch (error) {
          return { error: String(error) };
        }
      };
      if (typeof beacon?.dumpStructure !== "function") return { structure: null, perWindow: null, alert: null };
      const perWindow = {};
      for (const id of ids) {
        const structure = await parse(() => beacon.dumpStructure(id));
        perWindow[id] = {
          nodeCount: structure?.nodes?.length ?? 0,
          stats: await parse(() => beacon.dumpFrameStats(id)),
          scenes: (structure?.nodes ?? []).filter((n) => String(n.kind ?? "").toLowerCase().includes("scene")).map((n) => ({ path: n.path, rect: n.rect })),
        };
      }
      return { perWindow, alert: document.querySelector('[role="alert"]')?.textContent?.slice(0, 1200) ?? null };
    }, windows)
    .catch((error) => ({ error: String(error).slice(0, 300) }));
  sample.t = at();
  samples.push(sample);
  if (second % shotEvery === 0 || second === seconds) {
    // 🧹️ Trunk's own dev-server build-failure overlay is the FIRST body child and covers the whole
    // page whenever any crate in the workspace is red — including a peer's, on a serve that is still
    // correctly running the last good bundle. It is dev-server chrome, never app content, so it is
    // removed before the shot and the removal is reported in the console log.
    const removed = await page
      .evaluate(() => {
        const overlay = Array.from(document.body.children).find((node) => node.tagName === "DIV" && node.id !== "root");
        if (!overlay) return null;
        const text = (overlay.textContent ?? "").slice(0, 120);
        overlay.remove();
        return text;
      })
      .catch(() => null);
    if (removed) lines.push(`${at()} PROBE removed trunk overlay ${JSON.stringify(removed)}`);
    await page.screenshot({ path: join(outDir, `shot-${String(second).padStart(3, "0")}s.png`), type: "png" }).catch(() => {});
  }
}

const has = (needle) => lines.filter((line) => line.includes(needle));
const last = samples.at(-1) ?? {};
const census = has("resident-roots=").map((line) => {
  const roots = Number(line.split("resident-roots=")[1]?.split(" ")[0] ?? 0);
  const bytes = Number(line.split("resident-bytes=")[1]?.split("/")[0] ?? 0);
  return { roots, bytes, surface: line.split("surface=")[1]?.split(" ")[0] ?? null };
});
const preview = last.perWindow?.["procedural-preview"] ?? {};
const meshLines = has("meshes_json").concat(has("meshesHead")).slice(-4);
const statusLines = has("facesDone").concat(has("phase")).slice(-6);
const verdict = {
  url,
  seconds: Math.round(at() / 1000),
  alert: last.alert ?? null,
  perWindow: last.perWindow ?? null,
  previewStats: preview.stats ?? null,
  previewScenes: preview.scenes ?? [],
  counts: {
    capacity: has("Capacity").length,
    residentRefusal: has("retained document permit failed").length,
    settleRounds: has("ui chain settled after").length,
    settleExhausted: has("ui chain exhausted").length,
    renderBegin: has("wgpu-shell render begin").length,
    surfaceFault: has("wgpu-shell surface fault").length,
    extrudeSolid: has("extrude@solid").length,
    unknownTag115: has("unknown tag 115").length,
    invokeExtensionFaulted: has("invokeExtension faulted").length,
  },
  residentPeakRoots: census.reduce((max, row) => Math.max(max, row.roots), 0),
  residentPeakBytes: census.reduce((max, row) => Math.max(max, row.bytes), 0),
  residentSamples: census.slice(-10),
  settleLines: has("ui chain settled after").slice(-6).concat(has("ui chain exhausted").slice(-3)),
  capacityLines: has("retained document permit failed").slice(-6),
  meshLines,
  statusLines,
  failureLines: lines.filter((line) => line.includes("pageerror") || line.includes("panicked") || line.includes("surface fault")).slice(-14),
};
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "samples.json"), JSON.stringify(samples.slice(-4), null, 2));
writeFileSync(join(outDir, "verdict.json"), JSON.stringify(verdict, null, 2));
console.log("DONE", JSON.stringify(verdict, null, 2));
await browser.close();
