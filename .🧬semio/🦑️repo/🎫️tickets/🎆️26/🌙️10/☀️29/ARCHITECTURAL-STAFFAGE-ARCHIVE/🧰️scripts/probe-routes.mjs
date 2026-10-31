import { chromium } from "playwright";

const routes = [
  "http://localhost:6071/#/",
  "http://localhost:6071/#/all",
  "http://localhost:6071/#/humans",
  "http://localhost:6071/#/vegetation",
  "http://localhost:6071/#/compare",
  "http://localhost:6071/#/playground",
  "http://localhost:6071/#/collections",
];

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1280, height: 800 } });
const report = [];
for (const url of routes) {
  const logs = [];
  const onConsole = (msg) => { if (msg.type() === "error" || msg.type() === "warning") logs.push(msg.type() + ": " + msg.text()); };
  const onError = (err) => logs.push("pageerror: " + String(err));
  const onFail = (req) => logs.push("fail: " + req.url() + " " + (req.failure()?.errorText ?? ""));
  page.on("console", onConsole);
  page.on("pageerror", onError);
  page.on("requestfailed", onFail);
  const bad = [];
  const onRes = (res) => { if (res.status() >= 400) bad.push(res.status() + " " + res.url()); };
  page.on("response", onRes);
  let nav = "ok";
  try { await page.goto(url, { waitUntil: "networkidle", timeout: 45000 }); } catch (error) { nav = String(error).slice(0, 300); }
  await page.waitForTimeout(800);
  const h1 = await page.locator("h1").first().innerText().catch(() => "");
  report.push({ url, nav, title: await page.title(), h1, logs: logs.slice(0, 12), bad: bad.slice(0, 8) });
  page.off("console", onConsole);
  page.off("pageerror", onError);
  page.off("requestfailed", onFail);
  page.off("response", onRes);
}
const card = page.locator("a").filter({ hasText: "English Oak" }).first();
if (await card.count()) {
  await card.click();
  await page.waitForTimeout(1500);
  report.push({ url: page.url(), nav: "click-oak", title: await page.title(), h1: await page.locator("h1").first().innerText().catch(() => ""), logs: [], bad: [] });
}
process.stdout.write(JSON.stringify(report, null, 2));
await browser.close();
