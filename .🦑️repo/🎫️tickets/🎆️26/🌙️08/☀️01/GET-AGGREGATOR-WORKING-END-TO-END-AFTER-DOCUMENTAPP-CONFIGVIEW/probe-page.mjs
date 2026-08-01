import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const logs = [];
page.on("console", (m) => {
  if (m.type() === "error" || m.text().includes("[DEBUG]")) logs.push(`${m.type()}: ${m.text().slice(0, 300)}`);
});
page.on("pageerror", (e) => logs.push(`pageerror: ${String(e).slice(0, 300)}`));
await page.goto("http://127.0.0.1:6023/", { waitUntil: "domcontentloaded", timeout: 120000 });
await page.waitForTimeout(10000);
const info = await page.evaluate(() => ({
  title: document.title,
  canvas: document.querySelectorAll("canvas").length,
  bodySample: (document.body?.innerText || "").slice(0, 800),
  hasAbbau: (document.body?.innerText || "").includes("Abbau Aufbau"),
}));
console.log(JSON.stringify({ info, logs: logs.slice(0, 40) }, null, 2));
await page.screenshot({ path: new URL("./probe.png", import.meta.url).pathname });
await browser.close();
