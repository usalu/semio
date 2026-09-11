import { chromium } from "playwright";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto("http://127.0.0.1:6014/", { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(8000);
await page.keyboard.press("Escape").catch(() => {});
await page.waitForTimeout(500);
const data = await page.evaluate(() => {
  const add = document.getElementById("shell-menu.action.openAddObjectDialog");
  const quick = document.querySelector('[data-slot="window-engagement-quick-actions"]');
  return {
    addId: add?.id ?? null,
    addText: (add?.innerText || add?.getAttribute("aria-label") || "").slice(0, 80),
    quick: quick ? (quick as HTMLElement).innerText.slice(0, 120) : null,
    ids: Array.from(document.querySelectorAll("[id]")).map((el) => el.id).filter((id) => /openAdd|quick-actions|engagement/i.test(id)).slice(0, 30),
  };
});
console.log(JSON.stringify(data, null, 2));
await browser.close();
