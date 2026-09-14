/** 🔎️ Why no window ever becomes `[data-slot="window"][data-active="true"]` in the generate mode.
 *
 * Reads the dock's own DOM, not a theory: for edit AND generate it records which window carries
 * `data-active`, what the dock tabs say, where the silhouette gaps sit, and which of three
 * affordances (dock tab / window body / tree row) actually moves the active flag. Then it presses
 * `mod+shift+g` in each mode and records what the shell invoked.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=generate-chord/recon bun 🐍️generate-activation-recon.mjs
 * @see 🐍️react-gap-probe.mjs, 📓️react-generate-chord-restage-2026-09-14.md
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "generate-chord/recon");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 180);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
const page = await context.newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 800)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 800)}`));

const report = { url, steps: [] };
const note = async (step, detail) => {
  report.steps.push({ step, detail, t: Date.now() - t0 });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1400)}`);
  writeFileSync(join(outDir, "recon.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step}.png`) }).catch(() => {});
};

const dock = () => page.evaluate(() => {
  const rect = (el) => { const r = el.getBoundingClientRect(); return { x: Math.round(r.x), y: Math.round(r.y), w: Math.round(r.width), h: Math.round(r.height) }; };
  return {
    activeWindow: document.querySelector('[data-slot="window"][data-active="true"]')?.id ?? null,
    windows: [...document.querySelectorAll('[data-slot="window"]')].map((el) => ({ id: el.id, active: el.getAttribute("data-active"), ...rect(el) })),
    tabs: [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].map((el) => ({ id: el.getAttribute("data-window-id"), active: el.getAttribute("data-active"), stackActive: el.getAttribute("data-stack-active"), ...rect(el) })),
    stacks: [...document.querySelectorAll('[data-slot="mode-dock-stack"]')].map((el) => ({ path: el.getAttribute("data-stack-path"), ...rect(el) })),
    gaps: [...document.querySelectorAll("[data-window-silhouette-gap]")].map((el) => ({ slot: el.getAttribute("data-slot"), stack: el.closest('[data-slot="mode-dock-stack"]')?.getAttribute("data-stack-path") ?? null, ...rect(el) })),
    treeRows: [...document.querySelectorAll('[data-slot="window"] [role="treeitem"]')].map((el) => ({ id: el.id, window: el.closest('[data-slot="window"]')?.id ?? null, ...rect(el) })).slice(0, 12),
    meshes: [...document.querySelectorAll("[data-meshes-json]")].map((el) => ({ surfaceId: el.getAttribute("data-surface-id"), meshes: (() => { try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? ""); return Array.isArray(v) ? v.length : 0; } catch { return 0; } })() })),
  };
});
const invoked = (mark) => [...new Set(lines.slice(mark).flatMap((l) => [...l.matchAll(/"actionId":"([^"]+)"/g)].map((m) => m[1])))];
const until = async (read, pred, seconds) => { let v = await read(); for (let i = 0; i < seconds && !pred(v); i++) { await page.waitForTimeout(1000); v = await read(); } return v; };

await page.goto(url, { waitUntil: "domcontentloaded" });
await until(dock, (d) => d.meshes.some((m) => m.meshes > 0), bootWait);
await note("boot", await dock());

// 📐️ The exact predicate `isSurfaceActiveBackgroundPointer` runs, evaluated in the page for a point.
await page.exposeFunction("__semioNow", () => Date.now());
const backgroundAt = (x, y) => page.evaluate(([px, py]) => {
  const el = document.elementFromPoint(px, py);
  const stack = el?.closest?.('[data-slot="mode-dock-stack"]') ?? null;
  const scope = stack ?? document;
  const hits = [];
  for (const gap of scope.querySelectorAll("[data-window-silhouette-gap]")) {
    if (stack && gap.closest('[data-slot="mode-dock-stack"]') !== stack) continue;
    const r = gap.getBoundingClientRect();
    if (r.width <= 0 || r.height <= 0) continue;
    if (px >= r.left && px <= r.right && py >= r.top && py <= r.bottom) hits.push({ slot: gap.getAttribute("data-slot"), x: Math.round(r.x), y: Math.round(r.y), w: Math.round(r.width), h: Math.round(r.height) });
  }
  return { target: el ? `${el.tagName}#${el.id}.${(el.className ?? "").toString().slice(0, 60)}` : null, inGapOfOwnStack: hits.length > 0, hits, stack: stack?.getAttribute("data-stack-path") ?? null };
}, [x, y]);

// ── edit mode: which affordance moves the flag ───────────────────────────────
{
  const mark = lines.length;
  await page.keyboard.press("Meta+Shift+g");
  await page.waitForTimeout(3000);
  await note("edit-chord-untouched", { activeWindow: (await dock()).activeWindow, invoked: invoked(mark) });
}
{
  const before = await dock();
  const main = before.windows.find((w) => w.id === "procedural-main");
  const point = main ? { x: main.x + 40, y: main.y + main.h - 40 } : null;
  const bg = point ? await backgroundAt(point.x, point.y) : null;
  if (point) await page.mouse.click(point.x, point.y);
  await page.waitForTimeout(1500);
  await note("edit-body-click", { point, bg, before: before.activeWindow, after: (await dock()).activeWindow });
}
{
  const mark = lines.length;
  await page.keyboard.press("Meta+Shift+g");
  await page.waitForTimeout(4000);
  await note("edit-chord", { activeWindow: (await dock()).activeWindow, invoked: invoked(mark) });
}

// ── generate mode ────────────────────────────────────────────────────────────
await page.keyboard.press("Meta+Alt+ArrowRight");
await page.waitForTimeout(8000);
const inGenerate = await dock();
await note("generate-entered", inGenerate);

// 🪪️ NEVER the tab centre: a dock tab carries its own maximize/close chrome buttons, and a centre
// click toggles maximize (which collapses the layout to one stack and makes every later reading a
// lie). The label sits at the tab's leading edge.
for (const [label, pick] of [
  ["generate-row-click", (d) => { const r = d.treeRows.find((x) => x.window === "generation3d-generations"); return r && { x: r.x + Math.min(60, r.w / 2), y: r.y + r.h / 2 }; }],
  ["generate-body-click", (d) => { const w = d.windows.find((x) => x.id === "generation3d-generate-form"); return w && { x: w.x + w.w / 2, y: w.y + w.h - 60 }; }],
  ["generate-tab-label-click", (d) => { const t = d.tabs.find((x) => x.id === "generation3d-generations"); return t && { x: t.x + 14, y: t.y + t.h / 2 }; }],
]) {
  const d = await dock();
  const point = pick(d);
  const bg = point ? await backgroundAt(point.x, point.y) : null;
  const mark = lines.length;
  if (point) await page.mouse.click(point.x, point.y);
  await page.waitForTimeout(1800);
  const after = await dock();
  await note(label, { point, bg, before: d.activeWindow, after: after.activeWindow, tabs: after.tabs, invoked: invoked(mark) });
}

{
  const mark = lines.length;
  await page.keyboard.press("Meta+Shift+g");
  await page.waitForTimeout(6000);
  const after = await dock();
  await note("generate-chord", { activeWindow: after.activeWindow, invoked: invoked(mark), meshes: after.meshes });
}

writeFileSync(join(outDir, "recon.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] RECON DONE");
await browser.close();
