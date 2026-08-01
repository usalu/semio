import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const logs = [];
page.on("console", (m) => {
  if (m.type() === "error" || m.text().includes("[DEBUG]") || m.text().includes("panic"))
    logs.push(`${m.type()}: ${m.text().slice(0, 400)}`);
});
page.on("pageerror", (e) => logs.push(`pageerror: ${String(e).slice(0, 400)}`));
await page.goto("http://127.0.0.1:6023/", { waitUntil: "domcontentloaded", timeout: 120000 });
await page.waitForSelector("canvas", { timeout: 120000 });
await page.waitForFunction(() => document.title.includes("Aggregator"), { timeout: 120000 });
for (let i = 0; i < 6; i++) {
  const skip = page.getByRole("button", { name: /Überspringen|Skip/i });
  if (await skip.count()) {
    await skip.first().click({ timeout: 5000 }).catch(() => {});
    await page.waitForTimeout(500);
  } else break;
}
await page.waitForTimeout(5000);
const text = await page.evaluate(() => document.body?.innerText || "");
const info = {
  title: await page.title(),
  canvas: await page.locator("canvas").count(),
  hasAbbau: text.includes("Abbau Aufbau"),
  hasConcrete: text.includes("Concrete Forest"),
  mentions: [...text.matchAll(/Abbau|Aufbau|Beispiel|Forest|example/gi)].slice(0, 40).map((m) => m[0]),
  sample: text.slice(0, 1200),
  errorCount: logs.length,
  logs: logs.slice(0, 20),
};
writeFileSync(new URL("./probe-after-skip.json", import.meta.url), JSON.stringify(info, null, 2));
console.log(JSON.stringify(info, null, 2));
await page.screenshot({ path: new URL("./probe-after-skip.png", import.meta.url).pathname });
await browser.close();
