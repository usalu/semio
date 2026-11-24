/** 🔭️ Records a fresh FEM editor's job lifecycle and completion-turn lines in headless Chromium. Usage: node job-repaint-probe.mjs <url> [seconds] */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const url = process.argv[2] ?? "http://127.0.0.1:6086/", seconds = Number(process.argv[3] ?? 150);
const browser = await chromium.launch();
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
  const started = Date.now();
  page.on("console", message => {
    const text = message.text();
    if (/isolated job|job completion|live fields|window demand|panicked|refused|semantic|fault|Framework OS boot failed/i.test(text) && !/close-phase/.test(text)) console.log(`${((Date.now() - started) / 1000).toFixed(1)}s ${text.slice(0, 260).replace(/[^\x20-\x7e]/g, "")}`);
  });
  page.on("pageerror", error => console.log(`pageerror ${error.message.slice(0, 260)}`));
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
  await page.waitForTimeout(seconds * 1000);
  await page.screenshot({ path: fileURLToPath(new URL(`./🗑️generated/job-repaint-probe-${new URL(url).port}.png`, import.meta.url)) });
} finally {
  await browser.close();
}
