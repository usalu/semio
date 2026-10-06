/** 🍩️ Probe: on the swiped quiz overview of a phone, a finger held halfway across an edge shows the far page sliding in beside the view (it
 * is shifted next to it), and once released the index wraps with no shift left. Screenshots go to 🗑️generated/wrap. Run: bun wrap_probe.ts <origin> */
import { mkdirSync } from "node:fs";
import { join } from "node:path";
import { chromium, type CDPSession, type Page } from "playwright";

const [origin = "http://127.0.0.1:6061"] = process.argv.slice(2);
const out = join(import.meta.dir, "🗑️generated", "wrap");
mkdirSync(out, { recursive: true });
const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 375, height: 812 }, isMobile: true, hasTouch: true, deviceScaleFactor: 2, reducedMotion: "reduce" });
const page = await context.newPage();
const problems: string[] = [];
page.on("pageerror", (error) => problems.push(error.message));
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

const state = (target: Page) =>
  target.evaluate(() => ({
    strip: document.querySelector<HTMLElement>("[data-layered-strip]")!.style.transform,
    shifted: [...document.querySelectorAll<HTMLElement>("[data-layered-pane], [data-layered-cell]")].filter((element) => element.style.transform !== "").map((element) => `${element.dataset.layeredPane ?? `cell:${element.dataset.layeredCell}`}=${element.style.transform}`),
    visible: [...document.querySelectorAll<HTMLElement>("[data-layered-cell]")].filter((cell) => {
      const box = cell.getBoundingClientRect();
      return box.right > 1 && box.left < window.innerWidth - 1 && box.bottom > 1 && box.top < window.innerHeight - 1;
    }).map((cell) => cell.dataset.layeredCell),
  }));
const hold = async (cdp: CDPSession, x: number, y: number, dx: number, dy: number) => {
  await cdp.send("Input.dispatchTouchEvent", { type: "touchStart", touchPoints: [{ x, y }] });
  for (let step = 1; step <= 12; step += 1) {
    await cdp.send("Input.dispatchTouchEvent", { type: "touchMove", touchPoints: [{ x: x + (dx * step) / 12, y: y + (dy * step) / 12 }] });
    await page.waitForTimeout(25);
  }
};
const release = async (cdp: CDPSession) => {
  await cdp.send("Input.dispatchTouchEvent", { type: "touchEnd", touchPoints: [] });
  await page.waitForTimeout(700);
};
const cdp = await context.newCDPSession(page);
for (const [name, x, y, dx, dy] of [["left-edge", 100, 400, 180, 0], ["top-edge", 187, 250, 0, 380]] as const) {
  await hold(cdp, x, y, dx, dy);
  console.log(`[DEBUG] ${name} held: ${JSON.stringify(await state(page))}`);
  await page.screenshot({ path: join(out, `${name}-held.png`) });
  await release(cdp);
  console.log(`[DEBUG] ${name} released: ${JSON.stringify(await state(page))}`);
  await page.screenshot({ path: join(out, `${name}-released.png`) });
}
console.log(`[DEBUG] problems: ${JSON.stringify(problems)}`);
await browser.close();
