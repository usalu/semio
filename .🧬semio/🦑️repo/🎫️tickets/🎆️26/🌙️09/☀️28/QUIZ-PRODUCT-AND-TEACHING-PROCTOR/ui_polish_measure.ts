/** 📐️ Browser measurements of the quiz website the unit tests cannot make (work package "UI polish", items 1 and 4),
 * against a running stack (`bun ui_polish_measure.ts <still|pages|results|reflow|contrast|all> [site origin] [report
 * file]`, default the stack on 6071; `pages` is the second half of `still` alone, `results` the results screen while
 * the answers of everyone arrive — run it first on a fresh data directory; with `UI_POLISH_SHOTS=<directory>` it also
 * writes a screenshot of each results screen):
 *
 * - `still` — two devices in one run: the boxes of every item of a classification, a sorting and a matching before the
 *   other learner joins, while the other thinks along, and after the other left, on the desktop and the phone layout
 *   and at every text size. Nothing may move.
 * - `reflow` — German (the long labels), at 1280 × 720 and its 200 % and 400 % zoom equivalents (640 × 360, 320 × 180),
 *   normal and largest text: the home cards, a run of every task kind, the results and the leaderboard. The page must not
 *   scroll sideways, no text may be cut by a box that hides its overflow, and no two texts may lie over each other.
 * - `contrast` — forced colours in both colour schemes: every state the stylesheet shows through a colour must differ
 *   in computed style from the state beside it.
 *
 * Every learner is a fresh browser context with its own storage; names are random, so runs do not meet.
 * @see ../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🧱️stack/🟦️.ts — the stack these measurements run against */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { chromium, type Browser, type BrowserContext, type Page } from "playwright";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const site = resolve(repoRoot, "🎓️teaching/🏛️architecture/❓️quiz");
const [mode = "all", origin = "http://127.0.0.1:6071", report] = process.argv.slice(2);
process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH;

//#region 📚️Catalog
type Locale = "en" | "de";
type Text = Readonly<Record<Locale, string>>;
interface Task {
  readonly kind: "classification" | "sorting" | "matching";
  readonly id: string;
  readonly title: Text;
  readonly items: readonly { readonly id: string }[];
  readonly categories?: readonly { readonly id: string }[];
  readonly dimensions?: readonly { readonly id: string }[];
}
interface Quiz {
  readonly id: string;
  readonly tasks: readonly Task[];
}
const json = <T,>(path: string): T => JSON.parse(readFileSync(path, "utf8")) as T;
const CATALOG = json<{ readonly id: string; readonly quizzes: readonly string[] }>(resolve(site, "🔣️.json"));
const QUIZZES = CATALOG.quizzes.map((path) => json<Quiz>(resolve(site, path)));
const quizOf = (id: string): Quiz => QUIZZES.find((quiz) => quiz.id === id)!;
//#endregion 📚️Catalog

//#region 🖥️Devices
interface Layout {
  readonly name: string;
  readonly viewport: { readonly width: number; readonly height: number };
  readonly phone?: boolean;
}
interface Wish {
  readonly locale: Locale;
  readonly textSize?: "normal" | "large" | "larger" | "largest";
  readonly theme?: "system" | "light" | "dark";
  readonly forced?: "light" | "dark";
}
interface Device {
  readonly context: BrowserContext;
  readonly page: Page;
  readonly name: string;
  readonly problems: string[];
}

/** 🖥️ One learner's browser in `layout`, with the preferences of `wish` stored before the site first runs. */
async function device(browser: Browser, layout: Layout, wish: Wish): Promise<Device> {
  const context = await browser.newContext({
    viewport: layout.viewport,
    isMobile: layout.phone === true,
    hasTouch: layout.phone === true,
    locale: wish.locale === "de" ? "de-DE" : "en-GB",
    baseURL: origin,
    ...(wish.forced === undefined ? {} : { forcedColors: "active" as const, colorScheme: wish.forced }),
  });
  await context.addInitScript(
    ([key, preferences]) => {
      if (localStorage.getItem(key!) === null) localStorage.setItem(key!, preferences!);
    },
    [`semio.quiz.${CATALOG.id}.preferences`, JSON.stringify({ locale: wish.locale, theme: wish.theme ?? "system", textSize: wish.textSize ?? "normal", showCursors: true, showAnswers: true, animateIcons: false, pets: "off" })] as const,
  );
  const page = await context.newPage();
  const problems: string[] = [];
  page.on("console", (message) => message.type() === "error" && problems.push(`console: ${message.text()}`));
  page.on("pageerror", (error) => problems.push(`page: ${error.message}`));
  return { context, page, name: `Polish ${Math.random().toString(36).slice(2, 8)}`, problems };
}

const screen = (page: Page, name: string) => page.locator(`#quiz-main [data-card="${name}"]`).and(page.locator(":not([inert] *)"));
const primary = (scope: ReturnType<Page["locator"]>) => scope.locator('[data-overview-card-action="primary"]');

