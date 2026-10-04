/** ⏱️ Ticket tool of work package C4: measures how long Playwright takes, on this machine right now, for a mouse move, for a move in steps and for an animation frame of the site — on the introduction, on the home overview of an anonymous learner with its pets and on a quiz's page — the pace of the learner's hand the browser spec can reach.
 *
 * Usage (from the repository root): node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/c4_move_speed.mjs" [--url http://127.0.0.1:6243]
 */
import { chromium } from "playwright";

const at = process.argv.indexOf("--url");
const url = at < 0 ? "http://127.0.0.1:6243/" : process.argv[at + 1];
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const measure = async (where) => {
  let start = Date.now();
  for (let index = 0; index < 40; index++) await page.mouse.move(300 + 200 * Math.cos(index / 3), 500 + 200 * Math.sin(index / 3));
  const single = (Date.now() - start) / 40;
  start = Date.now();
  await page.mouse.move(400, 400, { steps: 40 });
  const stepped = (Date.now() - start) / 40;
  start = Date.now();
  for (let index = 0; index < 20; index++) await page.evaluate(() => new Promise((done) => requestAnimationFrame(() => done(undefined))));
  const frame = (Date.now() - start) / 20;
  process.stdout.write(`${JSON.stringify({ where, moveMs: single, stepMs: stepped, frameMs: frame })}\n`);
};
await page.goto(url);
await page.locator('#quiz-main [data-card="introduction"]').waitFor();
await measure("introduction");
await page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]').click();
await page.locator('#quiz-main [data-card="identity"] input[type="radio"][value="anonymous"]').check();
await page.locator('#quiz-main [data-card="identity"] [data-overview-card-action="primary"]').click();
await page.locator(".pet-layer svg.pet").first().waitFor({ timeout: 60_000 });
await measure("home");
await page.evaluate(() => (window.location.hash = "heating"));
await page.locator('[data-layered-pane="heating"][data-opened]').waitFor();
await measure("heating page");
await browser.close();
