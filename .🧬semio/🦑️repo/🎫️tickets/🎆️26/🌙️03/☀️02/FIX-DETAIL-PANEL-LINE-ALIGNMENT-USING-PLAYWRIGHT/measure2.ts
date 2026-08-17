import { chromium } from "playwright";

async function measure() {
  const browser = await chromium.launch();
  const page = await browser.newPage();
  await page.goto("http://localhost:6006/iframe.html?id=elements-aggregation-tree--default&viewMode=story");
  await page.waitForTimeout(3000);
  const rows = await page.locator('[data-slot="tree-section-row"], [data-slot="tree-item-row"]').all();
  console.log("Found " + rows.length + " tree rows");
  for (const row of rows) {
    const label = await row.locator('[data-slot="tree-label"]').textContent();
    const rowBox = await row.boundingBox();
    if (!rowBox) {
      console.log(label + " - no box");
      continue;
    }
    const paddingLeft = await row.evaluate((el: HTMLElement) => parseFloat(getComputedStyle(el).paddingLeft));
    const chevronCount = await row.locator("svg").count();
    let chevronCenterX: number | null = null;
    if (chevronCount > 0) {
      const chevronBox = await row.locator("svg").first().boundingBox();
      if (chevronBox) chevronCenterX = chevronBox.x + chevronBox.width / 2;
    }
    const spacerCount = await row.locator("div.flex-shrink-0").first().count();
    let spacerCenterX: number | null = null;
    if (chevronCenterX === null) {
      const allDivs = await row.locator(":scope > div").all();
      for (const div of allDivs) {
        const cls = await div.getAttribute("class");
        if (cls && cls.includes("w-[14px]")) {
          const box = await div.boundingBox();
          if (box) spacerCenterX = box.x + box.width / 2;
          break;
        }
      }
    }
    const elementCenter = chevronCenterX ?? spacerCenterX;
    const lineWrappers = await row.locator(":scope > div.absolute").first().locator(":scope > div").all();
    const linePositions: number[] = [];
    for (const lw of lineWrappers) {
      const lwBox = await lw.boundingBox();
      if (lwBox) {
        const innerLine = await lw.locator("div").first().count();
        if (innerLine > 0) {
          const lineBox = await lw.locator("div").first().boundingBox();
          if (lineBox) linePositions.push(lineBox.x + lineBox.width / 2);
        }
      }
    }
    console.log(JSON.stringify({ label, paddingLeft, elementCenter: elementCenter?.toFixed(1), linePositions: linePositions.map((p) => p.toFixed(1)) }));
  }
  const contentRows = await page.locator('[data-slot="tree-content"]').all();
  console.log("Found " + contentRows.length + " tree-content rows");
  for (const row of contentRows) {
    const paddingLeft = await row.evaluate((el: HTMLElement) => parseFloat(getComputedStyle(el).paddingLeft));
    const lineWrappers = await row.locator(":scope > div.absolute").first().locator(":scope > div").all();
    const linePositions: number[] = [];
    for (const lw of lineWrappers) {
      const lineBox = await lw.locator("div").first().boundingBox();
      if (lineBox) linePositions.push(lineBox.x + lineBox.width / 2);
    }
    console.log(JSON.stringify({ type: "content", paddingLeft, linePositions: linePositions.map((p) => p.toFixed(1)) }));
  }
  await browser.close();
}
measure().catch((e) => {
  console.error(e);
  process.exit(1);
});
