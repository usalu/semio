import { chromium } from "playwright";
/** 🌐️ Lists the extension and install related requests of the 6018 page. */
const b = await chromium.launch({ headless: true });
const p = await b.newPage();
p.on("response", async r => { const u = decodeURIComponent(r.url()); if (/extension-modules|flow-extension/.test(u)) console.log(r.status(), u.slice(0, 200), false ? (await r.text().catch(() => "")).slice(0, 300) : ""); });
await p.goto("http://127.0.0.1:6018/?plugin=generation3d"); await p.waitForTimeout(20000); await b.close();
