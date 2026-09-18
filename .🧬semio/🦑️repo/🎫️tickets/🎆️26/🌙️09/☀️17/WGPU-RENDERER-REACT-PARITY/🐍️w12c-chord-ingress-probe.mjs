/** ⌨️🎥 W12c — chord + camera ingress probe.
 *
 * Boots ONE renderer and drives the family C / family D gestures in isolation, dumping the action
 * journal AND the renderer's own console lines around each gesture, so "the chord never arrived",
 * "the chord arrived and was swallowed" and "the chord arrived and dispatched" are three different,
 * MEASURED answers instead of one empty journal.
 *
 * Usage:
 *   cd <ticket> && SEMIO_W12C_TARGET=wgpu SEMIO_W12C_URL=http://127.0.0.1:6213/?plugin=puzzle3d \
 *     SEMIO_W12C_OUT=w12c-ingress-1 bun 🐍️w12c-chord-ingress-probe.mjs
 * Env: SEMIO_W12C_TARGET (wgpu|react) · SEMIO_W12C_URL · SEMIO_W12C_OUT · SEMIO_W12C_BOOT (s, 240) ·
 *      SEMIO_W12C_SETTLE (ms, 2000) · SEMIO_W12C_GREP (console filter, default the key/camera terms)
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const target = process.env.SEMIO_W12C_TARGET ?? "wgpu";
const url = process.env.SEMIO_W12C_URL ?? (target === "react" ? "http://127.0.0.1:6313/?plugin=puzzle3d" : "http://127.0.0.1:6213/?plugin=puzzle3d");
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_W12C_OUT ?? "w12c-ingress");
const bootSeconds = Number(process.env.SEMIO_W12C_BOOT ?? 240);
const settleMs = Number(process.env.SEMIO_W12C_SETTLE ?? 2000);
const grep = new RegExp(process.env.SEMIO_W12C_GREP ?? "key routing|edit chord|handle_event Key|dispatch_normalized_event Key|camera settle|world3d|chord|keyboard failed|wgpu-shell", "i");
const viewport = { width: 1600, height: 1000 };
const mod = process.platform === "darwin" ? "Meta" : "Control";
mkdirSync(outDir, { recursive: true });

const browser = await chromium.launch({ headless: process.env.SEMIO_W12C_HEADED !== "1", args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const context = await browser.newContext({ viewport, deviceScaleFactor: 1 });
await context.addInitScript(() => {
  try {
    localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
    localStorage.setItem("semio.runtime.diagnostics", "1");
  } catch {}
});
const page = await context.newPage();
const consoleLines = [];
const started = Date.now();
page.on("console", (message) => consoleLines.push(`${Date.now() - started} ${message.type()} ${message.text()}`.slice(0, 1200)));
page.on("pageerror", (error) => consoleLines.push(`${Date.now() - started} page error ${String(error).slice(0, 600)}`));
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180_000 });

const dump = (probe) =>
  page
    .evaluate(async (which) => {
      const beacon = globalThis.semioWgpuIntrospection;
      if (typeof beacon?.[which] !== "function") return { unavailable: which };
      try {
        const raw = await beacon[which]();
        return raw ? JSON.parse(raw) : null;
      } catch (error) {
        return { error: String(error).slice(0, 300) };
      }
    }, probe)
    .catch((error) => ({ error: String(error).slice(0, 300) }));

const actions = async () => {
  if (target === "react") return page.evaluate(() => (globalThis.__semioInputLedger?.recent ?? []).map((row) => ({ seq: row.inputSeq, action: row.action, origin: row.origin, outcome: row.outcome?.kind ?? "open" }))).catch(() => []);
  const chrome = await dump("dumpChrome");
  return (chrome?.actions ?? []).map((entry) => ({ seq: entry.seq, action: entry.action, origin: entry.origin ?? "shell", windowId: entry.windowId, args: entry.args }));
};

for (let second = 0; second < bootSeconds; second += 1) {
  await page.waitForTimeout(1000);
  if (target === "react") {
    const ready = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready")).catch(() => null);
    if (ready && second > 4) break;
  } else {
    const structure = await dump("dumpStructure");
    if (structure && !structure.unavailable && !structure.error && second > 4) break;
  }
}
await page.locator("canvas").first().evaluate((element) => element.focus?.()).catch(() => {});

const surfacePoint = async () => {
  if (target === "react") {
    const host = await page
      .evaluate(() => {
        const element = [...document.querySelectorAll('[data-slot="pane-host"], [data-slot="window-body"]')].sort((a, b) => b.getBoundingClientRect().width * b.getBoundingClientRect().height - a.getBoundingClientRect().width * a.getBoundingClientRect().height)[0];
        if (!element) return null;
        const rect = element.getBoundingClientRect();
        return [rect.x, rect.y, rect.width, rect.height];
      })
      .catch(() => null);
    return host ? [host[0] + host[2] * 0.7, host[1] + host[3] * 0.45] : [viewport.width / 2, viewport.height / 2];
  }
  const meshes = await dump("dumpMeshStats");
  const surface = (meshes?.surfaces ?? [])[0];
  if (surface?.rect) return [surface.rect[0] + surface.rect[2] * 0.7, surface.rect[1] + surface.rect[3] * 0.45];
  return [viewport.width / 2, viewport.height / 2];
};

const drag = async (from, to, button) => {
  await page.mouse.move(...from);
  await page.mouse.down({ button });
  for (let step = 1; step <= 8; step += 1) await page.mouse.move(from[0] + ((to[0] - from[0]) * step) / 8, from[1] + ((to[1] - from[1]) * step) / 8);
  await page.mouse.up({ button });
};

const hits = async () => {
  const chrome = await dump("dumpChrome");
  return (chrome?.hits ?? []).map((hit) => ({ id: hit.controlId, kind: hit.kind, rect: hit.rect }));
};
const centre = (rect) => [rect[0] + rect[2] / 2, rect[1] + rect[3] / 2];
const clickControl = async (match) => {
  const rows = await hits();
  const row = rows.find((hit) => (hit.id ?? "") === match) ?? rows.find((hit) => (hit.id ?? "").endsWith(match)) ?? rows.find((hit) => (hit.id ?? "").includes(match));
  if (!row) return console.log(`      (no control ${match})`);
  await page.mouse.click(...centre(row.rect));
};

/** ⌨️ A chord LIVENESS ladder: `mod+z` after every chrome gesture of the parity journey, so the exact
 * gesture after which the shell stops routing chords is MEASURED rather than guessed. */
