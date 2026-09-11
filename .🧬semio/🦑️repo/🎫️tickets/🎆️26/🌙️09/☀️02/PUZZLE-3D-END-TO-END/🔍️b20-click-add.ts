import { chromium } from "playwright";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const logs = [];
page.on("console", (msg) => {
  const t = msg.text();
  if (/openAdd|openDialog|makeOwned|dialog|addObject/i.test(t)) logs.push(t.slice(0, 240));
});
await page.goto("http://127.0.0.1:6014/", { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(10000);
await page.keyboard.press("Escape").catch(() => {});
await page.waitForTimeout(800);
const before = await page.evaluate(() => ({
  n: document.querySelectorAll('[id="shell-menu.action.openAddObjectDialog"]').length,
  dialogs: document.querySelectorAll('[role="dialog"], [data-slot="dialog"]').length,
}));
console.log("before", before);
await page.evaluate(() => {
  const buttons = Array.from(document.querySelectorAll('[id="shell-menu.action.openAddObjectDialog"]'));
  const last = buttons.at(-1);
  last?.click();
});
await page.waitForTimeout(2000);
const after = await page.evaluate(() => {
  const dialogs = Array.from(document.querySelectorAll('[role="dialog"], [role="alertdialog"], [data-slot="dialog"], [data-slot="dialog-content"], [data-slot="uidialog"]'));
  return {
    dialogs: dialogs.map((d) => `${d.getAttribute("data-slot") || d.getAttribute("role")}:${d.id}:${(d as HTMLElement).innerText.replace(/\s+/g, " ").slice(0, 200)}`),
    bodyText: document.body.innerText.includes("Choose the kind") || document.body.innerText.includes("Hinzufügen") || document.body.innerText.includes("Add Object"),
  };
});
console.log("after", JSON.stringify(after, null, 2));
console.log("logs", logs);
await browser.close();
