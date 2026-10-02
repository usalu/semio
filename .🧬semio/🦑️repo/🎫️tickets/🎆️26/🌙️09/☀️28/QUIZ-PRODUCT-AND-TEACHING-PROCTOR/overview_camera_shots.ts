/** 📸️ Overview evidence: drives the running dev site (`http://localhost:6061` unless `QUIZ_SITE` says otherwise) in
 * Chromium, first with normal motion, then as a device that reports reduced motion (as every browser in a Remote
 * Desktop session does); moves the mouse between the cards and onto a card and writes screenshots plus where every
 * page lies to `🗑️generated/overview-moves/`. `bun overview_camera_shots.ts [width height]`. */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { chromium, type Page } from "playwright";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";

const repoRoot = join(import.meta.dir, "../../../../../../..");
const out = join(import.meta.dir, "🗑️generated", "overview-moves");
const site = process.env.QUIZ_SITE ?? "http://localhost:6061";
const [width, height] = [Number(process.argv[2] ?? 1440), Number(process.argv[3] ?? 900)];
const tag = `${width}x${height}`;
mkdirSync(out, { recursive: true });
process.env.PLAYWRIGHT_BROWSERS_PATH = repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH;

/** 📍️ Where every page lies relative to the overview, in pixels, and whether it is live. */
async function pages(page: Page): Promise<readonly { readonly id: string; readonly x: number; readonly y: number; readonly width: number; readonly height: number; readonly live: boolean }[]> {
  return page.locator("[data-layered-pane]").evaluateAll((panes) => {
    const frame = document.querySelector("[data-layered-overview]")!.getBoundingClientRect();
    return panes.map((element) => {
      const box = element.getBoundingClientRect();
      return { id: (element as HTMLElement).dataset.layeredPane ?? "", x: Math.round(box.left - frame.left), y: Math.round(box.top - frame.top), width: Math.round(box.width), height: Math.round(box.height), live: element.querySelector("[data-page]") !== null };
    });
  });
}

const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width, height }, reducedMotion: "no-preference", locale: "en-GB" });
const page = await context.newPage();
const problems: string[] = [];
page.on("console", (message) => message.type() === "error" && problems.push(`console: ${message.text()}`));
page.on("pageerror", (error) => problems.push(`page: ${error.message}`));
await page.goto(site);
await page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]').click();
const form = page.locator('#quiz-main [data-card="identity"]');
await form.locator('input[type="radio"][value="pseudonym"]').check();
await form.locator('input[type="text"]').fill(`Camera ${Math.random().toString(36).slice(2, 8)}`);
await form.locator('[data-overview-card-action="primary"]').click();
const root = page.locator("[data-layered-overview]");
await root.waitFor();
await page.waitForTimeout(1500);
const frame = (await root.boundingBox())!;
const shots: Record<string, unknown> = {};
const report: Record<string, unknown> = { site, viewport: { width, height }, frame, mode: await root.getAttribute("data-mode"), pan: await root.getAttribute("data-pan"), strip: await root.locator("[data-layered-strip]").evaluate((element) => ({ width: (element as HTMLElement).style.width, height: (element as HTMLElement).style.height })), shots };
const shot = async (name: string): Promise<void> => {
  await page.screenshot({ path: join(out, `${tag}-${name}.jpg`), type: "jpeg", quality: 80 });
  const veil = root.locator("[data-layered-veil]");
  shots[name] = { veil: (await veil.count()) === 0 ? "none" : await veil.getAttribute("data-veil"), pages: await pages(page) };
};
if (report.mode === "strip") {
  await shot("rest");
  for (const [name, fx, fy] of [["between-cards-bottom-right", 1, 1], ["between-cards-top-right", 1, 0], ["between-cards-bottom-middle", 0.5, 1]] as const) {
    await page.mouse.move(frame.x + Math.min(frame.width - 2, Math.max(2, fx * frame.width)), frame.y + Math.min(frame.height - 2, Math.max(2, fy * frame.height)), { steps: 6 });
    await page.waitForTimeout(1500);
    await shot(name);
  }
  const board = (await page.locator('[data-layered-card="board"] [data-overview-card]').boundingBox())!;
  await page.mouse.move(board.x + board.width / 2, board.y + 12, { steps: 6 });
  await page.waitForTimeout(200);
  await shot("hover-leaderboard-gliding");
  await page.waitForTimeout(900);
  await shot("hover-leaderboard-clear");
  await page.mouse.move(frame.x + 2, frame.y + frame.height / 2, { steps: 6 });
  await page.waitForTimeout(1500);
  await shot("between-cards-left-again");
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.waitForTimeout(400);
  report.reducedPan = await root.getAttribute("data-pan");
  await page.mouse.move(frame.x + frame.width - 2, frame.y + frame.height - 2, { steps: 6 });
  await page.waitForTimeout(1500);
  await shot("reduced-motion-between-cards-bottom-right");
  await page.mouse.move(board.x + board.width / 2, board.y + 12, { steps: 6 });
  await page.waitForTimeout(1100);
  await shot("reduced-motion-hover-leaderboard");
} else {
  await shot("list");
}
report.problems = problems;
writeFileSync(join(out, `${tag}-report.json`), `${JSON.stringify(report, null, 2)}\n`);
console.log(`[DEBUG] ${tag}: mode ${String(report.mode)}, pan ${String(report.pan)}, strip ${JSON.stringify(report.strip)}, reduced pan ${String(report.reducedPan)}, problems ${problems.length}`);
await browser.close();
