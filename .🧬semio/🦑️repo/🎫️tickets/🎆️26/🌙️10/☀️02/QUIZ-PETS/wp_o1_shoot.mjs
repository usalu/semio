/** 📸️ Ticket tool (work packages O): screenshots the pages of the sheets `wp_o1_stage_sheets.ts` wrote and measures, per row, how far the drawing of the species reaches around its feet (stroke included), against its declared size box.
 *
 * Usage (from the repository root): node "<ticket>/wp_o1_shoot.mjs" --dir <directory> [--themes light,dark] [--rows <substring>] <species id>...
 * Writes `<id>-<theme>-<page>.png` and `<id>-extent.txt` into the directory.
 */
import { readFileSync, writeFileSync } from "node:fs";
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
const directory = resolve(option("dir", "."));
const themes = option("themes", "light,dark").split(",");
const ids = args;

const browser = await chromium.launch();
try {
  for (const id of ids) {
    for (const theme of themes) {
      const tab = await browser.newPage({ viewport: { width: 1576, height: 1000 }, deviceScaleFactor: 1 });
      await tab.goto(pathToFileURL(join(directory, `${id}-${theme}.html`)).href);
      const pages = await tab.locator("section.page").count();
      for (let page = 0; page < pages; page++) await tab.locator(`section.page[data-page="${page}"]`).screenshot({ path: join(directory, `${id}-${theme}-${page}.png`) });
      if (theme === themes[0]) {
        const extents = await tab.evaluate(() => {
          const rows = [];
          for (const row of document.querySelectorAll(".row")) {
            let left = Infinity;
            let right = -Infinity;
            let top = Infinity;
            let bottom = -Infinity;
            for (const cell of row.querySelectorAll(".cell")) {
              const svg = cell.querySelector("svg");
              const target = cell.querySelector(".target");
              if (!svg || !target) continue;
              const unit = Number(cell.dataset.unit);
              const frame = svg.getBoundingClientRect();
              const ox = frame.left + Number(cell.dataset.ox) * unit;
              const oy = frame.top + Number(cell.dataset.oy) * unit;
              const facing = Number(target.dataset.facing);
              for (const shape of target.querySelectorAll("path, ellipse, rect, line, circle")) {
                const length = shape.getTotalLength();
                const matrix = shape.getScreenCTM();
                if (!matrix || !(length > 0)) continue;
                const stretch = Math.sqrt(Math.abs(matrix.a * matrix.d - matrix.b * matrix.c)) / unit;
                const half = shape.classList.contains("pet-stroke-none") || shape.classList.contains("pet-pupil") ? 0 : (Number(shape.getAttribute("stroke-width") ?? 0) / 2) * stretch;
                const steps = Math.max(8, Math.ceil(length / 0.6));
                for (let step = 0; step <= steps; step++) {
                  const point = shape.getPointAtLength((length * step) / steps).matrixTransform(matrix);
                  const across = ((point.x - ox) / unit) * facing;
                  const down = (point.y - oy) / unit;
                  left = Math.min(left, across - half);
                  right = Math.max(right, across + half);
                  top = Math.min(top, down - half);
                  bottom = Math.max(bottom, down + half);
                }
              }
            }
            rows.push({ row: row.dataset.row, left, right, top, bottom });
          }
          return rows;
        });
        const species = JSON.parse(readFileSync(join(directory, `${id}-size.json`), "utf8"));
        const lines = [`# ${id}: declared box x −${species.width / 2}…${species.width / 2}, y −${species.height}…0 (feet at 0; + = towards the face/right, below ground > 0)`];
        for (const row of extents) lines.push(`${row.row.padEnd(46)} x ${row.left.toFixed(1).padStart(6)} … ${row.right.toFixed(1).padStart(5)}   y ${row.top.toFixed(1).padStart(6)} … ${row.bottom.toFixed(1).padStart(5)}   over: left ${Math.max(0, -species.width / 2 - row.left).toFixed(1)} right ${Math.max(0, row.right - species.width / 2).toFixed(1)} top ${Math.max(0, -species.height - row.top).toFixed(1)} below ${Math.max(0, row.bottom).toFixed(1)}`);
        writeFileSync(join(directory, `${id}-extent.txt`), `${lines.join("\n")}\n`);
        console.log(lines.join("\n"));
      }
      await tab.close();
      console.log(`shot ${id} ${theme}: ${pages} pages`);
    }
  }
} finally {
  await browser.close();
}
