import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
await page.goto("http://127.0.0.1:6023/", { waitUntil: "domcontentloaded", timeout: 120000 });
await page.waitForTimeout(3000);
const env = await page.evaluate(async () => {
  // try to read from a module - inject script tag won't get import.meta from app
  return {
    title: document.title,
    htmlLang: document.documentElement.lang,
  };
});
console.log(env);
// fetch the main module and grep brand?
const indexHtml = await (await fetch("http://127.0.0.1:6023/")).text();
console.log(indexHtml.slice(0, 1500));
await browser.close();
