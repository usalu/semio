import { chromium } from "playwright";
const browser = await chromium.launch();
for (const hash of process.argv.slice(2)) {
  const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "en-US" })).newPage();
  let n = 0;
  page.on("console", (m) => m.type() === "error" && /Maximum update/.test(m.text()) && n++);
  await page.goto("http://localhost:6071/" + hash, { waitUntil: "domcontentloaded" });
  await page.waitForTimeout(9000);
  console.log(hash, "=> maxdepth errors", n);
}
await browser.close();
