import { chromium } from "playwright";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto("http://127.0.0.1:6014/", { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(8000);
await page.keyboard.press("Escape").catch(() => {});
await page.waitForTimeout(400);

const snap = async (tag) => {
  const data = await page.evaluate(() => {
    const ids = ["puzzle3dMainPerspective", "puzzle3dMainTop"];
    const panes = ids.map((id) => {
      const el = document.getElementById(`framework.window.${id}.engagement`);
      return { id, folded: el?.getAttribute("data-folded"), text: (el?.innerText || "").replace(/\s+/g, " ").slice(0, 200) };
    });
    const add = document.getElementById("shell-menu.action.openAddObjectDialog");
    return { panes, addId: add?.id ?? null, addText: add?.innerText ?? null };
  });
  console.log(tag, JSON.stringify(data));
};

await snap("boot");

for (const sel of [
  '[id="framework.window.puzzle3dMainTop.engagement.toggle"]',
  '[id="framework.window.puzzle3dMainPerspective.search.toggle"]',
  '[id="framework.window.puzzle3dMainTop.search.toggle"]',
]) {
  const loc = page.locator(sel).first();
  const n = await loc.count();
  console.log("try", sel, "count", n);
  if (!n) continue;
  try {
    await loc.click({ timeout: 4000 });
    console.log("clicked", sel);
  } catch (e) {
    console.log("blocked", sel, e.message.split("\n")[0]);
    await loc.click({ force: true, timeout: 4000 }).then(() => console.log("force", sel)).catch((err) => console.log("force-fail", sel, err.message.split("\n")[0]));
  }
  await page.waitForTimeout(1200);
  await snap("after " + sel);
}

await browser.close();
