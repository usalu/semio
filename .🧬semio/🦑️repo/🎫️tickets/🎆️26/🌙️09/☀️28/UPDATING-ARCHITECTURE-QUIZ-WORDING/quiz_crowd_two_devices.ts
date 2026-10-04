/** 💭️ Two devices against the dev proctor's real presence sockets for §17 (`.claude/launch.json` `teaching-proctor` +
 * `architecture-quiz`): device A at http://localhost:6061 (1440 px, light, English) and device B at
 * http://127.0.0.1:6061 (1280 px, dark, German) — two origins, two local stores, two learners. It shows (a) A's home at
 * rest as the live grid of every page, the leaderboard polled and updating, and B's pointer inside the pages behind the
 * cards (the leaderboard page, the Energy Demand page), (b) both in the same quiz: B's pointer and drag on an item of
 * A's sheet and B's draft in "what others think now", (c) A's results beside every submitted run. Screenshots and facts
 * go to `🗑️generated/crowd-final/`. Other sessions may be online on the dev proctor, so the walk waits for names.
 *
 * Presence is ephemeral and admitted without a known learner, so the walk creates no learner, run or answer: learner
 * views, the leaderboard (B's total grows with every poll), runs and the submitted crowd are answered here with
 * synthetic learners (Ann K. and Ben), every command is refused before it leaves the page, and only the presence
 * WebSockets reach the proctor.
 *
 * Usage: bun <this file>
 */

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium, type Browser, type BrowserContext, type Page } from "playwright";
import { decodeQueryEnvelope, encodeQueryResult } from "../../../../../../../🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🟦️.ts";
import { crowdView, learnerTag, runSeed, scoreRun, sheetOf, type Answer, type Quiz, type Sheet, type SheetTask } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const out = join(here, "🗑️generated", "crowd-final");
mkdirSync(out, { recursive: true });
const tenant = "architecture";
const encoder = new TextEncoder();
const decoder = new TextDecoder();
const energy = join(here, "..", "..", "..", "..", "..", "..", "..", "🎓️teaching", "🏛️architecture", "⚡️energy");
const demand = JSON.parse(readFileSync(join(energy, "📊️demand", "❓️quiz", "🔣️.json"), "utf8")) as Quiz;

interface Device {
  readonly name: string;
  readonly site: string;
  readonly learner: string;
  readonly handle: string;
  readonly width: number;
  readonly height: number;
  readonly scheme: "light" | "dark";
  readonly locale: "en" | "de";
  readonly open: string;
  readonly submitted?: string;
}

const A: Device = { name: "a", site: "http://localhost:6061/", learner: "a0a0a0a0".repeat(4), handle: "Ann K.", width: 1440, height: 900, scheme: "light", locale: "en", open: "3".repeat(32), submitted: "5".repeat(32) };
const B: Device = { name: "b", site: "http://127.0.0.1:6061/", learner: "b0b0b0b0".repeat(4), handle: "Ben", width: 1280, height: 800, scheme: "dark", locale: "de", open: "4".repeat(32) };

/** 🎲️ A complete answer to a sheet task, varied by `k`. */
function answerFor(task: SheetTask, k: number): Answer {
  switch (task.kind) {
    case "classification":
      return { kind: "classification", assignments: Object.fromEntries(task.items.map((item, index) => [item.id, task.categories[(index + k) % task.categories.length]!.id])) };
    case "sorting":
      return { kind: "sorting", order: task.items.map((item) => item.id).map((_, index, ids) => ids[(index + k) % ids.length]!) };
    case "matching":
      return { kind: "matching", assignments: Object.fromEntries(task.dimensions.map((dimension) => [dimension.id, Object.fromEntries(task.items.map((item, index) => [item.id, (index + k) % dimension.cards.length]))])) };
  }
}

function answersFor(sheet: Sheet, k: number): Record<string, Answer> {
  return Object.fromEntries(sheet.tasks.map((task) => [task.id, answerFor(task, k)]));
}

const submittedSheet = sheetOf(demand, runSeed(A.submitted!));
const submittedAnswers = answersFor(submittedSheet, 0);
const crowd = crowdView(demand, [
  scoreRun(demand, submittedSheet, submittedAnswers)!,
  ...[1, 2, 3].map((k) => {
    const sheet = sheetOf(demand, runSeed(String(k).repeat(32)));
    return scoreRun(demand, sheet, answersFor(sheet, k % 2))!;
  }),
]);

