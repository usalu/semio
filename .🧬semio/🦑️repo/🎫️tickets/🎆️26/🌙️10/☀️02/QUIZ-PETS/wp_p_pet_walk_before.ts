/** 🐕️ The pets of the site in a real browser: they show up on the home screen within seconds, as decoration that is
 * hidden from assistive technology and never takes a click from a card or a control; the cast follows the learner
 * (the species of the quiz whose page or run is on screen, the home cast everywhere else) and rests during a run; the
 * learner's preference stops them (`still`) or removes them (`off`), and a learner who prefers reduced motion gets them
 * motionless. They stand on what the cards show — the tab, the edge of the body beside it, the footer line —, keep
 * their distance from each other and from a task, turn see-through while the pointer rests on them, and on a phone at
 * most two small ones show up. Every device stays clean throughout: no console error, no failed request, no policy
 * violation.
 * @see ../../../🐾️pets/🔣️.json — the ensemble whose casts are expected on screen
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive
 * @see ../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🐾️pets/🟦️.tsx — the quiz's glue to the pets
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html — why `still` and `off` exist */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Page } from "@playwright/test";
import { QUIZZES, card, enter, expect, pane, playQuiz, primary, screen, test, type Device, type Text } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🚶️learner/🟦️.ts";

/** 🎟️ A cast as the ensemble authors it. */
interface Cast {
  readonly scene: string;
  readonly core: readonly string[];
  readonly rotation: readonly string[];
}

/** 📦️ What the walk needs of a species as authored: the box of its body and how high it floats above what it stands on. */
interface Build {
  readonly width: number;
  readonly height: number;
  readonly hover: number;
}

const MENAGERIE = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../🎓️teaching/🏛️architecture/🐾️pets");
const ENSEMBLE = JSON.parse(readFileSync(resolve(MENAGERIE, "🔣️.json"), "utf8")) as { readonly casts: readonly Cast[]; readonly species: readonly string[] };
const CASTS = ENSEMBLE.casts;
const BUILDS: Readonly<Record<string, Build>> = Object.fromEntries(
  ENSEMBLE.species.map((path) => {
    const species = JSON.parse(readFileSync(resolve(MENAGERIE, path), "utf8")) as { readonly id: string; readonly size: { readonly width: number; readonly height: number }; readonly locomotion: { readonly gait: string; readonly hover?: number } };
    return [species.id, { width: species.size.width, height: species.size.height, hover: species.locomotion.gait === "float" ? (species.locomotion.hover ?? 0) : 0 }];
  }),
);

/** 🃏️ The parts of a card of the screen in front: its tab and its body. */
const CARD_PARTS = '#quiz-main [data-card] [data-slot="window-chrome-chip-cap"], #quiz-main [data-card] [data-slot="window-chrome-body-surface"]';

/** 🧱️ What pets stand on in the quiz, as the client names it for its pet layer: the parts of every card, and the
 * footer line. */
const EDGES = `${CARD_PARTS}, .quiz-app > footer`;

/** 🫥️ How visible a pet is while the pointer rests on it. */
const SEE_THROUGH = 0.35;

/** 🏡️ The scene of every screen that belongs to no quiz. */
const HOME = "home";

/** 🎚️ What a learner may choose for the pets, in the order the preferences offer it. */
const CHOICES = ["off", "still", "calm", "lively"] as const;

/** 🏷️ The name of the pets' choice in the preferences (`quiz.preferences.pets`). */
const PETS_LABEL: Text = { en: "Pets", de: "Tierchen" };

/** ⏱️ How long the first pet may take to show once the home screen is there: "within a few seconds". */
const APPEAR_MS = 10_000;

/** 🚶️ How long a change of cast may take: the pets that leave walk off and fade, the new ones fade in. */
const RECAST_MS = 45_000;

function castOf(scene: string): Cast {
  const cast = CASTS.find((candidate) => candidate.scene === scene);
  if (cast === undefined) throw new Error(`the ensemble has no cast for ${scene}`);
  return cast;
}

function layer(device: Device): ReturnType<Device["page"]["locator"]> {
  return device.page.locator(".pet-layer");
}

function pets(device: Device): ReturnType<Device["page"]["locator"]> {
  return device.page.locator(".pet-layer svg.pet");
}

/** 🐾️ The species on screen, by id. */
async function shownPets(device: Device): Promise<readonly string[]> {
  return pets(device).evaluateAll((drawn) => drawn.map((pet) => pet.getAttribute("data-pet") ?? ""));
}

