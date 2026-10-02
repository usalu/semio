/** 🧭️ Drives the staged site folder — the deliverable itself, served as static files the way a CDN serves them — in
 * Chromium while its baked proctor does not answer: a pseudonym, the physics quiz with every sorting task answered by
 * typed numeric guesses only (the true values, so the items must order themselves into the true order), every other
 * task from the catalog's solution, the submission, the results with their "Your guess" column, and a reload.
 *
 *   bun drive_staged_site.ts <site directory> [port] [screenshot directory]
 */
import { mkdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium, type BrowserContext, type Page } from "playwright";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";
import { answerTask, ascending, card, connection, enter, expectFeedback, goHome, openTask, percent, playQuiz, quizOf, screen, shownItems, shownName, shownTask, submitRun, type Device } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🚶️learner/🟦️.ts";
import { serveStaticSite } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🧱️stack/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, "../../../../../../..");
const directory = resolve(process.argv[2] ?? "");
const port = Number(process.argv[3] ?? "6191");
const shots = resolve(process.argv[4] ?? join(here, "🗑️generated", "staged-site"));
mkdirSync(shots, { recursive: true });

const proctor = /connect-src (\S+)/u.exec(readFileSync(join(directory, "index.html"), "utf8"))?.[1] ?? "";
console.log(`site folder ${directory}, baked proctor origin ${proctor}`);

process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH;
const site = await serveStaticSite(directory, port);
const browser = await chromium.launch();
const context: BrowserContext = await browser.newContext({ baseURL: site.origin, viewport: { width: 1440, height: 900 }, locale: "en-GB" });
const page: Page = await context.newPage();
const pageErrors: string[] = [];
const violations: string[] = [];
const elsewhere: string[] = [];
await context.exposeBinding("quizSecurityViolation", (_source, violation: string) => void violations.push(violation));
await context.addInitScript(() => document.addEventListener("securitypolicyviolation", (event) => (window as unknown as { quizSecurityViolation(violation: string): void }).quizSecurityViolation(`${event.effectiveDirective} blocked ${event.blockedURI || "inline code"}`)));
page.on("pageerror", (error) => pageErrors.push(error.message));
page.on("response", (response) => {
  if (!response.url().startsWith(proctor) && response.status() >= 400) elsewhere.push(`${response.request().method()} ${response.url()} answered ${response.status()}`);
});
const device: Device = { context, page, locale: "en", problems: [], expectingFailures: async (body) => (await body(), 0) };
let failed = false;

try {
  const quiz = quizOf("physics");
  const name = `Staged Proof ${Math.random().toString(36).slice(2, 8)}`;
  const started = Date.now();
  await enter(device, { kind: "pseudonym", handle: name });
  console.log(`overview after ${Date.now() - started} ms as ${await shownName(device)}`);
  await playQuiz(device, quiz.id);
  const steps = screen(page, "run").locator("nav button");
  for (let index = 0; index < quiz.tasks.length; index++) {
    await openTask(device, index);
    const shown = await shownTask(device);
    const source = quiz.tasks.find((candidate) => candidate.id === shown)!;
    if (source.kind !== "sorting") {
      await answerTask(device, quiz, "perfect");
      continue;
    }
    const items = await shownItems(device);
    const before = [...items];
    const area = screen(page, "task");
    for (const item of items) {
      const value = source.items.find((candidate) => candidate.id === item)!.value!;
      const field = area.locator(`[data-quiz-item="${item}"] input`);
      await field.fill(`${value.toExponential()} ${source.quantity!.unit}`);
      await field.press("Enter");
      if ((await field.getAttribute("aria-invalid")) === "true") throw new Error(`the guess ${value.toExponential()} ${source.quantity!.unit} for ${item} was not read`);
    }
    const after = await shownItems(device);
    const truth = ascending(source, items);
    const shownGuesses = await area.locator("[data-quiz-item] input").evaluateAll((fields) => fields.map((field) => (field as HTMLInputElement).value));
    console.log(`sorting ${shown}: ${items.length} guesses typed; order before ${before.join(" ")}`);
    console.log(`  order after  ${after.join(" ")}`);
    console.log(`  true order   ${truth.join(" ")} → ${JSON.stringify(after) === JSON.stringify(truth) ? "the guesses ordered the items" : "NOT ordered by the guesses"}`);
    console.log(`  the fields show ${shownGuesses.join(" | ")}`);
    if (JSON.stringify(after) !== JSON.stringify(truth)) failed = true;
    await steps.nth(index).locator("[data-complete]").waitFor();
    await page.screenshot({ path: join(shots, `sorting-${shown}.png`), fullPage: true });
  }
  await submitRun(device);
  const results = await expectFeedback(device, quiz, "perfect");
  const guessColumns = await page.getByRole("columnheader", { name: "Your guess", exact: true }).count();
  const guessCells = await page.locator('table:not(.quiz-plot):has(th:text-is("Your guess")) tbody tr').evaluateAll((rows) => rows.map((row) => [...row.querySelectorAll("th, td")].slice(0, 4).map((cell) => (cell.textContent ?? "").trim()).join(" · ")));
  console.log(`results: ${results.score} % in ${quiz.id}; "Your guess" column headings: ${guessColumns}`);
  for (const row of guessCells.slice(0, 4)) console.log(`  ${row}`);
  if (results.score !== 100 || guessColumns === 0) failed = true;
  await page.screenshot({ path: join(shots, "results.png"), fullPage: true });
  await goHome(device);
  await page.reload();
  await page.locator("[data-layered-overview]").waitFor();
  console.log(`after a reload: ${await shownName(device)}, best ${percent(await card(page, quiz.id).locator("li", { hasText: "%" }).innerText())} % in ${quiz.id}`);
  await page.waitForTimeout(5_000);
  console.log(`connection: ${(await connection(device).innerText()).replace(/\s+/gu, " ").trim()}`);
} catch (error) {
  failed = true;
  console.log(`FAILED: ${error instanceof Error ? error.stack : String(error)}`);
  await page.screenshot({ path: join(shots, "failure.png"), fullPage: true }).catch(() => undefined);
} finally {
  console.log(`other failed requests: ${elsewhere.length === 0 ? "none" : elsewhere.join(" | ")}`);
  console.log(`page errors: ${pageErrors.length === 0 ? "none" : pageErrors.join(" | ")}`);
  console.log(`content security policy violations: ${violations.length === 0 ? "none" : violations.join(" | ")}`);
  await browser.close();
  await site.close();
}
if (failed || pageErrors.length > 0 || violations.length > 0) process.exit(1);
