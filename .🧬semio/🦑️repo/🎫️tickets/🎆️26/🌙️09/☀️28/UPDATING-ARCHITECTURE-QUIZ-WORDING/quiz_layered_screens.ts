/** 🥞️ Screenshots and facts of the layered quiz home in the dev site (`.claude/launch.json` `teaching-proctor` +
 * `architecture-quiz`) at 1440, 768 and 375 px in light and dark, into `🗑️generated/layered-final/`: at rest (the first
 * page blurred behind the glass), each card hovered (its page clear), a page opened by its heading link and closed with
 * Escape, and the phone list. Facts per shot: the veil's visibility and clip, the strip's transform, the revealed and
 * opened pane, where focus is, horizontal overflow and every console error.
 *
 * Like `quiz_ui_screenshots.ts`, the learner view, leaderboard and run queries are answered here with a synthetic demo
 * learner and every command is refused before it leaves the page, so the walk writes nothing to the dev proctor.
 *
 * Usage: bun <this file> [site url]
 */

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium, type BrowserContext, type Page } from "playwright";
import { decodeQueryEnvelope, encodeQueryResult } from "../../../../../../../🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🟦️.ts";
import { learnerTag, runSeed, scoreRun, sheetOf, type Answer, type Quiz, type SheetTask } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const out = join(here, "🗑️generated", "layered-final");
mkdirSync(out, { recursive: true });
const site = process.argv[2] ?? "http://localhost:6061/";
const tenant = "architecture";
const learner = "c0ffee00".repeat(4);
const encoder = new TextEncoder();
const decoder = new TextDecoder();
const PAGES = ["learner", "physics", "intro", "heating", "board", "cooling", "badges", "demand", "prefs"] as const;

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
const learnerView = {
  learner,
  identity: { kind: "pseudonym", handle: "Ann K." },
  runs: [
    { run: physicsRun, quiz: "physics", status: "submitted", score: 0.823, startedAt: 1_790_600_000_000, submittedAt: 1_790_600_600_000 },
    { run: "2".repeat(32), quiz: "cooling", status: "submitted", score: 1, startedAt: 1_790_601_000_000, submittedAt: 1_790_601_500_000 },
    { run: demandRun, quiz: "demand", status: "open", startedAt: 1_790_602_000_000 },
  ],
  badges: [
    { badge: "cooling-expert", run: "2".repeat(32), at: 1_790_601_500_000 },
    { badge: "pattern-seer", run: "2".repeat(32), at: 1_790_601_500_000 },
  ],
  best: { physics: 0.823, cooling: 1 },
  total: 182.3,
};
const runViews: Readonly<Record<string, unknown>> = {
  [physicsRun]: { run: physicsRun, learner, quiz: "physics", status: "submitted", sheet: physicsSheet, answers: physicsAnswers, result: scoreRun(physics, physicsSheet, physicsAnswers), startedAt: 1_790_600_000_000, submittedAt: 1_790_600_600_000 },
  [demandRun]: { run: demandRun, learner, quiz: "demand", status: "open", sheet: sheetOf(demand, runSeed(demandRun)), answers: {}, startedAt: 1_790_602_000_000 },
};
const others = ["Mira", "solar_fox", null, "Jonas B.", "kwh_kid", "Lena", "Tim R.", "u_value", "passivhaus"];
const leaderboard = {
  rows: [...others.map((handle, index) => ({ rank: index + 1, handle, tag: (index + 1).toString(16).padStart(8, "a") })), { rank: 10, handle: "Ann K.", tag: learnerTag(learner) }].map((entry, index) => ({
    rank: entry.rank,
    tag: entry.tag,
    identity: entry.handle === null ? { kind: "anonymous" } : { kind: "pseudonym", handle: entry.handle },
    total: 390 - index * 21.5,
    reachedAt: 1_790_600_000_000 + index,
    best: { physics: 0.9 - index / 40, heating: 0.8 - index / 50, cooling: 0.7, demand: 0.6 },
    badges: ["physics-expert", "completionist"].slice(0, Math.max(0, 3 - Math.floor(index / 3))),
    runs: 3 + (index % 3),
    lastActivity: 1_790_602_000_000 - index * 60_000,
  })),
};
const snapshot = (value: unknown) => JSON.stringify(encodeQueryResult({ kind: "snapshot", value: encoder.encode(JSON.stringify(value)), frontier: null }));