/** 📍️ Where every pet on screen stands. */
async function places(device: Device): Promise<string> {
  return (await pets(device).evaluateAll((drawn) => drawn.map((pet) => `${pet.getAttribute("data-pet")} ${(pet as SVGElement).style.transform}`))).join("\n");
}

/** 🎞️ Everything the last frame wrote: where every pet stands, how visible it is, and every bone, pupil, lid and mouth. */
async function drawing(device: Device): Promise<string> {
  const frames = await pets(device).evaluateAll((drawn) =>
    drawn.map((pet) => [pet.getAttribute("data-pet"), (pet as SVGElement).style.transform, (pet as SVGElement).style.opacity, ...[...pet.querySelectorAll("*")].map((node) => ["transform", "d", "cx", "cy"].map((name) => node.getAttribute(name) ?? "").join(","))].join("|")),
  );
  return frames.join("\n");
}

/** 🗿️ Checks that pets are on screen and do not move at all: three drawings, a second apart, are the same. */
async function expectMotionless(device: Device): Promise<void> {
  await expect(async () => {
    const first = await drawing(device);
    expect(first).not.toBe("");
    for (let again = 0; again < 2; again++) {
      await device.page.waitForTimeout(1_000);
      expect(await drawing(device)).toBe(first);
    }
  }).toPass({ timeout: RECAST_MS });
}

/** 🏃️ Checks that the pets on screen are alive: the drawing changes. */
async function expectMoving(device: Device): Promise<void> {
  const first = await drawing(device);
  await expect.poll(() => drawing(device)).not.toBe(first);
}

/** 🎭️ Waits until the pets on screen are those of `scene`: nobody of another cast is left, and — where `whole` — its
 * whole core is there. */
async function expectCast(device: Device, scene: string, whole: boolean): Promise<void> {
  const cast = castOf(scene);
  const allowed = scene === HOME ? cast.core : [...cast.core, ...cast.rotation];
  await expect
    .poll(
      async () => {
        const shown = await shownPets(device);
        return { strangers: shown.filter((id) => !allowed.includes(id)), missing: whole ? cast.core.filter((id) => !shown.includes(id)) : [], twice: shown.length - new Set(shown).size };
      },
      { timeout: RECAST_MS },
    )
    .toEqual({ strangers: [], missing: [], twice: 0 });
}

/** 🎯️ The controls, cards and pets of the page whose place would hand a click to the pet layer: the element under the
 * middle of each must never belong to it. */
async function takenTargets(device: Device): Promise<readonly string[]> {
  return device.page.evaluate(() => {
    const under = (x: number, y: number): boolean => x >= 0 && y >= 0 && x < window.innerWidth && y < window.innerHeight && document.elementFromPoint(x, y)?.closest(".pet-layer") != null;
    const taken: string[] = [];
    for (const target of document.querySelectorAll("[data-overview-card], a[href], button, input, select, summary")) {
      if (target.closest("[inert], [hidden]") !== null) continue;
      const box = target.getBoundingClientRect();
      if (box.width > 0 && box.height > 0 && under(box.left + box.width / 2, box.top + box.height / 2)) taken.push(`${target.tagName.toLowerCase()} ${(target.textContent ?? "").trim().slice(0, 40)}`);
    }
    for (const pet of document.querySelectorAll(".pet-layer svg.pet")) {
      const boxes = [...pet.children].map((part) => part.getBoundingClientRect()).filter((box) => box.width > 0 && box.height > 0);
      if (boxes.length === 0) continue;
      const [left, right, top, bottom] = [Math.min(...boxes.map((box) => box.left)), Math.max(...boxes.map((box) => box.right)), Math.min(...boxes.map((box) => box.top)), Math.max(...boxes.map((box) => box.bottom))];
      if (under((left + right) / 2, (top + bottom) / 2)) taken.push(`pet ${pet.getAttribute("data-pet")}`);
    }
    return taken;
  });
}

/** 🚪️ Enters the site anonymously and waits for the first pets on the home screen. */
async function enterWithPets(device: Device): Promise<void> {
  await enter(device, { kind: "anonymous" });
  await expect(pets(device)).not.toHaveCount(0, { timeout: APPEAR_MS });
}

/** 🧍️ A pet as painted: its feet, the size it is drawn at and how visible it is. */
interface Standing {
  readonly id: string;
  readonly x: number;
  readonly y: number;
  readonly size: number;
  readonly opacity: number;
}

