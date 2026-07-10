import { chromium } from "playwright";

const url = process.argv[2] ?? "http://127.0.0.1:6020/";
const browser = await chromium.launch();
const page = await browser.newPage();
const logs: string[] = [];
page.on("console", (msg) => logs.push(`[${msg.type()}] ${msg.text()}`));
page.on("pageerror", (err) => logs.push(`[pageerror] ${err.message}\n${err.stack ?? ""}`));
await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 });
await page.waitForTimeout(20_000);
const state = await page.evaluate(() => ({
  title: document.title,
  rootLen: document.getElementById("root")?.innerHTML.length ?? 0,
  hasAppName: Boolean(document.querySelector('[data-slot="app-name"]')),
}));
console.log(JSON.stringify({ url, state, logs: logs.filter((l) => !l.includes("[vite]")) }, null, 2));
await browser.close();
