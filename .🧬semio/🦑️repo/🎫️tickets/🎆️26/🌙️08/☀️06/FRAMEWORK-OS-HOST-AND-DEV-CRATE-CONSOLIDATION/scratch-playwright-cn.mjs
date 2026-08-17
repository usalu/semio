import { chromium } from "playwright";
const ticketDir = process.argv[2];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const pageErrors = [];
page.on("pageerror", (err) => {
  pageErrors.push({ message: err.message, stack: err.stack?.slice(0, 2000) });
});
page.on("console", (msg) => {
  if (msg.type() === "error") {
    pageErrors.push({ console: msg.text().slice(0, 1000), location: msg.location() });
  }
});
await page.goto("http://127.0.0.1:6029/", { waitUntil: "networkidle", timeout: 90000 }).catch(e => pageErrors.push({ nav: String(e) }));
await page.waitForTimeout(8000);
await Bun.write(`${ticketDir}/🧪demonstrator-cn-errors.json`, JSON.stringify(pageErrors, null, 2));
console.log(JSON.stringify(pageErrors, null, 2).slice(0, 8000));
await browser.close();