/** 🗺️ Where every pet of a page stands. */
async function standing(page: Page): Promise<readonly Standing[]> {
  return page.locator(".pet-layer svg.pet").evaluateAll((drawn) =>
    drawn.map((pet) => {
      const place = /translate\(([-\d.]+)px, ([-\d.]+)px\) scale\(([-\d.]+), ([-\d.]+)\)/u.exec((pet as SVGElement).style.transform);
      return { id: pet.getAttribute("data-pet") ?? "", x: Number(place?.[1]), y: Number(place?.[2]), size: Number(place?.[4]), opacity: Number((pet as SVGElement).style.opacity || "1") };
    }),
  );
}

/** ✂️ In the page: what one sees of the elements `selector` matches — each box cut down to what its scrolling and
 * clipping ancestors let through, without anything inert, hidden or scrolled away — and whether its top edge shows. */
function seenBoxes(selector: string): { left: number; top: number; right: number; bottom: number; topped: boolean }[] {
  return [...document.querySelectorAll(selector)].flatMap((element) => {
    if (element.closest("[inert], [hidden], .pet-layer") !== null) return [];
    const box = element.getBoundingClientRect();
    let [left, top, right, bottom] = [Math.max(box.left, 0), Math.max(box.top, 0), Math.min(box.right, window.innerWidth), Math.min(box.bottom, window.innerHeight)];
    for (let node = element.parentElement; node !== null && node !== document.body; node = node.parentElement) {
      const style = getComputedStyle(node);
      if (style.overflowX === "visible" && style.overflowY === "visible") continue;
      const clip = node.getBoundingClientRect();
      [left, top, right, bottom] = [Math.max(left, clip.left), Math.max(top, clip.top), Math.min(right, clip.right), Math.min(bottom, clip.bottom)];
    }
    return right > left && bottom > top ? [{ left, top, right, bottom, topped: top <= box.top }] : [];
  });
}

/** 👣️ What is wrong with where the pets of a page stand, one line each; none when every pet that is whole stands with
 * its feet on an edge the screen shows (the top of a tab, of a body or of the footer; a floater its hover above it),
 * no two bodies on one edge overlap, and no body covers a control or stands in front of a card. */
async function misplaced(page: Page): Promise<readonly string[]> {
  const shown = (await standing(page)).filter((pet) => pet.opacity > 0.99 || pet.opacity === SEE_THROUGH);
  const edges = (await page.evaluate(seenBoxes, EDGES)).filter((edge) => edge.topped);
  const kept = await page.evaluate(seenBoxes, `a[href], button, input, select, textarea, summary, ${CARD_PARTS}`);
  const wrong: string[] = [];
  for (const pet of shown) {
    const build = BUILDS[pet.id];
    if (build === undefined) {
      wrong.push(`${pet.id} is no species of the ensemble`);
      continue;
    }
    const feet = pet.y + build.hover * pet.size;
    const half = (build.width * pet.size) / 2;
    if (!edges.some((edge) => Math.abs(edge.top - feet) <= 1.5 && pet.x >= edge.left - 1 && pet.x <= edge.right + 1)) wrong.push(`${pet.id} at ${pet.x}, ${pet.y} stands on no edge`);
    for (const other of shown) {
      const mate = BUILDS[other.id];
      if (mate === undefined || other.id <= pet.id) continue;
      if (Math.abs(other.y + mate.hover * other.size - feet) <= 1.5 && Math.abs(other.x - pet.x) < ((build.width + mate.width) / 2) * pet.size - 1) wrong.push(`${pet.id} stands in ${other.id}`);
    }
    for (const box of kept) if (Math.min(pet.x + half, box.right) - Math.max(pet.x - half, box.left) > 1 && Math.min(pet.y, box.bottom) - Math.max(pet.y - build.height * pet.size, box.top) > 1) wrong.push(`${pet.id} at ${pet.x}, ${pet.y} covers something it must keep clear of`);
  }
  return wrong;
}

