import { chromium } from "@playwright/test";

async function main() {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
  const page = await context.newPage();

  await page.goto("http://127.0.0.1:6006/iframe.html?id=elements-aggregation-tree--panel&viewMode=story");
  await page.waitForTimeout(3000);

  const data = await page.evaluate(() => {
    const results: any[] = [];

    // For each tree item row, find the indentation lines and chevron within it
    document.querySelectorAll('[data-slot="tree-item-row"], [data-slot="tree-section-row"]').forEach((row) => {
      const rowRect = row.getBoundingClientRect();
      const style = window.getComputedStyle(row);
      const slot = row.getAttribute("data-slot");
      const label = row.querySelector('[data-slot="tree-label"]')?.textContent || "?";

      // Find chevron (first svg within a button, or first svg if no button)
      const button = row.querySelector("button");
      const chevronSvg = button?.querySelector("svg");
      const iconSvg = row.querySelector(":scope > span > svg") || row.querySelector(":scope > span svg");

      let chevronCenterX = null;
      let chevronOffsetFromRow = null;
      if (chevronSvg) {
        const chevronRect = chevronSvg.getBoundingClientRect();
        chevronCenterX = chevronRect.x + chevronRect.width / 2;
        chevronOffsetFromRow = chevronCenterX - rowRect.x;
      }

      let iconCenterX = null;
      let iconOffsetFromRow = null;
      if (iconSvg && iconSvg !== chevronSvg) {
        const iconRect = iconSvg.getBoundingClientRect();
        iconCenterX = iconRect.x + iconRect.width / 2;
        iconOffsetFromRow = iconCenterX - rowRect.x;
      }

      // Find indentation lines within this row
      const lineContainer = row.querySelector(".pointer-events-none");
      const lines: any[] = [];
      if (lineContainer) {
        lineContainer.querySelectorAll(".bg-muted-foreground\\/40").forEach((line) => {
          const lineRect = line.getBoundingClientRect();
          if (lineRect.height > 0) {
            lines.push({
              x: lineRect.x,
              offsetFromRow: lineRect.x - rowRect.x,
              width: lineRect.width,
            });
          }
        });
      }

      results.push({
        slot,
        label,
        paddingLeft: style.paddingLeft,
        rowX: rowRect.x,
        chevronCenterX,
        chevronOffsetFromRow,
        iconCenterX,
        iconOffsetFromRow,
        lines,
      });
    });

    return results;
  });

  console.log("[DEBUG] === STORYBOOK TREE ALIGNMENT ANALYSIS ===");
  for (const item of data) {
    console.log(`\n[DEBUG] ${item.slot} "${item.label}" (paddingLeft=${item.paddingLeft})`);
    console.log(`  rowX=${item.rowX.toFixed(1)}`);
    if (item.chevronCenterX) {
      console.log(`  chevron centerX=${item.chevronCenterX.toFixed(1)} (offset from row: ${item.chevronOffsetFromRow.toFixed(1)})`);
    }
    if (item.iconCenterX) {
      console.log(`  icon centerX=${item.iconCenterX.toFixed(1)} (offset from row: ${item.iconOffsetFromRow.toFixed(1)})`);
    }
    if (item.lines.length > 0) {
      for (const line of item.lines) {
        console.log(`  line x=${line.x.toFixed(1)} (offset from row: ${line.offsetFromRow.toFixed(1)})`);
      }
    }
  }

  // Also compute what the formula EXPECTS
  console.log("\n[DEBUG] === EXPECTED VALUES ===");
  const detailPanelIndentPx = (level: number): number => (level === 0 ? 0 : 5 + 5 * level);
  const indentationLinePx = (i: number): number => detailPanelIndentPx(i) + 7;
  for (let level = 0; level <= 4; level++) {
    console.log(`level ${level}: paddingLeft=${detailPanelIndentPx(level)}, chevronCenter=${detailPanelIndentPx(level) + 7}`);
    for (let i = 0; i < level; i++) {
      console.log(`  line[${i}] expected offset from row: ${indentationLinePx(i)}`);
    }
  }

  await browser.close();
}

main().catch(console.error);
