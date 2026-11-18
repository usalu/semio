import { createRequire } from "node:module";
const require = createRequire(process.cwd() + "/package.json");
const { chromium } = require("playwright");
const [url, waitSeconds = "240"] = process.argv.slice(2);
const browser = await chromium.launch({ args: ["--js-flags=--max-old-space-size=8192", "--enable-unsafe-swiftshader"] });
const page = await browser.newPage();
const lines = [];
page.on("console", m => { const t = m.text(); if (/DEBUG|boot failed|semantic|panic|refus|Error/i.test(t)) lines.push(`[${m.type()}] ${t.slice(0, 1500)}`); });
page.on("crash", () => lines.push("[crash] renderer crashed")); page.on("close", () => lines.push("[close] page closed")); page.on("pageerror", e => lines.push(`[pageerror] ${String(e).slice(0, 800)}`));
await page.goto(url, { waitUntil: "commit", timeout: 120000 });
const deadline = Date.now() + Number(waitSeconds) * 1000;
let text = "";
while (Date.now() < deadline && !page.isClosed()) { await page.waitForTimeout(10000); text = await page.evaluate(() => document.body?.innerText.slice(0, 600) ?? "").catch(() => text); if (/boot failed/i.test(lines.join("\n")) || (text.includes("DIN") && !text.includes("No plugins loaded") && text.length > 200)) break; }
console.log("PAGE TEXT:\n" + text + "\n---CONSOLE---\n" + lines.slice(-20).join("\n"));
await page.screenshot({ path: process.env.SHOT ?? "/dev/null" }).catch(e => console.log("screenshot failed", String(e).slice(0, 200)));
await browser.close();
