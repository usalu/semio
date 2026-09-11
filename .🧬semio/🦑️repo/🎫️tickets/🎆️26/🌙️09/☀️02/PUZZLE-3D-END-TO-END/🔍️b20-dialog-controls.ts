import { chromium } from "playwright";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto("http://127.0.0.1:6014/", { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(10000);
await page.keyboard.press("Escape").catch(() => {});
await page.waitForTimeout(600);
await page.evaluate(() => {
  const buttons = Array.from(document.querySelectorAll('[id="shell-menu.action.openAddObjectDialog"]'));
  buttons.at(-1)?.click();
});
await page.waitForTimeout(1500);
const data = await page.evaluate(() => {
  const dialog = document.querySelector('[data-slot="dialog-content"]');
  if (!dialog) return { missing: true };
  const els = Array.from(dialog.querySelectorAll("*")).map((el) => ({
    tag: el.tagName,
    id: el.id,
    role: el.getAttribute("role"),
    slot: el.getAttribute("data-slot"),
    text: (el as HTMLElement).innerText?.replace(/\s+/g, " ").trim().slice(0, 60),
  })).filter((row) => row.role || row.slot || row.id || ["BUTTON", "SELECT", "INPUT"].includes(row.tag));
  return { text: (dialog as HTMLElement).innerText.replace(/\s+/g, " ").slice(0, 300), els: els.slice(0, 80) };
});
console.log(JSON.stringify(data, null, 2));
await browser.close();
