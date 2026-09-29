/** 📸️ Screenshots of the quiz card grid in the dev site (`.claude/launch.json` `teaching-proctor` + `architecture-quiz`)
 * at 1440, 768 and 375 px in light and dark, into `🗑️generated/ui-final/`, with the computed home grid columns, the
 * horizontal overflow and every console error per page.
 *
 * The first visit (introduction, identity) runs against the dev proctor untouched. The identified screens run the real
 * site too, with the catalog from the dev proctor; only the learner view and the leaderboard queries are answered here
 * with a fixed demo learner and eight ranked learners, and every command is refused before it leaves the page — so the
 * walk creates no learner, run or answer in the dev proctor.
 *
 * Usage: bun <this file> [site url]
 */

import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium, type BrowserContext, type Page } from "playwright";
import { decodeQueryEnvelope, encodeQueryResult } from "../../../../../../../🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🟦️.ts";
import { learnerTag, runSeed, scoreRun, sheetOf, type Answer, type Quiz, type SheetTask } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts";
import { readFileSync } from "node:fs";

const here = dirname(fileURLToPath(import.meta.url));
const out = join(here, "🗑️generated", "ui-final");
mkdirSync(out, { recursive: true });
const site = process.argv[2] ?? "http://localhost:6061/";
const tenant = "architecture";
const learner = "c0ffee00".repeat(4);
const encoder = new TextEncoder();
const decoder = new TextDecoder();

const learnerView = {
  learner,
  identity: { kind: "pseudonym", handle: "Ann K." },
  runs: [
    { run: "1".repeat(32), quiz: "physics", status: "submitted", score: 0.823, startedAt: 1_790_600_000_000, submittedAt: 1_790_600_600_000 },
    { run: "2".repeat(32), quiz: "cooling", status: "submitted", score: 1, startedAt: 1_790_601_000_000, submittedAt: 1_790_601_500_000 },
    { run: "3".repeat(32), quiz: "demand", status: "open", startedAt: 1_790_602_000_000 },
  ],
  badges: [
    { badge: "cooling-expert", run: "2".repeat(32), at: 1_790_601_500_000 },
    { badge: "pattern-seer", run: "2".repeat(32), at: 1_790_601_500_000 },
  ],
  best: { physics: 0.823, cooling: 1 },
  total: 182.3,
};

const energy = join(here, "..", "..", "..", "..", "..", "..", "..", "🎓️teaching", "🏛️architecture", "⚡️energy");
const quizOf = (folder: string): Quiz => JSON.parse(readFileSync(join(energy, folder, "❓️quiz", "🔣️.json"), "utf8")) as Quiz;

function sloppy(task: SheetTask): Answer {
  switch (task.kind) {
    case "classification":
      return { kind: "classification", assignments: Object.fromEntries(task.items.map((item, index) => [item.id, task.categories[index % task.categories.length]!.id])) };
    case "sorting":
      return { kind: "sorting", order: task.items.map((item) => item.id) };
    case "matching":
      return { kind: "matching", assignments: Object.fromEntries(task.dimensions.map((dimension) => [dimension.id, Object.fromEntries(task.items.map((item, index) => [item.id, index]))])) };
  }
}

const physics = quizOf("🧲️physics");
const physicsRun = "1".repeat(32);
const physicsSheet = sheetOf(physics, runSeed(physicsRun));
const physicsAnswers = Object.fromEntries(physicsSheet.tasks.map((task) => [task.id, sloppy(task)]));
const demand = quizOf("📊️demand");
const demandRun = "3".repeat(32);
const runViews: Readonly<Record<string, unknown>> = {
  [physicsRun]: { run: physicsRun, learner, quiz: "physics", status: "submitted", sheet: physicsSheet, answers: physicsAnswers, result: scoreRun(physics, physicsSheet, physicsAnswers), startedAt: 1_790_600_000_000, submittedAt: 1_790_600_600_000 },
  [demandRun]: { run: demandRun, learner, quiz: "demand", status: "open", sheet: sheetOf(demand, runSeed(demandRun)), answers: {}, startedAt: 1_790_602_000_000 },
};

