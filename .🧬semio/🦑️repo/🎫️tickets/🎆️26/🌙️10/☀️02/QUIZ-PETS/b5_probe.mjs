/** 🔬️ Ticket tool of work package B5: looks at mischief in the running site in headless Chromium — enters as an anonymous learner, makes the pets lively, opens a quiz's page and leaves the pointer in a corner, then waits for a pet to lift a copy of a task row: it records what the page shows (the rows, the card that holds them, every pet's activity and footing, the transparent original and the copy in the layer with their boxes), takes screenshots while the copy is out, takes the row back by pointing at it and records the pusher being thrown off; once more for focus (the row made focusable by the probe).
 *
 * Usage (from the repository root, with the stack of `b5_stack.sh` up):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/b5_probe.mjs" [--url http://127.0.0.1:6247] [--out <dir>] [--quiz heating] [--tempo 1] [--scheme light|dark] [--wait 120]
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const url = option("url", "http://127.0.0.1:6247");
const scheme = option("scheme", "light");
const out = resolve(option("out", `b5-probe-${scheme}`));
const quiz = option("quiz", "heating");
const tempo = option("tempo", "1");
const wait = Number(option("wait", "120")) * 1000;
mkdirSync(out, { recursive: true });
const log = [];
const note = (entry) => {
  log.push({ at: Date.now(), ...entry });
  process.stdout.write(`${JSON.stringify(entry)}\n`);
};

const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "en-GB", colorScheme: scheme === "dark" ? "dark" : "light" });
await context.addInitScript((value) => {
  const mark = () => {
    document.documentElement?.setAttribute("data-pets-tempo", value);
    return document.documentElement !== null;
  };
  if (mark()) return;
  const watch = new MutationObserver(() => mark() && watch.disconnect());
  watch.observe(document, { childList: true });
}, tempo);
const page = await context.newPage();
page.on("console", (message) => message.type() === "error" && note({ console: message.text() }));
page.on("pageerror", (error) => note({ pageerror: error.message }));
await page.goto(url);
await page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]').click();
await page.locator('#quiz-main [data-card="identity"] input[type="radio"][value="anonymous"]').check();
await page.locator('#quiz-main [data-card="identity"] [data-overview-card-action="primary"]').click();
await page.locator("[data-layered-overview]").waitFor();
await page.locator(".pet-layer svg.pet").first().waitFor({ timeout: 60_000 });
await page.evaluate(() => (window.location.hash = "prefs"));
await page.locator('[data-layered-pane="prefs"][data-opened]').waitFor();
await page.locator('[data-layered-pane="prefs"]').getByRole("group", { name: "Pets", exact: true }).getByRole("button").nth(3).click();
await page.evaluate((id) => (window.location.hash = id), quiz);
await page.locator(`[data-layered-pane="${quiz}"][data-opened]`).waitFor();
await page.mouse.move(4, 4);
note({ fine: await page.evaluate(() => window.matchMedia("(pointer: fine)").matches), width: await page.evaluate(() => window.innerWidth) });

const scene = () =>
  page.evaluate(() => {
    const box = (element) => {
      const rect = element.getBoundingClientRect();
      return { x: Math.round(rect.x * 10) / 10, y: Math.round(rect.y * 10) / 10, width: Math.round(rect.width * 10) / 10, height: Math.round(rect.height * 10) / 10 };
    };
    return {
      rows: [...document.querySelectorAll("[data-pet-prop]")].map((row) => ({ key: row.getAttribute("data-pet-prop"), box: box(row), style: row.getAttribute("style") })),
      cards: [...document.querySelectorAll('#quiz-main [data-card] [data-slot="window-chrome-body-surface"], #quiz-main [data-card] [data-slot="window-chrome-chip-cap"]')].filter((part) => part.closest("[inert]") === null).map((part) => ({ card: part.closest("[data-card]")?.getAttribute("data-card"), slot: part.getAttribute("data-slot"), box: box(part) })),
      pets: [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => ({ id: pet.getAttribute("data-pet"), activity: pet.getAttribute("data-pet-activity"), footing: pet.getAttribute("data-pet-footing"), mood: pet.getAttribute("data-pet-mood"), box: box(pet) })),
      copies: [...document.querySelectorAll(".pet-layer > [inert][aria-hidden='true']")].map((copy) => ({ box: box(copy), transform: copy.style.transform, opacity: copy.style.opacity })),
    };
  });
note({ scene: await scene() });

const lifted = page.locator("[data-pet-prop]").and(page.locator('[style*="opacity: 0"]'));
const copy = page.locator(".pet-layer > [inert][aria-hidden='true']");
for (const [round, reclaim] of [[1, "hover"], [2, "focus"]]) {
  const started = Date.now();
  let seen = false;
  while (Date.now() - started < wait) {
    if ((await lifted.count()) === 1 && (await copy.count()) === 1) {
      seen = true;
      break;
    }
    await page.waitForTimeout(100);
  }
  note({ round, lifted: seen, seconds: (Date.now() - started) / 1000 });
  if (!seen) {
    note({ scene: await scene() });
    await page.screenshot({ path: resolve(out, `${round}-nothing.png`) });
    continue;
  }
  await page.waitForTimeout(Number(tempo) >= 4 ? 120 : 1200);
  const shown = await scene();
  note({ round, shown });
  await page.screenshot({ path: resolve(out, `${round}-lifted.png`) });
  const original = await lifted.boundingBox();
  if (original !== null) await page.screenshot({ path: resolve(out, `${round}-lifted-close.png`), clip: { x: Math.max(original.x - 160, 0), y: Math.max(original.y - 120, 0), width: Math.min(original.width + 320, 1440), height: 280 } });
  const pusher = shown.pets.find((pet) => pet.activity === "push");
  note({ round, pusher: pusher ?? null });
  if (reclaim === "hover") await page.mouse.move(original.x + original.width / 2, original.y + original.height / 2);
  else await lifted.evaluate((element) => (element.matches("a[href], button, input, select, textarea, [tabindex]") ? element : (element.querySelector("a[href], button, input, select, textarea, [tabindex]") ?? Object.assign(element, { tabIndex: -1 }))).focus());
  const back = Date.now();
  while (Date.now() - back < 2000 && ((await lifted.count()) > 0 || (await copy.count()) > 0)) await page.waitForTimeout(16);
  note({ round, reclaim, restoredWithin: Date.now() - back, lifted: await lifted.count(), copies: await copy.count() });
  await page.waitForTimeout(150);
  const after = await scene();
  note({ round, after: after.pets });
  await page.screenshot({ path: resolve(out, `${round}-reclaimed.png`) });
  await page.waitForTimeout(2500);
  note({ round, later: (await scene()).pets });
  await page.screenshot({ path: resolve(out, `${round}-later.png`) });
  if (reclaim === "focus") await page.evaluate(() => document.activeElement?.blur());
  await page.mouse.move(4, 4);
}
writeFileSync(resolve(out, "report.json"), `${JSON.stringify(log, null, 2)}\n`);
await browser.close();
