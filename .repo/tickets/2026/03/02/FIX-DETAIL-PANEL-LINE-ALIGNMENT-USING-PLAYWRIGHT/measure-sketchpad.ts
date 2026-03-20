import path from "path";
import { chromium } from "playwright";

async function measure() {
  const browser = await chromium.launch();
  const page = await browser.newPage();
  await page.goto("http://localhost:5173");
  await page.waitForTimeout(2000);
  const kitZipPath = path.resolve("/workspaces/semio/semio/assets/semio/metabolism.zip");
  const fileChooserPromise = page.waitForEvent("filechooser");
  const openButton = page.locator('text=Open Kit').first();
  const hasOpenButton = await openButton.count();
  if (hasOpenButton > 0) {
    await openButton.click();
  } else {
    const importBtn = page.getByRole("button").filter({ hasText: /import|open/i }).first();
    const hasImport = await importBtn.count();
    if (hasImport > 0) {
      await importBtn.click();
    }
  }
  try {
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles(kitZipPath);
    await page.waitForTimeout(3000);
  } catch (e) {
    console.log("Could not import via file chooser, trying drag-drop or navigation");
  }
  await page.waitForTimeout(2000);
  const rows = await page.locator('[data-slot="tree-section-row"], [data-slot="tree-item-row"]').all();
  console.log("Sketchpad: Found " + rows.length + " tree rows");
  for (const row of rows) {
    const label = await row.locator('[data-slot="tree-label"]').textContent();
    const rowBox = await row.boundingBox();
    if (!rowBox) { console.log(label + " - no box"); continue; }
    const paddingLeft = await row.evaluate((el: HTMLElement) => parseFloat(getComputedStyle(el).paddingLeft));
    const chevronCount = await row.locator("svg").count();
    let chevronCenterX: number | null = null;
    if (chevronCount > 0) {
      const chevronBox = await row.locator("svg").first().boundingBox();
      if (chevronBox) chevronCenterX = chevronBox.x + chevronBox.width / 2;
    }
    const lineWrappers = await row.locator(":scope > div.absolute").first().locator(":scope > div").all();
    const linePositions: number[] = [];
    for (const lw of lineWrappers) {
      const innerCount = await lw.locator("div").count();
      if (innerCount > 0) {
        const lineBox = await lw.locator("div").first().boundingBox();
        if (lineBox) linePositions.push(lineBox.x + lineBox.width / 2);
      }
    }
    console.log(JSON.stringify({ label, paddingLeft, elementCenter: chevronCenterX?.toFixed(1), linePositions: linePositions.map(p => p.toFixed(1)) }));
  }
  await page.screenshot({ path: "/workspaces/semio/.repo/tickets/2026/03/02/FIX-DETAIL-PANEL-LINE-ALIGNMENT-USING-PLAYWRIGHT/sketchpad-tree.png", fullPage: true });
  console.log("Screenshot saved");
  await browser.close();
}
measure().catch((e) => { console.error(e); process.exit(1); });