const others = ["Mira", "solar_fox", null, "Jonas B.", "kwh_kid", "Lena", "Tim R."];
const leaderboard = {
  rows: [...others.map((handle, index) => ({ rank: index + 1, handle, tag: (index + 1).toString(16).padStart(8, "a") })), { rank: 8, handle: "Ann K.", tag: learnerTag(learner) }].map((entry, index) => ({
    rank: entry.rank,
    tag: entry.tag,
    identity: entry.handle === null ? { kind: "anonymous" } : { kind: "pseudonym", handle: entry.handle },
    total: 390 - index * 28.5,
    reachedAt: 1_790_600_000_000 + index,
    best: { physics: 0.9, heating: 0.8, cooling: 0.7 },
    badges: ["physics-expert", "completionist"].slice(0, Math.max(0, 3 - Math.floor(index / 3))),
    runs: 3 + (index % 3),
    lastActivity: 1_790_602_000_000 - index * 60_000,
  })),
};

const snapshot = (value: unknown) => JSON.stringify(encodeQueryResult({ kind: "snapshot", value: encoder.encode(JSON.stringify(value)), frontier: null }));

async function demo(context: BrowserContext): Promise<void> {
  await context.addInitScript(
    ({ tenant, learner }) => {
      localStorage.setItem(`semio.quiz.${tenant}.introduced`, "true");
      localStorage.setItem(`semio.quiz.${tenant}.learner`, JSON.stringify({ id: learner, identity: { kind: "pseudonym", handle: "Ann K." } }));
      localStorage.setItem(`semio.quiz.${tenant}.preferences`, JSON.stringify({ locale: "en", theme: "system", textSize: "normal" }));
    },
    { tenant, learner },
  );
  await context.route("**/commands", (route) => route.abort());
  await context.route("**/queries", async (route) => {
    const envelope = decodeQueryEnvelope(JSON.parse(route.request().postData() ?? "{}"));
    const query = JSON.parse(decoder.decode(envelope.arguments)) as { readonly type: string };
    if (query.type === "learner") return route.fulfill({ status: 200, contentType: "application/json", body: snapshot(learnerView) });
    if (query.type === "leaderboard") return route.fulfill({ status: 200, contentType: "application/json", body: snapshot(leaderboard) });
    if (query.type === "run") return route.fulfill({ status: 200, contentType: "application/json", body: snapshot(runViews[(query as unknown as { readonly run: string }).run]) });
    return route.continue();
  });
}

interface Report {
  readonly name: string;
  readonly width: number;
  readonly scheme: string;
  readonly columns?: number;
  readonly overflow: number;
  readonly errors: readonly string[];
  readonly headings: readonly string[];
  readonly wrapped: readonly string[];
  readonly nowrapHeads: boolean;
}

const reports: Report[] = [];

async function shoot(page: Page, name: string, width: number, scheme: string, errors: string[]): Promise<void> {
  await page.waitForTimeout(600);
  const facts = await page.evaluate(() => {
    const grid = document.querySelector(".quiz-home-grid");
    const main = document.querySelector("main");
    return {
      columns: grid === null ? undefined : getComputedStyle(grid).gridTemplateColumns.split(" ").filter(Boolean).length,
      overflow: Math.max(document.documentElement.scrollWidth - window.innerWidth, main === null ? 0 : main.scrollWidth - main.clientWidth),
      headings: [...document.querySelectorAll("h1, h2")].map((heading) => `${heading.tagName}:${heading.textContent}`),
      wrapped: [...document.querySelectorAll("table th[scope=col], table .quiz-nowrap")].flatMap((cell) => {
        const walker = document.createTreeWalker(cell, NodeFilter.SHOW_TEXT);
        const rects: DOMRect[] = [];
        for (let node = walker.nextNode(); node !== null; node = walker.nextNode()) {
          const range = document.createRange();
          range.selectNodeContents(node);
          rects.push(...[...range.getClientRects()].filter((rect) => rect.width > 0));
        }
        const style = getComputedStyle(cell);
        const line = Number.parseFloat(style.lineHeight) || Number.parseFloat(style.fontSize) * 1.2;
        const lines = rects.length === 0 ? 0 : Math.round((Math.max(...rects.map((rect) => rect.bottom)) - Math.min(...rects.map((rect) => rect.top))) / line);
        return lines > 1 ? [`${cell.textContent?.trim()} (${lines} lines)`] : [];
      }),
      nowrapHeads: [...document.querySelectorAll("table th[scope=col]")].every((cell) => getComputedStyle(cell).whiteSpace === "nowrap"),
    };
  });
  reports.push({ name, width, scheme, ...facts, errors: [...errors] });
  await page.screenshot({ path: join(out, `${name}-${width}-${scheme}.png`), fullPage: false });
  const scroller = await page.$("main");
  if (scroller !== null && width < 1024) {
    const height = await page.evaluate(() => document.querySelector("main")!.scrollHeight);
    await page.evaluate(() => {
      const main = document.querySelector("main")!;
      main.style.overflow = "visible";
      main.style.flex = "none";
      document.querySelector<HTMLElement>(".quiz-app")!.style.height = "auto";
    });
    await page.setViewportSize({ width, height: Math.min(6000, height + 60) });
    await page.screenshot({ path: join(out, `${name}-${width}-${scheme}-full.png`), fullPage: true });
  }
}