/** 🚶️ The first visit: introduction, a pseudonym, the overview. */
async function enter(learner: Device): Promise<void> {
  const { page } = learner;
  await page.goto("/");
  await screen(page, "introduction").waitFor();
  await primary(screen(page, "introduction")).click();
  const form = screen(page, "identity");
  await form.locator('input[type="radio"][value="pseudonym"]').check();
  await form.locator('input[type="text"]').fill(learner.name);
  await primary(form).click();
  await page.locator("[data-layered-overview]").waitFor();
}

/** ▶️ Starts (or resumes) `quiz` from its card and opens its task `task`, wherever the sheet put it. */
async function play(learner: Device, quiz: string, task?: string): Promise<void> {
  const { page } = learner;
  await primary(page.locator(`[data-layered-card="${quiz}"]`)).click();
  await screen(page, "run").waitFor();
  if (task !== undefined) await openTask(learner, task);
}

async function openTask(learner: Device, task: string): Promise<void> {
  const { page } = learner;
  const steps = screen(page, "run").locator("nav button");
  for (let index = 0; index < (await steps.count()); index++) {
    await steps.nth(index).click();
    await page.waitForFunction((at) => document.querySelectorAll('#quiz-main [data-card="run"] nav button')[at]?.getAttribute("aria-current") === "step", index);
    if ((await screen(page, "task").getAttribute("data-presence-anchor")) === `task:${task}`) return;
  }
  throw new Error(`the run has no task ${task}`);
}

/** ✍️ Gives every item of the task on screen an answer (whatever comes first), so the run can be submitted and the
 * others see a draft for every item. */
async function answerTask(learner: Device, task: Task): Promise<void> {
  const card = screen(learner.page, "task");
  if (task.kind === "classification") {
    const selects = card.locator("[data-quiz-item] select");
    const items = await card.locator("[data-quiz-item]").evaluateAll((elements) => elements.map((element) => (element as HTMLElement).dataset.quizItem!));
    for (const [index, item] of items.entries()) await card.locator(`[data-quiz-item="${item}"] select`).selectOption(task.categories![index % task.categories!.length]!.id);
    await learner.page.waitForFunction((count) => document.querySelectorAll('#quiz-main [data-card="task"] [data-quiz-drop^="category:"] [data-quiz-item]').length === count, await selects.count());
  } else if (task.kind === "sorting") {
    const keep = card.getByRole("button", { name: /Reihenfolge übernehmen|Keep this order/u });
    if ((await keep.count()) > 0) await keep.click();
    else await card.locator("[data-quiz-item] button").nth(1).click();
  } else {
    for (const dimension of task.dimensions!) {
      const rows = card.locator(`[data-quiz-drop^="slot:${dimension.id}:"]`);
      for (let index = 0; index < (await rows.count()); index++) {
        const select = rows.nth(index).locator("select");
        const free = await select.evaluate((element) => [...(element as HTMLSelectElement).options].find((option) => option.value !== "" && !option.disabled && !option.selected)?.value);
        if (free !== undefined && (await select.inputValue()) === "") await select.selectOption(free);
      }
    }
  }
}

/** 📨️ Answers every task of the run on screen and submits it; the results show. */
async function finish(learner: Device, quiz: Quiz): Promise<void> {
  for (const task of quiz.tasks) {
    await openTask(learner, task.id);
    await answerTask(learner, task);
  }
  const submit = primary(screen(learner.page, "run"));
  await learner.page.waitForFunction(() => document.querySelector('#quiz-main [data-card="run"] [data-overview-card-action="primary"]')?.getAttribute("aria-disabled") !== "true");
  await submit.click();
  await primary(learner.page.getByRole("alertdialog")).click();
  await screen(learner.page, "results").waitFor();
}

async function openPage(learner: Device, id: string): Promise<void> {
  await learner.page.evaluate((page) => (window.location.hash = page), id);
  await learner.page.locator(`[data-layered-pane="${id}"][data-opened]`).waitFor();
}
//#endregion 🖥️Devices

//#region 🧘️Still
const LAYOUTS: readonly Layout[] = [
  { name: "desktop 1440 × 900", viewport: { width: 1440, height: 900 } },
  { name: "phone 375 × 812", viewport: { width: 375, height: 812 }, phone: true },
];

type Boxes = Readonly<Record<string, readonly [number, number, number, number]>>;

/** 📏️ The box of everything that must stay where it is in the task on screen: every item (or row), the card's footer,
 * and the height of everything that scrolls. */
async function boxes(page: Page): Promise<Boxes> {
  return page.evaluate(() => {
    const round = (value: number): number => Math.round(value * 100) / 100;
    const card = [...document.querySelectorAll<HTMLElement>('#quiz-main [data-card="task"]')].find((element) => element.closest("[inert]") === null)!;
    const marks: Record<string, [number, number, number, number]> = {};
    const add = (key: string, element: Element | null): void => {
      if (element === null) return;
      const box = element.getBoundingClientRect();
      marks[key] = [round(box.x), round(box.y), round(box.width), round(box.height)];
    };
    for (const element of card.querySelectorAll<HTMLElement>("[data-quiz-item], [data-quiz-drop^='slot:'], [data-quiz-drop^='category:']")) add(element.dataset.quizItem ?? element.dataset.quizDrop!, element);
    add("card", card);
    add("footer", card.querySelector('[data-overview-card-action="primary"]'));
    marks.scroll = [0, 0, document.scrollingElement!.scrollWidth, Math.max(...[...document.querySelectorAll<HTMLElement>("#quiz-main, #quiz-main *")].map((element) => element.scrollHeight))];
    return marks;
  });
}