test("pets live on the home screen as decoration that never takes a click", async ({ device }) => {
  const learner = await device("en");
  await enterWithPets(learner);
  await expect(learner.page.locator(".quiz-app")).toHaveAttribute("data-pets", "calm");
  await expect(layer(learner)).toHaveCount(1);
  await expect(layer(learner)).toHaveAttribute("aria-hidden", "true");
  await expect(layer(learner)).toHaveCSS("pointer-events", "none");
  await expect(layer(learner).locator("a, button, input, select, textarea, summary, [tabindex], [role], [id], style, script")).toHaveCount(0);
  await expectCast(learner, HOME, false);
  expect((await shownPets(learner)).length).toBeGreaterThan(0);
  await expectMoving(learner);
  expect(await takenTargets(learner)).toEqual([]);

  await primary(card(learner.page, "board")).click();
  await expect(pane(learner.page, "board")).toHaveAttribute("data-opened", "");
  expect(await takenTargets(learner)).toEqual([]);
  await learner.page.locator("[data-layered-overview-button]").click();
  await expect(pane(learner.page, "board")).not.toHaveAttribute("data-opened", "");
  await expect(pets(learner)).not.toHaveCount(0);
  expect(await learner.page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(0);
});

test("the cast follows the learner: a quiz's page and run show its species, the home screen its own again", async ({ device }) => {
  const learner = await device("de");
  await enterWithPets(learner);
  for (const quiz of QUIZZES) {
    await learner.page.evaluate((id) => (window.location.hash = id), quiz.id);
    await expect(pane(learner.page, quiz.id)).toHaveAttribute("data-opened", "");
    await expectCast(learner, quiz.id, true);
    expect(await takenTargets(learner)).toEqual([]);
    await learner.page.keyboard.press("Escape");
    await expect(pane(learner.page, quiz.id)).not.toHaveAttribute("data-opened", "");
    await expectCast(learner, HOME, false);
    await expect(pets(learner)).not.toHaveCount(0);
  }

  const played = QUIZZES[0]!;
  await playQuiz(learner, played.id);
  await expectCast(learner, played.id, false);
  await expect(async () => {
    const first = await places(learner);
    for (let again = 0; again < 3; again++) {
      await learner.page.waitForTimeout(1_000);
      expect(await places(learner), "pets stay where they are while a run is on screen").toBe(first);
    }
  }).toPass({ timeout: RECAST_MS });
  expect(await takenTargets(learner)).toEqual([]);
  await expect(screen(learner.page, "run")).toBeVisible();
});

test("the learner's choice stops the pets, removes them and brings them back, and is kept", async ({ device }) => {
  const learner = await device("en");
  await enterWithPets(learner);
  const app = learner.page.locator(".quiz-app");
  const preferences = pane(learner.page, "prefs");
  const choices = preferences.getByRole("group", { name: PETS_LABEL[learner.locale], exact: true }).getByRole("button");
  const choose = async (choice: (typeof CHOICES)[number]): Promise<void> => {
    await choices.nth(CHOICES.indexOf(choice)).click();
    await expect(choices.nth(CHOICES.indexOf(choice))).toHaveAttribute("aria-pressed", "true");
    await expect(app).toHaveAttribute("data-pets", choice);
  };
  await learner.page.evaluate(() => (window.location.hash = "prefs"));
  await expect(preferences).toHaveAttribute("data-opened", "");
  await expect(choices).toHaveCount(CHOICES.length);
  await expect(choices.nth(CHOICES.indexOf("calm"))).toHaveAttribute("aria-pressed", "true");
  await expectMoving(learner);

  await choose("still");
  await expect(pets(learner)).not.toHaveCount(0);
  await expectMotionless(learner);

  await choose("off");
  await expect(layer(learner)).toHaveCount(0);
  await expect(learner.page.locator("svg.pet")).toHaveCount(0);

  await learner.page.reload();
  await expect(preferences).toHaveAttribute("data-opened", "");
  await expect(app).toHaveAttribute("data-pets", "off");
  await expect(choices.nth(CHOICES.indexOf("off"))).toHaveAttribute("aria-pressed", "true");
  await expect(layer(learner)).toHaveCount(0);

  await choose("lively");
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });
  await expect(layer(learner)).toHaveAttribute("aria-hidden", "true");
  await expectMoving(learner);
});

test("a learner who prefers reduced motion gets motionless pets", async ({ device }) => {
  const learner = await device("de");
  await learner.page.emulateMedia({ reducedMotion: "reduce" });
  await enterWithPets(learner);
  await expect(learner.page.locator(".quiz-app")).toHaveAttribute("data-pets", "calm");
  await expectCast(learner, HOME, false);
  await expectMotionless(learner);
  await learner.page.mouse.move(400, 300, { steps: 5 });
  await learner.page.mouse.move(900, 600, { steps: 5 });
  await expectMotionless(learner);
  expect(await takenTargets(learner)).toEqual([]);

  await learner.page.emulateMedia({ reducedMotion: "no-preference" });
  await expectMoving(learner);
});

