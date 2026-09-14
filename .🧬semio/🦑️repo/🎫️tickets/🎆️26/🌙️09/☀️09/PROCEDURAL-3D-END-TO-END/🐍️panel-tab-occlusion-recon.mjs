/** 🔍️ Who owns the pixel at the Inspection panel tab's centre — the tab itself, or the window-chrome
 * chip Playwright reported intercepting the click. Answers with the whole `elementsFromPoint` stack so
 * an overlap is attributed to the element that actually paints over it, never guessed from CSS. */
import { chromium } from "playwright";
const URL = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6023/?plugin=generation3d";
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("pageerror", (error) => console.log(`[DEBUG] pageerror ${error.message}`));
await page.goto(URL, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 90; i++) {
  const ready = await page.evaluate(() => Boolean(document.querySelector("button#framework\\.panel\\.inspection")));
  if (ready) break;
  await page.waitForTimeout(1000);
}
await page.waitForTimeout(6000);
await page.hover("button#framework\\.panel\\.inspection").catch(() => {});
await page.waitForTimeout(800);
const report = await page.evaluate(() => {
  const describe = (el) => el ? `${el.tagName.toLowerCase()}#${el.id || "-"}[${el.getAttribute("data-slot") ?? el.getAttribute("data-dock") ?? "-"}]` : "none";
  const tabs = [...document.querySelectorAll('[data-slot="panel-tab-button"]')].map((el) => {
    const r = el.getBoundingClientRect();
    const cx = Math.round(r.x + r.width / 2), cy = Math.round(r.y + r.height / 2);
    const stack = document.elementsFromPoint(cx, cy);
    return { id: el.id, rect: [Math.round(r.x), Math.round(r.y), Math.round(r.width), Math.round(r.height)], centre: [cx, cy], top: describe(stack[0]), reachable: stack.some((node) => node === el), stack: stack.slice(0, 4).map(describe) };
  });
  const chips = [...document.querySelectorAll('[data-window-silhouette-chip="true"]')].map((el) => {
    const r = el.getBoundingClientRect();
    return { dock: el.getAttribute("data-dock"), rect: [Math.round(r.x), Math.round(r.y), Math.round(r.width), Math.round(r.height)], text: (el.textContent ?? "").slice(0, 60) };
  });
  return { tabs, chips };
});
console.log(JSON.stringify(report, null, 2));
await browser.close();