function moved(before: Boxes, after: Boxes): readonly string[] {
  return Object.keys({ ...before, ...after }).flatMap((key) => {
    const [left, right] = [before[key], after[key]];
    if (left === undefined || right === undefined) return [`${key}: ${left === undefined ? "appeared" : "vanished"}`];
    const delta = left.map((value, index) => Math.abs(value - right[index]!));
    return Math.max(...delta) > 0.5 ? [`${key}: ${JSON.stringify(left)} → ${JSON.stringify(right)}`] : [];
  });
}

async function still(browser: Browser): Promise<readonly Record<string, unknown>[]> {
  const rows: Record<string, unknown>[] = [];
  const cases = [
    { quiz: "physics", task: "power-or-energy" },
    { quiz: "physics", task: "powers" },
    { quiz: "heating", task: "u-values" },
  ];
  for (const layout of LAYOUTS) {
    for (const textSize of ["normal", "large", "larger", "largest"] as const) {
      for (const { quiz, task } of cases) {
        const kind = quizOf(quiz).tasks.find((entry) => entry.id === task)!;
        const watcher = await device(browser, layout, { locale: "de", textSize });
        await enter(watcher);
        await play(watcher, quiz, task);
        await watcher.page.waitForTimeout(1_500);
        const alone = await boxes(watcher.page);
        const source = (): Promise<string | null> => screen(watcher.page, "task").locator("[data-crowd-source]").getAttribute("data-crowd-source");
        const sourceAlone = await source();
        const other = await device(browser, LAYOUTS[0]!, { locale: "en" });
        await enter(other);
        await play(other, quiz, task);
        await answerTask(other, kind);
        await watcher.page.waitForFunction(() => document.querySelector('#quiz-main [data-card="task"] [data-crowd-source="live"]') !== null && document.querySelector('#quiz-main [data-card="task"] [data-crowd-item]:not([data-crowd-empty])') !== null, undefined, { timeout: 20_000 });
        await watcher.page.waitForTimeout(1_000);
        const together = await boxes(watcher.page);
        const lines = await screen(watcher.page, "task").locator("[data-crowd-item]").evaluateAll((elements) => ({ all: elements.length, filled: elements.filter((element) => !element.hasAttribute("data-crowd-empty")).length, heights: [...new Set(elements.map((element) => Math.round(element.getBoundingClientRect().height * 100) / 100))] }));
        await other.context.close();
        await watcher.page.waitForFunction(() => document.querySelector('#quiz-main [data-card="task"] [data-crowd-source="live"]') === null, undefined, { timeout: 30_000 });
        await watcher.page.waitForTimeout(1_000);
        const after = await boxes(watcher.page);
        rows.push({ layout: layout.name, text: textSize, kind: kind.kind, task, items: Object.keys(alone).length - 3, source: `${sourceAlone} → live → ${await source()}`, lines, movedWhenJoined: moved(alone, together), movedWhenLeft: moved(together, after), problems: watcher.problems });
        console.log(`[still] ${layout.name}, ${textSize} text, ${kind.kind}: ${lines.filled}/${lines.all} lines filled, line heights ${JSON.stringify(lines.heights)}; moved on join ${moved(alone, together).length}, on leave ${moved(together, after).length}`);
        await watcher.context.close();
      }
    }
  }
  for (const layout of LAYOUTS) rows.push(...(await stillPages(browser, layout)));
  return rows;
}

/** 📏️ The box of every card `selector` matches in front, by its place in the document. */
async function cardBoxes(page: Page, selector: string): Promise<Boxes> {
  return page.evaluate((query) => {
    const marks: Record<string, [number, number, number, number]> = {};
    for (const [index, card] of [...document.querySelectorAll<HTMLElement>(query)].filter((element) => element.closest("[inert]") === null).entries()) {
      const box = card.getBoundingClientRect();
      marks[`${index}:${card.dataset.card}`] = [box.x, box.y, box.width, box.height].map((value) => Math.round(value * 100) / 100) as [number, number, number, number];
    }
    return marks;
  }, selector);
}

/** 🏠️ The overview and a quiz's page while another learner starts that quiz, thinks along and leaves: the cards of the
 * overview and the cards of the page above the one that says what the others think must stay where they are. */
