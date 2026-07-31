import { chromium } from "playwright";
async function run() {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  await page.goto("http://localhost:6006/iframe.html?id=elements-aggregation-tree--default&viewMode=story");
  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(2000);
  const result = await page.evaluate(() => {
    const leafRows = Array.from(document.querySelectorAll('[data-slot="tree-item-row"]'));
    return leafRows.map((row) => {
      const label = row.querySelector('[data-slot="tree-label"]')?.textContent?.trim();
      const children = Array.from(row.children);
      const hasButton = children.some((c) => c.tagName === "BUTTON");
      const nonAbsoluteDivs = children.filter((c) => {
        if (c.tagName !== "DIV") return false;
        const cs = window.getComputedStyle(c);
        return cs.position !== "absolute";
      });
      return {
        label,
        hasButton,
        nonAbsoluteDivCount: nonAbsoluteDivs.length,
        childCount: children.length,
        childTags: children.map((c) => c.tagName).join(","),
      };
    });
  });
  for (const r of result) {
    const isLeaf = !r.hasButton;
    console.log(`${isLeaf ? "LEAF" : "PARENT"} [${r.label}] children=${r.childCount} tags=${r.childTags} nonAbsDivs=${r.nonAbsoluteDivCount}`);
  }
  await browser.close();
}
run().catch(console.error);
