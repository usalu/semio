/** 👀️ Watches the pets of the dev page's home screen for a while: every few seconds where each one stands and what its depiction says it does. Usage: node watch-probe.mjs [seconds] [tempo] */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const seconds = Number(process.argv[2] ?? 40);
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
await page.addInitScript((tempo) => { const mark = () => document.documentElement?.setAttribute("data-pets-tempo", tempo); mark(); new MutationObserver(mark).observe(document, { childList: true }); }, process.argv[3] ?? "8");
await page.goto("http://127.0.0.1:6061/");
await page.waitForSelector("svg.pet", { timeout: 180000 });
for (let passed = 0; passed <= seconds; passed += 5) {
  console.log(passed + "s " + (await page.evaluate(() => {
    const footer = Math.round(document.querySelector(".course-app > footer").getBoundingClientRect().top);
    return "footer=" + footer + " | " + [...document.querySelectorAll("svg.pet")].map((pet) => { const r = pet.getBoundingClientRect(); return [...pet.attributes].filter((a) => a.name.startsWith("data-") && a.value.length < 24).map((a) => a.name.slice(5) + "=" + a.value).join(",") + " @" + Math.round(r.x + r.width / 2) + "," + Math.round(r.bottom); }).join(" | ");
  })));
  await page.waitForTimeout(5000);
}
await browser.close();
