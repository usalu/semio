import { createRequire } from "node:module";
const require = createRequire(process.cwd() + "/package.json");
const { chromium } = require("playwright");
const [url, waitSeconds = "240"] = process.argv.slice(2);
const browser = await chromium.launch({ args: ["--js-flags=--max-old-space-size=8192", "--enable-unsafe-swiftshader", "--in-process-gpu", "--disable-background-timer-throttling", "--disable-renderer-backgrounding", "--disable-backgrounding-occluded-windows", "--disable-features=IntensiveWakeUpThrottling"] });
const page = await browser.newPage();
const lines = [];
page.on("console", m => { const t = m.text(); if (/intake|DEBUG\] ui|boot failed|panic|refus|Error|in flight|turn step|semantic failure|authority failure|turn step|self-wake|exchange_generic|advance_step|gate |failed-decoded|FAILED|displaced|surface capture|metadata advance|cleanup job/i.test(t) && !/stream-mux|gate ingress\.|turn phase|more-work inputs|manifest advance_step|command decoded output/.test(t)) lines.push(`[${m.type()}] ${t.slice(0, 1500)}`); });
page.on("crash", () => lines.push("[crash] renderer crashed")); page.on("close", () => lines.push("[close] page closed")); page.on("pageerror", e => lines.push(`[pageerror] ${String(e).slice(0, 800)}`));
await page.goto(url, { waitUntil: "commit", timeout: 120000 });
const deadline = Date.now() + Number(waitSeconds) * 1000;
let text = "";
while (Date.now() < deadline && !page.isClosed()) { await page.waitForTimeout(10000); text = await page.evaluate(() => document.body?.innerText.slice(0, 3000) ?? "").catch(() => text); if (/boot failed/i.test(lines.join("\n")) || (/Compliant ETICS/.test(text) && /U-value|Uₘ|Feuchte|kWh|W\/\(m²K\)|check|Nachweis|pass|fail/i.test(text.replace(/semio · DIN 4108[\s\S]*?Viewer/, '')))) break; }
console.log("PAGE TEXT:\n" + text + "\n---CONSOLE---\n" + lines.slice(-20000).join("\n"));
await page.screenshot({ path: process.env.SHOT ?? "/dev/null" }).catch(e => console.log("screenshot failed", String(e).slice(0, 200)));
await browser.close();
