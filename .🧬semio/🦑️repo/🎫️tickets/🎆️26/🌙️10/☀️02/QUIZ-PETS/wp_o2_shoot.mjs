/** 📷️ Ticket tool (work package O2): screenshots the sheets `wp_o2_stage_sheet.ts` wrote — one PNG per group of sections — and measures, per section, how far the painted drawing reaches beyond the box of its species (in species pixels; stroke included).
 *
 * Usage (from the repository root):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_o2_shoot.mjs" <directory> [--themes light,dark] [--group 4] [pet ids…]
 */
import { readdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  if (index < 0) return fallback;
  const [, value] = args.splice(index, 2);
  return value;
};
const themes = option("themes", "light,dark").split(",");
const group = Number(option("group", "4"));
const directory = resolve(args.shift());
const wanted = args;

const browser = await chromium.launch();
try {
  const tab = await browser.newPage({ viewport: { width: 2400, height: 1200 }, deviceScaleFactor: 1 });
  for (const file of readdirSync(directory).filter((name) => name.endsWith(".html"))) {
    const id = file.replace(/\.html$/, "");
    if (wanted.length > 0 && !wanted.includes(id)) continue;
    await tab.goto(pathToFileURL(join(directory, file)).href);
    const report = await tab.evaluate(() => {
      const lines = [];
      for (const section of document.querySelectorAll("section.sheet.light")) {
        let left = 0;
        let right = 0;
        let top = 0;
        let bottom = -Infinity;
        for (const cell of section.querySelectorAll(".cell")) {
          const origin = cell.getBoundingClientRect();
          const zoom = Number(cell.dataset.zoom);
          const feetX = origin.left + Number(cell.dataset.feetX);
          const feetY = origin.top + Number(cell.dataset.feetY);
          const half = Number(cell.dataset.width) / 2;
          const tall = Number(cell.dataset.height);
          for (const shape of cell.querySelectorAll("svg.pet path, svg.pet ellipse, svg.pet rect, svg.pet line, svg.pet circle")) {
            const box = shape.getBoundingClientRect();
            if (box.width === 0 && box.height === 0) continue;
            const style = getComputedStyle(shape);
            const stroke = style.stroke === "none" ? 0 : (parseFloat(style.strokeWidth) || 0) / 2;
            const grown = { left: (box.left - feetX) / zoom - stroke, right: (box.right - feetX) / zoom + stroke, top: (box.top - feetY) / zoom - stroke, bottom: (box.bottom - feetY) / zoom + stroke };
            if (box.width * box.height === 0 && style.stroke === "none") continue;
            left = Math.max(left, -half - grown.left);
            right = Math.max(right, grown.right - half);
            top = Math.max(top, -tall - grown.top);
            bottom = Math.max(bottom, grown.bottom);
          }
        }
        lines.push(`${section.dataset.title.padEnd(96)} beyond box: left ${left.toFixed(1)} right ${right.toFixed(1)} top ${top.toFixed(1)} | lowest point ${bottom.toFixed(1)} below the feet`);
      }
      return lines.join("\n");
    });
    writeFileSync(join(directory, `${id}-reach.txt`), `${report}\n`);
    console.log(report);
    for (const theme of themes) {
      const names = await tab.evaluate((name) => [...document.querySelectorAll(`section.sheet.${name}`)].map((section) => section.dataset.name), theme);
      for (let start = 0; start < names.length; start += group) {
        const part = names.slice(start, start + group);
        const clip = await tab.evaluate((list) => {
          let left = Infinity;
          let top = Infinity;
          let right = -Infinity;
          let bottom = -Infinity;
          for (const name of list) {
            const box = document.querySelector(`section[data-name="${name}"]`).getBoundingClientRect();
            left = Math.min(left, box.left + scrollX);
            top = Math.min(top, box.top + scrollY);
            right = Math.max(right, box.right + scrollX);
            bottom = Math.max(bottom, box.bottom + scrollY);
          }
          return { x: left, y: top, width: right - left, height: bottom - top };
        }, part);
        const path = join(directory, `${id}-${theme}-${String(start / group).padStart(2, "0")}.png`);
        await tab.screenshot({ path, fullPage: true, clip });
        console.log(`wrote ${path} (${Math.round(clip.width)}×${Math.round(clip.height)})`);
      }
    }
  }
} finally {
  await browser.close();
}
