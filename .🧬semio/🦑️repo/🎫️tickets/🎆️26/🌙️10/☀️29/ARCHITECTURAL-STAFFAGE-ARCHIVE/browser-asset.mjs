/** 🌐️ Confirms one asset page draws, switches view, and switches language without a page error. */
import playwright from "/home/deck/semio/node_modules/playwright/index.js";

const { chromium } = playwright;
const browser = await chromium.connectOverCDP("http://127.0.0.1:9222");
const context = browser.contexts()[0] ?? await browser.newContext();
const page = context.pages()[0] ?? await context.newPage();
const problems = [];
page.on("pageerror", (error) => problems.push(error.message.split("\n")[0]));
page.on("console", (message) => { if (message.type() === "error") problems.push(message.text().slice(0, 240)); });
await page.goto("http://127.0.0.1:8767/#/humans/walking-commuter?view=thermal", { waitUntil: "domcontentloaded", timeout: 30000 });
const english = page.getByRole("button", { name: "English" });
if (await english.count()) await english.click();
await page.waitForTimeout(4000);
const body = await page.locator("body").innerText();
const title = body.includes("Walking Commuter");
const refused = body.includes("no suitable representation") || body.includes("no available representation");
const thermal = /temperature|Temperatur|surface/i.test(body);
console.log(JSON.stringify({ title, refused, thermal, problems, svgs: await page.locator("svg").count() }));
await page.getByTitle("Deutsch").first().click();
await page.waitForTimeout(1000);
const germanBody = await page.locator("body").innerText();
console.log(JSON.stringify({ german: germanBody.includes("Pendler"), refusedAfter: germanBody.includes("keine geeignete") || germanBody.includes("no suitable") }));
await browser.close();