const snapshot = (value: unknown) => JSON.stringify(encodeQueryResult({ kind: "snapshot", value: encoder.encode(JSON.stringify(value)), frontier: null }));

function learnerView(device: Device) {
  const runs = [{ run: device.open, quiz: "demand", status: "open", startedAt: 1_790_602_000_000 }];
  if (device.submitted !== undefined) runs.push({ run: device.submitted, quiz: "demand", status: "submitted", score: scoreRun(demand, submittedSheet, submittedAnswers)!.score, startedAt: 1_790_601_000_000, submittedAt: 1_790_601_500_000 } as never);
  return { learner: device.learner, identity: { kind: "pseudonym", handle: device.handle }, runs, badges: [], best: device.submitted === undefined ? {} : { demand: scoreRun(demand, submittedSheet, submittedAnswers)!.score }, total: 0 };
}

let boardQueries = 0;
function leaderboard() {
  boardQueries += 1;
  return {
    rows: [
      { device: A, total: 200 },
      { device: B, total: 150 + 10 * boardQueries },
    ]
      .sort((left, right) => right.total - left.total)
      .map(({ device, total }, index) => ({ rank: index + 1, tag: learnerTag(device.learner), identity: { kind: "pseudonym", handle: device.handle }, total, reachedAt: 1_790_600_000_000 + index, best: { demand: 0.8 }, badges: [], runs: 1, lastActivity: 1_790_602_000_000 - index * 60_000 })),
  };
}

function runView(device: Device, run: string) {
  if (run === device.open) return { run, learner: device.learner, quiz: "demand", status: "open", sheet: sheetOf(demand, runSeed(run)), answers: {}, startedAt: 1_790_602_000_000 };
  if (run === device.submitted) return { run, learner: device.learner, quiz: "demand", status: "submitted", sheet: submittedSheet, answers: submittedAnswers, result: scoreRun(demand, submittedSheet, submittedAnswers), startedAt: 1_790_601_000_000, submittedAt: 1_790_601_500_000 };
  return undefined;
}

async function device(browser: Browser, device: Device, errors: string[]): Promise<{ readonly context: BrowserContext; readonly page: Page }> {
  const context = await browser.newContext({ viewport: { width: device.width, height: device.height }, colorScheme: device.scheme, locale: device.locale === "de" ? "de-DE" : "en-US" });
  await context.addInitScript(
    ({ tenant, learner, handle, scheme, locale }) => {
      localStorage.setItem(`semio.quiz.${tenant}.introduced`, "true");
      localStorage.setItem(`semio.quiz.${tenant}.learner`, JSON.stringify({ id: learner, identity: { kind: "pseudonym", handle } }));
      localStorage.setItem(`semio.quiz.${tenant}.preferences`, JSON.stringify({ locale, theme: scheme, textSize: "normal", showCursors: true, showAnswers: true }));
    },
    { tenant, learner: device.learner, handle: device.handle, scheme: device.scheme, locale: device.locale },
  );
  await context.route("**/commands", (route) => route.abort());
  await context.route("**/queries", async (route) => {
    const envelope = decodeQueryEnvelope(JSON.parse(route.request().postData() ?? "{}"));
    const query = JSON.parse(decoder.decode(envelope.arguments)) as { readonly type: string; readonly run?: string; readonly quiz?: string };
    const fulfil = (value: unknown) => route.fulfill({ status: 200, contentType: "application/json", body: snapshot(value) });
    if (query.type === "learner") return fulfil(learnerView(device));
    if (query.type === "leaderboard") return fulfil(leaderboard());
    if (query.type === "crowd") return query.quiz === "demand" ? fulfil(crowd) : fulfil({ quiz: query.quiz, runs: 0, tasks: [] });
    if (query.type === "run" && query.run !== undefined && runView(device, query.run) !== undefined) return fulfil(runView(device, query.run));
    return route.continue();
  });
  const page = await context.newPage();
  page.on("console", (message) => message.type() === "error" && !message.text().includes("ERR_FAILED") && errors.push(`${device.name} console: ${message.text()}`));
  page.on("pageerror", (error) => errors.push(`${device.name} pageerror: ${error.stack ?? error.message}`));
  await page.goto(device.site);
  await page.locator('[data-card="board"] tbody tr').first().waitFor();
  return { context, page };
}

