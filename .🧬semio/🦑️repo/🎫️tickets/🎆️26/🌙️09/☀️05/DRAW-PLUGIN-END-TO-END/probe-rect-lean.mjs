import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const out = process.argv[2]; const diag = process.argv[3] !== "nodiag"; mkdirSync(out, { recursive: true });
const t0 = Date.now(); const lines = [];
const log = s => { lines.push(`${Date.now() - t0} ${s}`); writeFileSync(join(out, "probe.txt"), lines.join("\n")); };
const consolePatch = () => {
  const keep = /units with no change|publication-stalled|waiting on|refused|\[PROBE\]|\[DEBUG\]/;
  for (const k of ["log", "debug", "info"]) { const o = console[k].bind(console); console[k] = (...a) => { if (keep.test(String(a[0]))) o(...a); }; }
  const e = console.error.bind(console);
  console.error = (...a) => { if (a.length === 0 || String(a[0]) === "") e("[PROBE] empty console.error", a.length, new Error().stack); else e(...a); };
};
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
if (diag) await context.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
await context.addInitScript(`(${consolePatch})();`);
const page = await context.newPage();
page.on("worker", w => { w.evaluate(consolePatch).catch(() => {}); });
page.on("console", m => log(`${m.type()} ${m.text().slice(0, 1500)}`));
page.on("pageerror", e => log(`pageerror ${e}`));
page.on("framenavigated", f => { if (f === page.mainFrame()) log("NAVIGATED " + f.url()); });
await page.goto("http://127.0.0.1:6064/?plugin=draw", { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, null, { timeout: 400000 });
log("ready");
const canvas = page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"] canvas').first();
await canvas.waitFor({ state: "visible", timeout: 60000 });
log("canvas visible");
await page.waitForTimeout(5000);
if (process.argv[5] === "switch") {
  const picker = page.locator('[id="playground.navbar.fixture"]').first();
  await picker.click({ timeout: 60000 }); await page.getByRole("option", { name: "Blank Drawing", exact: true }).click({ timeout: 60000 }); log("blank selected"); await page.waitForTimeout(4000);
  await picker.click({ timeout: 60000 }); await page.getByRole("option", { name: "Demo", exact: true }).click({ timeout: 60000 }); log("demo selected"); await page.waitForTimeout(6000);
}
const click = id => page.locator(`[id="${id}"]`).first().evaluate(el => el.click());
const status = page.locator('[id="drawing-composite"] [data-slot="engagement-status"]');
const engagement = page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first();
for (const open of [true, false]) { if (await status.isVisible() !== open) await engagement.click({ timeout: 60000 }); await status.waitFor({ state: open ? "visible" : "hidden", timeout: 60000 }); log("engagement " + open); }
const layerText0 = async () => { try { if (!(await status.isVisible())) await engagement.click({ timeout: 20000 }); await status.waitFor({ state: "visible", timeout: 20000 }); const t = await status.innerText(); await engagement.click({ timeout: 20000 }); await status.waitFor({ state: "hidden", timeout: 20000 }); return t; } catch (e) { return "ERR " + String(e).slice(0, 100); } };
log("PRESTATUS " + await layerText0());
log("unfold"); await page.locator('[id^="framework.window."][id$=".utilityBar.unfold"]').first().evaluate(el => el.click()); log("unfolded");
await page.locator('[id^="ui.utilities."][id$=".group.group:Drawing"]').first().evaluate(el => el.click());
log("group ok"); await click("shapeRect");
await page.waitForFunction(() => document.getElementById("shapeRect")?.getAttribute("aria-pressed") === "true", null, { timeout: 60000 });
log("utility pressed");
const rect = await canvas.boundingBox();
await page.mouse.move(rect.x + rect.width * 0.35, rect.y + rect.height * 0.35);
await page.mouse.down();
await page.mouse.move(rect.x + rect.width * 0.6, rect.y + rect.height * 0.6, { steps: 8 });
log("before up");
await page.mouse.up();
log("after up");
await page.waitForTimeout(Number(process.argv[4] ?? 120000));
const layerTextDefined = 1; const layerText = async () => { try { if (!(await status.isVisible())) await engagement.click({ timeout: 20000 }); await status.waitFor({ state: "visible", timeout: 20000 }); return await status.innerText(); } catch (e) { return "ERR " + String(e).slice(0, 100); } };
log("STATUS " + await layerText());
log("LAYERROWS " + await page.locator('[role="treeitem"][id^="panel:framework.panel.artifact/drawing-play-layers."]').count());
log("done");
await browser.close();
