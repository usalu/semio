import { chromium } from "playwright";

const url = "http://127.0.0.1:6029/";
const ticketDir = process.argv[2];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const consoleLogs = [];
const pageErrors = [];
const reqFailed = [];

page.on("console", (msg) => {
  consoleLogs.push({ type: msg.type(), text: msg.text().slice(0, 800) });
});
page.on("pageerror", (err) => {
  pageErrors.push({ message: String(err.message || err).slice(0, 500), stack: String(err.stack || "").slice(0, 1500) });
});
page.on("requestfailed", (req) => {
  reqFailed.push({ url: req.url().slice(0, 300), error: req.failure()?.errorText });
});

let navError = null;
try {
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 60000 });
  await page.waitForTimeout(10000);
} catch (e) {
  navError = String(e);
}

const title = await page.title().catch(() => "");
const bodyText = await page.locator("body").innerText().catch(() => "");
const hasCnError = pageErrors.some((e) => /cn is not defined/.test(e.message + e.stack));
const screenshotPath = `${ticketDir}/🧪demonstrator-e2e-screenshot.png`;
await page.screenshot({ path: screenshotPath, fullPage: false }).catch(() => {});

const errors = consoleLogs.filter((l) => l.type === "error");
const out = {
  title,
  navError,
  hasCnError,
  bodyTextSample: bodyText.slice(0, 1000),
  pageErrors,
  consoleErrors: errors.slice(0, 30),
  reqFailed: reqFailed.slice(0, 20),
  screenshotPath,
};
await Bun.write(`${ticketDir}/🧪demonstrator-playwright-v2.json`, JSON.stringify(out, null, 2));
console.log(JSON.stringify({
  title: out.title,
  navError: out.navError,
  hasCnError: out.hasCnError,
  pageErrorCount: out.pageErrors.length,
  pageErrors: out.pageErrors.slice(0, 10),
  consoleErrorCount: out.consoleErrors.length,
  consoleErrors: out.consoleErrors.slice(0, 10),
  bodyTextSample: out.bodyTextSample.slice(0, 400),
}, null, 2));
await browser.close();
process.exit(hasCnError || navError || out.pageErrors.some(e => /ReferenceError|is not defined/.test(e.message)) ? 1 : 0);
