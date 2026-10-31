import { chromium } from "playwright";

const urls = process.argv.slice(2);
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
for (const url of urls) {
  const logs = [];
  const onConsole = (msg) => { if (msg.type() === "error") logs.push(msg.text().slice(0, 400)); };
  const onError = (err) => logs.push("pageerror: " + String(err).slice(0, 400));
  page.on("console", onConsole);
  page.on("pageerror", onError);
  let nav = "ok";
  try { await page.goto(url, { waitUntil: "domcontentloaded", timeout: 30000 }); await page.waitForTimeout(1200); }
  catch (error) { nav = String(error).slice(0, 240); }
  const h1 = await page.locator("h1").first().innerText().catch(() => "");
  process.stdout.write(JSON.stringify({ url, nav, title: await page.title(), h1, logs }) + "\n");
  page.off("console", onConsole);
  page.off("pageerror", onError);
}
await browser.close();
