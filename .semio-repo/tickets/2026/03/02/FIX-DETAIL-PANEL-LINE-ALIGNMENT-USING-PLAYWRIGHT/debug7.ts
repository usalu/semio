import { chromium } from "@playwright/test";
async function main() {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
  await page.goto("http://127.0.0.1:6006/iframe.html?id=elements-aggregation-tree--panel&viewMode=story");
  await page.waitForSelector('[data-slot="tree-item-row"]', { timeout: 15000 });
  const data = await page.evaluate(() => {
    const items = document.querySelectorAll('[data-slot="tree-item-row"]');
    const first = items[0];
    if (!first) return null;
    const rect = first.getBoundingClientRect();
    const btn = first.querySelector('button');
    const svg = btn?.querySelector('svg');
    const lineDiv = first.querySelector('.pointer-events-none');
    const lineDivChild = lineDiv?.firstElementChild;
    return {
      svgClass: svg?.getAttribute('class'),
      svgComputedWidth: svg ? getComputedStyle(svg).width : null,
      lineChildLeft: lineDivChild ? getComputedStyle(lineDivChild).left : null,
      paddingLeft: getComputedStyle(first).paddingLeft,
    };
  });
  console.log(JSON.stringify(data, null, 2));
  await browser.close();
}
main();