async function stillPages(browser: Browser, layout: Layout): Promise<readonly Record<string, unknown>[]> {
  const rows: Record<string, unknown>[] = [];
  const quiz = quizOf("physics");
  const task = quiz.tasks[0]!;
  const watcher = await device(browser, layout, { locale: "de" });
  await enter(watcher);
  await play(watcher, quiz.id, task.id);
  await screen(watcher.page, "run").locator('[data-overview-card-action="secondary"]').click();
  await watcher.page.locator("[data-layered-overview]").waitFor();
  const learning = (count: number) => watcher.page.waitForFunction(([id, expected]) => document.querySelector(`[data-layered-card="${id}"] [data-learning]`)?.getAttribute("data-learning") === String(expected), [quiz.id, count] as const, { timeout: 30_000 });
  await learning(0);
  await watcher.page.waitForTimeout(1_000);
  const home = "[data-layered-card] [data-card]";
  const alone = await cardBoxes(watcher.page, home);
  const other = await device(browser, LAYOUTS[0]!, { locale: "en" });
  await enter(other);
  await play(other, quiz.id, task.id);
  await learning(1);
  await watcher.page.waitForTimeout(500);
  const together = await cardBoxes(watcher.page, home);
  await other.context.close();
  await learning(0);
  await watcher.page.waitForTimeout(500);
  const after = await cardBoxes(watcher.page, home);
  await openPage(watcher, quiz.id);
  const page = `[data-layered-pane="${quiz.id}"] [data-card="quiz"], [data-layered-pane="${quiz.id}"] [data-card="quiz-tasks"], [data-layered-pane="${quiz.id}"] [data-card="quiz-crowd"]`;
  const crowd = (source: string) => watcher.page.waitForFunction(([id, expected]) => document.querySelector(`[data-layered-pane="${id}"] [data-card="quiz-crowd"] [data-crowd-source]`)?.getAttribute("data-crowd-source") === expected, [quiz.id, source] as const, { timeout: 30_000 });
  await crowd("submitted");
  await watcher.page.waitForTimeout(800);
  const answered = await cardBoxes(watcher.page, page);
  const thinker = await device(browser, LAYOUTS[0]!, { locale: "en" });
  await enter(thinker);
  await play(thinker, quiz.id, task.id);
  await answerTask(thinker, task);
  await crowd("live");
  await watcher.page.waitForTimeout(800);
  const thinking = await cardBoxes(watcher.page, page);
  await thinker.context.close();
  await crowd("submitted");
  await watcher.page.waitForTimeout(800);
  const left = await cardBoxes(watcher.page, page);
  rows.push({ layout: layout.name, text: "normal", kind: "overview cards", cards: Object.keys(alone).length, movedWhenJoined: moved(alone, together), movedWhenLeft: moved(together, after), problems: watcher.problems });
  rows.push({ layout: layout.name, text: "normal", kind: "quiz page", cards: Object.keys(thinking).length, movedWhenJoined: moved(answered, thinking), movedWhenLeft: moved(thinking, left) });
  console.log(`[still] ${layout.name}, overview: ${Object.keys(alone).length} cards; moved when another learner started the quiz ${moved(alone, together).length}, when that learner left ${moved(together, after).length}`);
  console.log(`[still] ${layout.name}, quiz page: ${Object.keys(thinking).length} cards; moved when another learner thought along ${moved(answered, thinking).length}, when that learner left ${moved(thinking, left).length}${[...moved(answered, thinking), ...moved(thinking, left)].length === 0 ? "" : ` — ${[...moved(answered, thinking), ...moved(thinking, left)].join("; ")}`}`);
  await watcher.context.close();
  return rows;
}
/** 🏁️ The results while the answers of everyone arrive: a learner submits a run with every query of the page held back
 * for a few seconds, so the tables first stand with what was known when the run began (on fresh data: nobody) and then
 * with every submitted run, the learner's own included. No card, table or row may change its box, and no table its
 * columns. One quiz per case, so on a fresh data directory each case meets a quiz nobody has submitted yet. */
