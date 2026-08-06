import { chromium } from "playwright";

const url = "http://127.0.0.1:6029/";
const ticketDir = process.argv[2];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const consoleLogs = [];
const pageErrors = [];
const reqFailed = [];

page.on("console", (msg) => {
  consoleLogs.push({ type: msg.type(), text: msg.text().slice(0, 500) });
});
page.on("pageerror", (err) => {
  pageErrors.push(String(err).slice(0, 800));
});
page.on("requestfailed", (req) => {
  reqFailed.push({ url: req.url().slice(0, 300), error: req.failure()?.errorText });
});

let navError = null;
try {
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 60000 });
  // allow wasm/plugin boot
  await page.waitForTimeout(12000);
} catch (e) {
  navError = String(e);
}

const bodyText = await page.locator("body").innerText().catch(() => "");
const title = await page.title().catch(() => "");
const rootHtml = await page.locator("#root, #app, body").first().innerHTML().catch(() => "");
const screenshotPath = `${ticketDir}/🧪demonstrator-e2e-screenshot.png`;
await page.screenshot({ path: screenshotPath, fullPage: true }).catch(() => {});

const errors = consoleLogs.filter((l) => l.type === "error");
const warnings = consoleLogs.filter((l) => l.type === "warning");
const debug = consoleLogs.filter((l) => l.text.includes("[DEBUG]"));

const out = {
  title,
  navError,
  bodyTextSample: bodyText.slice(0, 800),
  rootHtmlSample: rootHtml.slice(0, 800),
  pageErrors,
  reqFailed: reqFailed.slice(0, 40),
  consoleErrorCount: errors.length,
  consoleErrors: errors.slice(0, 40),
  consoleWarningCount: warnings.length,
  consoleWarnings: warnings.slice(0, 20),
  debugLogs: debug.slice(0, 30),
  consoleSample: consoleLogs.slice(0, 50),
  screenshotPath,
};
await Bun.write(`${ticketDir}/🧪demonstrator-playwright.json`, JSON.stringify(out, null, 2));
console.log(JSON.stringify({
  title: out.title,
  navError: out.navError,
  pageErrors: out.pageErrors,
  consoleErrorCount: out.consoleErrorCount,
  consoleErrors: out.consoleErrors,
  reqFailedCount: out.reqFailed.length,
  reqFailed: out.reqFailed.slice(0, 15),
  bodyTextSample: out.bodyTextSample.slice(0, 300),
  debugCount: out.debugLogs.length,
}, null, 2));
await browser.close();
