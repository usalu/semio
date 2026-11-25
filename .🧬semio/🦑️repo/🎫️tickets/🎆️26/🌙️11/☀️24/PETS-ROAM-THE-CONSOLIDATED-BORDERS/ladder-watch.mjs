/** 🪜️ Watches the dev page's home screen at the gear spec's window for a while at eight times the pace and says every footing and activity each pet was seen in, and whether a ladder or a rope was drawn. Usage: node ladder-watch.mjs [seconds] */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const seconds = Number(process.argv[2] ?? 120);
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.addInitScript(() => { const mark = () => document.documentElement?.setAttribute("data-pets-tempo", "8"); mark(); new MutationObserver(mark).observe(document, { childList: true }); });
await page.goto("http://127.0.0.1:6061/");
await page.waitForSelector("svg.pet", { timeout: 180000 });
await page.evaluate(() => {
  window.seen = {}; window.frames = 0; window.gear = new Set();
  const look = () => { window.frames += 1; for (const pet of document.querySelectorAll("svg.pet")) { const id = pet.getAttribute("data-pet"); (window.seen[id] ??= new Set()).add(pet.getAttribute("data-pet-activity") + "/" + pet.getAttribute("data-pet-footing")); } for (const extra of document.querySelectorAll(".pet-layer > :not(svg.pet)")) window.gear.add(extra.tagName + "." + (extra.getAttribute("class") ?? "") + (extra.getAttribute("data-pet-gear") ?? "")); requestAnimationFrame(look); };
  requestAnimationFrame(look);
});
await page.waitForTimeout(seconds * 1000);
console.log(JSON.stringify(await page.evaluate(() => ({ frames: window.frames, gear: [...window.gear], seen: Object.fromEntries(Object.entries(window.seen).map(([id, set]) => [id, [...set].join(" ")])) })), null, 1));
await browser.close();
