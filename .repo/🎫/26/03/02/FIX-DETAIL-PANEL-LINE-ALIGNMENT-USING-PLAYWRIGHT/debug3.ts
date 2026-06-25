import { chromium } from "@playwright/test";
import path from "node:path";

async function measureTreeAlignment(page: any, label: string) {
  console.log(`\n[DEBUG] === ${label} ===`);

  const data = await page.evaluate(() => {
    const results: any[] = [];

    document.querySelectorAll('[data-slot="tree-item-row"], [data-slot="tree-section-row"]').forEach(row => {
      const rowRect = row.getBoundingClientRect();
      const style = window.getComputedStyle(row);
      const slot = row.getAttribute('data-slot');
      const label = row.querySelector('[data-slot="tree-label"]')?.textContent || '?';

      // Find ALL SVGs to identify chevrons specifically
      const allSvgs: any[] = [];
      row.querySelectorAll(':scope > svg, :scope > button > svg, :scope > div > svg, :scope > span > svg, :scope > span svg').forEach((svg: any) => {
        const rect = svg.getBoundingClientRect();
        const parent = svg.parentElement;
        allSvgs.push({
          tag: parent?.tagName,
          class: svg.getAttribute('class')?.substring(0, 40),
          x: rect.x,
          centerX: rect.x + rect.width / 2,
          width: rect.width,
          offsetFromRow: rect.x - rowRect.x,
          centerOffsetFromRow: rect.x + rect.width / 2 - rowRect.x,
        });
      });

      // Find the first non-absolute positioned SVG (the chevron for expandable items)
      const lineContainer = row.querySelector('.pointer-events-none');
      const lines: any[] = [];
      if (lineContainer) {
        lineContainer.querySelectorAll('div > div').forEach((line: any) => {
          const lineRect = line.getBoundingClientRect();
          if (lineRect.height > 0 && lineRect.width <= 2) {
            lines.push({
              offsetFromRow: lineRect.x - rowRect.x,
              centerOffsetFromRow: lineRect.x + lineRect.width / 2 - rowRect.x,
            });
          }
        });
      }

      results.push({
        slot,
        label,
        paddingLeft: style.paddingLeft,
        rowX: rowRect.x,
        rowWidth: rowRect.width,
        svgs: allSvgs,
        lines,
      });
    });

    // Also get tree-content elements
    const treeContents: any[] = [];
    document.querySelectorAll('[data-slot="tree-content"]').forEach(el => {
      const rect = el.getBoundingClientRect();
      const style = window.getComputedStyle(el);
      const lineContainer = el.querySelector('.pointer-events-none');
      const lines: any[] = [];
      if (lineContainer) {
        lineContainer.querySelectorAll('div > div').forEach((line: any) => {
          const lineRect = line.getBoundingClientRect();
          if (lineRect.height > 0 && lineRect.width <= 2) {
            lines.push({
              offsetFromContent: lineRect.x - rect.x,
            });
          }
        });
      }
      treeContents.push({
        paddingLeft: style.paddingLeft,
        x: rect.x,
        width: rect.width,
        lines,
      });
    });

    return { rows: results, treeContents };
  });

  for (const item of data.rows) {
    const lineInfo = item.lines.length > 0 ? ` | lines at offsets: ${item.lines.map((l: any) => l.offsetFromRow.toFixed(1)).join(', ')}` : '';
    const svgInfo = item.svgs.length > 0 ? ` | svgs: ${item.svgs.map((s: any) => `${s.tag}@${s.centerOffsetFromRow.toFixed(1)}(${s.width.toFixed(0)}px)`).join(', ')}` : '';
    console.log(`  ${item.slot} "${item.label}" pl=${item.paddingLeft}${svgInfo}${lineInfo}`);
  }

  if (data.treeContents.length > 0) {
    console.log(`  tree-content elements: ${data.treeContents.length}`);
    for (const tc of data.treeContents) {
      const lineInfo = tc.lines.length > 0 ? ` | lines: ${tc.lines.map((l: any) => l.offsetFromContent.toFixed(1)).join(', ')}` : '';
      console.log(`    pl=${tc.paddingLeft}${lineInfo}`);
    }
  }

  return data;
}

async function main() {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1400, height: 900 } });

  // 1. Check storybook (reference)
  const sbPage = await context.newPage();
  await sbPage.goto("http://127.0.0.1:6006/iframe.html?id=elements-aggregation-tree--panel&viewMode=story");
  await sbPage.waitForTimeout(3000);
  await measureTreeAlignment(sbPage, "STORYBOOK (Panel)");
  await sbPage.close();

  // 2. Check sketchpad - navigate to Kit (port 5173)
  const skPage = await context.newPage();
  await skPage.goto("http://127.0.0.1:5173/");
  await skPage.waitForTimeout(5000);

  // Load metabolism kit
  const kitPath = path.resolve("/workspaces/semio/assets/compose/kit_metabolism.zip");
  const [fileChooser] = await Promise.all([
    skPage.waitForEvent('filechooser', { timeout: 10000 }).catch(() => null),
    skPage.locator('role=button').filter({ hasText: /open|load|import/i }).first().click({ timeout: 5000 }).catch(() => null)
  ]);

  console.log("\n[DEBUG] File chooser:", fileChooser ? "found" : "not found");
  console.log("[DEBUG] Sketchpad URL after home:", skPage.url());

  // Check if we have any tree elements already visible
  const homeTreeCount = await skPage.locator('[data-slot="tree-section-row"]').count();
  console.log(`[DEBUG] Home tree sections: ${homeTreeCount}`);

  if (homeTreeCount > 0) {
    await measureTreeAlignment(skPage, "SKETCHPAD HOME");
  }

  // Try to take a screenshot of whatever we have
  await skPage.screenshot({ path: "/workspaces/semio/.repo/tickets/2026/03/02/FIX-DETAIL-PANEL-LINE-ALIGNMENT-USING-PLAYWRIGHT/sketchpad.png", fullPage: true });

  await skPage.close();

  // 3. Check play dev (port 4000)
  const playPage = await context.newPage();
  await playPage.goto("http://127.0.0.1:4000/");
  await playPage.waitForTimeout(5000);
  console.log("\n[DEBUG] Play URL:", playPage.url());

  const playTreeCount = await playPage.locator('[data-slot="tree-section-row"]').count();
  console.log(`[DEBUG] Play tree sections: ${playTreeCount}`);

  if (playTreeCount > 0) {
    await measureTreeAlignment(playPage, "PLAY");
  }

  await playPage.screenshot({ path: "/workspaces/semio/.repo/tickets/2026/03/02/FIX-DETAIL-PANEL-LINE-ALIGNMENT-USING-PLAYWRIGHT/play.png", fullPage: true });
  await playPage.close();

  await browser.close();
}

main().catch(console.error);
