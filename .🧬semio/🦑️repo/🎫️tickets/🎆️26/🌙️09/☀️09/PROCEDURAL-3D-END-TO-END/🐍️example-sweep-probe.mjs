/** 🎨 Example-switch sweep probe: picks every example through the navbar combobox and polls the flow
 * window's published node ids once a second, recording exactly WHEN the surface flips to the picked
 * example's graph and when the preview re-arms.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=sweep-1 SEMIO_PROBE_HOLD=60 bun 🐍️example-sweep-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "example-sweep");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 220);
const hold = Number(process.env.SEMIO_PROBE_HOLD ?? 60);
const only = (process.env.SEMIO_PROBE_PICK ?? "").split(",").map((s) => s.trim()).filter(Boolean);
const diagnostics = process.env.SEMIO_PROBE_DIAG !== "0";
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
if (diagnostics) await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 3000)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`));

const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map((el) => {
    let meshes = 0; try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); meshes = Array.isArray(v) ? v.length : 0; } catch {}
    const st = parse(el.getAttribute("data-status-json"));
    const fx = parse(el.getAttribute("data-fixture-json"));
    const widgetIds = fx && Array.isArray(fx.widgets) ? fx.widgets.map((w) => { const inner = w && typeof w === "object" ? Object.values(w)[0] : null; return (w && w.id) ?? (inner && inner.id) ?? null; }).filter(Boolean) : null;
    return { surfaceId: el.getAttribute("data-surface-id"), meshes, phase: st?.phase ?? null, ratio: st?.progress?.ratio ?? null, fault: st?.fault?.code ?? null, nodeIds: st && typeof st === "object" && !st.phase ? Object.keys(st) : null, widgetIds };
  });
  const combo = document.querySelector('[role="combobox"]');
  return { hosts, meshes: hosts.reduce((n, h) => n + h.meshes, 0), example: combo?.innerText?.replace(/\s+/g, " ").trim() ?? null };
});
const readRow = (s) => {
  const main = s.hosts.find((h) => h.surfaceId === "window:procedural-main");
  const preview = s.hosts.find((h) => h.surfaceId && h.surfaceId.endsWith("-preview"));
  return { widgetIds: main?.widgetIds ?? [], nodeIds: main?.nodeIds ?? [], meshes: preview?.meshes ?? 0, phase: preview?.phase ?? null, ratio: preview?.ratio ?? null, example: s.example };
};
const results = [];
await page.goto(url, { waitUntil: "domcontentloaded" });
let last = null;
for (let i = 0; i < bootWait; i++) { await page.waitForTimeout(1000); last = readRow(await snap()); if (last.widgetIds.length && last.phase === "idle" && last.meshes > 0) break; }
console.log(`[DEBUG] boot ${JSON.stringify(last)}`);
results.push({ label: "boot", flipSeconds: null, meshSeconds: null, final: last });
await page.screenshot({ path: join(outDir, "0-boot.png") });

const combo = () => page.locator('[role="combobox"]').first();
await combo().click({ timeout: 5000 }); await page.waitForTimeout(500);
let options = (await page.locator('[role="option"]').allInnerTexts()).map((t) => t.replace(/\s+/g, " ").trim()).filter(Boolean);
await page.keyboard.press("Escape"); await page.waitForTimeout(300);
if (only.length) options = options.filter((t) => only.includes(t));
console.log("[DEBUG] options", JSON.stringify(options));

for (const text of options) {
  const before = readRow(await snap());
  const pickedAt = Date.now();
  const mark = lines.length;
  await combo().click({ timeout: 5000 }); await page.waitForTimeout(400);
  await page.locator('[role="option"]').filter({ hasText: text }).first().click({ timeout: 5000 });
  let flipSeconds = null, meshSeconds = null, row = before;
  const trail = [];
  for (let i = 0; i < hold; i++) {
    await page.waitForTimeout(1000);
    row = readRow(await snap());
    trail.push({ s: Math.round((Date.now() - pickedAt) / 1000), widgets: row.widgetIds, nodes: row.nodeIds, meshes: row.meshes, phase: row.phase });
    if (flipSeconds === null && JSON.stringify(row.widgetIds) !== JSON.stringify(before.widgetIds)) flipSeconds = Math.round((Date.now() - pickedAt) / 1000);
    if (meshSeconds === null && flipSeconds !== null && row.meshes > 0 && row.phase === "idle") meshSeconds = Math.round((Date.now() - pickedAt) / 1000);
    if (flipSeconds !== null && meshSeconds !== null) break;
  }
  console.log(`[DEBUG] ${text}: flip=${flipSeconds}s mesh=${meshSeconds}s widgets=${JSON.stringify(row.widgetIds)} nodes=${JSON.stringify(row.nodeIds)} meshes=${row.meshes} phase=${row.phase}`);
  results.push({ label: text, flipSeconds, meshSeconds, final: row, trail });
  await page.screenshot({ path: join(outDir, `${results.length}-${text.replace(/[^a-z0-9]+/gi, "-")}.png`) });
  writeFileSync(join(outDir, `console-${text.replace(/[^a-z0-9]+/gi, "-")}.txt`), lines.slice(mark).join("\n"));
}
writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] DONE", results.length);
await browser.close();
