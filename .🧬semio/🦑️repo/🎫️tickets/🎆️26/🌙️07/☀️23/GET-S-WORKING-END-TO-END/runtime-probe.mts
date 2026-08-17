import { chromium } from "playwright";

const logs: { type: string; text: string }[] = [];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("console", (msg) => logs.push({ type: msg.type(), text: msg.text() }));
page.on("pageerror", (err) => logs.push({ type: "pageerror", text: String(err) }));
await page.goto("http://127.0.0.1:6070/", { waitUntil: "networkidle", timeout: 120000 });
await page.waitForTimeout(8000);
const title = await page.title();
const body = await page.innerText("body");
console.log("TITLE", title);
console.log("BODY_LEN", body.length);
console.log("BODY_SNIP", body.slice(0, 800).replace(/\n/g, " | "));
console.log("LOG_COUNT", logs.length);
for (const row of logs.slice(0, 100)) {
  console.log(`[${row.type}] ${row.text.slice(0, 400)}`);
}
await browser.close();
