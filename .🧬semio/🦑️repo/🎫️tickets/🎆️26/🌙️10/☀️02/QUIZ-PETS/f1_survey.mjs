/** 📡️ Ticket tool of work package F1: dumps what the pet layer of the running site measures — the `surveyed` payload of the layer's own `survey` with the quiz's selectors (surfaces, keep-outs, walls, fixtures) — on the home overview while the pointer sweeps it (the panorama pans: every card moves) and on a quiz's page, together with the cast on stage and the size of every species, so the core can be played on the real page's boxes.
 *
 * Usage (from the repository root, with the dev stack of `f1_stack.sh` up):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/f1_survey.mjs" [--url http://127.0.0.1:6249] [--out <dir>] [--quiz heating] [--sweeps 24]
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const ticket = dirname(fileURLToPath(import.meta.url));
const url = option("url", "http://127.0.0.1:6249");
const out = resolve(option("out", resolve(ticket, "🗑️generated/f1/survey")));
const quiz = option("quiz", "heating");
const sweeps = Number(option("sweeps", "24"));
const root = resolve(ticket, "../../../../../../..").replaceAll("\\", "/");
mkdirSync(out, { recursive: true });
const petsRoot = resolve(root, "🎓️teaching/🏛️architecture/🐾️pets");
const ensemble = JSON.parse(readFileSync(resolve(petsRoot, "🔣️.json"), "utf8"));
const sizes = Object.fromEntries(ensemble.species.map((path) => JSON.parse(readFileSync(resolve(petsRoot, path), "utf8"))).map((kind) => [kind.id, { size: kind.size, gear: kind.gear, gait: kind.locomotion.gait, grounds: kind.grounds }]));

const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "en-GB" });
const page = await context.newPage();
const errors = [];
page.on("pageerror", (error) => errors.push(error.message));
await page.goto(url);
await page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]').click();
await page.locator('#quiz-main [data-card="identity"] input[type="radio"][value="anonymous"]').check();
await page.locator('#quiz-main [data-card="identity"] [data-overview-card-action="primary"]').click();
await page.locator("[data-layered-overview]").waitFor();
await page.locator(".pet-layer svg.pet").first().waitFor({ timeout: 60_000 });
await page.mouse.move(4, 4);
await page.waitForTimeout(2500);

const measure = (base) =>
  page.evaluate(async (base) => {
    const quizPets = await import(`/@fs/${base}/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🐾️pets/🎪️stage/🟦️.tsx`);
    const target = await import(`/@fs/${base}/🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx`);
    const layer = document.querySelector(".pet-layer");
    const seen = target.survey(document, { surfaces: quizPets.QUIZ_PET_SURFACES, keepouts: `${target.PET_KEEPOUTS}, ${quizPets.QUIZ_PET_KEEPOUTS}`, props: quizPets.QUIZ_PET_PROPS, frame: layer });
    const pets = [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => ({ id: pet.getAttribute("data-pet"), footing: pet.getAttribute("data-pet-footing"), activity: pet.getAttribute("data-pet-activity"), transform: pet.style.transform }));
    return { surveyed: seen, scale: target.petScale(seen.width), pets };
  }, base);

const home = [];
home.push({ pointer: [4, 4], ...(await measure(root)) });
for (let step = 1; step <= sweeps; step++) {
  const x = 1440 * (0.5 + 0.46 * Math.sin((2 * Math.PI * step) / 9));
  const y = 900 * (0.5 + 0.4 * Math.sin((2 * Math.PI * step) / 5 + 0.7));
  await page.mouse.move(x, y, { steps: 6 });
  await page.waitForTimeout(120);
  home.push({ pointer: [Math.round(x), Math.round(y)], ...(await measure(root)) });
}
await page.mouse.move(4, 4);
await page.waitForTimeout(800);
await page.screenshot({ path: resolve(out, "home.png") });
writeFileSync(resolve(out, "home.json"), `${JSON.stringify({ sizes, samples: home }, null, 1)}\n`);

await page.evaluate((id) => (window.location.hash = id), quiz);
await page.locator(`[data-layered-pane="${quiz}"][data-opened]`).waitFor();
await page.mouse.move(4, 4);
await page.waitForTimeout(2500);
const pageSample = await measure(root);
await page.screenshot({ path: resolve(out, `${quiz}.png`) });
writeFileSync(resolve(out, `${quiz}.json`), `${JSON.stringify({ sizes, sample: pageSample }, null, 1)}\n`);
const summary = (sample) => ({ width: sample.surveyed.width, height: sample.surveyed.height, scale: sample.scale, surfaces: sample.surveyed.surfaces.length, keepouts: sample.surveyed.keepouts.length, walls: sample.surveyed.walls.length, fixtures: sample.surveyed.fixtures.length, pets: sample.pets.map((pet) => `${pet.id}:${pet.footing}`) });
process.stdout.write(`${JSON.stringify({ home: summary(home[0]), moved: home.map((sample) => sample.surveyed.surfaces[0]?.x0), page: summary(pageSample), errors })}\n`);
await browser.close();