const browser = await chromium.launch({ channel: "chrome" });
for (const [width, height, schemes] of [
  [1440, 900, ["light", "dark"]],
  [1280, 800, ["light"]],
  [1024, 768, ["light"]],
  [768, 1024, ["light", "dark"]],
  [375, 812, ["light", "dark"]],
] as const) {
  if (process.env.ONLY_WIDTH !== undefined && String(width) !== process.env.ONLY_WIDTH) continue;
  for (const scheme of schemes) {
    const first = await browser.newContext({ viewport: { width, height }, colorScheme: scheme, locale: "en-US" });
    const errors: string[] = [];
    const visit = await first.newPage();
    visit.on("console", (message) => message.type() === "error" && errors.push(message.text()));
    visit.on("pageerror", (error) => errors.push(error.message));
    await visit.goto(site);
    await visit.getByRole("heading", { level: 1 }).first().waitFor();
    await shoot(visit, "introduction", width, scheme, errors);
    await first.close();

    const context = await browser.newContext({ viewport: { width, height }, colorScheme: scheme, locale: "en-US" });
    await demo(context);
    const homeErrors: string[] = [];
    const page = await context.newPage();
    page.on("console", (message) => message.type() === "error" && !message.text().includes("ERR_FAILED") && homeErrors.push(`console: ${message.text()} @ ${JSON.stringify(message.location())}`));
    page.on("pageerror", (error) => homeErrors.push(`pageerror: ${error.stack ?? error.message}`));
    await page.goto(site);
    await page.locator('[data-card="board"] tbody tr').first().waitFor();
    await shoot(page, "home", width, scheme, homeErrors);
    await page.setViewportSize({ width, height });
    await page.goto(site);
    await page.locator('[data-card="board"] tbody tr').first().waitFor();
    await page.getByRole("button", { name: "Full leaderboard" }).click();
    await page.getByRole("heading", { level: 1, name: "Leaderboard" }).waitFor();
    await shoot(page, "leaderboard", width, scheme, homeErrors);
    await page.setViewportSize({ width, height });
    await page.goto(site);
    await page.locator('[data-card="board"] tbody tr').first().waitFor();
    await page.getByRole("button", { name: "All badges" }).click();
    await page.getByRole("heading", { level: 1, name: "Badges" }).waitFor();
    await shoot(page, "badges", width, scheme, homeErrors);
    await page.setViewportSize({ width, height });
    await page.goto(site);
    await page.locator('[data-card="board"] tbody tr').first().waitFor();
    await page.getByRole("region", { name: "Energy Demand" }).getByRole("button", { name: "Resume quiz" }).click();
    await page.locator('[data-card="task"]').waitFor();
    await shoot(page, "run", width, scheme, homeErrors);
    await page.setViewportSize({ width, height });
    await page.goto(site);
    await page.locator('[data-card="board"] tbody tr').first().waitFor();
    await page.getByRole("region", { name: "Physical Understanding" }).getByRole("button", { name: "View last result" }).click();
    await page.locator('[data-card="task-result"]').first().waitFor();
    await shoot(page, "results", width, scheme, homeErrors);
    await context.close();
  }
}
await browser.close();
writeFileSync(join(out, "report.json"), `${JSON.stringify(reports, null, 2)}\n`);
for (const report of reports) console.log(`[DEBUG] ${report.name} ${report.width} ${report.scheme} columns=${report.columns ?? "-"} overflow=${report.overflow} errors=${report.errors.length} wrapped=${report.wrapped.length} nowrapHeads=${report.nowrapHeads}`);
