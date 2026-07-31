import { chromium } from "@playwright/test";

async function main() {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
  const page = await context.newPage();

  await page.goto("http://127.0.0.1:6006/iframe.html?id=elements-aggregation-tree--panel&viewMode=story");
  await page.waitForTimeout(6000);

  const data = await page.evaluate(() => {
    const sections = document.querySelectorAll('[data-slot="tree-section-row"]');
    const items = document.querySelectorAll('[data-slot="tree-item-row"]');

    const result: any = { sections: [], items: [] };

    sections.forEach((sec) => {
      const rect = sec.getBoundingClientRect();
      const svg = sec.querySelector(":scope > svg");
      const svgRect = svg?.getBoundingClientRect();
      const svgClass = svg?.getAttribute("class");
      const svgWidth = svg ? window.getComputedStyle(svg).width : null;
      result.sections.push({
        label: sec.querySelector('[data-slot="tree-label"]')?.textContent,
        svgClass,
        svgWidth,
        svgCenterOffset: svgRect ? svgRect.x + svgRect.width / 2 - rect.x : null,
      });
    });

    items.forEach((item) => {
      const rect = item.getBoundingClientRect();
      const button = item.querySelector("button");
      const svg = button?.querySelector("svg");
      const svgClass = svg?.getAttribute("class");
      const svgWidth = svg ? window.getComputedStyle(svg).width : null;
      const svgCenterOffset = svg ? svg.getBoundingClientRect().x + svg.getBoundingClientRect().width / 2 - rect.x : null;

      const lineContainer = item.querySelector(".pointer-events-none");
      const lines = lineContainer
        ? Array.from(lineContainer.children).map((child) => ({
            computedLeft: window.getComputedStyle(child).left,
            rectOffset: child.getBoundingClientRect().x - rect.x,
          }))
        : [];

      result.items.push({
        label: item.querySelector('[data-slot="tree-label"]')?.textContent,
        paddingLeft: window.getComputedStyle(item).paddingLeft,
        svgClass,
        svgWidth,
        svgCenterOffset,
        lines: lines.slice(0, 4),
      });
    });

    return result;
  });

  console.log("[DEBUG] Storybook after HMR:");
  for (const s of data.sections) {
    console.log(`  Section "${s.label}": svgClass=${s.svgClass}, svgWidth=${s.svgWidth}, center=${s.svgCenterOffset?.toFixed(1)}`);
  }
  for (const i of data.items) {
    const lineStr = i.lines.map((l: any) => `${l.computedLeft}(${l.rectOffset.toFixed(1)})`).join(", ");
    console.log(`  Item "${i.label}": pl=${i.paddingLeft}, svgClass="${i.svgClass}", svgWidth=${i.svgWidth}, center=${i.svgCenterOffset?.toFixed(1)}, lines=[${lineStr}]`);
  }

  await browser.close();
}

main().catch(console.error);
