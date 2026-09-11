import { chromium } from "playwright";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto("http://127.0.0.1:6014/", { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(8000);
await page.keyboard.press("Escape").catch(() => {});
await page.waitForTimeout(400);
await page.locator('[id="shell-menu.action.openAddObjectDialog"]').first().click({ force: true, timeout: 5000 });
await page.waitForTimeout(2000);
const data = await page.evaluate(() => {
  const dialogs = Array.from(document.querySelectorAll('[role="dialog"], [role="alertdialog"], [data-slot="dialog"], [data-slot="dialog-content"]'));
  return dialogs.map((d) => ({
    id: d.id,
    slot: d.getAttribute("data-slot"),
    role: d.getAttribute("role"),
    text: (d as HTMLElement).innerText.replace(/\n/g, " | ").slice(0, 800),
    html: (d as HTMLElement).innerHTML.slice(0, 2500),
    controls: Array.from(d.querySelectorAll("button, [role='combobox'], select, input, [data-slot]")).map((el) => `${el.tagName}:${el.id || ""}:${el.getAttribute("role") || ""}:${el.getAttribute("data-slot") || ""}:${(el as HTMLElement).innerText.replace(/\s+/g, " ").trim().slice(0, 40)}`).slice(0, 40),
  }));
});
console.log(JSON.stringify(data, null, 2));
await browser.close();
