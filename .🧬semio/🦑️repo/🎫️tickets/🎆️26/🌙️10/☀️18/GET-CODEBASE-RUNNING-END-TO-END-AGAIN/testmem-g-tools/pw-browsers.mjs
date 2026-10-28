// pw-browsers.mjs <count>: launch <count> headless chromium browsers with one context+page each, hold 6 s, close (RSS is sampled by tree-rss.py)
import { createRequire } from "node:module";
const { chromium } = createRequire("/Users/ueli/Documents/semio/package.json")("playwright");
const count = Number(process.argv[2] ?? 1);
const page = "data:text/html," + encodeURIComponent("<html><body>" + "<div>row</div>".repeat(2000) + "<canvas id=c width=800 height=600></canvas><script>const x=document.getElementById('c').getContext('2d');x.fillRect(0,0,800,600);</script></body></html>");
const browsers = await Promise.all(Array.from({ length: count }, () => chromium.launch({ headless: true })));
const pages = await Promise.all(browsers.map(async (browser) => { const context = await browser.newContext({ viewport: { width: 1440, height: 900 } }); const tab = await context.newPage(); await tab.goto(page); return tab; }));
await new Promise((done) => setTimeout(done, 6000));
await Promise.all(browsers.map((browser) => browser.close()));
console.log("done", pages.length);
