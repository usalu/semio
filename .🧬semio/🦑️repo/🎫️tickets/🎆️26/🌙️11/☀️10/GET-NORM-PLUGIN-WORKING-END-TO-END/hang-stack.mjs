import { createRequire } from "node:module";
const require = createRequire(process.cwd() + "/package.json");
const { chromium } = require("playwright");
const [url, after = "25"] = process.argv.slice(2);
const browser = await chromium.launch({ args: ["--disable-background-timer-throttling", "--disable-renderer-backgrounding"] });
const page = await browser.newPage();
const cdp = await page.context().newCDPSession(page);
await cdp.send("Debugger.enable");
let printed = 0;
cdp.on("Debugger.paused", async ev => { if (printed++ > 3) { await cdp.send("Debugger.resume").catch(() => {}); return; } console.log(`--- pause ${printed} reason ${ev.reason}`); for (const f of ev.callFrames.slice(0, 25)) console.log(`${f.functionName || "<anon>"} @ ${decodeURIComponent(f.url).replace(/^.*?(🧰️framework|🏢️semio-tech|node_modules|\.cache)/, "$1").slice(0, 160)}:${f.location.lineNumber + 1}`); await cdp.send("Debugger.resume").catch(() => {}); });
await page.goto(url, { waitUntil: "commit", timeout: 120000 });
for (let i = 0; i < 3; i++) { await new Promise(r => setTimeout(r, Number(after) * 1000)); await cdp.send("Debugger.pause").catch(e => console.log("pause failed", String(e).slice(0, 100))); await new Promise(r => setTimeout(r, 3000)); }
await browser.close();
