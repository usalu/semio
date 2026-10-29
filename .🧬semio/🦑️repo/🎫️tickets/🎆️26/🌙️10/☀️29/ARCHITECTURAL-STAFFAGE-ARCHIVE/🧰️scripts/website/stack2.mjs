import { chromium } from "playwright";
const browser = await chromium.launch();
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "en-US" })).newPage();
page.on("console", async (m) => {
  if (!/Maximum update/.test(m.text())) return;
  const args = await Promise.all(m.args().map((a) => a.jsonValue().catch(() => "?")));
  console.log("ARGS", JSON.stringify(args).replace(/http:\/\/localhost:6071\/@fs\/home\/deck\/semio\/[^ )]*\//g, "").slice(0, 2500));
});
await page.goto("http://localhost:6071/" + process.argv[2], { waitUntil: "domcontentloaded" });
await page.waitForTimeout(12000);
await browser.close();
