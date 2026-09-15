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
/** ⚙️ The locale switch OPENS the Settings panel, and `Escape` closes its popover, not the panel. Left
 * standing it owns a dock slot the panels below are read from. */
await page.locator("#framework\\.settings").first().click({ timeout: 8000 }).catch(() => {});
await page.waitForTimeout(2000);

const dump = async (tag) => {
  const text = await page.evaluate(() => document.body.innerText.replace(/\s+/g, " "));
  writeFileSync(join(outDir, `${tag}.txt`), text);
  await page.screenshot({ path: join(outDir, `${tag}.png`) });
  return text;
};
/** 🗣️ `catalog_slider`/`catalog_note`/`catalog_neuron` are GONE: their only resolver
 * (`generation3d_catalog_label`) had no production caller at all, so the labels were dead rather than
 * untranslated and were deleted with the resolver (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). What is
 * left here is what the Inspection panel really paints. */
const EXPECT = { no_selection: "Keine Auswahl", value_field: "Wert", range_field: "Bereich" };
const seen = {};
const record = (tag, text) => { for (const [k, v] of Object.entries(EXPECT)) if (text.includes(v) && !(k in seen)) seen[k] = tag; };

/** 🎯️ Panel tabs are clicked at their LEFT EDGE, never their centre: the top-right dock's `Collapse`
 * fold control (`button#framework.panel.top-right.fold`) paints over the trailing two thirds of the
 * Inspection tab, so a centre click collapses the dock instead of opening the panel — which is why this
 * probe reached NOTHING and reported every field missing (measured in
 * `🐍️panel-tab-occlusion-recon.mjs`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END gap G2). */
/** 🗂️ Opens a panel and CONFIRMS its own body is on screen. A tab TOGGLES, so a single click can leave
 * the panel shut — and this probe used to read a page where the panel never opened and report every
 * German field "missing", which looks exactly like an untranslated label. The body marker is what tells
 * the two apart. */
const openTab = async (id, bodyMarker) => {
  const tab = page.locator(`button#${id.replace(/\./gu, "\\.")}`);
  if ((await tab.count()) === 0) { console.log(`[DEBUG] tab ${id} absent`); return false; }
  for (let attempt = 0; attempt < 3; attempt += 1) {
    const box = await tab.first().boundingBox().catch(() => null);
    await tab.first().click({ position: { x: 8, y: Math.round((box?.height ?? 22) / 2) }, timeout: 8000 }).catch((e) => console.log(`[DEBUG] tab ${id} ${String(e).replace(/\s+/gu, " ").slice(0, 120)}`));
    await page.waitForTimeout(2500);
    if (!bodyMarker) return true;
    const present = await page.evaluate((marker) => document.querySelectorAll(`[id*="${marker}"]`).length, bodyMarker);
    if (present > 0) { console.log(`[DEBUG] tab ${id} opened (${present} ${bodyMarker} nodes, attempt ${attempt})`); return true; }
  }
  console.log(`[DEBUG] tab ${id} never showed ${bodyMarker}`);
  return false;
};

// Inspection with NO selection → "Keine Auswahl"
await openTab("framework.panel.inspection", "procedural-play-inspector");
record("inspection-empty", await dump("1-inspection-empty"));
console.log("[DEBUG] after inspection-empty", JSON.stringify(seen));

/** 🌳️ The outline that names a widget lives in the ARTIFACT panel now, and that panel shares the dock
 * slot with Inspection — so the row is clicked there, and Inspection is re-opened to read the fields the
 * selection paints. */
await openTab("framework.panel.artifact", "procedural-play-graph");
const row = page.locator("[data-slot='panel'] [role='treeitem']").filter({ hasText: /Column Height/ }).first();
if (await row.count()) {
  await row.click({ timeout: 8000 }).catch((e) => console.log(`[DEBUG] outline row ${String(e).replace(/\s+/gu, " ").slice(0, 120)}`));
  await page.waitForTimeout(3500);
  console.log("[DEBUG] clicked Column Height row");
  await openTab("framework.panel.inspection", "procedural-play-inspector");
  record("inspection-selected", await dump("2-inspection-selected"));
} else console.log("[DEBUG] Column Height treeitem not found");
console.log("[DEBUG] after inspection-selected", JSON.stringify(seen));

/** 🕳️ "Keine Auswahl" is NOT the no-selection state. `inspection::render` paints the schema + widget
 * count when nothing is selected, and reaches `labels.no_selection` only on its SECOND branch: a node IS
 * selected and no fixture widget carries that id — i.e. an operator node or a port row, never one of the
 * input widgets. Selecting one of those is the only way a user ever reads this label. */
await openTab("framework.panel.artifact", "procedural-play-graph");
/** 🗑️ DELETING the selected node is the reach. Every graph node here IS a fixture widget (widgets = 7,
 * one per node) and port rows carry no activate binding at all, so no ordinary click can leave a
 * selection whose id names nothing. A delete can: the selection id outlives the widget it named for the
 * render that follows, which is exactly the branch `labels.no_selection` lives on. */
const victim = page.locator("[data-slot='panel'] [role='treeitem']").filter({ hasText: /Side Count|Radius/u }).first();
if (await victim.count()) {
  await victim.click({ timeout: 8000 }).catch((e) => console.log(`[DEBUG] victim row ${String(e).replace(/\s+/gu, " ").slice(0, 120)}`));
  await page.waitForTimeout(3000);
  await page.keyboard.press("Delete");
  await page.waitForTimeout(4500);
  await openTab("framework.panel.inspection", "procedural-play-inspector");
  record("inspection-non-widget", await dump("4-inspection-non-widget"));
  console.log("[DEBUG] after inspection-non-widget", JSON.stringify(seen));
} else console.log("[DEBUG] no deletable treeitem found");

// Catalogue, scrolled, for the input widget kinds
await openTab("framework.panel.catalogue", "procedural-play-catalogue");
await page.waitForTimeout(500);
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
