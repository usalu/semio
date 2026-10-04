/** 📸️ Evidence for design §20: boots a private throw-away stack (a freshly built proctor on 8895 over its own data
 * directory, the dev site on 6165), lets a crowd of learners submit the physics quiz with perfect, flawed and scrambled
 * answers, and photographs what one more learner sees — the run with the others out of sight, the figure after asking,
 * the results with the score histograms, the quiz's page — in both appearances and at phone width, into
 * `🗑️generated/crowd-plots/`. It never touches the development proctor or its data. `bun crowd_plots_shots.ts [crowd]`. */
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { chromium, type Browser } from "playwright";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";
import { PROCTOR_DEV_CATALOG, buildProctor, launchProctor, proctorReady } from "../../../../../../../🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts";
import { SITE_FIRST_ANSWER_MS, awaitReady, httpAnswers, launchOwned } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🧱️stack/🟦️.ts";
import { answerRun, classify, enter, goHome, handle, moveDown, openTask, pane, playQuiz, quizOf, screen, shownItems, shownTask, submitRun, type Device, type Locale } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🚶️learner/🟦️.ts";

const repoRoot = join(import.meta.dir, "../../../../../../..");
const bundleRoot = join(repoRoot, "🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript");
const out = join(import.meta.dir, "🗑️generated", "crowd-plots");
const [proctorPort, sitePort] = [8895, 6165];
const [proctor, site] = [`http://127.0.0.1:${proctorPort}`, `http://127.0.0.1:${sitePort}`];
const crowd = Number(process.argv[2] ?? 9);
const quiz = quizOf("physics");
rmSync(out, { recursive: true, force: true });
mkdirSync(join(out, "proctor-data"), { recursive: true });
process.env.PLAYWRIGHT_BROWSERS_PATH = repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH;
const say = (line: string): void => console.log(`[DEBUG] ${line}`);
const problems: string[] = [];

async function device(browser: Browser, locale: Locale, viewport: { readonly width: number; readonly height: number }, colorScheme: "light" | "dark"): Promise<Device> {
  const context = await browser.newContext({ baseURL: site, viewport, colorScheme, locale: locale === "en" ? "en-GB" : "de-DE", reducedMotion: "reduce" });
  const page = await context.newPage();
  page.on("pageerror", (error) => problems.push(`page error: ${error.message}`));
  page.on("console", (message) => message.type() === "error" && problems.push(`console error: ${message.text()}`));
  return { context, page, locale, problems: [], expectingFailures: async (body) => (await body(), 0) };
}

/** 🎲️ A deterministic stream of picks, so every evidence run draws the same crowd. */
function dice(seed: number): (bound: number) => number {
  let state = seed;
  return (bound) => {
    state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
    return state % bound;
  };
}

/** 🌀️ Spoils a correctly answered run: in every classification some items go to a category picked by the dice, in every
 * sorting some items step down. */
async function scramble(learner: Device, pick: (bound: number) => number): Promise<void> {
  for (let index = 0; index < quiz.tasks.length; index++) {
    await openTask(learner, index);
    const shown = await shownTask(learner);
    const task = quiz.tasks.find((candidate) => candidate.id === shown)!;
    const items = await shownItems(learner);
    if (task.kind === "classification") for (const item of items.filter(() => pick(2) === 0)) await classify(learner, item, task.categories![pick(task.categories!.length)]!.id);
    if (task.kind === "sorting") for (const item of items.slice(0, -1).filter(() => pick(2) === 0)) await moveDown(learner, item).catch(() => undefined);
  }
}

