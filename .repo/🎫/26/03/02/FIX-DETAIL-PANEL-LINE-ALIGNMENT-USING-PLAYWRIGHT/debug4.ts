import { chromium } from "@playwright/test";

async function main() {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
  await page.goto("http://127.0.0.1:6006/iframe.html?id=elements-aggregation-tree--panel&viewMode=story");
  await page.waitForTimeout(3000);

  const data = await page.evaluate(() => {
    // Get the second tree-item-row (Types, level 1)
    const rows = document.querySelectorAll('[data-slot="tree-item-row"]');
    const typesRow = rows[0]; // Types (first tree item, level 1)
    if (!typesRow) return { error: "No types row" };

    const rowRect = typesRow.getBoundingClientRect();
    const rowStyle = window.getComputedStyle(typesRow);

    // Get the pointer-events-none container (IndentationLines outer)
    const lineContainer = typesRow.querySelector(".pointer-events-none");
    if (!lineContainer) return { error: "No line container" };

    const lcRect = lineContainer.getBoundingClientRect();
    const lcStyle = window.getComputedStyle(lineContainer);

    // Get ALL children of lineContainer
    const children = Array.from(lineContainer.children).map((child) => {
      const rect = child.getBoundingClientRect();
      const style = window.getComputedStyle(child);
      return {
        tag: child.tagName,
        classes: child.className,
        left: style.left,
        position: style.position,
        rectX: rect.x,
        rectWidth: rect.width,
        rectHeight: rect.height,
        offsetFromRow: rect.x - rowRect.x,
        children: Array.from(child.children).map((grandchild) => {
          const gcRect = grandchild.getBoundingClientRect();
          const gcStyle = window.getComputedStyle(grandchild);
          return {
            width: gcStyle.width,
            rectX: gcRect.x,
            rectWidth: gcRect.width,
            offsetFromRow: gcRect.x - rowRect.x,
          };
        }),
      };
    });

    // Get the button (chevron container)
    const button = typesRow.querySelector("button");
    const buttonRect = button?.getBoundingClientRect();
    const buttonStyle = button ? window.getComputedStyle(button) : null;
    const chevronSvg = button?.querySelector("svg");
    const svgRect = chevronSvg?.getBoundingClientRect();
    const svgStyle = chevronSvg ? window.getComputedStyle(chevronSvg) : null;

    // Also check TreeSection chevron (first section)
    const sections = document.querySelectorAll('[data-slot="tree-section-row"]');
    const kitSection = sections[0];
    const sectionChevron = kitSection?.querySelector("svg");
    const sectionChevronRect = sectionChevron?.getBoundingClientRect();
    const sectionChevronStyle = sectionChevron ? window.getComputedStyle(sectionChevron) : null;

    return {
      row: {
        paddingLeft: rowStyle.paddingLeft,
        position: rowStyle.position,
        rectX: rowRect.x,
        rectWidth: rowRect.width,
        boxSizing: rowStyle.boxSizing,
      },
      lineContainer: {
        classes: lineContainer.className,
        position: lcStyle.position,
        left: lcStyle.left,
        rectX: lcRect.x,
        rectWidth: lcRect.width,
        offsetFromRow: lcRect.x - rowRect.x,
        boxSizing: lcStyle.boxSizing,
      },
      lineChildren: children,
      button: button
        ? {
            padding: buttonStyle!.padding,
            margin: buttonStyle!.margin,
            border: buttonStyle!.border,
            boxSizing: buttonStyle!.boxSizing,
            display: buttonStyle!.display,
            width: buttonStyle!.width,
            rectX: buttonRect!.x,
            rectWidth: buttonRect!.width,
            offsetFromRow: buttonRect!.x - rowRect.x,
          }
        : null,
      chevronSvg: chevronSvg
        ? {
            width: svgStyle!.width,
            height: svgStyle!.height,
            classes: chevronSvg.getAttribute("class"),
            rectX: svgRect!.x,
            rectWidth: svgRect!.width,
            offsetFromRow: svgRect!.x - rowRect.x,
            centerFromRow: svgRect!.x + svgRect!.width / 2 - rowRect.x,
          }
        : null,
      sectionChevron: sectionChevron
        ? {
            width: sectionChevronStyle!.width,
            height: sectionChevronStyle!.height,
            classes: sectionChevron.getAttribute("class"),
            rectX: sectionChevronRect!.x,
            rectWidth: sectionChevronRect!.width,
          }
        : null,
    };
  });

  console.log(JSON.stringify(data, null, 2));

  await browser.close();
}

main().catch(console.error);
