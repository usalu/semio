import { chromium } from "playwright";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto("http://127.0.0.1:6014/", { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(8000);
await page.keyboard.press("Escape").catch(() => {});
await page.waitForTimeout(400);

const snapshot = async (tag) => {
  const data = await page.evaluate(() => {
    const engagement = document.getElementById("framework.window.puzzle3dMainPerspective.engagement");
    const toggle = document.getElementById("framework.window.puzzle3dMainPerspective.engagement.toggle");
    const ids = Array.from(document.querySelectorAll("[id]")).map((el) => el.id).filter((id) => /openAdd|addObject|action\.|engagement|actionPane|dialog/i.test(id));
    const texts = Array.from(document.querySelectorAll("button, [role='button']")).map((b) => (b.innerText || b.getAttribute("aria-label") || "").replace(/\s+/g, " ").trim()).filter(Boolean).slice(0, 60);
    return {
      folded: engagement?.getAttribute("data-folded"),
      toggleDisabled: toggle?.getAttribute("disabled"),
      toggleAria: toggle?.getAttribute("aria-disabled"),
      rootText: (engagement?.innerText || "").replace(/\s+/g, " ").slice(0, 400),
      childCount: engagement?.childElementCount ?? -1,
      ids,
      texts,
    };
  });
  console.log(tag, JSON.stringify(data, null, 2));
};

await snapshot("before");
const toggle = page.locator('[id="framework.window.puzzle3dMainPerspective.engagement.toggle"]');
console.log("toggle count", await toggle.count());
await toggle.click({ timeout: 5000 });
await page.waitForTimeout(1500);
await snapshot("after-normal-click");
await toggle.click({ force: true, timeout: 5000 });
await page.waitForTimeout(1500);
await snapshot("after-force-click");
await browser.close();
