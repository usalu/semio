import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
const outDir = dirname(fileURLToPath(import.meta.url));
const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const fails = [];
const consoles = [];
page.on("response", (r) => { if (r.status() >= 400) fails.push({ status: r.status(), url: r.url() }); });
page.on("console", (m) => consoles.push({ type: m.type(), text: m.text().slice(0, 500) }));
page.on("pageerror", (e) => consoles.push({ type: "pageerror", text: e.message.slice(0, 1000) }));
await page.goto("http://127.0.0.1:6018/", { waitUntil: "domcontentloaded", timeout: 120000 });
await page.waitForTimeout(15000);
const rootHtml = await page.locator("#root").evaluate((el) => el.innerHTML).catch(() => "<missing>");
const report = {
  title: await page.title(),
  rootHtml: String(rootHtml).slice(0, 3000),
  bodyText: (await page.locator("body").innerText()).slice(0, 2000),
  canvasCount: await page.locator("canvas").count(),
  fails: fails.slice(0, 50),
  consoles: consoles.slice(0, 50),
};
writeFileSync(outDir + "/e2e-probe.json", JSON.stringify(report, null, 2));
await page.screenshot({ path: outDir + "/e2e-probe.png", fullPage: false });
console.log(JSON.stringify(report, null, 2));
await browser.close();
