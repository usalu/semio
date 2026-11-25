/** 📐️ Prints the boxes from the utility bar toggle up to the document root. */
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } })).newPage();
await page.routeWebSocket(address => address.searchParams.has("token"), () => undefined);
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 90000 });
await page.waitForTimeout(4000);
console.log(await page.evaluate(() => { const out = []; for (let e = document.querySelector('[id$=".utilityBar.unfold"]'); e; e = e.parentElement) { const r = e.getBoundingClientRect(), s = getComputedStyle(e); out.push(`${e.tagName.toLowerCase()} ${e.id || ""} slot=${e.dataset.slot ?? ""} ${Math.round(r.x)},${Math.round(r.y)} ${Math.round(r.width)}x${Math.round(r.height)} ${s.position} top=${s.top} bottom=${s.bottom} h=${e.style.height} st=${e.scrollTop} ${String(e.className).slice(0, 110)}`); } return out.join("\n"); }));
await browser.close();
