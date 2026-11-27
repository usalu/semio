/** 🧾️ Dumps every console line of a fresh FEM app boot in headless Chromium plus a screenshot. Usage: node console-all-probe.mjs <url> <seconds> <tag> */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const [url = "http://127.0.0.1:6087/", seconds = "120", tag = "all"] = process.argv.slice(2);
const browser = await chromium.launch();
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
  const started = Date.now();
  const stamp = () => ((Date.now() - started) / 1000).toFixed(1);
  page.on("console", message => console.log(`${stamp()}s ${message.type()} ${message.text().slice(0, 3000)}`));
  page.on("pageerror", error => console.log(`${stamp()}s pageerror ${error.message.slice(0, 3000)}`));
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
  await page.waitForTimeout(Number(seconds) * 1000);
  await page.screenshot({ path: fileURLToPath(new URL(`./🗑️generated/probe27/${tag}.png`, import.meta.url)) });
} finally {
  await browser.close();
}
