import { chromium } from "playwright";
async function run() {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ viewport: { width: 600, height: 800 } });
  await page.goto("http://localhost:6006/iframe.html?id=elements-aggregation-tree--panel&viewMode=story");
  await page.waitForLoadState("domcontentloaded");
  await page.waitForTimeout(5000);
  const treeContainer = page.locator(".border.p-4").first();
  await treeContainer.screenshot({ path: "/tmp/storybook-tree-panel.png" });
  console.log("Panel story screenshot saved");
  await page.goto("http://localhost:6006/iframe.html?id=elements-aggregation-tree--default&viewMode=story");
  await page.waitForLoadState("domcontentloaded");
  await page.waitForTimeout(3000);
  const defaultContainer = page.locator(".border.p-4").first();
  await defaultContainer.screenshot({ path: "/tmp/storybook-tree-default.png" });
  console.log("Default story screenshot saved");
  const data = await page.evaluate(() => {
    const rows = document.querySelectorAll('[data-slot="tree-section-row"], [data-slot="tree-item-row"]');
    const result: any[] = [];
    rows.forEach((row) => {
      const rect = row.getBoundingClientRect();
      const label = row.querySelector('[data-slot="tree-label"]')?.textContent?.trim();
      const slot = row.getAttribute("data-slot");
      const children = Array.from(row.children).map((c) => {
        const cr = c.getBoundingClientRect();
        const cs = window.getComputedStyle(c);
        return {
          tag: c.tagName,
          class: c.className?.toString().substring(0, 80),
          left: cr.left - rect.left,
          width: cr.width,
          height: cr.height,
          position: cs.position,
          display: cs.display,
        };
      });
      result.push({ slot, label, paddingLeft: window.getComputedStyle(row).paddingLeft, children });
    });
    return result;
  });
  console.log("\n=== ROW CHILDREN DETAILS ===");
  for (const r of data) {
    console.log(`\n${r.slot} [${r.label}] padL=${r.paddingLeft}:`);
    for (const c of r.children) {
      console.log(`  ${c.tag} pos=${c.position} disp=${c.display} left=${c.left.toFixed(1)} w=${c.width.toFixed(1)} cls="${c.class}"`);
    }
  }
  await browser.close();
}
run().catch(console.error);