async function stillResults(browser: Browser): Promise<readonly Record<string, unknown>[]> {
  const rows: Record<string, unknown>[] = [];
  const cases = [
    { layout: LAYOUTS[0]!, textSize: "normal" },
    { layout: LAYOUTS[0]!, textSize: "largest" },
    { layout: LAYOUTS[1]!, textSize: "normal" },
    { layout: LAYOUTS[1]!, textSize: "largest" },
  ] as const;
  for (const [index, { layout, textSize }] of cases.entries()) {
    const quiz = QUIZZES[index % QUIZZES.length]!;
    const learner = await device(browser, layout, { locale: "de", textSize });
    const { page } = learner;
    await enter(learner);
    await play(learner, quiz.id);
    let held = 0;
    await page.route("**/queries", async (route) => {
      held += 1;
      await new Promise((accept) => setTimeout(accept, 4_000));
      await route.continue();
    });
    await finish(learner, quiz);
    const seen = () =>
      page.evaluate(() => {
        const round = (value: number): number => Math.round(value * 100) / 100;
        const root = [...document.querySelectorAll<HTMLElement>("#quiz-main .quiz-results")].find((element) => element.closest("[inert]") === null)!;
        const marks: Record<string, [number, number, number, number]> = {};
        for (const [at, element] of [...root.querySelectorAll<HTMLElement>("[data-card], table, tr, [data-crowd-source]")].entries()) {
          const box = element.getBoundingClientRect();
          marks[`${at}:${element.tagName.toLowerCase()}`] = [round(box.x), round(box.y), round(box.width), round(box.height)];
        }
        const tables = [...root.querySelectorAll("table")];
        const everyone = tables.flatMap((table) => {
          const column = [...table.querySelectorAll("thead th")].findIndex((head) => head.textContent?.trim() === "Alle");
          return column < 0 ? [] : [...table.querySelectorAll("tbody tr")].map((row) => row.children[column]?.textContent ?? "");
        });
        return {
          marks,
          columns: tables.map((table) => table.querySelectorAll("thead th").length).join(),
          everyone: everyone.join(" | "),
          cells: everyone.length,
          source: root.querySelector("[data-crowd-source]")?.getAttribute("data-crowd-source") ?? null,
          rows: root.querySelectorAll("tbody tr").length,
        };
      });
    await page.waitForTimeout(500);
    const before = await seen();
    let after = before;
    for (const started = Date.now(); Date.now() - started < 30_000 && after.everyone === before.everyone && after.source === before.source; after = await seen()) await page.waitForTimeout(250);
    if (after.everyone === before.everyone && after.source === before.source) throw new Error(`the answers of everyone never changed on the results of ${quiz.id} (source ${before.source}, ${before.cells} cells)`);
    await page.waitForTimeout(800);
    after = await seen();
    if (process.env.UI_POLISH_SHOTS !== undefined) await page.screenshot({ path: resolve(process.env.UI_POLISH_SHOTS, `results-${quiz.id}-${layout.viewport.width}-${textSize}.png`), fullPage: true });
    rows.push({ layout: layout.name, text: textSize, kind: "results", quiz: quiz.id, rows: after.rows, held, source: `${before.source} → ${after.source}`, columns: `${before.columns} → ${after.columns}`, everyoneBefore: before.everyone.slice(0, 160), everyoneAfter: after.everyone.slice(0, 160), moved: moved(before.marks, after.marks), problems: learner.problems });
    console.log(`[still] ${layout.name}, ${textSize} text, results of ${quiz.id}: ${after.rows} rows, ${after.cells} cells of everyone, the answers arrived (${before.source} → ${after.source}); moved ${moved(before.marks, after.marks).length}, columns ${before.columns === after.columns ? "unchanged" : `${before.columns} → ${after.columns}`}${moved(before.marks, after.marks).length === 0 ? "" : ` — ${moved(before.marks, after.marks).slice(0, 4).join("; ")}`}`);
    await learner.context.close();
  }
  return rows;
}
//#endregion 🧘️Still

//#region 🔎️Reflow
const ZOOMS: readonly Layout[] = [
  { name: "100 % (1280 × 720)", viewport: { width: 1280, height: 720 } },
  { name: "200 % (640 × 360)", viewport: { width: 640, height: 360 } },
  { name: "400 % (320 × 180)", viewport: { width: 320, height: 180 } },
];

interface Finding {
  readonly kind: "page scrolls sideways" | "text cut" | "texts overlap" | "scrolls sideways inside" | "navigation bar title shortened";
  readonly detail: string;
}

/** 🔎️ What is wrong with the text of the screen in front: sideways scrolling of the page, text cut by a box that hides
 * its overflow, texts lying over each other, and (not a defect: tables are exempt from reflow) boxes that scroll sideways
 * on their own. Inert pages, hidden text and decoration are left out. */