async function demo(context: BrowserContext, scheme: string): Promise<void> {
  await context.addInitScript(
    ({ tenant, learner, scheme }) => {
      localStorage.setItem(`semio.quiz.${tenant}.introduced`, "true");
      localStorage.setItem(`semio.quiz.${tenant}.learner`, JSON.stringify({ id: learner, identity: { kind: "pseudonym", handle: "Ann K." } }));
      localStorage.setItem(`semio.quiz.${tenant}.preferences`, JSON.stringify({ locale: "en", theme: scheme, textSize: "normal", showCursors: true }));
    },
    { tenant, learner, scheme },
  );
  await context.route("**/commands", (route) => route.abort());
  await context.route("**/queries", async (route) => {
    const envelope = decodeQueryEnvelope(JSON.parse(route.request().postData() ?? "{}"));
    const query = JSON.parse(decoder.decode(envelope.arguments)) as { readonly type: string; readonly run?: string };
    if (query.type === "learner") return route.fulfill({ status: 200, contentType: "application/json", body: snapshot(learnerView) });
    if (query.type === "leaderboard") return route.fulfill({ status: 200, contentType: "application/json", body: snapshot(leaderboard) });
    if (query.type === "run") return route.fulfill({ status: 200, contentType: "application/json", body: snapshot(runViews[query.run ?? ""]) });
    return route.continue();
  });
}

async function facts(page: Page) {
  return page.evaluate(() => {
    const veil = document.querySelector<HTMLElement>("[data-layered-veil]");
    const strip = document.querySelector<HTMLElement>("[data-layered-strip]");
    const main = document.querySelector("main");
    return {
      hash: location.hash,
      veil: veil === null ? null : { visibility: getComputedStyle(veil).visibility, clip: getComputedStyle(veil).clipPath },
      strip: strip?.style.transform ?? null,
      revealed: [...document.querySelectorAll("[data-layered-pane][data-revealed]")].map((pane) => pane.getAttribute("data-layered-pane")),
      opened: [...document.querySelectorAll("[data-layered-pane][data-opened]")].map((pane) => pane.getAttribute("data-layered-pane")),
      live: [...document.querySelectorAll("[data-layered-pane]")].filter((pane) => pane.querySelector("[data-page]") !== null).length,
      cards: document.querySelectorAll(".quiz-home-grid section[data-card], [data-layered-card] section[data-card]").length,
      focus: document.activeElement === null ? null : `${document.activeElement.tagName}:${document.activeElement.textContent?.trim().slice(0, 40) ?? ""}`,
      overflow: Math.max(document.documentElement.scrollWidth - window.innerWidth, main === null ? 0 : main.scrollWidth - main.clientWidth),
    };
  });
}

const report: Record<string, unknown>[] = [];
const browser = await chromium.launch({ channel: "chrome" });
for (const [width, height] of [
  [1440, 900],
  [768, 1024],
  [375, 812],
] as const) {
  for (const scheme of ["light", "dark"] as const) {
    const context = await browser.newContext({ viewport: { width, height }, colorScheme: scheme, locale: "en-US", hasTouch: width < 768, isMobile: width < 768 });
    await demo(context, scheme);
    const errors: string[] = [];
    const page = await context.newPage();
    page.on("console", (message) => message.type() === "error" && !message.text().includes("ERR_FAILED") && errors.push(`console: ${message.text()}`));
    page.on("pageerror", (error) => errors.push(`pageerror: ${error.stack ?? error.message}`));
    await page.goto(site);
    await page.locator('[data-card="board"] tbody tr').first().waitFor();
    await page.waitForTimeout(1500);
    const shot = async (name: string): Promise<void> => {
      await page.waitForTimeout(700);
      report.push({ name, width, scheme, ...(await facts(page)), errors: [...errors] });
      await page.screenshot({ path: join(out, `quiz-${name}-${width}-${scheme}.png`) });
    };
    await shot("rest");
    if (width >= 768) {
      const hovered = scheme === "light" ? PAGES : (["board", "heating"] as const);
      for (const id of hovered) {
        const card = page.locator(`[data-layered-card="${id}"] section[data-card]`);
        await card.scrollIntoViewIfNeeded();
        const box = (await card.boundingBox())!;
        await page.mouse.move(box.x + box.width / 2, box.y + Math.min(box.height / 2, 24), { steps: 6 });
        await shot(`hover-${id}`);
      }
      await page.mouse.move(2, height - 2, { steps: 4 });
      await page.waitForTimeout(400);
    } else {
      await page.locator('[data-layered-card="board"] section[data-card]').scrollIntoViewIfNeeded();
      await shot("list-board");
    }
    await page.locator('[data-layered-card="heating"] section[data-card] h2 a').click();
    await page.waitForFunction(() => location.hash === "#heating");
    await shot("open-heating");
    await page.keyboard.press("Escape");
    await page.waitForFunction(() => location.hash === "");
    await shot("closed");
    await context.close();
  }
}
await browser.close();
writeFileSync(join(out, "report.json"), `${JSON.stringify(report, null, 2)}\n`);
for (const entry of report) console.log(`[DEBUG] ${JSON.stringify(entry)}`);
