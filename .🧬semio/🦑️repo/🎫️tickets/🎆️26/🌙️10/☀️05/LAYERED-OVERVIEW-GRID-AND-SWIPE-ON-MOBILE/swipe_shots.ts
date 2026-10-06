/** 📸️ Real-browser check of a layered landing on a touch phone: it must be the swipe mode of the same grid, and real touch swipes
 * (CDP touch events) along both axes must settle on the neighbouring cell, spring back where none follows, and never scroll the page.
 * Writes a screenshot per step into 🗑️generated. Run: bun swipe_shots.ts <origin> <name> [reducedMotion=reduce|no-preference] [WIDTHxHEIGHT] */
import { mkdirSync } from "node:fs";
import { join } from "node:path";
import { chromium, type Page } from "playwright";

const [origin, name = "landing", motion = "reduce", size = "375x812"] = process.argv.slice(2);
const [width, height] = size.split("x").map(Number) as [number, number];
if (origin === undefined) throw new Error("origin required");
const out = join(import.meta.dir, "🗑️generated", `shots-${name}`);
mkdirSync(out, { recursive: true });

const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width, height }, isMobile: true, hasTouch: true, deviceScaleFactor: 2, reducedMotion: motion as "reduce" | "no-preference" });
const page = await context.newPage();
const problems: string[] = [];
page.on("pageerror", (error) => problems.push(`page error: ${error.message}`));
await page.goto(origin, { waitUntil: "domcontentloaded" });
const root = page.locator("[data-layered-overview]");
const introduction = page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]');
await root.or(introduction).first().waitFor({ timeout: 120_000 });
if (await introduction.isVisible()) {
  await introduction.click();
  const identity = page.locator('#quiz-main [data-card="identity"]');
  await identity.locator('input[type="radio"][value="anonymous"]').check();
  await identity.locator('[data-overview-card-action="primary"]').click();
}
await root.waitFor({ timeout: 60_000 });

const state = (target: Page) =>
  target.evaluate(() => {
    const overview = document.querySelector<HTMLElement>("[data-layered-overview]")!;
    const box = overview.getBoundingClientRect();
    const resting = [...document.querySelectorAll<HTMLElement>("[data-layered-cell]")].find((cell) => {
      const rect = cell.getBoundingClientRect();
      return Math.abs(rect.left - box.left) < 1 && Math.abs(rect.top - box.top) < 1;
    });
    return { mode: overview.dataset.mode, touchAction: overview.style.touchAction, transform: overview.querySelector<HTMLElement>("[data-layered-strip]")!.style.transform, resting: resting?.dataset.layeredCell ?? null, scrollX: window.scrollX, scrollY: window.scrollY, cells: document.querySelectorAll("[data-layered-cell]").length, cardScroll: resting === undefined ? null : [resting.scrollTop, resting.scrollHeight - resting.clientHeight], hints: [...document.querySelectorAll<HTMLElement>("[data-layered-neighbour]")].map((hint) => { const rect = hint.getBoundingClientRect(); return `${hint.dataset.layeredNeighbour}:${hint.dataset.pane}@${Math.round(rect.left)},${Math.round(rect.top)},${Math.round(rect.width)}x${Math.round(rect.height)}${getComputedStyle(hint).opacity === "0" ? ":hidden" : ""}`; }) };
  });

const swipe = async (dx: number, dy: number, ms = 160) => {
  const box = (await root.boundingBox())!;
  const [x, y] = [box.x + box.width / 2 - dx / 2, box.y + box.height / 2 - dy / 2];
  const cdp = await context.newCDPSession(page);
  await cdp.send("Input.dispatchTouchEvent", { type: "touchStart", touchPoints: [{ x, y }] });
  for (let step = 1; step <= 12; step += 1) {
    await cdp.send("Input.dispatchTouchEvent", { type: "touchMove", touchPoints: [{ x: x + (dx * step) / 12, y: y + (dy * step) / 12 }] });
    await page.waitForTimeout(ms / 12);
  }
  await cdp.send("Input.dispatchTouchEvent", { type: "touchEnd", touchPoints: [] });
  await cdp.detach();
  await page.waitForTimeout(600);
};

let index = 0;
const shot = async (label: string) => {
  const seen = await state(page);
  console.log(`[DEBUG] ${label}: ${JSON.stringify(seen)}`);
  await page.screenshot({ path: join(out, `${String(index++).padStart(2, "0")}-${label}.png`) });
  return seen;
};

await page.waitForTimeout(1500);
await shot("rest");
await swipe(-0.64 * width, 0);
await shot("swiped-left");
await swipe(0, -0.5 * height);
await shot("swiped-up");
await swipe(0.64 * width, 0);
await shot("swiped-right");
await swipe(0, 0.5 * height);
await shot("swiped-down");
await swipe(0.7 * width, 0);
await shot("pulled-past-first-column");
await swipe(0, 0.4 * height);
await shot("pulled-past-first-row");
console.log(`[DEBUG] problems: ${JSON.stringify(problems)}`);
await browser.close();
