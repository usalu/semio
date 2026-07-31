import { chromium } from "playwright";
async function measure() {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  await page.goto("http://localhost:6006/iframe.html?id=elements-aggregation-tree--default&viewMode=story");
  await page.waitForLoadState("domcontentloaded");
  await page.waitForTimeout(5000);
  const measurements = await page.evaluate(() => {
    const results: any[] = [];
    const sectionRows = document.querySelectorAll('[data-slot="tree-section-row"]');
    sectionRows.forEach((row) => {
      const rowRect = row.getBoundingClientRect();
      const svgs = row.querySelectorAll("svg");
      const label = row.querySelector('[data-slot="tree-label"]');
      const chevronSvg = Array.from(svgs).find((svg) => {
        const cls = svg.getAttribute("class") || "";
        return cls.includes("lucide-chevron") || cls.includes("size-");
      });
      if (chevronSvg) {
        const chevRect = chevronSvg.getBoundingClientRect();
        results.push({
          type: "TreeSection",
          label: label?.textContent?.trim(),
          rowLeft: rowRect.left,
          paddingLeft: window.getComputedStyle(row).paddingLeft,
          chevronLeft: chevRect.left - rowRect.left,
          chevronWidth: chevRect.width,
          chevronCenter: chevRect.left + chevRect.width / 2 - rowRect.left,
        });
      } else {
        results.push({
          type: "TreeSection(noChevron)",
          label: label?.textContent?.trim(),
          rowLeft: rowRect.left,
          paddingLeft: window.getComputedStyle(row).paddingLeft,
        });
      }
    });
    const itemRows = document.querySelectorAll('[data-slot="tree-item-row"]');
    itemRows.forEach((row) => {
      const rowRect = row.getBoundingClientRect();
      const buttons = row.querySelectorAll("button");
      const label = row.querySelector('[data-slot="tree-label"]');
      let chevronInfo: any = null;
      buttons.forEach((btn) => {
        const svg = btn.querySelector("svg");
        if (svg) {
          const btnRect = btn.getBoundingClientRect();
          const svgRect = svg.getBoundingClientRect();
          const cs = window.getComputedStyle(btn);
          chevronInfo = {
            btnRelLeft: btnRect.left - rowRect.left,
            btnWidth: btnRect.width,
            svgRelLeft: svgRect.left - rowRect.left,
            svgWidth: svgRect.width,
            svgCenter: svgRect.left + svgRect.width / 2 - rowRect.left,
            btnPadLeft: cs.paddingLeft,
            btnBorderLeft: cs.borderLeftWidth,
            btnMarginLeft: cs.marginLeft,
            btnDisplay: cs.display,
            btnBoxSizing: cs.boxSizing,
          };
        }
      });
      results.push({
        type: chevronInfo ? "TreeItem(folder)" : "TreeItem(leaf)",
        label: label?.textContent?.trim(),
        rowLeft: rowRect.left,
        paddingLeft: window.getComputedStyle(row).paddingLeft,
        ...(chevronInfo || {}),
      });
    });
    const allRows = document.querySelectorAll('[data-slot="tree-section-row"], [data-slot="tree-item-row"]');
    const lineData: any[] = [];
    allRows.forEach((row) => {
      const rowRect = row.getBoundingClientRect();
      const label = row.querySelector('[data-slot="tree-label"]')?.textContent?.trim();
      const lineContainer = row.querySelector(".absolute.left-0.top-0.bottom-0");
      if (lineContainer) {
        const linePositioners = lineContainer.querySelectorAll(".absolute.top-0.bottom-0");
        linePositioners.forEach((lp) => {
          const lpRect = lp.getBoundingClientRect();
          const lineDiv = lp.querySelector(".w-px");
          if (lineDiv) {
            lineData.push({
              rowLabel: label,
              lineRelLeft: lpRect.left - rowRect.left,
              style: lp.getAttribute("style"),
              visible: lineDiv.getBoundingClientRect().width > 0,
            });
          }
        });
      }
    });
    return { rows: results, lines: lineData };
  });
  console.log("=== ROWS ===");
  for (const r of measurements.rows) {
    if (r.type === "TreeSection") {
      console.log(`${r.type} [${r.label}] padL=${r.paddingLeft} chevCenter=${r.chevronCenter?.toFixed(2)} chevW=${r.chevronWidth?.toFixed(1)}`);
    } else if (r.type === "TreeItem(folder)") {
      console.log(
        `${r.type} [${r.label}] padL=${r.paddingLeft} svgCenter=${r.svgCenter?.toFixed(2)} svgW=${r.svgWidth?.toFixed(1)} btnRelLeft=${r.btnRelLeft?.toFixed(2)} btnW=${r.btnWidth?.toFixed(1)} btnPadL=${r.btnPadLeft} btnBorderL=${r.btnBorderLeft} btnDisplay=${r.btnDisplay}`,
      );
    } else {
      console.log(`${r.type} [${r.label}] padL=${r.paddingLeft}`);
    }
  }
  console.log("\n=== LINES ===");
  for (const l of measurements.lines) {
    console.log(`Row[${l.rowLabel}] line relLeft=${l.lineRelLeft?.toFixed(2)} style="${l.style}" visible=${l.visible}`);
  }
  console.log("\n=== ALIGNMENT: chevron center vs indentation lines ===");
  const folderRows = measurements.rows.filter((r: any) => r.svgCenter != null || r.chevronCenter != null);
  for (const r of folderRows) {
    const center = r.chevronCenter ?? r.svgCenter;
    const padLeft = parseFloat(r.paddingLeft);
    console.log(`[${r.label}] (${r.type}): center=${center?.toFixed(2)} expectedLineAt(padL+7)=${(padLeft + 7).toFixed(2)} diff=${(center - (padLeft + 7)).toFixed(2)}`);
  }
  await page.screenshot({ path: "/tmp/tree-alignment.png", fullPage: true });
  console.log("\nScreenshot saved to /tmp/tree-alignment.png");
  await browser.close();
}
measure().catch(console.error);
