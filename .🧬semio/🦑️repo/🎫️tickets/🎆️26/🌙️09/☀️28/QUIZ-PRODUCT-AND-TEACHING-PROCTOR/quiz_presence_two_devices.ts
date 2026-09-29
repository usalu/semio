/** 👥️ Two devices against the dev proctor's real presence sockets (`.claude/launch.json` `teaching-proctor` +
 * `architecture-quiz`): device A at http://localhost:6061 (1440 px, light) and device B at http://127.0.0.1:6061
 * (768 px, dark) — two origins, so two local stores and two learners. Screenshots and facts go to
 * `🗑️generated/layered-final/presence-*`. Other sessions may be online on the dev proctor too, so the walk waits for
 * the two learners by name rather than for a count.
 *
 * Presence is ephemeral and the proctor admits it without a known learner, so the walk creates no learner, run or
 * answer: the learner view, leaderboard and run queries are answered here with two synthetic learners (Ann K. and
 * Ben), every command is refused before it leaves the page, and only the presence WebSockets reach the proctor.
 *
 * Usage: bun <this file>
 */

import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium, type Browser, type BrowserContext, type Page } from "playwright";
import { decodeQueryEnvelope, encodeQueryResult } from "../../../../../../../🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🟦️.ts";
import { learnerTag, runSeed, sheetOf, type Quiz } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts";
import { readFileSync } from "node:fs";

const here = dirname(fileURLToPath(import.meta.url));
const out = join(here, "🗑️generated", "layered-final");
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
}

const A: Device = { name: "a", site: "http://localhost:6061/", learner: "a0a0a0a0".repeat(4), handle: "Ann K.", width: 1440, height: 900, scheme: "light" };
const B: Device = { name: "b", site: "http://127.0.0.1:6061/", learner: "b0b0b0b0".repeat(4), handle: "Ben", width: 768, height: 1024, scheme: "dark" };
const demandRun = (device: Device) => (device === A ? "3" : "4").repeat(32);

const snapshot = (value: unknown) => JSON.stringify(encodeQueryResult({ kind: "snapshot", value: encoder.encode(JSON.stringify(value)), frontier: null }));

function learnerView(device: Device) {
  return {
    learner: device.learner,
    identity: { kind: "pseudonym", handle: device.handle },
    runs: [{ run: demandRun(device), quiz: "demand", status: "open", startedAt: 1_790_602_000_000 }],
    badges: [],
    best: {},
    total: 0,
  };
}

const leaderboard = {
  rows: [A, B].map((device, index) => ({
    rank: index + 1,
    tag: learnerTag(device.learner),
    identity: { kind: "pseudonym", handle: device.handle },
    total: 200 - index * 50,
    reachedAt: 1_790_600_000_000 + index,
    best: { physics: 0.9 - index / 10 },
    badges: [],
    runs: 1,
    lastActivity: 1_790_602_000_000 - index * 60_000,
  })),
};

async function device(browser: Browser, device: Device, errors: string[]): Promise<{ readonly context: BrowserContext; readonly page: Page }> {
  const context = await browser.newContext({ viewport: { width: device.width, height: device.height }, colorScheme: device.scheme, locale: "en-US" });
  await context.addInitScript(
    ({ tenant, learner, handle, scheme }) => {
      localStorage.setItem(`semio.quiz.${tenant}.introduced`, "true");
      localStorage.setItem(`semio.quiz.${tenant}.learner`, JSON.stringify({ id: learner, identity: { kind: "pseudonym", handle } }));
      localStorage.setItem(`semio.quiz.${tenant}.preferences`, JSON.stringify({ locale: "en", theme: scheme, textSize: "normal", showCursors: true }));
    },
    { tenant, learner: device.learner, handle: device.handle, scheme: device.scheme },
  );
  await context.route("**/commands", (route) => route.abort());
  await context.route("**/queries", async (route) => {
    const envelope = decodeQueryEnvelope(JSON.parse(route.request().postData() ?? "{}"));
    const query = JSON.parse(decoder.decode(envelope.arguments)) as { readonly type: string; readonly run?: string };
    if (query.type === "learner") return route.fulfill({ status: 200, contentType: "application/json", body: snapshot(learnerView(device)) });
    if (query.type === "leaderboard") return route.fulfill({ status: 200, contentType: "application/json", body: snapshot(leaderboard) });
    if (query.type === "run" && query.run === demandRun(device))
      return route.fulfill({ status: 200, contentType: "application/json", body: snapshot({ run: query.run, learner: device.learner, quiz: "demand", status: "open", sheet: sheetOf(demand, runSeed(query.run)), answers: {}, startedAt: 1_790_602_000_000 }) });
    return route.continue();
  });
  const page = await context.newPage();
  page.on("console", (message) => message.type() === "error" && !message.text().includes("ERR_FAILED") && errors.push(`${device.name} console: ${message.text()}`));
  page.on("pageerror", (error) => errors.push(`${device.name} pageerror: ${error.stack ?? error.message}`));
  await page.goto(device.site);
  await page.locator('[data-card="board"] tbody tr').first().waitFor();
  return { context, page };
}