async function shot(page: Page, name: string, clip?: { readonly x: number; readonly y: number; readonly width: number; readonly height: number }): Promise<void> {
  await page.waitForTimeout(400);
  await page.screenshot({ path: join(out, `${name}.png`), ...(clip === undefined ? {} : { clip }) });
}

/** 🖱️ Waits until a visible peer mark of `kind` named `label` is placed inside `root`, and measures it against its anchor. */
async function peerMark(page: Page, root: string, kind: "cursor" | "focus", label: string) {
  const selector = `${root} [data-peer="${kind}"]`;
  await page.waitForFunction(({ selector, label }) => [...document.querySelectorAll<HTMLElement>(selector)].some((element) => !element.hidden && element.textContent?.includes(label)), { selector, label }, { timeout: 15_000 });
  await page.waitForTimeout(400);
  return page.evaluate(
    ({ selector, label }) => {
      const element = [...document.querySelectorAll<HTMLElement>(selector)].find((candidate) => !candidate.hidden && candidate.textContent?.includes(label))!;
      const scope = element.closest("[data-layered-pane]") ?? document;
      const anchor = [...scope.querySelectorAll<HTMLElement>(`[data-presence-anchor="${element.dataset.anchor}"]`)].find((candidate) => candidate.closest("[inert]") === null || scope !== document)!;
      const box = anchor.getBoundingClientRect();
      const own = element.getBoundingClientRect();
      return {
        text: element.textContent,
        anchor: element.dataset.anchor,
        drag: element.dataset.drag ?? null,
        shared: { x: Number(element.dataset.x), y: Number(element.dataset.y) },
        drawn: { x: Math.round(((own.left - box.left) / box.width) * 1000) / 1000, y: Math.round(((own.top - box.top) / box.height) * 1000) / 1000 },
        pane: element.closest("[data-layered-pane]")?.getAttribute("data-layered-pane") ?? null,
        ariaHidden: element.closest("[aria-hidden]")?.getAttribute("aria-hidden") ?? null,
        colour: getComputedStyle(element).getPropertyValue("--quiz-peer").trim(),
      };
    },
    { selector, label },
  );
}

const facts: Record<string, unknown> = {};
const errors: string[] = [];
const browser = await chromium.launch({ channel: "chrome" });
const a = await device(browser, A, errors);
const b = await device(browser, B, errors);
const listed = (page: Page, handle: string) => page.waitForFunction((handle) => [...document.querySelectorAll("[data-presence-list] li")].some((item) => item.textContent?.includes(handle)), handle, { timeout: 20_000 });
await listed(a.page, B.handle);
await listed(b.page, A.handle);

