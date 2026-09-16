import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

// 🧪️ Artifact tree + reference mutation: open the Artifact panel, pick an object row (framework `cad`
// domain), pick a reference row (app-owned selection), then drive the inspection panel's bounded
// `patchCadPlayReference` rows and confirm the mutation lands (inspector width readout + history).
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6020/?plugin=cad";
const bootSeconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 45);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "tree-reference");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, 1500)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 1500)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(bootSeconds * 1000);
const report = {};
const note = (key, value) => { report[key] = value; lines.push(`${Date.now() - t0} probe ${key} ${JSON.stringify(value).slice(0, 600)}`); };
const clickRow = async (id) => {
  const row = page.locator(`[id="${id}"]`).first();
  await row.waitFor({ state: "attached", timeout: 8000 });
  await row.evaluate((el) => el.scrollIntoView({ block: "center" }));
  await page.waitForTimeout(300);
  const box = await row.boundingBox();
  if (!box) throw new Error(`row ${id} has no box`);
  await page.mouse.click(box.x + Math.min(90, box.width / 2), box.y + box.height / 2);
  await page.waitForTimeout(2500);
};
const selectionOf = (surface) => page.evaluate((s) => JSON.parse(document.querySelector(`[data-surface-id="${s}"]`)?.getAttribute("data-selection-json") ?? "{}"), surface);
const inspector = () => page.evaluate(() => [...document.querySelectorAll('[id^="panel:cad-play-inspector/"]')].map((e) => `${e.id.replace("panel:cad-play-inspector/", "")}=${e.textContent.trim().replace(/\s+/g, " ").slice(0, 60)}`));
const treeRows = () => page.evaluate(() => [...document.querySelectorAll('[id^="panel:cad-play-document/"]')].map((e) => ({ id: e.id.replace("panel:cad-play-document/", ""), selected: e.getAttribute("aria-selected") })));

try {
await page.getByRole("button", { name: "Artifact", exact: true }).first().click();
await page.waitForTimeout(2500);
const rows = await treeRows();
note("treeRows", rows.length);
note("treeRowIds", rows.map((r) => r.id).slice(0, 40));

await clickRow("panel:cad-play-document/object-hexagonal-cut-concrete-forest-left-bim-1");
note("afterObjectRow.building", (await selectionOf("window:cad-play-building")).selectedIds);
note("afterObjectRow.treeSelected", (await treeRows()).filter((r) => r.selected === "true").map((r) => r.id));
note("afterObjectRow.inspector", await inspector());
await page.screenshot({ path: join(outDir, "object-row.png"), type: "png" });

// 📂️ Reference sections are collapsed by default — expand the energy one before picking its row.
const sectionIds = await page.evaluate(() => [...document.querySelectorAll('[id^="panel:cad-play-document/cad-play-document.references"]')].map((e) => e.id));
note("referenceSections", sectionIds);
await clickRow("panel:cad-play-document/cad-play-document.references.aec.building.energy");
await clickRow("panel:cad-play-document/cad-reference:aec.building.energy:ref-concrete-forest");
note("afterReferenceRow.inspector", await inspector());
note("afterReferenceRow.building", (await selectionOf("window:cad-play-building")).selectedIds);
note("afterReferenceRow.treeSelected", (await treeRows()).filter((r) => r.selected === "true").map((r) => r.id));
note("afterReferenceRow.guestSelection", await page.evaluate(() => document.querySelector('[data-surface-id="window:cad-play-energy"]')?.getAttribute("data-guest-selection-json")?.slice(0, 300)));
await page.screenshot({ path: join(outDir, "reference-row.png"), type: "png" });

const widthBefore = (await inspector()).find((r) => r.startsWith("cad-play-inspector.reference.width="));
await clickRow("panel:cad-play-inspector/cad-play-inspector.reference.width.grow");
const widthAfter = (await inspector()).find((r) => r.startsWith("cad-play-inspector.reference.width="));
note("widthGrow", { before: widthBefore, after: widthAfter });
await clickRow("panel:cad-play-inspector/cad-play-inspector.reference.origin.x.plus");
note("originPlus", (await inspector()).find((r) => r.startsWith("cad-play-inspector.reference.origin=")));
await clickRow("panel:cad-play-inspector/cad-play-inspector.reference.locked");
note("lockToggle", (await inspector()).filter((r) => r.startsWith("cad-play-inspector.reference.locked")));
await page.screenshot({ path: join(outDir, "after-patches.png"), type: "png" });
// ↩️ Undo the last edit through the shell's keyboard chord; the inspector readout must revert.
await page.mouse.click(720, 300);
await page.waitForTimeout(800);
await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
await page.waitForTimeout(2500);
note("afterUndo.locked", (await inspector()).filter((r) => r.startsWith("cad-play-inspector.reference.locked")));
note("energySelectionAfterReference", await page.evaluate(() => JSON.parse(document.querySelector('[data-surface-id="window:cad-play-energy"]')?.getAttribute("data-guest-selection-json") ?? "{}")));
note("buildingGuestSelection", await page.evaluate(() => JSON.parse(document.querySelector('[data-surface-id="window:cad-play-building"]')?.getAttribute("data-guest-selection-json") ?? "{}")));
// 🧰️ Dislocate utility on the building pane with an object selected → the gumball arms.
await clickRow("panel:cad-play-document/object-hexagonal-cut-concrete-forest-left-bim-1");
const utilities = page.getByRole("button", { name: "Utilities", exact: true });
await utilities.nth(1).click().catch(() => {});
await page.waitForTimeout(1200);
note("utilityMenu", await page.evaluate(() => [...document.querySelectorAll('[role="menuitem"], [role="menuitemradio"], [role="option"]')].map((e) => e.textContent?.trim()).filter(Boolean).slice(0, 12)));
const dislocate = page.getByRole("menuitem", { name: /Dislocate|Transform|Move/i }).first();
if (await dislocate.count()) { await dislocate.click(); await page.waitForTimeout(2500); }
note("buildingGuestSelectionWithUtility", await page.evaluate(() => JSON.parse(document.querySelector('[data-surface-id="window:cad-play-building"]')?.getAttribute("data-guest-selection-json") ?? "{}")));
await page.screenshot({ path: join(outDir, "utility.png"), type: "png" });
const history = await page.evaluate(() => {
  const button = [...document.querySelectorAll("button")].find((b) => b.textContent?.trim() === "History");
  return button ? { found: true } : { found: false };
});
note("history", history);
} catch (error) {
  note("error", String(error).slice(0, 300));
  await page.screenshot({ path: join(outDir, "error.png"), type: "png" }).catch(() => {});
}
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
const faults = lines.filter((l) => /pageerror|dropped action|Unknown action|fixed-capacity|surface-render|typed-operation completion effects failed/i.test(l));
console.log("DONE faults", faults.length);
for (const f of faults.slice(0, 10)) console.log("FAULT", f.slice(0, 300));
console.log(JSON.stringify(report, null, 1).slice(0, 4000));
await browser.close();
