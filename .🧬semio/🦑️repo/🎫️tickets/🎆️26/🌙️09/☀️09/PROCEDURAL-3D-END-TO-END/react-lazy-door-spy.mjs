import { chromium } from "playwright";
const b = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const p = await (await b.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
await p.addInitScript(() => { let door; window.__asks = []; Object.defineProperty(window, "semioReactActivateOnDemand", { configurable: true, get: () => door && ((j) => { window.__asks.push(j); return door(j); }), set: (v) => { door = v; } }); });
await p.goto("http://127.0.0.1:6018/?plugin=generation3d");
await p.waitForTimeout(60000);
console.log("asks", JSON.stringify(await p.evaluate(() => window.__asks)));
await b.close();
