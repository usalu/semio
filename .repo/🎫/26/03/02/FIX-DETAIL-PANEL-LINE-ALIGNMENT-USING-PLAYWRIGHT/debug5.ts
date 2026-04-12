import { chromium } from "@playwright/test";

async function main() {
  const browser = await chromium.launch({ headless: true });
  // No cache
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 }, bypassCSP: true });
  const page = await context.newPage();
  
  // Force no-cache on storybook
  await page.route('**/*', route => route.continue({ headers: { ...route.request().headers(), 'Cache-Control': 'no-cache', 'Pragma': 'no-cache' } }));
  
  await page.goto("http://127.0.0.1:6006/iframe.html?id=elements-aggregation-tree--panel&viewMode=story&t=" + Date.now());
  await page.waitForTimeout(5000);
  
  const data = await page.evaluate(() => {
    const rows = document.querySelectorAll('[data-slot="tree-item-row"]');
    const typesRow = rows[0];
    if (!typesRow) return { error: "No types row found" };
    
    const rowRect = typesRow.getBoundingClientRect();
    const lineContainer = typesRow.querySelector('.pointer-events-none');
    const lc = lineContainer ? Array.from(lineContainer.children).map(child => {
      const style = window.getComputedStyle(child);
      return { left: style.left };
    }) : [];
    
    const button = typesRow.querySelector('button');
    const svg = button?.querySelector('svg');
    const svgClasses = svg?.getAttribute('class');
    const svgWidth = svg ? window.getComputedStyle(svg).width : null;
    
    const sections = document.querySelectorAll('[data-slot="tree-section-row"]');
    const sectionSvg = sections[0]?.querySelector('svg');
    const sectionSvgClasses = sectionSvg?.getAttribute('class');
    const sectionSvgWidth = sectionSvg ? window.getComputedStyle(sectionSvg).width : null;
    
    return {
      typesRow: {
        paddingLeft: window.getComputedStyle(typesRow).paddingLeft,
        lines: lc,
        svgClasses,
        svgWidth,
        sectionSvgClasses,
        sectionSvgWidth,
      },
    };
  });
  
  console.log("[DEBUG] After hard reload:", JSON.stringify(data, null, 2));
  
  // Now check port 5173 (sketchpad dev)
  const skPage = await context.newPage();
  await skPage.route('**/*', route => route.continue({ headers: { ...route.request().headers(), 'Cache-Control': 'no-cache', 'Pragma': 'no-cache' } }));
  await skPage.goto("http://127.0.0.1:5173/?t=" + Date.now());
  await skPage.waitForTimeout(5000);
  
  // Check if there are tree elements
  const skCount = await skPage.evaluate(() => ({
    sections: document.querySelectorAll('[data-slot="tree-section-row"]').length,
    items: document.querySelectorAll('[data-slot="tree-item-row"]').length,
  }));
  console.log("[DEBUG] Sketchpad tree elements:", JSON.stringify(skCount));
  
  await browser.close();
}

main().catch(console.error);
