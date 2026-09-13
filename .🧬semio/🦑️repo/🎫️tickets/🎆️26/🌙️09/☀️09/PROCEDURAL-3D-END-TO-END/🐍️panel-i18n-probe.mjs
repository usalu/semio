/** 🌍️ The Catalogue and Inspection panels in German — the label groups the main probe could not
 * reach because its panel-button locator was too strict. Opens each panel by DOM id, not by text. */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const BASE = process.env.SEMIO_PROBE_BASE ?? "http://127.0.0.1:6018";
const outDir = join(import.meta.dir, "🗑️generated", "react-i18n-a11y", process.env.SEMIO_PROBE_OUT ?? "panels");
mkdirSync(outDir, { recursive: true });
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
await page.goto(`${BASE}/?plugin=generation3d`, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 150; i++) { await page.waitForTimeout(1000); if (await page.locator("[data-surface-id]").count()) break; }
await page.waitForTimeout(9000);

await page.locator("#framework\\.settings").first().click();
await page.waitForTimeout(1500);
await page.locator("button#framework\\.settings\\.language").first().click();
await page.waitForTimeout(900);
await page.locator("[role='option']").filter({ hasText: /Deutsch/ }).first().click();
await page.waitForTimeout(3500);
await page.keyboard.press("Escape");
await page.waitForTimeout(1500);

const buttons = await page.evaluate(() => [...document.querySelectorAll("button")].map((b) => ({ id: b.id, text: (b.innerText || "").replace(/\s+/g, " ").trim().slice(0, 40) })).filter((b) => b.text));
console.log("[DEBUG] buttons", JSON.stringify(buttons.slice(0, 40)));

const EXPECT = { widgets: "Elemente", catalog_slider: "Schieberegler", catalog_note: "Notiz", catalog_preview: "Vorschau", widget_group: "Element", no_selection: "Keine Auswahl", id_field: "ID", value_field: "Wert", range_field: "Bereich", schema_prefix: "Schema:", widgets_prefix: "Elemente:" };
const seen = {};
for (const name of ["Katalog", "Inspektion", "Dokument"]) {
  const b = page.locator("button").filter({ hasText: new RegExp(name) }).first();
  if (!(await b.count())) { console.log(`[DEBUG] ${name}: button absent`); continue; }
  await b.click();
  await page.waitForTimeout(3000);
  const text = await page.evaluate(() => document.body.innerText.replace(/\s+/g, " "));
  await page.screenshot({ path: join(outDir, `panel-${name}.png`) });
  writeFileSync(join(outDir, `panel-${name}.txt`), text);
  for (const [k, v] of Object.entries(EXPECT)) if (text.includes(v)) seen[k] = name;
  console.log(`[DEBUG] ${name}: opened, ${text.length} chars`);
  console.log(`[DEBUG] ${name} head: ${text.slice(0, 400)}`);
}
const missing = Object.keys(EXPECT).filter((k) => !(k in seen));
console.log("=== German observed for:", JSON.stringify(seen));
console.log("=== still not observed:", JSON.stringify(missing));
writeFileSync(join(outDir, "result.json"), JSON.stringify({ seen, missing, expect: EXPECT }, null, 2));
console.log("DONE");
await browser.close();
