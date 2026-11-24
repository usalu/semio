/** 🔬️ Reads the swiped cell of the dev page at a very short phone window: its box, its paddings and what inside it can scroll. Usage: node swipe-probe.mjs */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 375, height: 200 }, hasTouch: true, isMobile: true });
await page.goto("http://127.0.0.1:6061/");
await page.waitForSelector("[data-layered-overview]", { timeout: 180000 });
await page.waitForTimeout(4000);
console.log(JSON.stringify(await page.evaluate(() => {
  const r = (e) => { const b = e.getBoundingClientRect(); return [b.x, b.y, b.width, b.height].map(Math.round).join(","); };
  const root = document.querySelector("[data-course-grid][data-grid-active] > [data-layered-overview]");
  const cells = [...root.querySelectorAll(":scope > [data-layered-overlay] > [data-layered-cell]")];
  const cell = cells.find((c) => { const b = c.getBoundingClientRect(); return b.x >= 0 && b.x < 100 && b.y >= 0 && b.y < 200; }) ?? cells[0];
  const style = getComputedStyle(cell);
  const scrollers = [cell, ...cell.querySelectorAll("*")].filter((n) => /auto|scroll/.test(getComputedStyle(n).overflowY)).map((n) => String(n.className).slice(0, 40) + "|" + r(n) + "|sh=" + n.scrollHeight + " ch=" + n.clientHeight);
  return { mode: root.dataset.mode, root: r(root), cell: cell.dataset.layeredCell + "|" + r(cell) + "|pt=" + style.paddingTop + " pb=" + style.paddingBottom + " ov=" + style.overflowY, card: r(cell.querySelector("[data-overview-card]")), scrollers, picker: r(document.querySelector(".course-page-picker")), header: r(document.querySelector(".course-app > header")), footer: r(document.querySelector(".course-app > footer")), bands: document.querySelector(".course-app").getAttribute("style") };
}), null, 1));
await browser.close();
