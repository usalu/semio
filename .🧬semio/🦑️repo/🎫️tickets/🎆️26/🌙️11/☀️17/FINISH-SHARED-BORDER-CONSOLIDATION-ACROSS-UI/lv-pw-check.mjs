import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright-core/index.mjs";
const b = await chromium.launch({ executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome", headless: true });
const p = await b.newPage(); await p.goto("http://127.0.0.1:5492/"); console.log(await p.title()); await b.close();
