/** 🔬️ Overview-camera probe: records the pointer events the overview sees while the mouse crosses it in a few steps, and
 * where the camera ends up, to tell a camera that lags from one that follows a stale pointer.
 * `bun overview_camera_probe.ts` against the running dev site (`QUIZ_SITE`, default `http://localhost:6061`). */
import { join } from "node:path";
import { chromium } from "playwright";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";

const repoRoot = join(import.meta.dir, "../../../../../../..");
const site = process.env.QUIZ_SITE ?? "http://localhost:6061";
process.env.PLAYWRIGHT_BROWSERS_PATH = repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH;

const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, reducedMotion: "no-preference", locale: "en-GB" });
const page = await context.newPage();
await page.goto(site);
await page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]').click();
const form = page.locator('#quiz-main [data-card="identity"]');
await form.locator('input[type="radio"][value="pseudonym"]').check();
await form.locator('input[type="text"]').fill(`Probe ${Math.random().toString(36).slice(2, 8)}`);
await form.locator('[data-overview-card-action="primary"]').click();
const root = page.locator("[data-layered-overview]");
await root.waitFor();
await page.waitForTimeout(1500);
const frame = (await root.boundingBox())!;
await page.evaluate(() => {
  const log: string[] = [];
  (window as unknown as { probe: string[] }).probe = log;
  const at = (event: PointerEvent): string => `${Math.round(event.clientX)},${Math.round(event.clientY)}`;
  const card = (event: Event): string => (event.target as Element).closest("[data-layered-card]")?.getAttribute("data-layered-card") ?? "-";
  window.addEventListener("pointermove", (event) => log.push(`move ${at(event)} ${event.pointerType} on ${card(event)}`), true);
  window.addEventListener("pointerover", (event) => log.push(`over ${at(event)} on ${card(event)}`), true);
  window.addEventListener("pointerout", (event) => log.push(`out ${at(event)} on ${card(event)}`), true);
});
const board = async (): Promise<string> =>
  page.locator('[data-layered-pane="board"]').evaluate((element) => {
    const box = element.getBoundingClientRect();
    return `${box.left.toFixed(1)},${box.top.toFixed(1)} ${box.width.toFixed(1)}x${box.height.toFixed(1)}`;
  });
for (const [name, x, y] of [["top-left", 2, 2], ["bottom-right", frame.width - 2, frame.height - 2], ["left-middle", 2, frame.height / 2]] as const) {
  await page.evaluate(() => void ((window as unknown as { probe: string[] }).probe.length = 0));
  await page.mouse.move(frame.x + x, frame.y + y, { steps: 6 });
  const samples: string[] = [];
  for (const wait of [100, 300, 600, 1000, 2000]) {
    await page.waitForTimeout(wait);
    samples.push(await board());
  }
  const events = await page.evaluate(() => (window as unknown as { probe: string[] }).probe.slice());
  const revealed = await page.locator("[data-layered-card][data-revealed]").evaluateAll((cards) => cards.map((element) => element.getAttribute("data-layered-card")));
  console.log(`[DEBUG] ${name} -> ${Math.round(frame.x + x)},${Math.round(frame.y + y)}\n  events: ${events.join(" | ")}\n  board: ${samples.join(" -> ")}\n  revealed: ${revealed.join(",") || "none"}`);
}
await browser.close();
