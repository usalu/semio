import { chromium } from "playwright";


const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto("http://127.0.0.1:6014/", { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(7000);
await page.keyboard.press("Escape").catch(() => {});
await page.waitForTimeout(500);

const dump = async (tag) => {
  const data = await page.evaluate(() => {
    const ids = Array.from(document.querySelectorAll("[id]"))
      .map((el) => el.id)
      .filter((id) => /engagement|addObject|openAdd|actionPane|action\./i.test(id))
      .slice(0, 80);
    const buttons = Array.from(document.querySelectorAll("button, [role='button'], [role='menuitem']"))
      .map((b) => `${b.id || "?"}|${(b.getAttribute("data-menu-action") || "")}|${(b.innerText || "").replace(/\n/g, " ").trim().slice(0, 40)}`)
      .filter((t) => /add|object|engagement|action/i.test(t))
      .slice(0, 40);
    const root = document.getElementById("framework.window.puzzle3dMainPerspective.engagement")
      || document.querySelector('[id*="engagement"]');
    return {
      ids,
      buttons,
      rootId: root?.id ?? null,
      rootText: (root?.innerText || "").replace(/\n/g, " | ").slice(0, 300),
    };
  });
  console.log(tag, JSON.stringify(data, null, 2));
};

await dump("before");
for (const sel of [
  '[id="framework.window.puzzle3dMainPerspective.engagement.toggle"]',
  '[id*="engagement"][id*="toggle"]',
  '[id*="puzzle3d-main-perspective"][id*="engagement"]',
]) {
  const loc = page.locator(sel).first();
  const n = await loc.count();
  console.log("locator", sel, "count", n, n ? await loc.getAttribute("id") : null);
  if (n) {
    await loc.click({ force: true, timeout: 4000 }).catch((e) => console.log("click fail", e.message));
    await page.waitForTimeout(1000);
    await dump("after " + sel);
    break;
  }
}
await browser.close();
