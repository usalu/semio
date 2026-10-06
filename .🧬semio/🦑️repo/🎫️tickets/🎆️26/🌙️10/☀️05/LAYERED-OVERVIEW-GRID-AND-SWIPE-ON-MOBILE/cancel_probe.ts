/** 🔎️ Probe: why does a real touch swipe to the right on the landscape phone (812 × 375) not move the swiped quiz overview? Logs the
 * element under the finger, its scroll-container ancestors with their touch-action, and the pointer events the overview root sees.
 * Run: bun cancel_probe.ts <origin> */
import { chromium } from "playwright";

const [origin = "http://127.0.0.1:6061"] = process.argv.slice(2);
const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 812, height: 375 }, isMobile: true, hasTouch: true, deviceScaleFactor: 2 });
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
await page.evaluate(() => {
  const overview = document.querySelector<HTMLElement>("[data-layered-overview]")!;
  (window as unknown as { seen: string[] }).seen = [];
  for (const type of ["pointerdown", "pointermove", "pointerup", "pointercancel"]) overview.addEventListener(type, (event) => (window as unknown as { seen: string[] }).seen.push(`${type}:${(event as PointerEvent).pointerType}:${Math.round((event as PointerEvent).clientX)}`), true);
});
const under = (x: number, y: number) =>
  page.evaluate(([px, py]) => {
    const chain: string[] = [];
    for (let element = document.elementFromPoint(px!, py!); element; element = element.parentElement) {
      const style = getComputedStyle(element);
      const scrolls = /auto|scroll/u.test(style.overflowX + style.overflowY);
      if (scrolls || style.touchAction !== "auto" || element === document.elementFromPoint(px!, py!)) chain.push(`${element.tagName.toLowerCase()}.${String(element.className).split(" ").slice(0, 3).join(".")} ox=${style.overflowX} oy=${style.overflowY} ta=${style.touchAction} sw=${element.scrollWidth}/${element.clientWidth}`);
      if (element.matches("[data-layered-overview]")) break;
    }
    return chain;
  }, [x, y]);
const swipe = async (x: number, y: number, dx: number) => {
  console.log(`[DEBUG] under ${x},${y}:\n  ${(await under(x, y)).join("\n  ")}`);
  const cdp = await context.newCDPSession(page);
  await cdp.send("Input.dispatchTouchEvent", { type: "touchStart", touchPoints: [{ x, y }] });
  for (let step = 1; step <= 12; step += 1) {
    await cdp.send("Input.dispatchTouchEvent", { type: "touchMove", touchPoints: [{ x: x + (dx * step) / 12, y }] });
    await page.waitForTimeout(13);
  }
  await cdp.send("Input.dispatchTouchEvent", { type: "touchEnd", touchPoints: [] });
  await page.waitForTimeout(600);
  const seen = await page.evaluate(() => (window as unknown as { seen: string[] }).seen.splice(0));
  const transform = await page.locator("[data-layered-strip]").evaluate((strip) => (strip as HTMLElement).style.transform);
  console.log(`[DEBUG] swipe ${dx} -> ${transform}; events ${seen[0]} … ${seen.slice(-2).join(" ")} (${seen.length})`);
};
await swipe(666, 187, -520);
await swipe(146, 187, 520);
await swipe(700, 100, -400);
await swipe(100, 100, 400);
await swipe(100, 300, 400);
await browser.close();
