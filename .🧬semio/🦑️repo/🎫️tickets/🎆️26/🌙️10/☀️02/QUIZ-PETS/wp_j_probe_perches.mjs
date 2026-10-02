/** 🪺️ Ticket tool of work package J: enters the dev site as an anonymous learner and prints, for the home screen (and
 * optionally an opened quiz page or a run), what the pets' own survey sees — surfaces, the keep-outs that reach into the
 * band above each of them, and the perches the core cuts from them for the cast on stage. The dev server serves the
 * product modules by path, so the page computes it with the very functions the layer uses.
 *
 * Usage (from the repository root):
 *   node ".../wp_j_probe_perches.mjs" --url http://127.0.0.1:6191 [--width 1440] [--height 900] [--quiz heating] [--run 1]
 */
import { resolve } from "node:path";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const url = option("url", "http://127.0.0.1:6191");
const width = Number(option("width", "1440"));
const height = Number(option("height", "900"));
const quiz = option("quiz", "");
const run = option("run", "") === "1";
const repo = resolve(".").replaceAll("\\", "/");

const probe = async (root) => {
  const target = await import(`/@fs/${root}/🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx`);
  const core = await import(`/@fs/${root}/🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/🟦️.ts`);
  const menagerie = (await import(`/@fs/${root}/🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts`)).ARCHITECTURE_MENAGERIE;
  const layer = document.querySelector(".pet-layer");
  const seen = target.survey(document, { surfaces: "#quiz-main [data-card]", keepouts: `${target.PET_KEEPOUTS}, [data-quiz-item], [data-quiz-drop]`, frame: layer });
  const onStage = [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => pet.getAttribute("data-pet"));
  const wanted = onStage.length > 0 ? onStage : menagerie.casts[0].core.slice(0, target.petCapacity(seen.width));
  const kinds = menagerie.species.filter((species) => wanted.includes(species.id));
  const tallest = Math.max(0, ...kinds.map((species) => species.size.height + (species.locomotion.hover ?? 0)));
  const widest = Math.max(0, ...kinds.map((species) => species.size.width));
  const perches = core.perchesOf(seen.surfaces, seen.keepouts, seen.width, seen.height, tallest, 1.5 * widest);
  const names = new Map([...document.querySelectorAll("#quiz-main [data-card]")].map((element) => [target.surfaceId(element), element.getAttribute("data-card")]));
  const round = (value) => Math.round(value);
  const shownPart = (element) => {
    let box = element.getBoundingClientRect();
    let [left, top, right, bottom] = [box.left, box.top, box.right, box.bottom];
    for (let node = element.parentElement; node !== null && node !== document.body; node = node.parentElement) {
      const style = getComputedStyle(node);
      if (style.overflowX === "visible" && style.overflowY === "visible") continue;
      box = node.getBoundingClientRect();
      [left, top, right, bottom] = [Math.max(left, box.left), Math.max(top, box.top), Math.min(right, box.right), Math.min(bottom, box.bottom)];
    }
    return right > left && bottom > top ? `${round(left)}..${round(right)} @${round(top)}..${round(bottom)}` : "nothing";
  };
  const floorBlockers = [...document.querySelectorAll(`${target.PET_KEEPOUTS}, [data-quiz-item], [data-quiz-drop]`)]
    .filter((element) => element.closest("[inert], [hidden]") === null && element.querySelector("#quiz-main [data-card]") === null && !element.matches("#quiz-main [data-card]"))
    .flatMap((element) => {
      const box = element.getBoundingClientRect();
      if (box.width <= 0 || box.height <= 0 || box.top - 4 >= seen.height || box.bottom + 4 <= seen.height - tallest || box.right + 4 <= 0 || box.left - 4 >= seen.width) return [];
      return [{ tag: element.tagName.toLowerCase(), text: (element.textContent ?? "").trim().slice(0, 30), box: `${round(box.left)}..${round(box.right)} @${round(box.top)}..${round(box.bottom)}`, shown: shownPart(element) }];
    });
  return {
    floorBlockers,
    stage: { width: seen.width, height: seen.height, tallest, widest, minimum: 1.5 * widest, onStage },
    surfaces: seen.surfaces.map((surface) => {
      const blocking = seen.keepouts.filter((box) => box.y < surface.y && box.y + box.height > surface.y - tallest && box.x < surface.x1 && box.x + box.width > surface.x0);
      return {
        card: names.get(surface.id) ?? surface.id,
        x0: round(surface.x0),
        x1: round(surface.x1),
        y: round(surface.y),
        tooHigh: surface.y < tallest,
        blockers: blocking.length,
        blocked: blocking.slice(0, 6).map((box) => `${round(box.x)}..${round(box.x + box.width)} @${round(box.y)}..${round(box.y + box.height)}`),
        perches: perches.filter((perch) => perch.surface === surface.id).map((perch) => `${round(perch.x0)}..${round(perch.x1)}`),
      };
    }),
    keepouts: seen.keepouts.length,
  };
};

const browser = await chromium.launch();
try {
  const context = await browser.newContext({ viewport: { width, height }, locale: "en-GB" });
  const page = await context.newPage();
  const front = (card) => page.locator(`#quiz-main [data-card="${card}"]`).and(page.locator(":not([inert] *)"));
  const primary = (scope) => scope.locator('[data-overview-card-action="primary"]');
  await page.goto(url, { waitUntil: "domcontentloaded" });
  await front("introduction").waitFor({ timeout: 150_000 });
  await primary(front("introduction")).click();
  await front("identity").locator('input[type="radio"][value="anonymous"]').check();
  await primary(front("identity")).click();
  await page.locator("[data-layered-overview]").waitFor();
  await page.locator(".pet-layer").waitFor({ state: "attached", timeout: 30_000 });
  await page.waitForTimeout(4000);
  const report = { home: await page.evaluate(probe, repo) };
  if (quiz !== "") {
    await page.evaluate((id) => (window.location.hash = id), quiz);
    await page.locator(`[data-layered-pane="${quiz}"][data-opened]`).waitFor();
    await page.waitForTimeout(5000);
    report.page = await page.evaluate(probe, repo);
    if (run) {
      await page.keyboard.press("Escape");
      await primary(page.locator(`[data-layered-card="${quiz}"]`)).click();
      await front("run").waitFor();
      await page.waitForTimeout(4000);
      report.run = await page.evaluate(probe, repo);
    }
  }
  process.stdout.write(`${JSON.stringify(report, null, 1)}\n`);
} finally {
  await browser.close();
}