async function inspect(page: Page, scope: string): Promise<readonly Finding[]> {
  return page.evaluate((selector) => {
    const findings: { kind: "page scrolls sideways" | "text cut" | "texts overlap" | "scrolls sideways inside" | "navigation bar title shortened"; detail: string }[] = [];
    const root = document.scrollingElement!;
    if (root.scrollWidth > root.clientWidth + 1) findings.push({ kind: "page scrolls sideways", detail: `document ${root.scrollWidth} > ${root.clientWidth}` });
    const scopes = [...document.querySelectorAll<HTMLElement>(selector)].filter((element) => element.closest("[inert]") === null);
    const label = (element: Element): string => `${element.tagName.toLowerCase()}${element.id === "" ? "" : `#${element.id}`}.${[...element.classList].slice(0, 3).join(".")}`;
    const said = (text: string): string => JSON.stringify(text.trim().replace(/\s+/gu, " ").slice(0, 48));
    const texts: { node: Text; owner: HTMLElement; box: DOMRect; seen: { left: number; top: number; right: number; bottom: number } }[] = [];
    const unseen = (owner: HTMLElement): boolean => {
      for (let ancestor: HTMLElement | null = owner; ancestor !== null; ancestor = ancestor.parentElement) {
        const style = getComputedStyle(ancestor);
        const frame = ancestor.getBoundingClientRect();
        if (style.overflow !== "visible" && frame.width <= 1.5 && frame.height <= 1.5) return true;
      }
      return false;
    };
    const seenPart = (owner: HTMLElement, box: DOMRect): { left: number; top: number; right: number; bottom: number } => {
      const seen = { left: box.left, top: box.top, right: box.right, bottom: box.bottom };
      for (let ancestor: HTMLElement | null = owner; ancestor !== null; ancestor = ancestor.parentElement) {
        const style = getComputedStyle(ancestor);
        if (style.display === "inline" || style.display === "contents") continue;
        const frame = ancestor.getBoundingClientRect();
        if (style.overflowX !== "visible") [seen.left, seen.right] = [Math.max(seen.left, frame.left), Math.min(seen.right, frame.right)];
        if (style.overflowY !== "visible") [seen.top, seen.bottom] = [Math.max(seen.top, frame.top), Math.min(seen.bottom, frame.bottom)];
      }
      return seen;
    };
    for (const within of scopes) {
      for (const scroller of [within, ...within.querySelectorAll<HTMLElement>("*")]) {
        const style = getComputedStyle(scroller);
        if (scroller.scrollWidth > scroller.clientWidth + 1 && (style.overflowX === "auto" || style.overflowX === "scroll") && scroller.clientWidth > 0) {
          const own = scroller.firstElementChild?.tagName === "TABLE";
          const edge = scroller.getBoundingClientRect().left + scroller.clientLeft + scroller.clientWidth - scroller.scrollLeft;
          const held = (element: HTMLElement): boolean => {
            for (let ancestor = element.parentElement; ancestor !== null && ancestor !== scroller; ancestor = ancestor.parentElement) if (getComputedStyle(ancestor).overflowX !== "visible") return true;
            return false;
          };
          const out = (element: Element): boolean => element.getBoundingClientRect().right - scroller.scrollLeft > edge + 1;
          const beyond = [...scroller.querySelectorAll<HTMLElement>("*")].filter((element) => out(element) && !held(element) && ![...element.children].some((child) => out(child) && getComputedStyle(element).overflowX === "visible"));
          findings.push({ kind: own ? "scrolls sideways inside" : "page scrolls sideways", detail: `${label(scroller)} ${scroller.scrollWidth} > ${scroller.clientWidth}${own ? " (its own table)" : `: ${[...new Set(beyond.map((element) => `${label(element)} ${said(element.textContent ?? "")}`))].slice(0, 4).join(" | ")}`}` });
        }
      }
      const walker = document.createTreeWalker(within, NodeFilter.SHOW_TEXT);
      for (let node = walker.nextNode() as Text | null; node !== null; node = walker.nextNode() as Text | null) {
        const owner = node.parentElement;
        if (owner === null || node.data.trim() === "") continue;
        if (owner.closest('[aria-hidden="true"], .sr-only, [hidden], [inert], option, select, script, style, noscript, [data-quiz-dialog] ~ *') !== null) continue;
        if (!owner.checkVisibility({ visibilityProperty: true, opacityProperty: true, contentVisibilityAuto: true })) continue;
        const range = document.createRange();
        range.selectNodeContents(node);
        const box = range.getBoundingClientRect();
        if (box.width < 1 || box.height < 1 || unseen(owner)) continue;
        texts.push({ node, owner, box, seen: seenPart(owner, box) });
      }
    }
    for (const { node, owner, box } of texts) {
      let [x, y] = [true, true];
      for (let ancestor: HTMLElement | null = owner; ancestor !== null && (x || y); ancestor = ancestor.parentElement) {
        const style = getComputedStyle(ancestor);
        if (style.display === "inline" || style.display === "contents") continue;
        const frame = ancestor.getBoundingClientRect();
        const ellipsis = style.textOverflow === "ellipsis";
        for (const axis of ["x", "y"] as const) {
          if (!(axis === "x" ? x : y)) continue;
          const overflow = axis === "x" ? style.overflowX : style.overflowY;
          if (overflow === "visible") continue;
          if (overflow === "auto" || overflow === "scroll") {
            if (axis === "x") x = false;
            else y = false;
            continue;
          }
          const [from, to, low, high] = axis === "x" ? [box.left, box.right, frame.left, frame.right] : [box.top, box.bottom, frame.top, frame.bottom];
          const inner = axis === "x" ? ancestor.scrollWidth > ancestor.clientWidth + 1 : ancestor.scrollHeight > ancestor.clientHeight + 1;
          if ((from < low - 1 || to > high + 1) && to > low && from < high) {
            findings.push({ kind: owner.closest('[data-slot="navbar"]') === null ? "text cut" : "navigation bar title shortened", detail: `${said(node.data)} in ${label(owner)} by ${label(ancestor)} (${axis}${ellipsis ? ", ends in an ellipsis" : ""}${inner ? "" : ", by an outer box"})` });
            if (axis === "x") x = false;
            else y = false;
          }
        }
      }
    }
    const visible = texts.filter(({ seen }) => seen.right - seen.left > 1 && seen.bottom - seen.top > 1 && seen.bottom > 0 && seen.top < innerHeight && seen.right > 0 && seen.left < innerWidth);
    for (let left = 0; left < visible.length; left++) {
      for (let right = left + 1; right < visible.length; right++) {
        const [a, b] = [visible[left]!, visible[right]!];
        if (a.owner === b.owner || a.owner.contains(b.owner) || b.owner.contains(a.owner)) continue;
        const [w, h] = [Math.min(a.seen.right, b.seen.right) - Math.max(a.seen.left, b.seen.left), Math.min(a.seen.bottom, b.seen.bottom) - Math.max(a.seen.top, b.seen.top)];
        if (w > 2 && h > 2) findings.push({ kind: "texts overlap", detail: `${said(a.node.data)} and ${said(b.node.data)} by ${Math.round(w)} × ${Math.round(h)} px` });
      }
    }
    return findings;
  }, scope);
}

