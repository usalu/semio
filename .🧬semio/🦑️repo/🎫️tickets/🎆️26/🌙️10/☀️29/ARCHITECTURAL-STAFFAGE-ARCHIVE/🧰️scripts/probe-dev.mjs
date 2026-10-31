import { chromium } from "playwright";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1280, height: 800 } });
const logs = [];
page.on("console", (msg) => logs.push({ type: msg.type(), text: msg.text() }));
page.on("pageerror", (err) => logs.push({ type: "pageerror", text: String(err) }));
page.on("requestfailed", (req) => logs.push({ type: "requestfailed", text: req.url() + " " + (req.failure()?.errorText ?? "") }));
const responses = [];
page.on("response", (res) => {
  if (res.status() >= 400) responses.push({ status: res.status(), url: res.url() });
});
const url = process.argv[2] ?? "http://localhost:6071/";
let nav = "ok";
try {
  await page.goto(url, { waitUntil: "networkidle", timeout: 45000 });
} catch (error) {
  nav = String(error);
}
await page.waitForTimeout(2500);
const text = await page.locator("body").innerText().catch((error) => String(error));
const root = await page.locator("#root").innerHTML().catch((error) => String(error));
const report = { url, nav, title: await page.title().catch(() => ""), text: text.slice(0, 2000), rootBytes: root.length, rootHead: root.slice(0, 500), logs, responses };
process.stdout.write(JSON.stringify(report, null, 2));
await browser.close();