async function shot(page: Page, name: string): Promise<void> {
  await page.waitForTimeout(400);
  await page.screenshot({ path: join(out, `presence-${name}.png`) });
}

async function peerMark(page: Page, kind: "cursor" | "focus", label: string) {
  const mark = page.locator(`[data-presence-layer] [data-peer="${kind}"]`, { hasText: label });
  await mark.waitFor({ state: "attached", timeout: 10_000 });
  await page.waitForFunction((selector) => [...document.querySelectorAll<HTMLElement>(selector)].some((element) => !element.hidden), `[data-presence-layer] [data-peer="${kind}"]`);
  await page.waitForTimeout(300);
  return mark.evaluate((element: HTMLElement) => {
    const anchor = document.querySelector<HTMLElement>(`[data-presence-anchor="${element.dataset.anchor}"]`)!;
    const box = anchor.getBoundingClientRect();
    const own = element.getBoundingClientRect();
    return { anchor: element.dataset.anchor, x: element.dataset.x, y: element.dataset.y, relativeX: (own.left - box.left) / box.width, relativeY: (own.top - box.top) / box.height, width: own.width, height: own.height, boxWidth: box.width, boxHeight: box.height, colour: getComputedStyle(element).getPropertyValue("--quiz-peer").trim(), ariaHidden: element.closest("[aria-hidden]")?.getAttribute("aria-hidden") };
  });
}

const facts: Record<string, unknown> = {};
const errors: string[] = [];
const browser = await chromium.launch({ channel: "chrome" });
const a = await device(browser, A, errors);
const b = await device(browser, B, errors);

const listed = (page: Page, handle: string, present = true) => page.waitForFunction(({ handle, present }) => [...document.querySelectorAll("[data-presence-list] li")].some((item) => item.textContent?.includes(handle)) === present, { handle, present }, { timeout: 15_000 });
await listed(a.page, B.handle);
await listed(b.page, A.handle);
facts.onlineA = await a.page.locator("[data-presence-status]").innerText();
await a.page.locator("[data-presence-list] summary").click();
facts.listA = await a.page.locator("[data-presence-list] li").allInnerTexts();

const heating = a.page.locator('[data-presence-anchor="home:quiz:heating"]');
const target = (await heating.boundingBox())!;
await a.page.mouse.move(target.x + target.width * 0.3, target.y + target.height * 0.6, { steps: 8 });
facts.cursorOnB = await peerMark(b.page, "cursor", A.handle);
facts.annMarkOnB = await b.page.locator('[data-card="board"] tr', { hasText: A.handle }).locator(".quiz-online").evaluate((element) => getComputedStyle(element).getPropertyValue("--quiz-peer").trim());
await shot(b.page, "home-b-sees-a-cursor-768-dark");
await shot(a.page, "home-a-pointing-1440-light");

for (let step = 0; step < 40; step += 1) {
  await b.page.keyboard.press("Tab");
  const inside = await b.page.evaluate(() => document.activeElement?.closest("[data-presence-anchor]")?.getAttribute("data-presence-anchor") ?? null);
  if (inside === "home:quiz:heating") break;
}
facts.focusOnA = await peerMark(a.page, "focus", B.handle);
await shot(a.page, "home-a-sees-b-focus-1440-light");

await b.page.getByRole("region", { name: "Energy Demand" }).getByRole("button", { name: "Resume quiz" }).click();
await b.page.locator('[data-card="task"]').waitFor();
await a.page.getByRole("region", { name: "Energy Demand" }).getByText("Learning now: 1").waitFor({ timeout: 10_000 });
facts.listAWhileBRuns = await a.page.locator("[data-presence-list] li").allInnerTexts();
facts.demandCardA = await a.page.getByRole("region", { name: "Energy Demand" }).innerText();
await shot(a.page, "home-a-sees-b-in-demand-1440-light");

await a.page.getByRole("region", { name: "Energy Demand" }).getByRole("button", { name: "Resume quiz" }).click();
await a.page.locator('[data-card="task"]').waitFor();
const run = (await a.page.locator('[data-presence-anchor="run"]').boundingBox())!;
await a.page.mouse.move(run.x + run.width * 0.8, run.y + run.height * 0.3, { steps: 8 });
facts.runCursorOnB = await peerMark(b.page, "cursor", A.handle);
await shot(b.page, "run-b-sees-a-cursor-768-dark");

await b.page.getByRole("button", { name: "Overview" }).first().click();
await b.page.locator('[data-card="board"] tbody tr').first().waitFor();
await a.page.waitForTimeout(800);
facts.cursorsOnAAfterBLeft = await a.page.locator("[data-presence-layer] [data-peer]").count();
await b.page.locator('[data-card="board"] tbody tr').first().waitFor();
facts.onlineMarksB = await b.page.locator('[data-card="board"] .quiz-online').count();
await shot(b.page, "home-b-online-marks-768-dark");

await a.context.close();
await listed(b.page, A.handle, false);
facts.onlineBAfterAClosed = await b.page.locator("[data-presence-status]").innerText();
await b.context.close();
await browser.close();
facts.errors = errors;
writeFileSync(join(out, "presence-report.json"), `${JSON.stringify(facts, null, 2)}\n`);
console.log(`[DEBUG] ${JSON.stringify(facts)}`);
