import { chromium } from "@playwright/test";

async function measureTreeAlignment(page: any, label: string) {
  console.log(`\n[DEBUG] === ${label} ===`);

  const data = await page.evaluate(() => {
    const results: any = { sections: [], items: [], lines: [], chevrons: [] };

    // Measure tree-section-row elements
    document.querySelectorAll('[data-slot="tree-section-row"]').forEach(el => {
      const rect = el.getBoundingClientRect();
      const style = window.getComputedStyle(el);
      const labelEl = el.querySelector('[data-slot="tree-label"]');
      const chevron = el.querySelector('svg');
      results.sections.push({
        label: labelEl?.textContent || '?',
        paddingLeft: style.paddingLeft,
        rectX: rect.x,
        rectY: rect.y,
        chevronX: chevron ? chevron.getBoundingClientRect().x : null,
        chevronCenterX: chevron ? chevron.getBoundingClientRect().x + chevron.getBoundingClientRect().width / 2 : null,
      });
    });

    // Measure tree-item-row elements  
    document.querySelectorAll('[data-slot="tree-item-row"]').forEach(el => {
      const rect = el.getBoundingClientRect();
      const style = window.getComputedStyle(el);
      const labelEl = el.querySelector('[data-slot="tree-label"]');
      const chevron = el.querySelector('svg');
      results.items.push({
        label: labelEl?.textContent || '?',
        paddingLeft: style.paddingLeft,
        rectX: rect.x,
        rectY: rect.y,
        chevronX: chevron ? chevron.getBoundingClientRect().x : null,
        chevronCenterX: chevron ? chevron.getBoundingClientRect().x + chevron.getBoundingClientRect().width / 2 : null,
      });
    });

    // Measure vertical indentation lines
    document.querySelectorAll('.bg-muted-foreground\\/40').forEach(el => {
      const rect = el.getBoundingClientRect();
      if (rect.height > 0 && rect.width === 1) {
        const parentSlot = el.closest('[data-slot]');
        results.lines.push({
          x: rect.x,
          centerX: rect.x + 0.5,
          height: rect.height,
          parentSlot: parentSlot?.getAttribute('data-slot') || 'unknown',
          parentLabel: parentSlot?.querySelector('[data-slot="tree-label"]')?.textContent || '?',
        });
      }
    });

    return results;
  });

  console.log(`[DEBUG] Sections: ${data.sections.length}`);
  data.sections.forEach((s: any) => {
    console.log(`  Section "${s.label}": paddingLeft=${s.paddingLeft}, chevronCenterX=${s.chevronCenterX?.toFixed(1)}`);
  });

  console.log(`[DEBUG] Items: ${data.items.length}`);
  data.items.forEach((i: any) => {
    console.log(`  Item "${i.label}": paddingLeft=${i.paddingLeft}, rectX=${i.rectX.toFixed(1)}, chevronCenterX=${i.chevronCenterX?.toFixed(1)}`);
  });

  // Group lines by x position
  const linesByX = new Map<string, number>();
  data.lines.forEach((l: any) => {
    const key = l.x.toFixed(1);
    linesByX.set(key, (linesByX.get(key) || 0) + 1);
  });
  console.log(`[DEBUG] Line positions (unique x):`);
  for (const [x, count] of linesByX) {
    console.log(`  x=${x}: ${count} lines`);
  }

  // Check alignment: for each item, compare line positions with parent chevron positions
  const containerX = data.sections[0]?.rectX ?? data.items[0]?.rectX ?? 0;
  console.log(`[DEBUG] Container left edge: ${containerX.toFixed(1)}`);

  // All unique chevron center positions
  const chevronCenters = new Set<string>();
  data.sections.forEach((s: any) => {
    if (s.chevronCenterX) chevronCenters.add(s.chevronCenterX.toFixed(1));
  });
  data.items.forEach((i: any) => {
    if (i.chevronCenterX) chevronCenters.add(i.chevronCenterX.toFixed(1));
  });
  console.log(`[DEBUG] Unique chevron center X positions: ${[...chevronCenters].join(', ')}`);
  console.log(`[DEBUG] Unique line X positions: ${[...linesByX.keys()].join(', ')}`);

  // Check alignment
  const lineXSet = new Set(linesByX.keys());
  const misaligned = [...lineXSet].filter(x => !chevronCenters.has(x) && !chevronCenters.has((parseFloat(x) - 0.5).toFixed(1)) && !chevronCenters.has((parseFloat(x) + 0.5).toFixed(1)));
  if (misaligned.length > 0) {
    console.log(`[DEBUG] MISALIGNED lines at: ${misaligned.join(', ')}`);
  } else {
    console.log(`[DEBUG] All lines ALIGNED with chevron centers`);
  }

  return data;
}

async function main() {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });

  // 1. Check storybook tree story
  const sbPage = await context.newPage();
  await sbPage.goto("http://127.0.0.1:6006/iframe.html?id=elements-aggregation-tree--panel&viewMode=story");
  await sbPage.waitForTimeout(3000);
  const sbData = await measureTreeAlignment(sbPage, "STORYBOOK TREE (Panel)");
  await sbPage.screenshot({ path: "/workspaces/semio/.repo/tickets/2026/03/02/FIX-DETAIL-PANEL-LINE-ALIGNMENT-USING-PLAYWRIGHT/storybook-tree.png" });
  await sbPage.close();

  // 2. Check sketchpad (port 5173) - need to navigate to Kit or Design
  const skPage = await context.newPage();
  await skPage.goto("http://127.0.0.1:5173/");
  await skPage.waitForTimeout(5000);
  console.log("\n[DEBUG] Sketchpad URL:", skPage.url());

  // Check what panels exist
  const panelData = await skPage.evaluate(() => {
    const panels = document.querySelectorAll('[data-panel]');
    return Array.from(panels).map(p => ({
      panel: p.getAttribute('data-panel'),
      visible: (p as HTMLElement).style.display,
      childCount: p.children.length
    }));
  });
  console.log("[DEBUG] Panels found:", JSON.stringify(panelData));

  // Try to find tree elements in the sketchpad
  const treeCount = await skPage.evaluate(() => ({
    sections: document.querySelectorAll('[data-slot="tree-section-row"]').length,
    items: document.querySelectorAll('[data-slot="tree-item-row"]').length,
    contents: document.querySelectorAll('[data-slot="tree-content"]').length,
    lines: document.querySelectorAll('.bg-muted-foreground\\/40').length,
  }));
  console.log("[DEBUG] Sketchpad tree elements:", JSON.stringify(treeCount));

  if (treeCount.sections > 0 || treeCount.items > 0) {
    await measureTreeAlignment(skPage, "SKETCHPAD HOME");
  }

  await skPage.screenshot({ path: "/workspaces/semio/.repo/tickets/2026/03/02/FIX-DETAIL-PANEL-LINE-ALIGNMENT-USING-PLAYWRIGHT/sketchpad-home.png" });
  await skPage.close();

  await browser.close();
}

main().catch(console.error);
