/** 📸️ Screenshots and measurements of the quiz navbar in a real browser:
 * `bun navbar_shots.ts [site origin] [case prefix]` drives a fresh learner through the dev site at several widths and
 * text sizes, writes `🗑️generated/<case>-<screen>.png` and, into `🗑️generated/report.json`, per case and screen where
 * the ways, the brand and the right side of the navbar lie, what of the brand shows and whether anything overlaps or
 * leaves the navbar. A case the dev server reloads under (another edit to the sources) is walked again. */
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium, type Page } from "playwright";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH;
const origin = process.argv[2] ?? "http://localhost:6061";
const out = resolve(import.meta.dir, "🗑️generated");
mkdirSync(out, { recursive: true });

interface Case {
  readonly id: string;
  readonly width: number;
  readonly height: number;
  readonly touch: boolean;
  readonly locale: "en" | "de";
  readonly textSize: "normal" | "large" | "larger" | "largest";
  readonly theme: "light" | "dark";
}

const CASES: readonly Case[] = [
  { id: "desktop-en", width: 1440, height: 900, touch: false, locale: "en", textSize: "normal", theme: "light" },
  { id: "desktop-de-dark", width: 1280, height: 720, touch: false, locale: "de", textSize: "normal", theme: "dark" },
  { id: "tablet-de", width: 820, height: 1180, touch: true, locale: "de", textSize: "normal", theme: "light" },
  { id: "phone-en", width: 375, height: 812, touch: true, locale: "en", textSize: "normal", theme: "light" },
  { id: "phone-de-larger", width: 375, height: 812, touch: true, locale: "de", textSize: "larger", theme: "light" },
  { id: "phone-de-largest", width: 375, height: 812, touch: true, locale: "de", textSize: "largest", theme: "light" },
  { id: "small-phone-de-largest", width: 360, height: 740, touch: true, locale: "de", textSize: "largest", theme: "dark" },
];

async function measure(page: Page) {
  return page.evaluate(() => {
    const nav = document.querySelector<HTMLElement>("header nav")!;
    const box = (element: Element | null) => {
      if (element === null) return null;
      const rect = element.getBoundingClientRect();
      return { left: Math.round(rect.left * 10) / 10, right: Math.round(rect.right * 10) / 10, top: Math.round(rect.top * 10) / 10, bottom: Math.round(rect.bottom * 10) / 10 };
    };
    const brand = nav.querySelector<HTMLElement>("[data-quiz-brand]")!;
    const frame = brand.getBoundingClientRect();
    const shows = (element: Element | null) => {
      if (element === null) return "absent";
      const rect = element.getBoundingClientRect();
      return rect.top >= frame.bottom - 1 || rect.width === 0 ? "wrapped away" : rect.right > frame.right + 0.5 ? "cut" : `${Math.round(rect.width)} px`;
    };
    const title = brand.querySelector<HTMLElement>(".quiz-brand-title");
    const controls = [...nav.querySelectorAll<HTMLElement>("[data-quiz-nav], .quiz-connection, [data-presence-status], [role=group] button")];
    const visibleBrand = [brand.querySelector(".quiz-brand-logo"), title].filter((part): part is Element => part !== null && shows(part) !== "wrapped away").map((part) => part.getBoundingClientRect());
    const overlaps = controls.filter((control) => {
      const rect = control.getBoundingClientRect();
      return visibleBrand.some((part) => Math.min(part.right, frame.right) > rect.left + 0.5 && part.left < rect.right - 0.5);
    }).length;
    return {
      viewport: window.innerWidth,
      ways: [...nav.querySelectorAll<HTMLElement>("[data-quiz-nav]")].map((way) => `${way.dataset.quizNav}${way.getAttribute("aria-disabled") === "true" ? " (off)" : ""} ${Math.round(way.getBoundingClientRect().left)}–${Math.round(way.getBoundingClientRect().right)} "${way.getAttribute("aria-label")}"`),
      brand: box(brand),
      brandCentre: Math.round((frame.left + frame.right) / 2),
      navCentre: Math.round(nav.getBoundingClientRect().width / 2),
      logo: shows(brand.querySelector(".quiz-brand-logo")),
      title: shows(title),
      titleCut: title === null ? null : title.scrollWidth > title.clientWidth,
      connection: box(nav.querySelector(".quiz-connection")),
      presence: box(nav.querySelector("[data-presence-status]")),
      language: box(nav.querySelector("[role=group]:last-of-type")),
      languageWords: [...nav.querySelectorAll<HTMLElement>("[role=group]:not(:has([data-quiz-nav])) button")].map((button) => button.innerText.trim()),
      overlaps,
      pageWiderBy: document.documentElement.scrollWidth - window.innerWidth,
      navWiderBy: nav.scrollWidth - nav.clientWidth,
      hash: window.location.hash,
    };
  });
}

