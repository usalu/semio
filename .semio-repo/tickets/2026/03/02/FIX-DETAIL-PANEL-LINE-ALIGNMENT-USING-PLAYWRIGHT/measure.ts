import { chromium } from "playwright";

async function measure() {
  const browser = await chromium.launch();
  const page = await browser.newPage();
  
  // Go to the Default Tree storybook story
  await page.goto("http://localhost:6006/iframe.html?id=elements-aggregation-tree--default&viewMode=story");
  await page.waitForTimeout(3000);
  
  // Measure all tree rows
  const rows = await page.locator('[data-slot="tree-section-row"], [data-slot="tree-item-row"]').all();
  console.log(`\nFound ${rows.length} tree rows\n`);
  
  for (const row of rows) {
    const label = await row.locator('[data-slot="tree-label"]').textContent().catch(() => "?");
    const rowBox = await row.boundingBox();
    if (!rowBox) continue;
    
    // Measure chevron (direct SVG or button-wrapped SVG)
    const chevron = row.locator('svg').first();
    const chevronBox = await chevron.boundingBox().catch(() => null);
    
    // Measure spacer div if no chevron
    const spacer = row.locator('div.w-\\[14px\\]');
    const spacerBox = await spacer.boundingBox().catch(() => null);
    
    const firstElement = chevronBox || spacerBox;
    const firstElementCenter = firstElement ? firstElement.x + firstElement.width / 2 : null;
    
    // Measure indentation lines
    const lines = await row.locator('.absolute.top-0.bottom-0 > div.w-px').all();
    const linePositions: number[] = [];
    for (const line of lines) {
      const lineBox = await line.boundingBox();
      if (lineBox) linePositions.push(lineBox.x + lineBox.width / 2);
    }
    
    const paddingLeft = await row.evaluate(el => parseFloat(getComputedStyle(el).paddingLeft));
    
    console.log(`"${label}" | paddingLeft=${paddingLeft}px | firstElementCenter=${firstElementCenter?.toFixed(1)} | lines=[${linePositions.map(p => p.toFixed(1)).join(", ")}]`);
  }
  
  // Also measure TreeContent rows
  const contentRows = await page.locator('[data-slot="tree-content"]').all();
  console.log(`\nFound ${contentRows.length} tree-content rows\n`);
  
  for (const row of contentRows) {
    const paddingLeft = await row.evaluate(el => parseFloat(getComputedStyle(el).paddingLeft));
    const rowBox = await row.boundingBox();
    const lines = await row.locator('.absolute.top-0.bottom-0 > div.w-px').all();
    const linePositions: number[] = [];
    for (const line of lines) {
      const lineBox = await line.boundingBox();
      if (lineBox) linePositions.push(lineBox.x + lineBox.width / 2);
    }
    console.log(`TreeContent | paddingLeft=${paddingLeft}px | lines=[${linePositions.map(p => p.toFixed(1)).join(", ")}]`);
  }
  
  // Take screenshot for visual inspection 
  await page.screenshot({ path: "$TICKET_DIR/storybook-tree.png", fullPage: true });
  console.log("\nScreenshot saved.");
  
  await browser.close();
}

measure().catch(console.error);
