/** 🏷️ Replays on the dev page what the deeds spec does at 320 px with doubled text: it writes a long name into the first row of deeds and watches whether the cast keeps its pets. Usage: node rename-probe.mjs [width] [name] */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: Number(process.argv[2] ?? 320), height: 2400 } });
const name = process.argv[3] ?? "Tälchen, das Diagramm der minimierten Fläche";
await page.goto("http://127.0.0.1:6061/#prefs");
await page.waitForSelector("[data-pets-player]", { timeout: 180000 });
await page.evaluate(() => { const root = document.documentElement; root.style.fontSize = `${Number.parseFloat(getComputedStyle(root).fontSize) * 2}px`; });
const state = () => page.evaluate(() => { const footer = document.querySelector(".course-app > footer").getBoundingClientRect(); return [...document.querySelectorAll("svg.pet")].map((pet) => { const r = pet.getBoundingClientRect(); return pet.getAttribute("data-pet") + ":" + pet.getAttribute("data-pet-activity") + "/" + pet.getAttribute("data-pet-footing") + "@" + Math.round(r.x) + "," + Math.round(r.bottom); }).join(" ") + " | rows=" + [...document.querySelectorAll("[data-pets-player]")].map((row) => row.getAttribute("data-pets-player") + "@" + Math.round(row.getBoundingClientRect().top) + "h" + Math.round(row.getBoundingClientRect().height)).join(",") + " footer=" + Math.round(footer.top); });
let last = "";
const watch = async (seconds, label) => { for (let passed = 0; passed < seconds * 2; passed += 1) { const now = await state(); if (now !== last) console.log(label + " +" + passed / 2 + "s " + now); last = now; await page.waitForTimeout(500); } };
await watch(12, "settle");
for (let round = 1; round <= 3; round += 1) {
  const renamed = await page.evaluate((name) => { const label = document.querySelector("[data-pets-player] span[id]"); if (label === null) return false; label.textContent = name; return true; }, name);
  console.log("rename " + round + ": " + renamed);
  await watch(12, "after rename " + round);
}
await browser.close();