const closing: (() => Promise<unknown>)[] = [];
try {
  const executable = await buildProctor(repoRoot);
  const inherited = Object.fromEntries(Object.entries(process.env).filter(([name]) => !name.startsWith("PROCTOR_")));
  const served = launchProctor(executable, repoRoot, ["serve"], { ...inherited, PROCTOR_PORT: String(proctorPort), PROCTOR_DATA: join(out, "proctor-data"), PROCTOR_CATALOG: join(repoRoot, ...PROCTOR_DEV_CATALOG), PROCTOR_MODE: "development" }, join(out, "proctor.log"));
  closing.push(() => served.stop(0));
  await awaitReady({ label: `the proctor at ${proctor}`, probe: () => proctorReady(proctor), exited: served.exited, say });
  const server = launchOwned(process.execPath, [join(bundleRoot, "📜️script.ts"), "dev-site"], bundleRoot, { ...process.env, TEACHING_ARCHITECTURE_QUIZ_PORT: String(sitePort), PROCTOR_PORT: String(proctorPort), TEACHING_ARCHITECTURE_QUIZ_CACHE: join(out, "node_modules", ".vite"), TEACHING_ARCHITECTURE_QUIZ_WATCH: "off" }, join(out, "site.log"));
  closing.push(() => server.stop());
  await awaitReady({ label: `the dev site at ${site}`, probe: () => httpAnswers(site, SITE_FIRST_ANSWER_MS), exited: server.exited, say });
  say(`stack ready: ${site} → ${proctor}`);

  const browser = await chromium.launch();
  closing.push(() => browser.close());
  const pick = dice(20261002);
  for (let index = 0; index < crowd; index++) {
    const learner = await device(browser, "en", { width: 1280, height: 900 }, "dark");
    await enter(learner, { kind: "pseudonym", handle: handle("Crowd") });
    await playQuiz(learner, quiz.id);
    const how = index % 3;
    await answerRun(learner, quiz, how === 1 ? "flawed" : "perfect");
    if (how === 2) await scramble(learner, pick);
    await submitRun(learner);
    say(`learner ${index + 1}/${crowd} submitted (${["perfect", "flawed", "scrambled"][how]})`);
    await learner.context.close();
  }

  const report: Record<string, unknown> = { site, proctor, crowd };
  const viewer = await device(browser, "en", { width: 1280, height: 2600 }, "dark");
  await enter(viewer, { kind: "pseudonym", handle: handle("Viewer") });
  await playQuiz(viewer, quiz.id);
  await openTask(viewer, 0);
  const task = screen(viewer.page, "task");
  report.lockedGate = await task.locator("[data-crowd-gate]").getAttribute("data-crowd-gate");
  report.lockedFigures = await viewer.page.locator("[data-crowd-figure]").count();
  await task.screenshot({ path: join(out, "01-run-locked-dark.png") });
  await answerRun(viewer, quiz, "flawed");
  for (let index = 0; index < quiz.tasks.length; index++) {
    await openTask(viewer, index);
    if (index === 0) await task.getByRole("button", { name: "Show it now" }).click();
    await task.locator('[data-crowd-figure="answers"]').first().waitFor();
    await viewer.page.waitForTimeout(600);
    await task.screenshot({ path: join(out, `02-run-asked-task-${index + 1}-dark.png`) });
  }
  report.askedRuns = await task.locator('[data-crowd-figure="answers"]').first().getAttribute("data-runs");
  await viewer.page.emulateMedia({ colorScheme: "light", reducedMotion: "reduce" });
  await task.screenshot({ path: join(out, "03-run-asked-light.png") });
  await viewer.page.emulateMedia({ colorScheme: "dark", reducedMotion: "reduce" });
  await submitRun(viewer);
  const scores = viewer.page.locator('#quiz-main [data-card="results"] [data-crowd-figure="scores"]');
  await viewer.page.waitForFunction((expected) => Number(document.querySelector('#quiz-main [data-card="results"] [data-crowd-figure="scores"]')?.getAttribute("data-runs")) === expected, crowd + 1);
  await viewer.page.waitForTimeout(600);
  report.resultRuns = await scores.getAttribute("data-runs");
  report.resultBins = await scores.locator("li").evaluateAll((bins) => bins.map((bin) => Number((bin as HTMLElement).dataset.count)));
  report.resultOwnBin = await scores.locator("li[data-own]").getAttribute("data-bin");
  report.sentences = await viewer.page.locator('#quiz-main [data-crowd-figure="answers"] td[data-own] .sr-only').evaluateAll((cells) => cells.slice(0, 6).map((cell) => cell.textContent));
  await viewer.page.locator("#quiz-main .quiz-results").screenshot({ path: join(out, "04-results-dark.png") });
  await viewer.page.emulateMedia({ colorScheme: "light", reducedMotion: "reduce" });
  await viewer.page.locator("#quiz-main .quiz-results").screenshot({ path: join(out, "05-results-light.png") });
  await viewer.page.emulateMedia({ colorScheme: "dark", reducedMotion: "reduce" });
  await goHome(viewer);
  await viewer.page.evaluate((page) => (window.location.hash = page), quiz.id);
  const crowdCard = pane(viewer.page, quiz.id).locator('[data-card="quiz-crowd"]');
  await crowdCard.locator('[data-crowd-figure="scores"]').waitFor();
  await viewer.page.waitForTimeout(600);
  report.quizPageGate = await crowdCard.locator("[data-crowd-gate]").count();
  await crowdCard.screenshot({ path: join(out, "06-quiz-page-dark.png") });
  await viewer.context.close();

  const fresh = await device(browser, "de", { width: 1280, height: 1400 }, "dark");
  await enter(fresh, { kind: "pseudonym", handle: handle("Neu") });
  await fresh.page.evaluate((page) => (window.location.hash = page), quiz.id);
  const locked = pane(fresh.page, quiz.id).locator('[data-card="quiz-crowd"]');
  await locked.waitFor();
  report.freshQuizPageGate = await locked.locator("[data-crowd-gate]").getAttribute("data-crowd-gate");
  report.freshQuizPageFigures = await locked.locator("[data-crowd-figure]").count();
  await locked.screenshot({ path: join(out, "07-quiz-page-locked-de.png") });
  await fresh.context.close();

  const phone = await device(browser, "en", { width: 390, height: 2600 }, "dark");
  await enter(phone, { kind: "pseudonym", handle: handle("Phone") });
  await playQuiz(phone, quiz.id);
  await answerRun(phone, quiz, "perfect");
  await submitRun(phone);
  await phone.page.waitForTimeout(1200);
  report.phoneOverflow = await phone.page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth);
  await phone.page.locator("#quiz-main .quiz-results").screenshot({ path: join(out, "08-results-phone-dark.png") });
  await phone.context.close();

  report.problems = problems;
  writeFileSync(join(out, "report.json"), `${JSON.stringify(report, null, 2)}\n`);
  say(JSON.stringify(report));
} finally {
  for (const end of closing.splice(0).reverse()) await end().catch((error: unknown) => say(error instanceof Error ? error.message : String(error)));
}
