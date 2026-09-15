/** 🔍 Recon: how the Inspection panel's number field for the `height` widget, and the generate-mode
 * Form slider, are reached on the current stage.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=slider/inspector-reach bun 🐍️inspector-reach-recon.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "slider/inspector-reach");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 500)}`));
page.on("pageerror", (e) => { lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 800)}`); console.log("[DEBUG] pageerror", String(e).slice(0, 400)); });
page.on("requestfailed", (r) => { lines.push(`${Date.now() - t0} requestfailed ${r.url().slice(0, 200)} ${r.failure()?.errorText}`); });
page.on("response", (r) => { if (r.status() >= 400) { lines.push(`${Date.now() - t0} http${r.status()} ${r.url().slice(0, 200)}`); console.log("[DEBUG] http", r.status(), r.url().slice(0, 160)); } });

const inspectorIds = () => page.evaluate(() => [...document.querySelectorAll('[id*="procedural-play-inspector"]')].map((el) => ({ id: el.id, tag: el.tagName, value: el.value ?? null, text: (el.textContent ?? "").trim().slice(0, 40) })));
const sliders = () => page.evaluate(() => [...document.querySelectorAll('[role="slider"]')].map((el) => ({ label: el.getAttribute("aria-label"), now: el.getAttribute("aria-valuenow"), owner: el.closest("[id]")?.id ?? null })));
const converged = () => page.evaluate(() => { const el = document.querySelector('[data-surface-id="window:procedural-preview"]'); try { const s = JSON.parse(el?.getAttribute("data-status-json") ?? "{}"); return JSON.parse(el?.getAttribute("data-meshes-json") ?? "[]").length > 0 && s.phase === "idle"; } catch { return false; } });

await page.goto(`${url}&example=hexagonal-mushroom-column`, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); if (await converged()) break; }
console.log("[DEBUG] converged, sliders", JSON.stringify(await sliders()));

await page.locator('[id="framework.panel.artifact"]').first().click({ timeout: 8000 }).catch((e) => console.log("[DEBUG] artifact", String(e).slice(0, 100)));
await page.waitForTimeout(2000);
await page.locator('[data-slot="panel"] [id="panel:procedural-play-graph/height"]').first().click({ timeout: 8000 }).catch((e) => console.log("[DEBUG] rowclick", String(e).slice(0, 160)));
await page.waitForTimeout(3000);
console.log("[DEBUG] inspector-after-row", JSON.stringify(await inspectorIds()));
await page.locator('[id="framework.panel.inspection"]').first().click({ timeout: 8000 }).catch((e) => console.log("[DEBUG] inspection", String(e).slice(0, 100)));
await page.waitForTimeout(3000);
console.log("[DEBUG] inspector-after-tab", JSON.stringify(await inspectorIds()));
await page.screenshot({ path: join(outDir, "1-inspection.png") });

await page.keyboard.press("Meta+Alt+ArrowRight");
await page.waitForTimeout(6000);
const add = page.locator(':text-is("Add Generation")').first();
console.log("[DEBUG] addRow", await add.count());
if (await add.count()) await add.click({ timeout: 6000 }).catch(() => {});
for (let i = 0; i < 90; i++) {
  await page.waitForTimeout(1000);
  const ok = await page.evaluate(() => { const el = document.querySelector('[data-surface-id="window:generation3d-generate-preview"]'); try { return JSON.parse(el?.getAttribute("data-meshes-json") ?? "[]").length > 0; } catch { return false; } });
  if (ok) break;
}
console.log("[DEBUG] form controls", JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[id*="generate.form."]')].map((el) => ({ id: el.id, tag: el.tagName, role: el.getAttribute("role") })))));
console.log("[DEBUG] sliders-generate", JSON.stringify(await sliders()));
await page.screenshot({ path: join(outDir, "2-generate.png") });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
await browser.close();
console.log("RECON DONE");
