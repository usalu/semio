/** 📜️ Reads on the dev page, on a phone as low as the card scroll spec makes it, what the cells of the swipe overview can scroll. Usage: node scroll-probe.mjs [width] [height] */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: Number(process.argv[2] ?? 375), height: Number(process.argv[3] ?? 200) } });
await page.goto("http://127.0.0.1:6061/");
await page.waitForSelector("[data-layered-overview] [data-layered-cell]", { timeout: 180000 });
await page.waitForTimeout(2500);
console.log(JSON.stringify(await page.evaluate(() => {
  const box = (e) => { const r = e.getBoundingClientRect(); return [r.x, r.y, r.width, r.height].map(Math.round).join(","); };
  const attrs = (e) => e.tagName.toLowerCase() + [...e.attributes].filter((a) => a.name.startsWith("data-")).map((a) => `[${a.name}${a.value ? "=" + a.value.slice(0, 16) : ""}]`).join("");
  const chain = []; for (let e = document.querySelector('[data-layered-cell="badges"]'); e && e !== document.body; e = e.parentElement) chain.push(box(e) + " " + attrs(e).slice(0, 200) + " ov=" + getComputedStyle(e).overflowY + " sh=" + e.scrollHeight + " ch=" + e.clientHeight);
  return { mode: document.querySelector("[data-layered-overview]")?.getAttribute("data-mode"), chain, cells: [...document.querySelectorAll("[data-layered-cell]")].map((e) => e.getAttribute("data-layered-cell") + " " + box(e) + " sh=" + e.scrollHeight + " ch=" + e.clientHeight + " ov=" + getComputedStyle(e).overflowY) };
}), null, 1));
await browser.close();
