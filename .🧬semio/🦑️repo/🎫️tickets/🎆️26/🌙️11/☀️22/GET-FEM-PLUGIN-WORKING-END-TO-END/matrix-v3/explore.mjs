import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
const [url, secs = "30", tag = "x"] = process.argv.slice(2);
const out = fileURLToPath(new URL("../🗑️generated/matrix-v3/", import.meta.url));
const browser = await chromium.launch();
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
  const logs = [];
  page.on("console", m => logs.push(`${m.type()} ${m.text().slice(0, 300)}`));
  page.on("pageerror", e => logs.push(`pageerror ${e.message.slice(0, 300)}`));
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
  await page.waitForTimeout(Number(secs) * 1000);
  writeFileSync(`${out}explore-${tag}.aria.txt`, await page.locator("body").ariaSnapshot());
  writeFileSync(`${out}explore-${tag}.text.txt`, await page.locator("body").innerText());
  writeFileSync(`${out}explore-${tag}.console.txt`, logs.join("\n"));
  await page.screenshot({ path: `${out}explore-${tag}.png` });
  console.log("ok", logs.length);
} finally { await browser.close(); }
