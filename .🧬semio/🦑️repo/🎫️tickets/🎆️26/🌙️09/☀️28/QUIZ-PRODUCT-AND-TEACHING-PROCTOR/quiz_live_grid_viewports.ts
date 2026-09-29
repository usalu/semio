/** 🥞️ The live grid of home at the design system's widths (§17), against the dev site (`.claude/launch.json`
 * `architecture-quiz`, with `teaching-proctor` for presence): for each viewport, whether every card fits inside its
 * page's cell (the card layer shares the grid's tracks, so a card taller than its cell would overlap its neighbours), the
 * rest mode and the layout. A synthetic learner answers the learner and leaderboard queries; commands are refused, so
 * nothing is created. Screenshots and facts go to `🗑️generated/crowd-final/viewport-*`.
 *
 * Usage: bun <this file>
 */

import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";
import { decodeQueryEnvelope, encodeQueryResult } from "../../../../../../../🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🟦️.ts";
import { learnerTag } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const out = join(here, "🗑️generated", "crowd-final");
mkdirSync(out, { recursive: true });
const tenant = "architecture";
const learner = "c0c0c0c0".repeat(4);
const encoder = new TextEncoder();
const decoder = new TextDecoder();
const snapshot = (value: unknown) => JSON.stringify(encodeQueryResult({ kind: "snapshot", value: encoder.encode(JSON.stringify(value)), frontier: null }));
const VIEWPORTS = [
  { width: 1440, height: 900 },
  { width: 1280, height: 720 },
  { width: 1024, height: 768 },
  { width: 1023, height: 768 },
  { width: 768, height: 1024 },
  { width: 375, height: 812 },
];

const facts: Record<string, unknown>[] = [];
const browser = await chromium.launch({ channel: "chrome" });
for (const viewport of VIEWPORTS) {
  const context = await browser.newContext({ viewport, colorScheme: "light", locale: "en-US" });
  await context.addInitScript(
    ({ tenant, learner }) => {
      localStorage.setItem(`semio.quiz.${tenant}.introduced`, "true");
      localStorage.setItem(`semio.quiz.${tenant}.learner`, JSON.stringify({ id: learner, identity: { kind: "pseudonym", handle: "Cleo" } }));
      localStorage.setItem(`semio.quiz.${tenant}.preferences`, JSON.stringify({ locale: "en", theme: "light", textSize: "normal", showCursors: true, showAnswers: true }));
    },
    { tenant, learner },
  );
  await context.route("**/commands", (route) => route.abort());
  await context.route("**/queries", async (route) => {
    const envelope = decodeQueryEnvelope(JSON.parse(route.request().postData() ?? "{}"));
    const query = JSON.parse(decoder.decode(envelope.arguments)) as { readonly type: string; readonly quiz?: string };
    const fulfil = (value: unknown) => route.fulfill({ status: 200, contentType: "application/json", body: snapshot(value) });
    if (query.type === "learner") return fulfil({ learner, identity: { kind: "pseudonym", handle: "Cleo" }, runs: [], badges: [], best: {}, total: 0 });
    if (query.type === "leaderboard") return fulfil({ rows: [{ rank: 1, tag: learnerTag(learner), identity: { kind: "pseudonym", handle: "Cleo" }, total: 90, reachedAt: 1, best: {}, badges: [], runs: 1, lastActivity: 1_790_602_000_000 }] });
    if (query.type === "crowd") return fulfil({ quiz: query.quiz, runs: 0, tasks: [] });
    return route.continue();
  });
  const page = await context.newPage();
  await page.goto("http://localhost:6061/");
  await page.locator('[data-card="board"]').first().waitFor();
  await page.waitForTimeout(1_000);
  const measured = await page.evaluate(() => {
    const overview = document.querySelector("[data-layered-overview]");
    const cells = [...document.querySelectorAll<HTMLElement>(".quiz-home-cell")].map((cell) => {
      const card = cell.querySelector("section[data-card]")!.getBoundingClientRect();
      const box = cell.getBoundingClientRect();
      return { card: cell.closest("[data-layered-card]")?.getAttribute("data-layered-card"), overflowTop: Math.round(box.top - card.top), overflowBottom: Math.round(card.bottom - box.bottom), cellHeight: Math.round(box.height), cardHeight: Math.round(card.height) };
    });
    return { mode: overview?.getAttribute("data-mode"), rest: overview?.getAttribute("data-rest"), cells, overflowing: cells.filter((cell) => cell.overflowTop > 0 || cell.overflowBottom > 0).map((cell) => cell.card) };
  });
  facts.push({ viewport, ...measured });
  await page.screenshot({ path: join(out, `viewport-${viewport.width}x${viewport.height}.png`) });
  if (viewport.width === 1440) {
    await page.locator('[data-card="quiz:demand"]').hover();
    for (const ms of [150, 350, 1_500]) {
      await page.waitForTimeout(ms);
      await page.screenshot({ path: join(out, `viewport-1440-reveal-demand-${ms}ms.png`) });
    }
    await page.mouse.move(5, 895);
    await page.goto("http://localhost:6061/#demand");
    await page.locator('[data-layered-pane="demand"] [data-card="quiz-crowd"]').waitFor();
    await page.waitForTimeout(800);
    await page.screenshot({ path: join(out, "viewport-1440-demand-page-opened.png") });
  }
  await context.close();
}
await browser.close();
writeFileSync(join(out, "viewport-report.json"), `${JSON.stringify(facts, null, 2)}\n`);
console.log(`[DEBUG] ${JSON.stringify(facts.map((entry) => ({ viewport: entry.viewport, mode: entry.mode, rest: entry.rest, overflowing: entry.overflowing })))}`);
