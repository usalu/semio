/** 📸️ Plays the physics quiz on each challenge and saves what a learner sees, on a desktop and a phone: the chooser on
 * the quiz page, an easy hint, the key ladder on medium, the guess fields and the results on hard, and an expert task
 * before and after its clock starts and at time up. Each shot also reports elements wider than the window.
 * @see ./challenge_shots.config.ts
 * @see ../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🚶️learner/🟦️.ts */
import { mkdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import {
  FAR_OFF,
  answerRun,
  ascending,
  chooseChallenge,
  classify,
  enter,
  expect,
  guessSorting,
  handle,
  itemOf,
  match,
  openTask,
  openTaskById,
  pane,
  playQuiz,
  primary,
  quizOf,
  screen,
  shownHints,
  shownItems,
  shownQuantity,
  sortInto,
  startClock,
  submitRun,
  swapExtremes,
  taskOf,
  test,
  type Device,
} from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🚶️learner/🟦️.ts";

const SHOTS = resolve(dirname(fileURLToPath(import.meta.url)), "🗑️generated", "integration", "shots");
mkdirSync(SHOTS, { recursive: true });
const physics = quizOf("physics");
const powers = taskOf(physics, "powers");
const heating = quizOf("heating");
const loads = taskOf(heating, "heating-load-and-demand");
const demand = quizOf("demand");
const profiles = taskOf(demand, "standard-profiles");

/** 📸️ Saves the window as `<project>-<name>.png` with `focus` scrolled into view, and reports what sticks out of it. */
async function shot(device: Device, project: string, name: string, focus?: ReturnType<typeof screen>): Promise<void> {
  if (focus !== undefined) await focus.scrollIntoViewIfNeeded();
  await device.page.waitForTimeout(400);
  await device.page.screenshot({ path: resolve(SHOTS, `${project}-${name}.png`) });
  const wide = await device.page.evaluate(() => {
    const width = document.documentElement.clientWidth;
    return [...document.querySelectorAll<HTMLElement>("#quiz-main *")]
      .filter((element) => element.getClientRects().length > 0 && !element.closest("[inert]"))
      .flatMap((element) => {
        const box = element.getBoundingClientRect();
        const clipped = element.scrollWidth > element.clientWidth + 1 && getComputedStyle(element).overflowX === "visible" && element.clientWidth > 0;
        return box.right > width + 1 || box.left < -1 || clipped ? [`${element.tagName.toLowerCase()}.${String(element.className).slice(0, 40)} [${Math.round(box.left)}..${Math.round(box.right)}] scroll ${element.scrollWidth}/${element.clientWidth} "${(element.innerText ?? "").slice(0, 40).replace(/\s+/gu, " ")}"`] : [];
      })
      .slice(0, 12);
  });
  console.log(`[DEBUG] ${project}-${name}: page ${await device.page.evaluate(() => `${document.documentElement.scrollWidth}/${document.documentElement.clientWidth}`)}${wide.length === 0 ? "" : `\n  ${wide.join("\n  ")}`}`);
}

/** 💬️ Waits until the task on screen questions exactly `items` and logs the questions. */
async function questioned(learner: Device, project: string, name: string, items: readonly string[]): Promise<void> {
  await expect.poll(async () => Object.keys(await shownHints(learner)).sort()).toEqual([...items].sort());
  for (const [item, hint] of Object.entries(await shownHints(learner))) console.log(`[DEBUG] ${project}-${name} ${item}: ${hint.question}`);
}

/** 🤨️ On the physics `powers` sorting: exchanges the two largest where they lie farther apart than the reach (the
 * questions with a count in words, "together only add up to" and "it takes") and saves `name` with the largest in view;
 * then exchanges the extremes (the order questions) and saves `<name>-tail` with the smallest in view. */
async function easyHint(learner: Device, project: string, name: string): Promise<void> {
  await openTaskById(learner, powers.id);
  const order = ascending(powers, await shownItems(learner));
  const value = (id: string): number => itemOf(powers, id).value!;
  const n = order.length;
  const [smallest, next, largest] = [order[0]!, order[n - 2]!, order[n - 1]!];
  const reach = Math.min(Math.sqrt(value(largest) / value(smallest)), 1000) * (1 + 1e-9);
  const pair = value(largest) / value(next) > reach;
  if (pair) {
    await sortInto(learner, [...order.slice(0, -2), largest, next]);
    await questioned(learner, project, name, [largest, next]);
    await shot(learner, project, name, screen(learner.page, "task").locator(`[data-quiz-item="${largest}"]`));
  }
  await sortInto(learner, [largest, ...order.slice(1, -1), smallest]);
  await questioned(learner, project, name, [largest, smallest]);
  await shot(learner, project, pair ? `${name}-tail` : name, screen(learner.page, "task").locator(`[data-quiz-item="${pair ? smallest : largest}"]`));
}

/** 🃏️ On the heating matching of two quantities: every true card, then the extremes' cards exchanged in the first
 * quantity ("in puncto Heizlast"); saves `name` with the largest in view. */
async function matchingHint(learner: Device, project: string, name: string): Promise<void> {
  await openTaskById(learner, loads.id);
  const items = await shownItems(learner);
  for (const { id, quantity } of loads.dimensions!) for (const item of items) await match(learner, id, item, await shownQuantity(learner, itemOf(loads, item).values![id]!, quantity));
  const [smallest, largest] = await swapExtremes(learner, loads, 0, items);
  await questioned(learner, project, name, [smallest, largest]);
  await shot(learner, project, name, screen(learner.page, "task").locator(`.quiz-slot[data-presence-anchor="item:${largest}"]`).first());
}

/** 🗂️ On the demand spider profiles: every standard in its own profile, then the first into the profile farthest from
 * its own on some axis relative to that axis's reach; saves `name` with it in view. */
async function profileHint(learner: Device, project: string, name: string): Promise<void> {
  await openTaskById(learner, profiles.id);
  const items = await shownItems(learner);
  for (const item of items) await classify(learner, item, itemOf(profiles, item).category!);
  const categories = profiles.categories!;
  const moved = items[0]!;
  const own = categories.find((category) => category.id === itemOf(profiles, moved).category)!.profile!;
  const reach = (axis: string): number => (Math.max(...categories.map((category) => category.profile![axis]!)) - Math.min(...categories.map((category) => category.profile![axis]!))) / 2;
  const ratio = (category: (typeof categories)[number]): number => Math.max(...profiles.axes!.map((axis) => Math.abs(category.profile![axis.id]! - own[axis.id]!) / reach(axis.id)));
  const target = categories.filter((category) => category.id !== itemOf(profiles, moved).category).reduce((best, category) => (ratio(category) > ratio(best) ? category : best));
  await classify(learner, moved, target.id);
  await questioned(learner, project, name, [moved]);
  await shot(learner, project, name, screen(learner.page, "task").locator(`[data-quiz-item="${moved}"]`));
}

test("the chooser and an easy hint", async ({ device }, info) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Shot Easy") });
  await chooseChallenge(learner, physics.id, "easy");
  await shot(learner, info.project.name, "1-chooser", pane(learner.page, physics.id).locator("fieldset").first());
  await primary(pane(learner.page, physics.id).locator('[data-card="quiz"]')).click();
  await expect(screen(learner.page, "run")).toBeVisible();
  await easyHint(learner, info.project.name, "2-easy-hint");
});

