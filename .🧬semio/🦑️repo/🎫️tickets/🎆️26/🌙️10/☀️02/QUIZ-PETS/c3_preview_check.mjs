/** 🧭️ Ticket tool of work package C3: drives the quiz pets preview in headless Chromium — the "Play with the pets" group
 * against the real layer and stage (who is named, what each deed does to that pet's drawing, the status line, the
 * keyboard), what the learner allows (the group goes with play, a pet under the resting pointer offers the hand only
 * while play is allowed, still pets allow nothing) and the marks of the page — and prints what it saw as JSON, with
 * screenshots. Nothing is asserted beyond what the glue owns; the stage's answers are recorded as they come.
 *
 * Usage (from the repository root, preview running on 6257, see `quiz_pets_preview/vite.config.ts`):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/c3_preview_check.mjs" [--base http://127.0.0.1:6257/] [--out <dir>]
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const address = option("base", "http://127.0.0.1:6257/");
const out = resolve(option("out", ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/🗑️generated/c3/browser"));
mkdirSync(out, { recursive: true });

const failed = [];
const report = {};
const expect = (name, holds, detail) => {
  if (!holds) failed.push(`${name}: ${JSON.stringify(detail)}`);
};

const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 1280, height: 860 } });
const page = await context.newPage();
const noise = [];
page.on("console", (message) => {
  if (!/\[vite\]|React DevTools/u.test(message.text())) noise.push(`${message.type()}: ${message.text()}`);
});
page.on("pageerror", (error) => noise.push(`pageerror: ${error}`));
await page.goto(address, { waitUntil: "load" });
await page.waitForSelector(".pet-layer svg.pet", { state: "attached", timeout: 30000 });
await page.waitForSelector('[role="group"][aria-label="Play with the pets"]', { timeout: 30000 });

const onStage = () => page.evaluate(() => [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => pet.getAttribute("data-pet")).sort());
const players = () =>
  page.evaluate(() =>
    [...document.querySelectorAll('[role="group"][aria-label="Play with the pets"] [data-pets-player]')].map((group) => ({
      species: group.getAttribute("data-pets-player"),
      name: document.getElementById(group.getAttribute("aria-labelledby") ?? "")?.textContent,
      buttons: [...group.querySelectorAll("button")].map((button) => button.textContent),
    })),
  );
const status = () => page.evaluate(() => document.querySelector('[role="group"][aria-label="Play with the pets"] [role="status"]')?.textContent ?? null);
const activityOf = (species) => page.evaluate((species) => document.querySelector(`.pet-layer svg.pet[data-pet="${species}"]`)?.getAttribute("data-pet-activity") ?? null, species);
const bodyOf = (species) =>
  page.evaluate((species) => {
    const pet = document.querySelector(`.pet-layer svg.pet[data-pet="${species}"]`);
    if (pet === null) return null;
    const boxes = [...pet.querySelectorAll("path, ellipse, rect, circle")].map((part) => part.getBoundingClientRect()).filter((box) => box.width > 0 && box.height > 0);
    const left = Math.min(...boxes.map((box) => box.left));
    const top = Math.min(...boxes.map((box) => box.top));
    const right = Math.max(...boxes.map((box) => box.right));
    const bottom = Math.max(...boxes.map((box) => box.bottom));
    return { x: (left + right) / 2, y: (top + bottom) / 2, width: right - left, height: bottom - top };
  }, species);
const watchActivity = async (species, milliseconds) => {
  const seen = [];
  const until = Date.now() + milliseconds;
  while (Date.now() < until) {
    const now = await activityOf(species);
    if (seen.at(-1) !== now) seen.push(now);
    await page.waitForTimeout(50);
  }
  return seen;
};
const deedButton = (species, label) => page.locator(`[data-pets-player="${species}"] button`, { hasText: label });

report.onStage = await onStage();
report.players = await players();
expect("players are the pets on stage", JSON.stringify(report.players.map((player) => player.species).sort()) === JSON.stringify(report.onStage), report);
expect("four deeds per pet", report.players.every((player) => JSON.stringify(player.buttons) === JSON.stringify(["Hello", "Trick", "Pet", "Toss"])), report.players);
expect("status empty at first", (await status()) === "", await status());
await page.screenshot({ path: resolve(out, "1-play-group.png"), fullPage: false });

const first = report.players[0].species;
const second = report.players.at(-1).species;
report.deeds = [];
for (const [species, label] of [
  [first, "Hello"],
  [first, "Trick"],
  [first, "Pet"],
  [first, "Toss"],
]) {
  const before = await activityOf(species);
  await deedButton(species, label).click();
  const activities = await watchActivity(species, 2500);
  report.deeds.push({ species, deed: label, before, activities, status: await status() });
  await page.waitForTimeout(1500);
}
await page.screenshot({ path: resolve(out, "2-after-deeds.png"), fullPage: false });
const said = { Hello: "says hello.", Trick: "does a trick.", Pet: "is petted.", Toss: "is tossed up." };
for (const deed of report.deeds) expect(`status after ${deed.deed}`, deed.status === `${report.players[0].name} ${said[deed.deed]}`, deed);

await deedButton(second, "Hello").focus();
await page.keyboard.press("Enter");
report.keyboard = { focused: await page.evaluate(() => document.activeElement?.textContent), status: await status(), activities: await watchActivity(second, 2000) };
expect("keyboard hello", report.keyboard.status === `${report.players.at(-1).name} says hello.` && report.keyboard.focused === "Hello", report.keyboard);

const grabAt = async (species) => {
  await page.mouse.move(5, 5);
  await page.waitForTimeout(300);
  const body = await bodyOf(species);
  if (body === null) return { body, cursor: "gone" };
  await page.mouse.move(body.x, body.y);
  await page.waitForTimeout(400);
  return { body, cursor: await page.evaluate(() => document.documentElement.getAttribute("data-pet-cursor")), under: await page.evaluate(({ x, y }) => document.elementFromPoint(x, y)?.tagName ?? null, body) };
};
report.handWithPlay = await grabAt(second);
const play = page.getByRole("checkbox", { name: "Pets react to clicks and can be picked up" });
await play.uncheck();
await page.waitForTimeout(400);
report.groupWithoutPlay = await page.locator('[role="group"][aria-label="Play with the pets"]').count();
report.handWithoutPlay = await grabAt(second);
expect("group gone without play", report.groupWithoutPlay === 0, report.groupWithoutPlay);
expect("no hand offered without play", report.handWithoutPlay.cursor === null, report.handWithoutPlay);
await play.check();
await page.waitForTimeout(400);
report.groupWithPlayAgain = await page.locator('[role="group"][aria-label="Play with the pets"]').count();
expect("group back with play", report.groupWithPlayAgain === 1, report.groupWithPlayAgain);

await page.getByRole("group", { name: "Pets" }).getByRole("button", { name: "Still" }).click();
await page.waitForTimeout(400);
report.still = {
  group: await page.locator('[role="group"][aria-label="Play with the pets"]').count(),
  playDisabled: await play.isDisabled(),
  mischiefDisabled: await page.getByRole("checkbox", { name: "Pets may play with the page" }).isDisabled(),
  note: await page.getByText("Only calm and lively pets play.").count(),
};
report.handWhileStill = await grabAt(second);
expect("still: nothing to play", report.still.group === 0 && report.still.playDisabled && report.still.mischiefDisabled && report.still.note === 1, report.still);
await page.screenshot({ path: resolve(out, "3-still.png"), fullPage: false });
await page.getByRole("group", { name: "Pets" }).getByRole("button", { name: "Calm" }).click();

await page.waitForSelector('[role="group"][aria-label="Play with the pets"]', { timeout: 30000 });
await page.locator('[role="group"][aria-label="Play with the pets"]').screenshot({ path: resolve(out, "4-group-en.png") });
await page.getByRole("button", { name: "Deutsch" }).first().click();
await page.waitForSelector('[role="group"][aria-label="Mit den Tierchen spielen"]', { timeout: 30000 });
await page.locator('[data-pets-player] button', { hasText: "Kunststück" }).first().click();
report.german = {
  players: await page.evaluate(() => [...document.querySelectorAll('[role="group"][aria-label="Mit den Tierchen spielen"] [data-pets-player]')].map((group) => `${document.getElementById(group.getAttribute("aria-labelledby") ?? "")?.textContent}: ${[...group.querySelectorAll("button")].map((button) => button.textContent).join(" | ")}`)),
  status: await page.evaluate(() => document.querySelector('[role="group"][aria-label="Mit den Tierchen spielen"] [role="status"]')?.textContent ?? null),
  allowances: await page.evaluate(() => [...document.querySelectorAll("[data-pets-allow]")].map((label) => label.textContent)),
};
await page.setViewportSize({ width: 390, height: 860 });
await page.waitForTimeout(4000);
report.narrow = {
  onStage: await onStage(),
  players: await page.evaluate(() => [...document.querySelectorAll("[data-pets-player]")].map((group) => group.getAttribute("data-pets-player"))),
  castLine: await page.evaluate(() => [...document.querySelectorAll("p")].find((line) => line.textContent?.startsWith("Gerade hier"))?.textContent ?? null),
};
expect("narrow: the group names exactly who is on stage", JSON.stringify([...report.narrow.players].sort()) === JSON.stringify(report.narrow.onStage), report.narrow);
await page.locator('[data-card="preferences"]').screenshot({ path: resolve(out, "5-settings-de-narrow.png") });
report.narrowOverflow = await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth);
expect("no sideways scroll at 390 px", report.narrowOverflow <= 0, report.narrowOverflow);

report.marks = await page.evaluate(() => ({
  props: [...document.querySelectorAll("[data-pet-prop]")].map((element) => `${element.tagName} ${element.getAttribute("data-pet-prop")}`),
  topics: [...document.querySelectorAll("[data-pet-topic]")].map((element) => `${element.tagName} ${element.getAttribute("data-pet-topic")}`),
  grips: document.querySelectorAll("[data-quiz-grip]").length,
}));
report.noise = noise;
expect("console clean", noise.length === 0, noise);
report.failed = failed;
writeFileSync(resolve(out, "report.json"), `${JSON.stringify(report, null, 2)}\n`);
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
await browser.close();
process.exitCode = failed.length === 0 ? 0 : 1;
