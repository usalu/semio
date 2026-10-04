/** 📸️ Screenshots of the quiz website for judging a layout by eye (work package "UI polish", item 4):
 * `bun ui_polish_shots.ts <directory> <WxH> <normal|largest> [site origin]` writes the home screen, a run of every task
 * kind, the results and the leaderboard, in German, device scale 2. */
import { resolve } from "node:path";
import { chromium } from "playwright";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH;
const [directory = ".", size = "320x180", textSize = "normal", origin = "http://127.0.0.1:6071"] = process.argv.slice(2);
const [width, height] = size.split("x").map(Number) as [number, number];
const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width, height }, deviceScaleFactor: 2, locale: "de-DE", baseURL: origin });
await context.addInitScript((preferences) => localStorage.getItem("semio.quiz.architecture.preferences") === null && localStorage.setItem("semio.quiz.architecture.preferences", preferences), JSON.stringify({ locale: "de", theme: "light", textSize, showCursors: true, showAnswers: true, animateIcons: false, pets: "off" }));
const page = await context.newPage();
const shot = async (name: string): Promise<void> => {
  await page.waitForTimeout(600);
  await page.screenshot({ path: resolve(directory, `${size}-${textSize}-${name}.png`) });
};
const front = (card: string) => page.locator(`#quiz-main [data-card="${card}"]`).and(page.locator(":not([inert] *)"));
await page.goto("/");
await front("introduction").locator('[data-overview-card-action="primary"]').click();
await front("identity").locator('[data-overview-card-action="primary"]').click();
await page.locator("[data-layered-overview]").waitFor();
await shot("home");
for (const [quiz, name] of [["physics", "physics"], ["heating", "heating"], ["demand", "demand"]] as const) {
  await page.locator(`[data-layered-card="${quiz}"] [data-overview-card-action="primary"]`).click();
  await front("run").waitFor();
  await shot(`run-${name}-top`);
  await front("task").scrollIntoViewIfNeeded();
  await front("task").locator("[data-quiz-item], [data-quiz-drop]").first().scrollIntoViewIfNeeded();
  await shot(`run-${name}-task`);
  if (quiz === "physics") {
    await front("run").locator("nav button").nth(1).click();
    await front("task").locator("[data-quiz-item]").nth(1).scrollIntoViewIfNeeded();
    await shot("run-physics-second-task");
    await front("task").locator('[data-overview-card-action="primary"]').scrollIntoViewIfNeeded();
    await shot("run-physics-second-footer");
  }
  await front("run").locator('[data-overview-card-action="secondary"]').click();
  await page.locator("[data-layered-overview]").waitFor();
}
await page.evaluate(() => (window.location.hash = "board"));
await page.locator('[data-layered-pane="board"][data-opened]').waitFor();
await shot("leaderboard");
await browser.close();