const browser = await chromium.launch();
const report: Record<string, unknown> = {};
const only = process.argv[3];

/** 🔁️ One case, start to end; the dev server reloads the page whenever a source file changes, which ends the walk. */
async function walk(one: Case): Promise<void> {
  const context = await browser.newContext({ viewport: { width: one.width, height: one.height }, deviceScaleFactor: 2, isMobile: one.touch, hasTouch: one.touch, locale: one.locale === "de" ? "de-DE" : "en-GB", baseURL: origin });
  await context.addInitScript((preferences) => localStorage.getItem("semio.quiz.architecture.preferences") === null && localStorage.setItem("semio.quiz.architecture.preferences", preferences), JSON.stringify({ locale: one.locale, theme: one.theme, textSize: one.textSize, showCursors: true, showAnswers: true, animateIcons: false, moveBackground: false, pets: "off", petsLiveliness: "calm" }));
  try {
  const page = await context.newPage();
  const problems: string[] = [];
  page.on("console", (message) => message.type() === "error" && problems.push(message.text()));
  page.on("pageerror", (error) => problems.push(String(error)));
  const front = (card: string) => page.locator(`#quiz-main [data-card="${card}"]`).and(page.locator(":not([inert] *)"));
  const way = (name: string) => page.locator(`header [data-quiz-nav="${name}"]`);
  const seen = async (screen: string) => {
    await page.waitForTimeout(700);
    await page.screenshot({ path: resolve(out, `${one.id}-${screen}.png`) });
    await page.locator("header").screenshot({ path: resolve(out, `${one.id}-${screen}-navbar.png`) });
    report[`${one.id} · ${screen}`] = await measure(page);
  };
  await page.goto("/");
  await front("introduction").waitFor();
  await seen("first-visit");
  await front("introduction").locator('[data-overview-card-action="primary"]').click();
  await front("identity").locator('[data-overview-card-action="primary"]').click();
  await page.locator("[data-layered-overview]").waitFor();
  await seen("overview");
  await page.evaluate(() => (window.location.hash = "board"));
  await page.locator('[data-layered-pane="board"][data-opened]').waitFor();
  await seen("leaderboard");
  await way("back").click();
  await page.locator("[data-layered-pane][data-opened]").waitFor({ state: "detached" });
  await page.locator('[data-layered-card="heating"] [data-overview-card-action="primary"]').click();
  await front("run").waitFor();
  await seen("run");
  await way("up").click();
  await page.locator('[data-layered-pane="heating"][data-opened]').waitFor();
  await seen("quiz-page");
  await way("back").click();
  await front("run").waitFor();
  await way("overview").click();
  await page.locator("[data-layered-overview]").waitFor();
  await seen("overview-again");
  report[`${one.id} · problems`] = problems;
  } finally {
    await context.close();
  }
}

for (const one of CASES.filter((candidate) => only === undefined || candidate.id.startsWith(only))) {
  for (let attempt = 1; ; attempt += 1) {
    try {
      await walk(one);
      break;
    } catch (error) {
      console.error(`[DEBUG] ${one.id}, attempt ${attempt}: ${String(error).split("\n")[0]}`);
      if (attempt === 4) throw error;
    }
  }
  writeFileSync(resolve(out, "report.json"), JSON.stringify(report, null, 2));
}
await browser.close();
