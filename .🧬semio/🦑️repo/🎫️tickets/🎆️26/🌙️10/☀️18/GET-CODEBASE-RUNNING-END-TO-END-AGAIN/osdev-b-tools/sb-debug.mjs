import { chromium } from "playwright";
const base = process.argv[2], id = process.argv[3], wait = Number(process.argv[4] ?? 90);
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const pending = new Set();
page.on("console", (m) => console.log("[console]", m.type(), m.text().slice(0, 300)));
page.on("pageerror", (x) => console.log("[pageerror]", String(x).slice(0, 300)));
page.on("request", (r) => pending.add(r.url()));
page.on("requestfinished", (r) => pending.delete(r.url()));
page.on("requestfailed", (r) => { pending.delete(r.url()); console.log("[reqfailed]", r.url().slice(-120), r.failure()?.errorText); });
page.on("response", (r) => r.status() >= 400 && console.log("[http]", r.status(), r.url().slice(-120)));
await page.goto(`${base}/iframe.html?id=${id}&viewMode=story`, { waitUntil: "commit", timeout: 300000 });
for (let i = 0; i < wait; i += 10) {
  await page.waitForTimeout(10000);
  const s = await page.evaluate(() => ({ body: document.body?.className, root: document.querySelector("#storybook-root")?.children.length, err: document.querySelector(".sb-errordisplay")?.innerText?.slice(0, 300) })).catch((e) => String(e));
  console.log(`[t+${i + 10}s]`, JSON.stringify(s), "pending", pending.size, [...pending].slice(0, 3).map((u) => u.slice(-80)).join(" | "));
}
await browser.close();