test("pets stand on what the cards show, keep their distance and stay beside a task", async ({ device }) => {
  const learner = await device("en");
  await enterWithPets(learner);
  await expect.poll(() => misplaced(learner.page), { timeout: RECAST_MS }).toEqual([]);
  await learner.page.reload();
  await expect(learner.page.locator("[data-layered-overview]")).toBeVisible();
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });
  await expect.poll(() => misplaced(learner.page), { timeout: RECAST_MS }).toEqual([]);
  const footer = await learner.page.locator(".quiz-app > footer").evaluate((line) => line.getBoundingClientRect().top);
  await expect
    .poll(async () => (await standing(learner.page)).filter((pet) => pet.opacity > 0.99 && pet.y < footer - 40).length, { message: "a returning learner finds pets on the cards, not only on the footer line", timeout: RECAST_MS })
    .toBeGreaterThan(0);
  for (let look = 0; look < 10; look++) {
    await learner.page.waitForTimeout(500);
    expect((await misplaced(learner.page)).filter((line) => line.includes(" stands in "))).toEqual([]);
  }

  const played = QUIZZES[0]!;
  await playQuiz(learner, played.id);
  await expectCast(learner, played.id, false);
  await expect.poll(() => misplaced(learner.page), { timeout: RECAST_MS }).toEqual([]);
  for (let look = 0; look < 6; look++) {
    await learner.page.waitForTimeout(500);
    expect(await misplaced(learner.page)).toEqual([]);
  }
  expect(await takenTargets(learner)).toEqual([]);
});

test("a pet turns see-through while the pointer rests on it and whole again when the pointer leaves", async ({ device }) => {
  const learner = await device("en");
  await enterWithPets(learner);
  await expectCast(learner, HOME, false);
  let chosen = "";
  await expect(async () => {
    const pet = (await standing(learner.page)).find((candidate) => candidate.opacity > 0.99);
    expect(pet).toBeDefined();
    chosen = pet!.id;
    await learner.page.mouse.move(pet!.x, pet!.y - (BUILDS[chosen]!.height * pet!.size) / 2, { steps: 4 });
    await expect.poll(async () => (await standing(learner.page)).find((candidate) => candidate.id === chosen)?.opacity, { timeout: 3_000 }).toBe(SEE_THROUGH);
  }).toPass({ timeout: RECAST_MS });
  expect((await standing(learner.page)).filter((pet) => pet.id !== chosen && pet.opacity === SEE_THROUGH)).toEqual([]);
  await learner.page.mouse.move(5, 5, { steps: 4 });
  await expect.poll(async () => (await standing(learner.page)).find((candidate) => candidate.id === chosen)?.opacity ?? 1, { timeout: 5_000 }).toBe(1);
  expect(await takenTargets(learner)).toEqual([]);
});

test("a phone shows at most two small pets, on edges it shows and never on a control", async ({ browser }, info) => {
  const context = await browser.newContext({ viewport: { width: 375, height: 812 }, isMobile: true, hasTouch: true, baseURL: info.project.use.baseURL, locale: "en-GB" });
  const page = await context.newPage();
  const problems: string[] = [];
  page.on("console", (message) => {
    if (message.type() === "error") problems.push(`console error: ${message.text()}`);
  });
  page.on("pageerror", (error) => problems.push(`page error: ${error.message}`));
  const phone: Device = { context, page, locale: "en", problems, expectingFailures: async () => 0 };
  const few = async (): Promise<void> => {
    const shown = await standing(page);
    expect(shown.length, "a newcomer waits until whoever it replaces is gone").toBeLessThanOrEqual(2);
    for (const pet of shown) expect(pet.size).toBe(0.8);
  };
  await enter(phone, { kind: "anonymous" });
  await expect(page.locator("[data-layered-overview]")).toHaveAttribute("data-mode", "list");
  await expect(pets(phone)).not.toHaveCount(0, { timeout: APPEAR_MS });
  await expect.poll(() => misplaced(page), { timeout: RECAST_MS }).toEqual([]);
  await few();

  const quiz = QUIZZES[0]!;
  await page.evaluate((id) => (window.location.hash = id), quiz.id);
  await expect(pane(page, quiz.id)).toHaveAttribute("data-opened", "");
  await expectCast(phone, quiz.id, false);
  await expect(pets(phone)).not.toHaveCount(0, { timeout: RECAST_MS });
  await expect.poll(() => misplaced(page), { timeout: RECAST_MS }).toEqual([]);
  await few();

  await page.keyboard.press("Escape");
  await expect(pane(page, quiz.id)).not.toHaveAttribute("data-opened", "");
  await card(page, quiz.id).scrollIntoViewIfNeeded();
  await playQuiz(phone, quiz.id);
  await expect.poll(() => misplaced(page), { timeout: RECAST_MS }).toEqual([]);
  await few();
  expect(await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(0);
  await context.close();
  expect(problems).toEqual([]);
});
