import { chromium } from "playwright";
import path from "path";
import { fileURLToPath } from "url";
const zipPath = path.resolve("/workspaces/semio/compose/assets/compose/metabolism.zip");
const browser = await chromium.launch({ headless: true, args: ["--no-sandbox","--disable-gpu"] });
const page = await browser.newPage();
// initHome
await page.goto("http://127.0.0.1:5173/", { waitUntil: "domcontentloaded" });
await page.waitForTimeout(2000);
const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
await fileInput.waitFor({ state: "attached", timeout: 10000 });
const [fileChooser] = await Promise.all([
  page.waitForEvent("filechooser", { timeout: 5000 }).catch(() => null),
  fileInput.dispatchEvent("click"),
]);
if (fileChooser) await fileChooser.setFiles(zipPath);
else { await fileInput.setInputFiles(zipPath); await fileInput.evaluate(el => el.dispatchEvent(new Event("change", { bubbles: true }))); }
await page.getByText("Metabolism", { exact: true }).first().waitFor({ state: "visible", timeout: 60000 });
await page.waitForTimeout(500);
const tableRow = page.locator('tr[data-row-id]').filter({ hasText: "Metabolism" }).first();
if (await tableRow.isVisible().catch(() => false)) await tableRow.dblclick({ force: true });
else await page.getByText("Metabolism").first().dblclick({ force: true });
await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
await page.waitForLoadState("networkidle");
await page.waitForTimeout(2000);
// initDesign 
await page.waitForTimeout(3000);
const allRowIds = await page.evaluate(() => Array.from(document.querySelectorAll("[data-row-id]")).map(el => el.getAttribute("data-row-id")).slice(0, 20));
const designRowIds = allRowIds.filter(id => id?.startsWith("design-"));
if (designRowIds.length === 0) {
  const designElement = page.getByText("Nakagin Capsule Tower", { exact: true }).first();
  if (await designElement.isVisible({ timeout: 5000 }).catch(() => false)) await designElement.dblclick({ force: true });
} else {
  const nakaginRowId = designRowIds.find(id => id?.includes("9a890dd4")) ?? designRowIds[designRowIds.length - 1];
  await page.evaluate(rowId => {
    const row = document.querySelector(`[data-row-id="${rowId}"]`);
    if (row) row.dispatchEvent(new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window }));
  }, nakaginRowId);
}
await page.waitForLoadState("networkidle");
await page.waitForTimeout(5000);
const diag = page.locator('#diagram .react-flow').first();
await diag.waitFor({ state: "visible", timeout: 60000 });
const nodes = diag.locator(".react-flow__node");
await nodes.first().waitFor({ state: "attached", timeout: 60000 });
for (let i = 0; i < 10; i++) { await page.waitForTimeout(2000); if (await nodes.count() >= 170) break; }
await page.waitForTimeout(3000);
// Count DOM elements
const stats = await page.evaluate(() => {
  const rf = document.querySelector('#diagram .react-flow');
  const nodesDom = rf?.querySelectorAll('.react-flow__node') ?? [];
  const edgesDom = rf?.querySelectorAll('.react-flow__edge') ?? [];
  const viewport = rf?.querySelector('.react-flow__viewport');
  const transform = viewport?.style.transform ?? "";
  const pane = rf?.querySelector('.react-flow__pane');
  const paneRect = pane?.getBoundingClientRect();
  return {
    totalNodes: nodesDom.length,
    totalEdges: edgesDom.length,
    viewportTransform: transform,
    paneWidth: paneRect?.width,
    paneHeight: paneRect?.height,
  };
});
console.log("Stats:", JSON.stringify(stats, null, 2));
await browser.close();
