/** 🌐️ Opens the release site in Chrome and records page errors for the routes a reader uses. */
import playwright from "/home/deck/semio/node_modules/playwright/index.js";

const { chromium } = playwright;

const base = "http://127.0.0.1:8767/";
const routes = ["#/", "#/all", "#/humans", "#/humans/walking-commuter", "#/humans/walking-commuter?view=thermal", "#/humans/walking-commuter?view=forces", "#/compare", "#/playground", "#/collections", "#/furniture"];
const browser = await chromium.connectOverCDP("http://127.0.0.1:9222");
const context = browser.contexts()[0] ?? await browser.newContext();
const page = context.pages()[0] ?? await context.newPage();
const problems = [];
page.on("pageerror", (error) => problems.push(`pageerror ${error.message.split("\n")[0]}`));
page.on("console", (message) => {
  if (message.type() === "error") problems.push(`console ${message.text().slice(0, 300)}`);
});
await page.goto(base, { waitUntil: "domcontentloaded", timeout: 30000 });
const english = page.getByRole("button", { name: "English" });
if (await english.count()) await english.click();
for (const route of routes) {
  problems.length = 0;
  await page.goto(base + route, { waitUntil: "domcontentloaded", timeout: 30000 });
  if (await english.count()) await english.click();
  await page.waitForTimeout(2500);
  const text = (await page.locator("body").innerText()).replace(/\s+/g, " ").slice(0, 240);
  const svgs = await page.locator("svg").count();
  const failed = await page.locator("[data-notice], [role=alert]").count();
  console.log(`ROUTE ${route} svg=${svgs} notices=${failed} problems=${problems.length} text=${text}`);
  for (const problem of problems) console.log(`  ${problem}`);
}
await browser.close();
