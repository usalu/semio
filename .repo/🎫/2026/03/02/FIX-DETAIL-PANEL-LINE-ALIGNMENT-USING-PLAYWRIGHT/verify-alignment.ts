import { chromium } from "playwright";

async function main() {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ bypassCSP: true });
  await context.clearCookies();
  const page = await context.newPage();
  await page.route("**/*", (route) => {
    route.continue({ headers: { ...route.request().headers(), "cache-control": "no-cache, no-store" } });
  });

  const storybookUrl = "http://127.0.0.1:6006/iframe.html?id=elements-aggregation-tree--panel&viewMode=story";
  console.log("[DEBUG] Navigating to storybook...");
  await page.goto(storybookUrl, { waitUntil: "networkidle", timeout: 30000 });
  await page.waitForTimeout(2000);

  const storybookData = await page.evaluate(() => {
    const rows = document.querySelectorAll('[data-slot="tree-section-row"], [data-slot="tree-item-row"]');
    const results: any[] = [];
    rows.forEach((row, idx) => {
      const rect = row.getBoundingClientRect();
      const style = getComputedStyle(row);
      const paddingLeft = parseFloat(style.paddingLeft);
      const button = row.querySelector("button");
      const svg = button
        ? button.querySelector("svg")
        : row.querySelector("svg");
      let svgInfo: any = null;
      if (svg) {
        const svgRect = svg.getBoundingClientRect();
        const svgStyle = getComputedStyle(svg);
        svgInfo = {
          class: svg.className.baseVal,
          width: svgRect.width,
          height: svgRect.height,
          left: svgRect.left,
          centerX: svgRect.left + svgRect.width / 2,
          relCenterX: svgRect.left + svgRect.width / 2 - rect.left,
        };
      }
      let buttonInfo: any = null;
      if (button) {
        const btnRect = button.getBoundingClientRect();
        const btnStyle = getComputedStyle(button);
        buttonInfo = {
          width: btnRect.width,
          height: btnRect.height,
          left: btnRect.left,
          display: btnStyle.display,
          padding: btnStyle.padding,
          border: btnStyle.border,
          boxSizing: btnStyle.boxSizing,
        };
      }
      const lineContainer = row.querySelector(".absolute.left-0.top-0.bottom-0.pointer-events-none");
      const lines: { left: number; relLeft: number }[] = [];
      if (lineContainer) {
        lineContainer.querySelectorAll(":scope > div").forEach((lineDiv: any) => {
          const lineRect = lineDiv.getBoundingClientRect();
          const lineStyle = getComputedStyle(lineDiv);
          lines.push({
            left: parseFloat(lineStyle.left),
            relLeft: lineRect.left - rect.left,
          });
        });
      }
      const label = row.querySelector('[data-slot="tree-label"]');
      results.push({
        idx,
        slot: row.getAttribute("data-slot"),
        paddingLeft,
        hasButton: !!button,
        button: buttonInfo,
        svg: svgInfo,
        lines,
        label: label?.textContent?.trim()?.substring(0, 20),
      });
    });
    return results;
  });

  console.log("[DEBUG] Storybook alignment data:");
  for (const row of storybookData) {
    const chevronCenter = row.svg?.relCenterX?.toFixed(1);
    const linePositions = row.lines.map((l: any) => l.left.toFixed(1)).join(", ");
    console.log(
      `  Row ${row.idx} [${row.slot}] "${row.label}" | paddingLeft=${row.paddingLeft}px | hasButton=${row.hasButton} | svgClass="${row.svg?.class}" | svgWidth=${row.svg?.width?.toFixed(1)}px | chevronCenter=${chevronCenter}px | lines=[${linePositions}]`
    );
    if (row.button) {
      console.log(`    Button: display=${row.button.display} width=${row.button.width?.toFixed(1)} padding="${row.button.padding}" border="${row.button.border}" boxSizing=${row.button.boxSizing}`);
    }
    for (const line of row.lines) {
      const diff = row.svg ? (line.left - (row.svg.relCenterX - row.paddingLeft)).toFixed(1) : "N/A";
      console.log(`    Line left=${line.left}px | diff from chevronCenter=${diff}px`);
    }
  }

  await browser.close();
}

main().catch(console.error);