try {
//#region (a) the live grid
facts.rest = await a.page.locator("[data-layered-overview]").getAttribute("data-rest");
facts.panes = await a.page.evaluate(() =>
  [...document.querySelectorAll("[data-layered-pane]")].map((pane) => ({ pane: pane.getAttribute("data-layered-pane"), live: pane.querySelector("[data-page]") !== null, inert: pane.hasAttribute("inert"), placeholder: pane.querySelector("[data-layered-placeholder]") !== null })),
);
facts.tracks = await a.page.locator(".quiz-home-grid").evaluate((element) => ({ columns: getComputedStyle(element).gridTemplateColumns, rows: getComputedStyle(element).gridTemplateRows, gap: getComputedStyle(element).gap }));
facts.cellsOverPages = await a.page.evaluate(() =>
  [...document.querySelectorAll<HTMLElement>(".quiz-home-cell")].map((cell) => {
    const id = cell.closest("[data-layered-card]")?.getAttribute("data-layered-card");
    const pane = document.querySelector(`[data-layered-pane="${id}"]`)!.getBoundingClientRect();
    const own = cell.getBoundingClientRect();
    return { id, centreDx: Math.round(own.left + own.width / 2 - (pane.left + pane.width / 2)), topDy: Math.round(own.top - pane.top), coverWider: Math.round(pane.width - own.width), coverTaller: Math.round(pane.height - own.height) };
  }),
);
const benRow = (page: Page) => page.locator('[data-card="board"] tbody tr', { hasText: B.handle }).first().innerText();
facts.boardBefore = { queries: boardQueries, row: await benRow(a.page) };
await a.page.waitForTimeout(10_600);
facts.boardAfterPoll = { queries: boardQueries, row: await benRow(a.page), page: await a.page.locator('[data-layered-pane="board"] tbody tr', { hasText: B.handle }).first().innerText() };

await b.page.goto(`${B.site}#board`);
await b.page.locator('[data-layered-pane="board"] [data-presence-anchor="leaderboard"]').waitFor();
const table = (await b.page.locator('[data-layered-pane="board"] [data-presence-anchor="leaderboard"]').boundingBox())!;
await b.page.mouse.move(table.x + table.width * 0.35, table.y + table.height * 0.4, { steps: 10 });
facts.bInBoardSeenByA = await peerMark(a.page, '[data-layered-pane="board"] [data-pane-peers]', "cursor", B.handle);
await shot(a.page, "a-home-live-grid-ben-in-leaderboard-1440-light");
const boardCell = (await a.page.locator('[data-layered-pane="board"]').boundingBox())!;
await shot(a.page, "a-home-board-cell-zoom", { x: boardCell.x, y: boardCell.y, width: boardCell.width, height: boardCell.height });
await shot(b.page, "b-leaderboard-page-pointing-1280-dark-de");

await b.page.goto(`${B.site}#demand`);
const quizCard = b.page.locator('[data-layered-pane="demand"] [data-presence-anchor="quiz:demand"]');
await quizCard.waitFor();
const quizBox = (await quizCard.boundingBox())!;
await b.page.mouse.move(quizBox.x + quizBox.width * 0.6, quizBox.y + quizBox.height * 0.5, { steps: 10 });
facts.bInDemandPageSeenByA = await peerMark(a.page, '[data-layered-pane="demand"] [data-pane-peers]', "cursor", B.handle);
await shot(a.page, "a-home-live-grid-ben-in-demand-page-1440-light");
//#endregion

//#region (b) thinking along
const sheetA = sheetOf(demand, runSeed(A.open));
const sheetB = sheetOf(demand, runSeed(B.open));
const first = sheetA.tasks[0]!;
const firstB = sheetB.tasks.find((task) => task.id === first.id)!;
const shared = first.items.map((item) => item.id).find((id) => firstB.items.some((item) => item.id === id))!;
facts.sharedItem = { task: first.id, kind: first.kind, item: shared, orderA: first.items.map((item) => item.id), orderB: firstB.items.map((item) => item.id) };

await b.page.goto(B.site);
await b.page.locator('[data-card="board"] tbody tr').first().waitFor();
await b.page.locator('[data-card="quiz:demand"] [data-overview-card-action="primary"]').click();
await b.page.locator('[data-card="task"]').waitFor();
await a.page.locator('[data-layered-pane="demand"] [data-crowd-source="live"]').waitFor({ timeout: 15_000 });
facts.demandPageOnAWhileBThinks = await a.page.locator('[data-layered-pane="demand"] [data-crowd-source]').innerText();
await a.page.locator('[data-card="quiz:demand"]').hover();
await a.page.waitForTimeout(1_200);
facts.revealedDemandPane = await a.page.locator('[data-layered-pane="demand"]').evaluate((element) => {
  const box = element.getBoundingClientRect();
  return { left: Math.round(box.left), top: Math.round(box.top), width: Math.round(box.width), height: Math.round(box.height), revealed: element.closest("[data-layered-overview]")?.querySelector('[data-layered-card="demand"]')?.hasAttribute("data-revealed") ?? false };
});
await shot(a.page, "a-home-demand-page-clear-ben-thinking-1440-light");

await a.page.locator('[data-card="quiz:demand"] [data-overview-card-action="primary"]').click();
await a.page.locator('[data-card="task"]').waitFor();
const title = demand.tasks.find((task) => task.id === first.id)!.title;
for (const [page, name] of [
  [a.page, title.en],
  [b.page, title.de],
] as const) {
  await page.getByRole("button", { name: new RegExp(name.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&"), "u") }).first().click();
  await page.locator(`[data-card="task"] [data-presence-anchor="task:${first.id}"], [data-presence-anchor="task:${first.id}"]`).first().waitFor();
}
const itemB = b.page.locator(`[data-quiz-item="${shared}"]`);
await itemB.waitFor();
if (first.kind === "classification") {
  const category = (first as Extract<SheetTask, { kind: "classification" }>).categories[1]!.id;
  await itemB.locator("select").selectOption(category);
  facts.bAnswered = { item: shared, category };
}
const itemBox = (await itemB.boundingBox())!;
await b.page.mouse.move(itemBox.x + itemBox.width * 0.5, itemBox.y + itemBox.height * 0.5, { steps: 10 });
facts.bOnItemSeenByA = await peerMark(a.page, "[data-presence-layer]", "cursor", B.handle);
await a.page.locator(`[data-crowd-item="${shared}"]`).waitFor({ timeout: 15_000 });
facts.crowdOnA = {
  source: await a.page.locator('[data-card="task"] [data-crowd-source]').innerText(),
  sentence: await a.page.locator(`[data-crowd-item="${shared}"] .sr-only`).innerText(),
  chips: await a.page.locator(`[data-crowd-item="${shared}"] > span.border`).allInnerTexts(),
  liveRegionAncestors: await a.page.locator(`[data-crowd-item="${shared}"]`).evaluate((element) => element.closest('[aria-live], [role="status"], [role="alert"]') !== null),
};
await shot(a.page, "a-run-ben-on-item-and-his-draft-1440-light");

const grip = itemB.locator("[data-quiz-grip]");
if ((await grip.count()) > 0) {
  const gripBox = (await grip.boundingBox())!;
  await b.page.mouse.move(gripBox.x + gripBox.width / 2, gripBox.y + gripBox.height / 2);
  await b.page.mouse.down();
  await b.page.mouse.move(gripBox.x + 60, gripBox.y + 30, { steps: 12 });
  facts.bDragSeenByA = await peerMark(a.page, "[data-presence-layer]", "cursor", "▸");
  await shot(a.page, "a-run-ben-drags-item-1440-light");
  await shot(b.page, "b-run-dragging-1280-dark-de");
  await b.page.keyboard.press("Escape");
  await b.page.mouse.up();
}
facts.crowdOnB = await b.page.evaluate(() => [...document.querySelectorAll('[data-card="task"] [data-crowd-source]')].map((element) => element.textContent));
//#endregion

//#region (c) results beside everyone
await a.page.goto(A.site);
await a.page.locator('[data-card="board"] tbody tr').first().waitFor();
await a.page.locator('[data-card="quiz:demand"]').getByRole("button", { name: "View last result" }).click();
await a.page.locator(".quiz-results").waitFor();
await a.page.locator('.quiz-results [data-crowd-source="submitted"]').waitFor({ timeout: 15_000 });
facts.resultsOnA = {
  source: await a.page.locator(".quiz-results [data-crowd-source]").innerText(),
  headers: await a.page.locator(".quiz-results table").first().locator("th[scope=col], thead th").allInnerTexts(),
  sentences: (await a.page.locator(".quiz-results [data-crowd-item] .sr-only").allInnerTexts()).slice(0, 4),
  nobody: await a.page.locator(".quiz-results td .sr-only", { hasText: "Nobody has answered this yet." }).count(),
};
await shot(a.page, "a-results-beside-everyone-1440-light");
await a.page.locator(".quiz-results table").nth(1).scrollIntoViewIfNeeded();
await shot(a.page, "a-results-matching-beside-everyone-1440-light");
//#endregion
} catch (error) {
  await a.page.screenshot({ path: join(out, "failure-a.png") });
  await b.page.screenshot({ path: join(out, "failure-b.png") });
  facts.failure = String(error);
  facts.errors = errors;
  writeFileSync(join(out, "crowd-report.json"), `${JSON.stringify(facts, null, 2)}\n`);
  throw error;
}

await a.context.close();
await b.context.close();
await browser.close();
facts.errors = errors;
writeFileSync(join(out, "crowd-report.json"), `${JSON.stringify(facts, null, 2)}\n`);
console.log(`[DEBUG] ${JSON.stringify(facts)}`);
