import { chromium } from "playwright";
const base = "http://127.0.0.1:6072/";
const routes = ["#/", "#/humans", "#/humans/walking-commuter?view=thermal", "#/vehicles/tower-crane?view=forces", "#/compare", "#/playground", "#/collections", "#/all"];
const browser = await chromium.launch();
for (const hash of routes) {
  const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "en-US" })).newPage();
  const problems = [];
  page.on("console", (m) => ["error", "warning"].includes(m.type()) && problems.push(`${m.type()}: ${m.text().slice(0, 200)}`));
  page.on("pageerror", (e) => problems.push(`pageerror: ${String(e).slice(0, 200)}`));
  page.on("response", (r) => r.status() >= 400 && problems.push(`HTTP ${r.status()} ${decodeURIComponent(r.url()).slice(0, 160)}`));
  page.on("requestfailed", (r) => problems.push(`failed ${decodeURIComponent(r.url()).slice(0, 160)}`));
  await page.goto(base + hash, { waitUntil: "load" });
  await page.waitForTimeout(12000);
  const heading = await page.evaluate(() => document.querySelector("h1")?.textContent?.slice(0, 40));
  console.log(hash, "| h1:", heading, "| problems:", problems.length, problems.slice(0, 5).join(" || "));
  await page.close();
}
await browser.close();
