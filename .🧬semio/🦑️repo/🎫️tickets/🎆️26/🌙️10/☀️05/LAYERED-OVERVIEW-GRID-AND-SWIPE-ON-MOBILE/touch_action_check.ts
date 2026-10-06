/** 🔎️ Check: on the swiped quiz overview of a phone every scroll container inside a card cell computes `touch-action: pinch-zoom`, and a
 * real swipe that starts on the learner card's presence list (`<details overflow:auto>`) moves to the next page. Run: bun touch_action_check.ts <origin> */
import { chromium } from "playwright";

const [origin = "http://127.0.0.1:6061"] = process.argv.slice(2);
const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 375, height: 812 }, isMobile: true, hasTouch: true, deviceScaleFactor: 2 });
const page = await context.newPage();
await page.goto(origin);
const root = page.locator("[data-layered-overview]");
const introduction = page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]');
await root.or(introduction).first().waitFor({ timeout: 120_000 });
if (await introduction.isVisible()) {
  await introduction.click();
  const identity = page.locator('#quiz-main [data-card="identity"]');
  await identity.locator('input[type="radio"][value="anonymous"]').check();
  await identity.locator('[data-overview-card-action="primary"]').click();
}
await root.waitFor();
await page.waitForTimeout(1500);
const scrollers = await page.evaluate(() =>
  [...document.querySelectorAll<HTMLElement>("[data-layered-cell] *")]
    .filter((element) => /auto|scroll/u.test(getComputedStyle(element).overflowX + getComputedStyle(element).overflowY))
    .map((element) => `${element.closest<HTMLElement>("[data-layered-cell]")!.dataset.layeredCell}: ${element.tagName.toLowerCase()}[${[...element.attributes].map((attribute) => attribute.name).filter((name) => name.startsWith("data-")).join(",")}] ta=${getComputedStyle(element).touchAction}`),
);
console.log(`[DEBUG] scroll containers inside cells:\n  ${scrollers.join("\n  ")}`);
const list = page.locator('[data-layered-cell="learner"] [data-presence-list]').first();
const box = await list.boundingBox();
console.log(`[DEBUG] presence list box ${JSON.stringify(box)}`);
if (box !== null) {
  const [x, y] = [Math.min(360, box.x + box.width / 2 + 100), box.y + box.height / 2];
  const cdp = await context.newCDPSession(page);
  await cdp.send("Input.dispatchTouchEvent", { type: "touchStart", touchPoints: [{ x, y }] });
  for (let step = 1; step <= 12; step += 1) {
    await cdp.send("Input.dispatchTouchEvent", { type: "touchMove", touchPoints: [{ x: x - step * 20, y }] });
    await page.waitForTimeout(13);
  }
  await cdp.send("Input.dispatchTouchEvent", { type: "touchEnd", touchPoints: [] });
  await page.waitForTimeout(700);
  console.log(`[DEBUG] after a swipe from the presence list: ${await page.locator("[data-layered-strip]").evaluate((strip) => (strip as HTMLElement).style.transform)}`);
}
await browser.close();