/** 🧾️ Inspects the screen in front from top to bottom: at the top, then after every screenful of its scroller. */
async function inspectAll(page: Page, scope: string, scroller: string): Promise<readonly Finding[]> {
  const found = new Map<string, Finding>();
  const steps = await page.evaluate((selector) => {
    const box = [...document.querySelectorAll<HTMLElement>(selector)].find((element) => element.closest("[inert]") === null && element.scrollHeight > element.clientHeight + 1);
    return box === undefined ? 1 : Math.min(40, Math.ceil(box.scrollHeight / Math.max(1, box.clientHeight)));
  }, scroller);
  for (let step = 0; step < steps; step++) {
    await page.evaluate(
      ([selector, at]) => {
        const box = [...document.querySelectorAll<HTMLElement>(selector as string)].find((element) => element.closest("[inert]") === null && element.scrollHeight > element.clientHeight + 1);
        box?.scrollTo(0, (at as number) * box.clientHeight);
      },
      [scroller, step] as const,
    );
    await page.waitForTimeout(150);
    for (const finding of await inspect(page, scope)) found.set(`${finding.kind}:${finding.detail}`, finding);
  }
  return [...found.values()];
}

async function reflow(browser: Browser): Promise<readonly Record<string, unknown>[]> {
  const rows: Record<string, unknown>[] = [];
  const seed = await device(browser, LAYOUTS[0]!, { locale: "de" });
  await enter(seed);
  await play(seed, "physics");
  await finish(seed, quizOf("physics"));
  await seed.context.close();
  for (const layout of ZOOMS) {
    for (const textSize of ["normal", "largest"] as const) {
      const learner = await device(browser, layout, { locale: "de", textSize });
      const record = async (name: string, scope: string, scroller: string): Promise<void> => {
        const findings = await inspectAll(learner.page, scope, scroller);
        const count = (kind: Finding["kind"]): number => findings.filter((finding) => finding.kind === kind).length;
        rows.push({ zoom: layout.name, text: textSize, screen: name, sideways: count("page scrolls sideways"), cut: count("text cut"), overlap: count("texts overlap"), inner: count("scrolls sideways inside"), navbar: count("navigation bar title shortened"), findings });
        console.log(`[reflow] ${layout.name}, ${textSize} text, ${name}: sideways ${count("page scrolls sideways")}, cut ${count("text cut")}, overlap ${count("texts overlap")}, tables scrolling on their own ${count("scrolls sideways inside")}, navigation bar title shortened ${count("navigation bar title shortened")}`);
      };
      await enter(learner);
      await learner.page.waitForTimeout(1_500);
      await record("home cards", "[data-layered-card], header, footer", "[data-layered-list], [data-layered-overview]");
      const tasks = [
        { quiz: "physics", task: "power-or-energy", name: "run: classification" },
        { quiz: "physics", task: "powers", name: "run: sorting" },
        { quiz: "heating", task: "u-values", name: "run: matching" },
        { quiz: "demand", task: "standard-profiles", name: "run: classification with spider diagrams" },
      ];
      for (const { quiz, task, name } of tasks) {
        await play(learner, quiz, task);
        await learner.page.waitForTimeout(500);
        await record(name, "#quiz-main, header, footer", "html, body, #quiz-main, #quiz-main *");
        await answerTask(learner, quizOf(quiz).tasks.find((entry) => entry.id === task)!);
        await learner.page.waitForTimeout(300);
        await record(`${name}, answered`, "#quiz-main, header, footer", "html, body, #quiz-main, #quiz-main *");
        if (quiz === "physics" && task === "powers") {
          await finish(learner, quizOf("physics"));
          await learner.page.waitForTimeout(800);
          await record("results", "#quiz-main, header, footer", "html, body, #quiz-main, #quiz-main *");
          await primary(screen(learner.page, "results")).click();
        } else await screen(learner.page, "run").locator('[data-overview-card-action="secondary"]').click();
        await learner.page.locator("[data-layered-overview]").waitFor();
      }
      await openPage(learner, "board");
      await learner.page.waitForTimeout(800);
      await record("leaderboard", '[data-layered-pane="board"], header, footer', '[data-layered-pane="board"] *');
      rows.push({ zoom: layout.name, text: textSize, screen: "(console)", problems: learner.problems });
      await learner.context.close();
    }
  }
  return rows;
}
//#endregion 🔎️Reflow

//#region 🌗️Contrast
/** 🎨️ The computed style that tells states apart, of the first element `selector` matches in front. */
async function paint(page: Page, selector: string): Promise<Record<string, string> | undefined> {
  return page.evaluate((query) => {
    const element = [...document.querySelectorAll<HTMLElement | SVGElement>(query)].find((candidate) => candidate.closest("[inert]") === null && candidate.checkVisibility());
    if (element === undefined) return undefined;
    const style = getComputedStyle(element);
    const box = element.getBoundingClientRect();
    return { background: style.backgroundColor, color: style.color, outline: `${style.outlineStyle} ${style.outlineWidth} ${style.outlineColor}`, border: `${style.borderTopStyle} ${style.borderTopWidth} ${style.borderTopColor}`, fill: style.fill, stroke: `${style.stroke} ${style.strokeWidth}`, size: `${Math.round(box.width)} × ${Math.round(box.height)}`, adjust: style.getPropertyValue("forced-color-adjust") };
  }, selector);
}

