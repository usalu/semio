import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/.cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test/");
const { chromium } = require("@playwright/test");
const b = await chromium.launch(); const p = await (await b.newContext({ viewport: { width: 1280, height: 800 } })).newPage();
await p.goto("http://127.0.0.1:6068/"); await p.locator('[data-slot="introduction-close"]').waitFor(); await p.waitForTimeout(3000);
console.log(await p.evaluate(() => { const g = [...document.querySelectorAll("[data-window-silhouette-gap]")]; const t = performance.now(); g.forEach((x) => x.getBoundingClientRect()); const t2 = performance.now(); return { gaps: g.length, ms: t2 - t, nodes: document.querySelectorAll("*").length }; }));
await b.close();
