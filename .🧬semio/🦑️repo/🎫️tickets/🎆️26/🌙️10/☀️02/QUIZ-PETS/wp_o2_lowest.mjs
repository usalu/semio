/** 🔎️ Ticket tool (work package O2): names, per section of a sheet written by `wp_o2_stage_sheet.ts`, the shape that reaches lowest below the feet and the one that reaches farthest beyond the box of its species.
 *
 * Usage (from the repository root): node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_o2_lowest.mjs" <sheet.html> [title filter]
 */
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { chromium } from "playwright";

const [file, filter = ""] = process.argv.slice(2);
const browser = await chromium.launch();
try {
  const tab = await browser.newPage({ viewport: { width: 2400, height: 1200 } });
  await tab.goto(pathToFileURL(resolve(file)).href);
  const lines = await tab.evaluate((wanted) => {
    const out = [];
    for (const section of document.querySelectorAll("section.sheet.light")) {
      if (!section.dataset.title.includes(wanted)) continue;
      let low = { value: -Infinity, what: "" };
      let wide = { value: -Infinity, what: "" };
      for (const cell of section.querySelectorAll(".cell")) {
        const origin = cell.getBoundingClientRect();
        const zoom = Number(cell.dataset.zoom);
        const feetX = origin.left + Number(cell.dataset.feetX);
        const feetY = origin.top + Number(cell.dataset.feetY);
        const half = Number(cell.dataset.width) / 2;
        const groups = [...cell.querySelectorAll("svg.pet > g")];
        for (const shape of cell.querySelectorAll("svg.pet path, svg.pet ellipse, svg.pet rect, svg.pet line, svg.pet circle")) {
          const box = shape.getBoundingClientRect();
          if (box.width === 0 && box.height === 0) continue;
          const style = getComputedStyle(shape);
          const stroke = style.stroke === "none" ? 0 : (parseFloat(style.strokeWidth) || 0) / 2;
          if (box.width * box.height === 0 && style.stroke === "none") continue;
          const what = `${cell.querySelector("figcaption").textContent} · group ${groups.indexOf(shape.closest("svg.pet > g"))} <${shape.tagName} ${shape.getAttribute("class")}> ${shape.getAttribute("d") ?? ""}`.slice(0, 150);
          const bottom = (box.bottom - feetY) / zoom + stroke;
          if (bottom > low.value) low = { value: bottom, what };
          const reach = Math.max((box.right - feetX) / zoom + stroke - half, -half - ((box.left - feetX) / zoom - stroke));
          if (reach > wide.value) wide = { value: reach, what };
        }
      }
      out.push(`${section.dataset.title}\n   lowest ${low.value.toFixed(2)}: ${low.what}\n   widest ${wide.value.toFixed(2)}: ${wide.what}`);
    }
    return out;
  }, filter);
  console.log(lines.join("\n"));
} finally {
  await browser.close();
}
