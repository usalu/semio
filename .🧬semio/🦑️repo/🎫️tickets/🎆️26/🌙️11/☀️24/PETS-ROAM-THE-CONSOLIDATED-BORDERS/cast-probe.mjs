/** 🎭️ Watches on the dev page how steady the cast of a page is: every second who is on stage, how often the row of deeds in the preferences was mounted anew, and what the survey yields. Usage: node cast-probe.mjs [hash] [seconds] [width] [height] */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const hash = process.argv[2] ?? "prefs";
const seconds = Number(process.argv[3] ?? 40);
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: Number(process.argv[4] ?? 1440), height: Number(process.argv[5] ?? 900) } });
await page.goto("http://127.0.0.1:6061/#" + hash);
await page.waitForSelector("svg.pet", { timeout: 180000 });
if (process.argv[6] === "double") await page.evaluate(() => { const root = document.documentElement; root.style.fontSize = `${Number.parseFloat(getComputedStyle(root).fontSize) * 2}px`; });
await page.evaluate(() => {
  window.mounts = 0; window.frames = 0;
  new MutationObserver((records) => { for (const record of records) for (const node of record.addedNodes) if (node instanceof Element && (node.matches("[data-pets-player]") || node.querySelector("[data-pets-player]"))) window.mounts += 1; }).observe(document.body, { childList: true, subtree: true });
  const tick = () => { window.frames += 1; requestAnimationFrame(tick); }; requestAnimationFrame(tick);
});
let last = "";
for (let passed = 0; passed <= seconds; passed += 1) {
  const now = await page.evaluate(() => [...document.querySelectorAll("svg.pet")].map((pet) => pet.getAttribute("data-pet")).join(",") + " | player=" + document.querySelectorAll("[data-pets-player]").length + " deeds=" + document.querySelectorAll("[data-pets-player] button").length + " mounts=" + window.mounts);
  if (now !== last) console.log(passed + "s " + now);
  last = now;
  await page.waitForTimeout(1000);
}
console.log("frames per second: " + Math.round((await page.evaluate(() => window.frames)) / seconds));
await browser.close();
