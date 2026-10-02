/** 🧭️ Proves a staged site folder the way its first visitor meets it: serves the folder as static files (as a CDN does),
 * opens it in Chromium, and — whatever the proctor baked into it does — reads the introduction, takes a pseudonym, plays
 * one quiz perfectly, submits it, reloads and reads the result again. Prints what the page said about its connection,
 * every request it made to the proctor origin and how each ended, and writes screenshots.
 *
 * `bun drive_published_site.ts <site directory> [port] [screenshot directory]` */
import { mkdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium, type BrowserContext, type Page } from "playwright";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";
import { answerRun, badgesFor, card, connection, enter, expectFeedback, goHome, percent, playQuiz, quizOf, screen, shownName, submitRun, type Device } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🚶️learner/🟦️.ts";
import { serveStaticSite } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🧱️stack/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, "../../../../../../..");
const directory = resolve(process.argv[2] ?? "");
const port = Number(process.argv[3] ?? "6190");
const shots = resolve(process.argv[4] ?? join(here, "🗑️generated", "published-site"));
mkdirSync(shots, { recursive: true });

const html = readFileSync(join(directory, "index.html"), "utf8");
const proctor = /connect-src (\S+)/u.exec(html)?.[1] ?? "";
console.log(`site folder ${directory}`);
console.log(`baked proctor origin ${proctor}`);

process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH;
const site = await serveStaticSite(directory, port);
const browser = await chromium.launch();
const context: BrowserContext = await browser.newContext({ baseURL: site.origin, viewport: { width: 1440, height: 900 }, locale: "en-GB" });
const page: Page = await context.newPage();
const toProctor: string[] = [];
const elsewhere: string[] = [];
const pageErrors: string[] = [];
const violations: string[] = [];
await context.exposeBinding("quizSecurityViolation", (_source, violation: string) => void violations.push(violation));
await context.addInitScript(() => document.addEventListener("securitypolicyviolation", (event) => (window as unknown as { quizSecurityViolation(violation: string): void }).quizSecurityViolation(`${event.effectiveDirective} blocked ${event.blockedURI || "inline code"}`)));
page.on("pageerror", (error) => pageErrors.push(error.message));
page.on("requestfailed", (request) => (request.url().startsWith(proctor) ? toProctor : elsewhere).push(`${request.method()} ${new URL(request.url()).pathname} failed: ${request.failure()?.errorText}`));
page.on("response", (response) => {
  if (response.url().startsWith(proctor)) toProctor.push(`${response.request().method()} ${new URL(response.url()).pathname} answered ${response.status()}`);
  else if (response.status() >= 400) elsewhere.push(`${response.request().method()} ${response.url()} answered ${response.status()}`);
});
const device: Device = { context, page, locale: "en", problems: [], expectingFailures: async (body) => (await body(), 0) };

try {
  const quiz = quizOf("cooling");
  const name = `Folder Proof ${Math.random().toString(36).slice(2, 8)}`;
  const started = Date.now();
  await enter(device, { kind: "pseudonym", handle: name });
  console.log(`overview after ${Date.now() - started} ms as ${await shownName(device)}`);
  await page.screenshot({ path: join(shots, "1-overview.png") });
  await playQuiz(device, quiz.id);
  await answerRun(device, quiz, "perfect");
  await submitRun(device);
  const results = await expectFeedback(device, quiz, "perfect");
  const earned = await screen(page, "results").locator("[data-earned] h3").allInnerTexts();
  console.log(`results: ${results.score} % in ${quiz.id}; badges earned: ${earned.join(", ") || "none"} (expected ${badgesFor([quiz.id]).map((badge) => badge.label.en).join(", ") || "none"})`);
  await page.screenshot({ path: join(shots, "2-results.png"), fullPage: true });
  await goHome(device);
  await page.reload();
  await page.locator("[data-layered-overview]").waitFor();
  const best = percent(await card(page, quiz.id).locator("li", { hasText: "%" }).innerText());
  console.log(`after a reload: ${await shownName(device)}, best ${best} % in ${quiz.id}, badges held ${await card(page, "badges").locator("li[data-earned]").count()}`);
  await page.waitForTimeout(5_000);
  console.log(`connection 5 s after the reload: tone ${await connection(device).getAttribute("data-tone")} — ${(await connection(device).innerText()).replace(/\s+/gu, " ").trim()}`);
  console.log(`leaderboard card: ${(await card(page, "board").locator("[data-board-local]").count()) > 0 ? "this device's own standing" : "no standing of this device"}; ${(await card(page, "board").innerText()).includes(name) ? "names the learner" : "does not name the learner"}`);
  await page.waitForTimeout(12_000);
  console.log(`connection 17 s after the reload: tone ${await connection(device).getAttribute("data-tone")} — ${(await connection(device).innerText()).replace(/\s+/gu, " ").trim()}`);
  const queued = await page.evaluate(() => Object.keys(localStorage).filter((key) => key.includes(".outbox/")).map((key) => (JSON.parse(localStorage.getItem(key) ?? "{}") as { command?: { type?: string } }).command?.type ?? "?"));
  console.log(`waiting on the device for the proctor: ${queued.length} commands (${[...new Set(queued)].map((type) => `${queued.filter((other) => other === type).length} × ${type}`).join(", ")})`);
  await page.screenshot({ path: join(shots, "3-overview-after-reload.png") });
} finally {
  const outcomes = new Map<string, number>();
  for (const line of toProctor) outcomes.set(line.replace(/^(\S+) (\/[a-z]+)\S* /u, "$1 $2 "), (outcomes.get(line.replace(/^(\S+) (\/[a-z]+)\S* /u, "$1 $2 ")) ?? 0) + 1);
  console.log(`requests to ${proctor}: ${toProctor.length}`);
  for (const [line, count] of outcomes) console.log(`  ${count} × ${line}`);
  console.log(`other failed requests: ${elsewhere.length === 0 ? "none" : elsewhere.join(" | ")}`);
  console.log(`page errors: ${pageErrors.length === 0 ? "none" : pageErrors.join(" | ")}`);
  console.log(`content security policy violations: ${violations.length === 0 ? "none" : violations.join(" | ")}`);
  console.log(`screenshots in ${shots}`);
  await browser.close();
  await site.close();
}
