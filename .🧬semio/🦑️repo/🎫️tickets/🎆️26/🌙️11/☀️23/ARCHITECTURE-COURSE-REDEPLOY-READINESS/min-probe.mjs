/** 📏️ Reads on the dev page, on a low phone, which boxes from the overview up to the app root ask for more height than the window leaves, and which rules give them a minimum. Usage: node min-probe.mjs [width] [height] */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: Number(process.argv[2] ?? 375), height: Number(process.argv[3] ?? 200) } });
await page.goto("http://127.0.0.1:6061/");
await page.waitForSelector("[data-layered-overview] [data-layered-cell]", { timeout: 180000 });
await page.waitForTimeout(2500);
console.log(JSON.stringify(await page.evaluate(() => {
  const out = [];
  for (let e = document.querySelector('[data-layered-cell="learner"]'); e && e !== document.documentElement; e = e.parentElement) {
    const s = getComputedStyle(e); const r = e.getBoundingClientRect();
    out.push([e.tagName.toLowerCase() + "." + String(e.className).slice(0, 50) + [...e.attributes].filter((a) => a.name.startsWith("data-")).map((a) => "[" + a.name + "]").join("").slice(0, 80), "h=" + Math.round(r.height), "min=" + s.minHeight, "height=" + s.height, "flex=" + s.flex, "display=" + s.display, "rows=" + s.gridTemplateRows.slice(0, 40), "inline=" + (e.getAttribute("style") ?? "").slice(0, 160)].join(" "));
  }
  const rules = [];
  for (const sheet of document.styleSheets) { let list; try { list = sheet.cssRules; } catch { continue; } const walk = (rs) => { for (const rule of rs) { if (rule.cssRules) walk(rule.cssRules); if (rule.style && (rule.style.minBlockSize || rule.style.minHeight) && /course|layered|overview/u.test(rule.selectorText ?? "")) rules.push(rule.selectorText.slice(0, 140) + " {min:" + (rule.style.minBlockSize || rule.style.minHeight) + "}"); } }; walk(list); }
  return { out, rules };
}), null, 1));
await browser.close();
