/** 🎟️ A refused sign-up in the browser (work package "UI polish", item 5), against a real proctor whose sign-up
 * allowance is one registration at once and one more every 36 s:
 *   $env:PROCTOR_LIMIT_SIGNUPS_BURST = "1"; $env:PROCTOR_LIMIT_SIGNUPS_PER_HOUR = "100"; pwsh ui_polish_stack.ps1 start
 *   bun ui_polish_signup.ts [site origin]
 * One learner spends the allowance. A second, in German, is refused: the form says what happened and when to try
 * again, nothing keeps running, a second click sends no sign-up, and the pseudonym of the first learner is recalled
 * during the wait. A third, in English, waits what the form names and gets in. Exits 1 when a check fails. */
import { resolve } from "node:path";
import { chromium, type BrowserContext, type Page } from "playwright";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH;
const [origin = "http://127.0.0.1:6071"] = process.argv.slice(2);
const TEXT = {
  en: { pseudonym: "Pseudonym", field: "Your pseudonym", submit: "Continue", busy: "Server busy", wait: /try again in about (\d+) s/u },
  de: { pseudonym: "Pseudonym", field: "Dein Pseudonym", submit: "Weiter", busy: "Server ausgelastet", wait: /in etwa (\d+) s noch einmal/u },
} as const;
type Locale = keyof typeof TEXT;

let failed = 0;
function check(name: string, ok: boolean, seen: unknown): void {
  if (!ok) failed += 1;
  console.log(`[signup] ${ok ? "ok  " : "FAIL"} ${name}: ${typeof seen === "string" ? seen : JSON.stringify(seen)}`);
}

const browser = await chromium.launch();

async function learner(locale: Locale): Promise<{ readonly context: BrowserContext; readonly page: Page; readonly signUps: { status: number; retryAfter: string | null; body: string }[] }> {
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 }, locale: locale === "de" ? "de-DE" : "en-GB", baseURL: origin });
  await context.addInitScript((preferences) => localStorage.getItem("semio.quiz.architecture.preferences") === null && localStorage.setItem("semio.quiz.architecture.preferences", preferences), JSON.stringify({ locale, theme: "system", textSize: "normal", showCursors: true, showAnswers: true, animateIcons: false, pets: "off" }));
  const page = await context.newPage();
  const signUps: { status: number; retryAfter: string | null; body: string }[] = [];
  page.on("response", (response) => {
    const request = response.request();
    if (request.method() !== "POST" || !new URL(request.url()).pathname.endsWith("/commands")) return;
    void response.text().then((body) => signUps.push({ status: response.status(), retryAfter: response.headers()["retry-after"] ?? null, body: body.length > 200 ? `${body.slice(0, 200)}…` : body }));
  });
  await page.goto("/");
  await page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]').click();
  await page.locator('#quiz-main [data-card="identity"]').waitFor();
  return { context, page, signUps };
}

const form = (page: Page) => page.locator('#quiz-main form:has([data-card="identity"])');
const submit = (page: Page, locale: Locale) => page.getByRole("button", { name: TEXT[locale].submit, exact: true });

async function typed(page: Page, locale: Locale, handle: string): Promise<void> {
  await page.getByRole("radio", { name: TEXT[locale].pseudonym, exact: true }).check();
  await page.getByRole("textbox", { name: TEXT[locale].field }).fill(handle);
}

const handle = `Polish ${Math.random().toString(36).slice(2, 8)}`;

const first = await learner("en");
await typed(first.page, "en", handle);
await submit(first.page, "en").click();
await first.page.locator("[data-layered-overview]").waitFor();
await first.page.waitForTimeout(300);
check("the first learner signs up and spends the allowance", first.signUps.length === 1 && first.signUps[0]!.status === 200, first.signUps.map((answer) => answer.status));

const second = await learner("de");
await submit(second.page, "de").click();
const alert = form(second.page).getByRole("alert");
await alert.waitFor();
await second.page.waitForTimeout(300);
const said = (await alert.textContent()) ?? "";
check("the proctor refuses the second sign-up with 429 and names the allowance", second.signUps.length === 1 && second.signUps[0]!.status === 429 && second.signUps[0]!.body.includes('"allowance":"sign-up"'), second.signUps);
check("the form says in German what happened and when to try again", said.startsWith("Aus deinem Netzwerk haben sich gerade sehr viele neu angemeldet") && TEXT.de.wait.test(said) && said.includes("Pseudonym weiter, das du schon hast"), said);
check("nothing keeps running: the form is not busy, shows no progress, the button is ready", (await form(second.page).getAttribute("aria-busy")) === "false" && (await form(second.page).getByRole("status").count()) === 0 && (await submit(second.page, "de").isEnabled()), { busy: await form(second.page).getAttribute("aria-busy"), status: await form(second.page).getByRole("status").count() });
check("the header does not call the server busy", !((await second.page.locator("body").innerText()).includes(TEXT.de.busy)), TEXT.de.busy);
await second.page.waitForTimeout(2_500);
check("no sign-up is sent again by itself", second.signUps.length === 1, second.signUps.length);
await submit(second.page, "de").click();
await second.page.waitForTimeout(800);
check("a second click before the wait is over sends no sign-up and keeps the explanation", second.signUps.length === 1 && TEXT.de.wait.test((await alert.textContent()) ?? ""), { signUps: second.signUps.length, said: await alert.textContent() });
await typed(second.page, "de", handle);
await submit(second.page, "de").click();
await second.page.locator("[data-layered-overview]").waitFor();
check("the existing pseudonym is recalled during the wait, without a sign-up", second.signUps.length === 1, { signUps: second.signUps.length, handle });

const third = await learner("en");
await submit(third.page, "en").click();
const english = form(third.page).getByRole("alert");
await english.waitFor();
const told = (await english.textContent()) ?? "";
const seconds = Number(TEXT.en.wait.exec(told)?.[1] ?? Number.NaN);
check("the form says the same in English", told.startsWith("Too many new learners have signed up from your network just now") && seconds >= 1 && seconds <= 36, told);
await third.page.waitForTimeout((seconds + 1) * 1000);
await submit(third.page, "en").click();
await third.page.locator("[data-layered-overview]").waitFor({ timeout: 20_000 });
await third.page.waitForTimeout(300);
check(`after the ${seconds} s the form named, the next try gets in`, third.signUps.at(-1)?.status === 200, third.signUps.map((answer) => answer.status));

for (const { context } of [first, second, third]) await context.close();
await browser.close();
console.log(`[signup] ${failed === 0 ? "all checks passed" : `${failed} check(s) failed`}`);
process.exit(failed === 0 ? 0 : 1);