test("an easy hint in German", async ({ device }, info) => {
  const learner = await device("de");
  await enter(learner, { kind: "pseudonym", handle: handle("Shot Leicht") });
  await playQuiz(learner, physics.id, "easy");
  await easyHint(learner, info.project.name, "2-easy-hint-de");
});

test("an easy matching hint in German", async ({ device }, info) => {
  const learner = await device("de");
  await enter(learner, { kind: "pseudonym", handle: handle("Shot Zuordnung") });
  await playQuiz(learner, heating.id, "easy");
  await matchingHint(learner, info.project.name, "2-easy-hint-matching-de");
});

test("an easy profile hint in German", async ({ device }, info) => {
  const learner = await device("de");
  await enter(learner, { kind: "pseudonym", handle: handle("Shot Profil") });
  await playQuiz(learner, demand.id, "easy");
  await profileHint(learner, info.project.name, "2-easy-hint-profile-de");
});

test("the key ladder on medium", async ({ device }, info) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Shot Medium") });
  await playQuiz(learner, physics.id, "medium");
  await openTaskById(learner, powers.id);
  await shot(learner, info.project.name, "3-medium-ladder", screen(learner.page, "task"));
});

test("the guess fields and the results on hard", async ({ device }, info) => {
  const learner = await device("en");
  await enter(learner, { kind: "pseudonym", handle: handle("Shot Hard") });
  await playQuiz(learner, physics.id, "hard");
  await openTaskById(learner, powers.id);
  await shot(learner, info.project.name, "4-hard-guesses", screen(learner.page, "task"));
  await answerRun(learner, physics, "perfect");
  await openTaskById(learner, powers.id);
  const far = ascending(powers, await shownItems(learner))[0]!;
  await guessSorting(learner, far, itemOf(powers, far).value! * FAR_OFF, powers.quantity!);
  await submitRun(learner);
  await shot(learner, info.project.name, "5-hard-results", screen(learner.page, "results"));
  const missed = learner.page.locator("#quiz-main [data-miss]").first();
  if ((await missed.count()) > 0) await shot(learner, info.project.name, "6-hard-results-miss", missed as ReturnType<typeof screen>);
});

test("an expert task before and after its clock starts, and at time up", async ({ device }, info) => {
  const learner = await device("en");
  await learner.page.clock.install();
  await enter(learner, { kind: "pseudonym", handle: handle("Shot Expert") });
  await playQuiz(learner, physics.id, "expert");
  await openTask(learner, 1);
  const task = screen(learner.page, "task");
  await expect(task.locator('[data-clock="closed"]')).toBeVisible();
  const allowed = /(\d+):(\d\d)/u.exec(await task.locator('[data-clock="closed"]').innerText())!;
  await shot(learner, info.project.name, "7-expert-closed", task);
  await startClock(learner);
  await shot(learner, info.project.name, "8-expert-running", task);
  await learner.page.clock.fastForward((Number(allowed[1]) * 60 + Number(allowed[2]) - 8) * 1000);
  await shot(learner, info.project.name, "9-expert-ten", task);
  await learner.page.clock.fastForward(10_000);
  await expect(task.locator('[data-clock="up"]')).toBeVisible();
  await shot(learner, info.project.name, "10-expert-up", task);
});
