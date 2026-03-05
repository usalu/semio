import { chromium } from "playwright";
async function run() {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ viewport: { width: 800, height: 1200 } });
  await page.goto("http://localhost:6006/iframe.html?id=elements-aggregation-tree--default&viewMode=story");
  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(2000);
  const data = await page.evaluate(() => {
    const rows = document.querySelectorAll('[data-slot="tree-section-row"], [data-slot="tree-item-row"]');
    const result: any[] = [];
    rows.forEach(row => {
      const rowRect = row.getBoundingClientRect();
      const slot = row.getAttribute("data-slot");
      const label = row.querySelector('[data-slot="tree-label"]')?.textContent?.trim();
      const style = window.getComputedStyle(row);
      const paddingLeft = parseFloat(style.paddingLeft);
      const chevronSvg = row.querySelector('svg');
      let chevronInfo: any = null;
      if (chevronSvg) {
        const svgRect = chevronSvg.getBoundingClientRect();
        const parent = chevronSvg.parentElement;
        const parentTag = parent?.tagName;
        const parentRect = parent?.getBoundingClientRect();
        chevronInfo = {
          svgLeft: svgRect.left - rowRect.left,
          svgWidth: svgRect.width,
          svgCenter: svgRect.left - rowRect.left + svgRect.width / 2,
          parentTag,
          parentLeft: parentRect ? parentRect.left - rowRect.left : null,
          parentWidth: parentRect ? parentRect.width : null,
          parentCenter: parentRect ? parentRect.left - rowRect.left + parentRect.width / 2 : null,
        };
      }
      const spacerDiv = Array.from(row.children).find(c => {
        if (c.tagName !== 'DIV') return false;
        const cs = window.getComputedStyle(c);
        return cs.position !== 'absolute' && cs.width === '14px' && cs.flexShrink === '0';
      });
      let spacerInfo: any = null;
      if (spacerDiv) {
        const sr = spacerDiv.getBoundingClientRect();
        spacerInfo = {
          left: sr.left - rowRect.left,
          width: sr.width,
          center: sr.left - rowRect.left + sr.width / 2,
        };
      }
      const iconSpan = row.querySelector('span.flex.items-center.justify-center.flex-shrink-0');
      let iconInfo: any = null;
      if (iconSpan) {
        const ir = iconSpan.getBoundingClientRect();
        iconInfo = {
          left: ir.left - rowRect.left,
          width: ir.width,
        };
      }
      const labelEl = row.querySelector('[data-slot="tree-label"]');
      let labelInfo: any = null;
      if (labelEl) {
        const lr = (labelEl as HTMLElement).getBoundingClientRect();
        labelInfo = {
          left: lr.left - rowRect.left,
        };
      }
      const linesContainer = row.querySelector('.absolute.left-0.top-0.bottom-0.pointer-events-none');
      const linePositions: number[] = [];
      if (linesContainer) {
        const lineEls = linesContainer.querySelectorAll('.absolute.top-0.bottom-0');
        lineEls.forEach(lineEl => {
          const lr = lineEl.getBoundingClientRect();
          linePositions.push(lr.left - rowRect.left);
        });
      }
      result.push({
        slot, label, paddingLeft, chevronInfo, spacerInfo, iconInfo, labelInfo, linePositions,
        rowLeft: rowRect.left, rowWidth: rowRect.width
      });
    });
    return result;
  });

  console.log("=== TREE ALIGNMENT ANALYSIS ===\n");
  for (const r of data) {
    const kind = r.slot === 'tree-section-row' ? 'Section' : 'Item';
    const isLeaf = r.slot === 'tree-item-row' && !r.chevronInfo;
    console.log(`${kind}${isLeaf ? ' (leaf)' : ''} [${r.label}] padL=${r.paddingLeft}`);
    if (r.chevronInfo) {
      console.log(`  Chevron: svgLeft=${r.chevronInfo.svgLeft.toFixed(1)} svgW=${r.chevronInfo.svgWidth.toFixed(1)} svgCenter=${r.chevronInfo.svgCenter.toFixed(1)} parentTag=${r.chevronInfo.parentTag} parentLeft=${r.chevronInfo.parentLeft?.toFixed(1)} parentW=${r.chevronInfo.parentWidth?.toFixed(1)}`);
    }
    if (r.spacerInfo) {
      console.log(`  Spacer: left=${r.spacerInfo.left.toFixed(1)} w=${r.spacerInfo.width.toFixed(1)} center=${r.spacerInfo.center.toFixed(1)}`);
    }
    if (r.iconInfo) {
      console.log(`  Icon: left=${r.iconInfo.left.toFixed(1)} w=${r.iconInfo.width.toFixed(1)}`);
    }
    if (r.labelInfo) {
      console.log(`  Label: left=${r.labelInfo.left.toFixed(1)}`);
    }
    if (r.linePositions.length > 0) {
      console.log(`  Lines: ${r.linePositions.map((l: number) => l.toFixed(1)).join(', ')}`);
    }
    console.log();
  }

  // Cross-check: for each row, do the lines align with ancestor chevron centers?
  console.log("=== ALIGNMENT CROSS-CHECK ===\n");
  const chevronCenters: Map<number, number> = new Map();
  for (const r of data) {
    if (r.chevronInfo) {
      chevronCenters.set(r.paddingLeft, r.chevronInfo.svgCenter);
    } else if (r.spacerInfo) {
      chevronCenters.set(r.paddingLeft, r.spacerInfo.center);
    }
  }
  console.log("Chevron/Spacer centers by padding level:");
  for (const [padL, center] of chevronCenters) {
    console.log(`  padL=${padL} -> center=${center.toFixed(1)}`);
  }

  console.log("\nLine vs ancestor center diffs:");
  for (const r of data) {
    if (r.linePositions.length > 0) {
      const diffs = r.linePositions.map((linePos: number, i: number) => {
        // Find which ancestor this line corresponds to
        const ancestorEntries = [...chevronCenters.entries()].sort((a, b) => a[0] - b[0]);
        if (i < ancestorEntries.length) {
          return { line: linePos, center: ancestorEntries[i][1], diff: linePos - ancestorEntries[i][1] };
        }
        return null;
      }).filter(Boolean);
      console.log(`  [${r.label}] lines vs ancestors: ${diffs.map((d: any) => `line=${d.line.toFixed(1)} center=${d.center.toFixed(1)} diff=${d.diff.toFixed(1)}`).join(' | ')}`);
    }
  }

  await browser.close();
}
run().catch(console.error);
