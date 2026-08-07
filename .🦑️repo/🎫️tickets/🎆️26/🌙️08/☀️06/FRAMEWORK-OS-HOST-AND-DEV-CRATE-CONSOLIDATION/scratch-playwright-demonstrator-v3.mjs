import { chromium } from "playwright";
const ticketDir = process.argv[2];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const pageErrors = [];
const consoleErrors = [];
page.on("pageerror", (err) => pageErrors.push({ message: String(err.message || err).slice(0, 400), stack: String(err.stack || "").slice(0, 800) }));
page.on("console", (msg) => {
  if (msg.type() === "error") consoleErrors.push(msg.text().slice(0, 500));
});
await page.goto("http://127.0.0.1:6029/", { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(12000);
const title = await page.title();
const bodyText = await page.locator("body").innerText().catch(() => "");
await page.screenshot({ path: `${ticketDir}/🧪demonstrator-e2e-screenshot.png` }).catch(() => {});
const critical = [...pageErrors.map((e) => e.message), ...consoleErrors].filter((t) =>
  /cn is not defined|Icon is not defined|Suspense is not defined|mod2\.default is not a function|default is not a function|DemonstratorPane|DemonstratorCard/.test(t)
);
const out = { title, bodyTextSample: bodyText.slice(0, 600), pageErrors, consoleErrors: consoleErrors.slice(0, 25), critical, ok: critical.length === 0 && !!bodyText.trim() };
await Bun.write(`${ticketDir}/🧪demonstrator-playwright-v3.json`, JSON.stringify(out, null, 2));
console.log(JSON.stringify({ title: out.title, ok: out.ok, critical: out.critical, pageErrorCount: pageErrors.length, bodyTextSample: out.bodyTextSample.slice(0, 350), pageErrors: pageErrors.slice(0, 5), consoleErrors: consoleErrors.slice(0, 8) }, null, 2));
await browser.close();
process.exit(out.ok ? 0 : 1);
