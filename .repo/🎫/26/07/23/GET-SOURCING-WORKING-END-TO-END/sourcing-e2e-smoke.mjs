/**
 * Temporary runtime smoke for sourcing-curate on the OS playground.
 * Expects `dev:sourcing` already serving http://127.0.0.1:6081/
 */
import { chromium } from "playwright";

const BASE = process.env.SOURCING_URL ?? "http://127.0.0.1:6081/";
const TIMEOUT_MS = Number(process.env.SOURCING_E2E_TIMEOUT_MS ?? 90_000);

const pageErrors = [];
const consoleErrors = [];
const debugLogs = [];

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("pageerror", (err) => pageErrors.push(String(err)));
page.on("console", (msg) => {
  const text = msg.text();
  if (msg.type() === "error") consoleErrors.push(text);
  if (text.includes("[DEBUG]")) debugLogs.push(text);
});

await page.goto(BASE, { waitUntil: "domcontentloaded", timeout: TIMEOUT_MS });
const deadline = Date.now() + TIMEOUT_MS;
let bodyText = "";
while (Date.now() < deadline) {
  bodyText = await page.locator("body").innerText().catch(() => "");
  if (/Pool|Curated|Kuratiert|Search|Suchen/i.test(bodyText) && !/Missing window:/i.test(bodyText)) break;
  await page.waitForTimeout(500);
}
bodyText = await page.locator("body").innerText().catch(() => "");

const critical = [...pageErrors, ...consoleErrors].filter(
  (line) => !/favicon|Download the React DevTools|ResizeObserver/i.test(line),
);

const result = {
  ok: /Pool|Curated|Kuratiert/i.test(bodyText) && critical.length === 0 && !/Missing window:/i.test(bodyText),
  bodySnippet: bodyText.slice(0, 800),
  pageErrors,
  consoleErrors: critical,
  debugLogs,
  url: page.url(),
};

console.log(`[DEBUG] sourcing e2e smoke: ${JSON.stringify(result, null, 2)}`);
await browser.close();
if (!result.ok) process.exit(1);