async function contrast(browser: Browser): Promise<readonly Record<string, unknown>[]> {
  const rows: Record<string, unknown>[] = [];
  const peer = await device(browser, LAYOUTS[0]!, { locale: "en" });
  await enter(peer);
  await play(peer, "physics");
  await finish(peer, quizOf("physics"));
  await primary(screen(peer.page, "results")).click();
  await peer.page.locator("[data-layered-overview]").waitFor();
  for (const scheme of ["light", "dark"] as const) {
    const learner = await device(browser, LAYOUTS[0]!, { locale: "de", forced: scheme });
    const compare = async (state: string, selector: string, other: string, properties: readonly string[]): Promise<void> => {
      await learner.page.waitForTimeout(600);
      const [here, there] = [await paint(learner.page, selector), await paint(learner.page, other)];
      const differing = here === undefined || there === undefined ? [] : properties.filter((property) => here[property] !== there[property]);
      const passed = differing.length > 0;
      rows.push({ scheme, state, passed, differing, here, there });
      console.log(`[contrast] forced colours, ${scheme}: ${state} — ${passed ? `differs in ${differing.join(", ")}` : here === undefined || there === undefined ? `NOT FOUND (${here === undefined ? selector : other})` : "NOT DISTINGUISHABLE"} ${JSON.stringify(Object.fromEntries(properties.map((property) => [property, `${here?.[property]} | ${there?.[property]}`])))}`);
    };
    await enter(learner);
    const active = await learner.page.evaluate(() => matchMedia("(forced-colors: active)").matches);
    rows.push({ scheme, state: "forced colours are active in the page", passed: active });
    await play(learner, "physics");
    await finish(learner, quizOf("physics"));
    await primary(screen(learner.page, "results")).click();
    await learner.page.locator("[data-layered-overview]").waitFor();
    await openPage(learner, "prefs");
    for (const [group, label] of [["Sprache", "language"], ["Farbschema", "theme"], ["Textgröße", "text size"]] as const) {
      const within = `[data-layered-pane="prefs"] [role="group"][aria-label="${group}"]`;
      await compare(`the pressed ${label} button against an unpressed one`, `${within} [aria-pressed="true"]`, `${within} [aria-pressed="false"]`, ["background", "color"]);
    }
    await learner.page.keyboard.press("Escape");
    await openPage(learner, "board");
    await learner.page.waitForTimeout(1_000);
    await compare("the own leaderboard row against another row", '[data-layered-pane="board"] tr.quiz-me', '[data-layered-pane="board"] tbody tr:not(.quiz-me)', ["outline", "background"]);
    await compare("an online mark against the row it stands in", '[data-layered-pane="board"] .quiz-online', '[data-layered-pane="board"] tbody tr', ["background"]);
    await learner.page.keyboard.press("Escape");
    await play(learner, "physics", "powers");
    await compare("the current task against another task", '#quiz-main [data-card="run"] nav button[aria-current="step"]', '#quiz-main [data-card="run"] nav button:not([aria-current])', ["background", "color"]);
    await learner.page.waitForFunction(() => document.querySelector('#quiz-main [data-card="task"] .quiz-crowd-marker') !== null, undefined, { timeout: 20_000 });
    await compare("a crowd marker against its track", '#quiz-main [data-card="task"] .quiz-crowd-marker', '#quiz-main [data-card="task"] .quiz-crowd-track', ["background"]);
    await screen(learner.page, "run").locator('[data-overview-card-action="secondary"]').click();
    await learner.page.locator("[data-layered-overview]").waitFor();
    await play(learner, "demand", "standard-profiles");
    await compare("the area of a spider diagram against its rings", "#quiz-main .quiz-radar-area", "#quiz-main .quiz-radar-ring", ["fill", "stroke"]);
    rows.push({ scheme, state: "(console)", problems: learner.problems });
    await learner.context.close();
  }
  await peer.context.close();
  return rows;
}
//#endregion 🌗️Contrast

const browser = await chromium.launch();
const results: Record<string, unknown> = {};
try {
  if (mode === "still" || mode === "all") results.still = await still(browser);
  if (mode === "pages") results.pages = [...(await stillPages(browser, LAYOUTS[0]!)), ...(await stillPages(browser, LAYOUTS[1]!))];
  if (mode === "results" || mode === "all") results.results = await stillResults(browser);
  if (mode === "reflow" || mode === "all") results.reflow = await reflow(browser);
  if (mode === "contrast" || mode === "all") results.contrast = await contrast(browser);
} finally {
  await browser.close();
}
if (report !== undefined) {
  mkdirSync(dirname(resolve(report)), { recursive: true });
  writeFileSync(resolve(report), `${JSON.stringify(results, null, 2)}\n`);
}