const clickMatching = async (pattern) => {
  const rows = await hits();
  const row = rows.find((hit) => pattern.test(`${hit.id ?? ""} ${hit.kind ?? ""}`));
  if (!row) return console.log(`      (no control ${pattern})`);
  await page.mouse.click(...centre(row.rect));
  return row;
};
const undo = () => page.keyboard.press(`${mod}+z`);
const steps = [
  { name: "baseline-undo", run: undo },
  { name: "click-tour-skip", run: () => clickControl("ui.introduction.skip") },
  { name: "panel-artifact", run: () => clickControl("framework.panel.artifact") },
  { name: "panel-catalogue", run: () => clickControl("framework.panel.catalogue") },
  { name: "panel-inspection", run: () => clickControl("framework.panel.inspection") },
  { name: "panel-tool-runs", run: () => clickControl("framework.panel.toolRun") },
  { name: "panel-chat", run: () => clickControl("framework.chat") },
  { name: "panel-chat-close", run: () => clickControl("framework.chat") },
  { name: "after-panels-undo", run: undo },
  { name: "chip-engagement", run: () => clickControl(".engagement.toggle") },
  { name: "chip-search", run: () => clickControl(".search.toggle") },
  { name: "chip-utilitybar", run: () => clickControl(".utilityBar.unfold") },
  { name: "after-chips-undo", run: undo },
  { name: "split-gutter-drag", run: async () => { const rows = await hits(); const gutter = rows.find((hit) => /resizable-handle|separator|PanelResize|DockSplit/i.test(`${hit.id ?? ""} ${hit.kind ?? ""}`)); if (!gutter) return console.log("      (no gutter)"); const at = centre(gutter.rect); await drag(at, [at[0] + 120, at[1]], "left"); } },
  { name: "after-gutter-undo", run: undo },
  { name: "after-gutter-escape", run: () => page.keyboard.press("Escape") },
  { name: "window-cap-focus", run: () => clickMatching(/(^|[.\-])focus$/i) },
  { name: "after-cap-focus-undo", run: undo },
  { name: "window-cap-close", run: () => clickMatching(/(^|[.\-])close$/i) },
  { name: "after-cap-close-undo", run: undo },
  { name: "window-reopen", run: () => clickMatching(/mode-dock-tab|dock\.tab|appSwitcher/i) },
  { name: "after-reopen-undo", run: undo },
  { name: "after-reopen-escape", run: () => page.keyboard.press("Escape") },
];

const report = [];
for (const step of steps) {
  const beforeActions = await actions();
  const beforeConsole = consoleLines.length;
  await step.run().catch((error) => consoleLines.push(`step error ${String(error).slice(0, 200)}`));
  await page.waitForTimeout(settleMs);
  const afterActions = await actions();
  const seen = new Set(beforeActions.map((row) => row.seq));
  const fresh = afterActions.filter((row) => !seen.has(row.seq));
  report.push({
    step: step.name,
    actions: fresh.map((row) => row.action),
    rows: fresh.slice(0, 16),
    console: consoleLines.slice(beforeConsole).filter((line) => grep.test(line)).slice(0, 40),
  });
  writeFileSync(join(outDir, "ingress.json"), JSON.stringify(report, null, 2));
  console.log(`${step.name}: [${fresh.map((row) => row.action).join(", ")}]`);
  for (const line of report.at(-1).console.slice(0, 12)) console.log(`    | ${line}`);
}
writeFileSync(join(outDir, "console.txt"), consoleLines.join("\n"));
await page.screenshot({ path: join(outDir, "final.png") }).catch(() => {});
await browser.close();
console.log(`\nwrote ${outDir}`);
