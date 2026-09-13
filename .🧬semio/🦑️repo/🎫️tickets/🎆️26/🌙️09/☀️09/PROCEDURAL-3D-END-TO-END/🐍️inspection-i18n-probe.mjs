/** 🌍️ The selection-dependent German label groups: Inspection with and without a selection, and the
 * Catalogue scrolled far enough to show the input-widget kinds. */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const BASE = process.env.SEMIO_PROBE_BASE ?? "http://127.0.0.1:6018";
const outDir = join(import.meta.dir, "🗑️generated", "react-i18n-a11y", process.env.SEMIO_PROBE_OUT ?? "inspection");
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

const dump = async (tag) => {
  const text = await page.evaluate(() => document.body.innerText.replace(/\s+/g, " "));
  writeFileSync(join(outDir, `${tag}.txt`), text);
  await page.screenshot({ path: join(outDir, `${tag}.png`) });
  return text;
};
const EXPECT = { catalog_slider: "Schieberegler", catalog_note: "Notiz", no_selection: "Keine Auswahl", value_field: "Wert", range_field: "Bereich", catalog_neuron: "Neuron" };
const seen = {};
const record = (tag, text) => { for (const [k, v] of Object.entries(EXPECT)) if (text.includes(v) && !(k in seen)) seen[k] = tag; };

// Inspection with NO selection → "Keine Auswahl"
await page.locator("button#framework\\.panel\\.inspection").click();
await page.waitForTimeout(3000);
record("inspection-empty", await dump("1-inspection-empty"));
console.log("[DEBUG] after inspection-empty", JSON.stringify(seen));

// Select a graph node in the outline, then re-read Inspection → Wert / Bereich
const row = page.locator("[role='treeitem']").filter({ hasText: /Column Height/ }).first();
if (await row.count()) {
  await row.click();
  await page.waitForTimeout(3500);
  record("inspection-selected", await dump("2-inspection-selected"));
  console.log("[DEBUG] clicked Column Height row");
} else console.log("[DEBUG] Column Height treeitem not found");
console.log("[DEBUG] after inspection-selected", JSON.stringify(seen));

// Catalogue, scrolled, for the input widget kinds
await page.locator("button#framework\\.panel\\.catalogue").click();
await page.waitForTimeout(3500);
record("catalogue", await dump("3-catalogue"));
const cat = await page.evaluate(() => {
  const items = [...document.querySelectorAll("[role='treeitem']")].map((e) => (e.innerText || "").replace(/\s+/g, " ").trim()).filter(Boolean);
  return items.slice(0, 80);
});
console.log("[DEBUG] catalogue treeitems", JSON.stringify(cat).slice(0, 1400));
record("catalogue-items", cat.join(" "));

console.log("=== observed:", JSON.stringify(seen));
console.log("=== missing:", JSON.stringify(Object.keys(EXPECT).filter((k) => !(k in seen))));
writeFileSync(join(outDir, "result.json"), JSON.stringify({ seen, missing: Object.keys(EXPECT).filter((k) => !(k in seen)), catalogueItems: cat }, null, 2));
console.log("DONE");
await browser.close();
