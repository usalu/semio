/** 🎯️ C10 hit probe: bounding box of a selector and what element actually receives a click at its centre. */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const url = process.argv[2], selector = process.argv[3];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
await page.waitForTimeout(12000);
const info = await page.evaluate((sel) => {
  const el = document.querySelector(sel);
  if (!el) return { found: false };
  const r = el.getBoundingClientRect();
  const hit = document.elementFromPoint(r.x + r.width / 2, r.y + r.height / 2);
  const chain = [];
  for (let n = el; n && chain.length < 12; n = n.parentElement) { const cs = getComputedStyle(n); chain.push(`${n.tagName}#${n.id}.${(n.getAttribute("data-slot") ?? "")} h=${n.getBoundingClientRect().height.toFixed(0)} top=${n.getBoundingClientRect().top.toFixed(0)} ov=${cs.overflow} disp=${cs.display} vis=${cs.visibility} op=${cs.opacity} scrollTop=${n.scrollTop}`); }
  return { found: true, rect: [r.x, r.y, r.width, r.height], hit: hit ? `${hit.tagName}#${hit.id} slot=${hit.getAttribute("data-slot")} text=${(hit.textContent ?? "").slice(0, 40)}` : null, chain };
}, selector);
console.log(JSON.stringify(info, null, 1));
await browser.close();
