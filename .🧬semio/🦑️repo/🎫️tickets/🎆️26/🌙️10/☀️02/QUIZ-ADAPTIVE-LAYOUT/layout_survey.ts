/** 📐️ A survey of the quiz client's layout in a real browser:
 * `bun layout_survey.ts [site origin] [case prefix] [label]` drives a fresh learner through every screen of the dev
 * site at phone, tablet and desktop widths — first visit, identity, overview, every kind of task open and answered,
 * results, and every page of home — and writes `🗑️generated/<label>/<case>-<screen>.png` (the screen grown to its whole
 * scrolling height) and `🗑️generated/<label>/report.json`: per case and screen whether anything is wider than the
 * page, which elements scroll sideways, which text is cut, and how every task row is laid out (one line or two). A case
 * the dev server reloads under (another edit to the sources) is walked again. */
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium, type Locator, type Page } from "playwright";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH;
const origin = process.argv[2] ?? "http://localhost:6061";
const only = process.argv[3] === undefined || process.argv[3] === "all" ? undefined : process.argv[3];
const label = process.argv[4] ?? "survey";
const screens = process.argv[5] === undefined ? undefined : process.argv[5].split(",");
const out = resolve(import.meta.dir, "🗑️generated", label);
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
  { id: "phone-320", width: 320, height: 640, touch: true, locale: "de", textSize: "normal", theme: "dark" },
  { id: "phone-375", width: 375, height: 812, touch: true, locale: "de", textSize: "normal", theme: "dark" },
  { id: "phone-430", width: 430, height: 932, touch: true, locale: "en", textSize: "normal", theme: "light" },
  { id: "phone-wide-600", width: 600, height: 960, touch: true, locale: "de", textSize: "normal", theme: "dark" },
  { id: "phone-landscape-812", width: 812, height: 375, touch: true, locale: "de", textSize: "normal", theme: "dark" },
  { id: "tablet-768", width: 768, height: 1024, touch: true, locale: "de", textSize: "normal", theme: "dark" },
  { id: "tablet-1024", width: 1024, height: 768, touch: true, locale: "en", textSize: "normal", theme: "light" },
  { id: "desktop-1280", width: 1280, height: 800, touch: false, locale: "de", textSize: "normal", theme: "dark" },
  { id: "desktop-1920", width: 1920, height: 1080, touch: false, locale: "de", textSize: "normal", theme: "dark" },
  { id: "phone-375-largest", width: 375, height: 812, touch: true, locale: "de", textSize: "largest", theme: "dark" },
  { id: "desktop-1280-largest", width: 1280, height: 800, touch: false, locale: "de", textSize: "largest", theme: "light" },
];

const QUIZZES = ["physics", "demand", "heating"] as const;
const PAGES = ["physics", "board", "learner", "badges", "prefs", "intro"] as const;

/** 🔎️ What the layout of the screen in front says about itself. */
async function measure(page: Page) {
  return page.evaluate(() => {
    const main = document.querySelector<HTMLElement>("#quiz-main")!;
    const clipped = new Set([...main.querySelectorAll<HTMLElement>(".sr-only, thead")].filter((element) => getComputedStyle(element).clipPath !== "none" || getComputedStyle(element).clip !== "auto"));
    const shown = (element: Element): boolean => {
      const rect = element.getBoundingClientRect();
      return rect.width > 0 && rect.height > 0 && element.closest("[inert]") === null && ![...clipped].some((hidden) => hidden.contains(element));
    };
    const name = (element: Element): string => {
      const card = element.closest("[data-card]")?.getAttribute("data-card") ?? "";
      const own = element.getAttribute("data-quiz-item") ?? element.getAttribute("data-card") ?? element.className.toString().split(" ").slice(0, 3).join(".");
      return `${card} › ${element.tagName.toLowerCase()}.${own}`;
    };
    const everything = [...main.querySelectorAll<HTMLElement>("*")].filter(shown);
    const sideways = everything.filter((element) => element.scrollWidth > element.clientWidth + 1 && ["auto", "scroll"].includes(getComputedStyle(element).overflowX)).map((element) => `${name(element)} +${element.scrollWidth - element.clientWidth}`);
    const cut = everything.filter((element) => element.scrollWidth > element.clientWidth + 1 && getComputedStyle(element).textOverflow === "ellipsis").map(name);
    const view = document.documentElement.clientWidth;
    const outside = everything.filter((element) => {
      const rect = element.getBoundingClientRect();
      return (rect.right > view + 1 || rect.left < -1) && element.closest(".overflow-x-auto, [data-layered-strip], [data-layered-pane]:not([data-opened])") === null;
    });
    const rows = [...main.querySelectorAll<HTMLElement>('[data-card="task"] [data-quiz-item], [data-card="task"] [data-quiz-drop^="slot:"]')].filter(shown).slice(0, 3);
    const lines = rows.map((row) => {
      const parts = [...row.querySelectorAll<HTMLElement>("select, input, button, [data-quiz-grip], .quiz-row-label")].filter(shown).map((part) => Math.round(part.getBoundingClientRect().top + part.getBoundingClientRect().height / 2));
      const bands = [...new Set(parts.map((centre) => Math.round(centre / 12)))].length;
      return `${Math.round(row.getBoundingClientRect().width)}×${Math.round(row.getBoundingClientRect().height)} in ${bands} band(s)`;
    });
    const smallTargets = everything
      .filter((element) => element.matches("button, select, input, a[href], summary, [data-quiz-grip]"))
      .filter((element) => {
        const rect = element.getBoundingClientRect();
        return rect.height < 23.5 || rect.width < 23.5;
      })
      .map((element) => `${name(element)} ${Math.round(element.getBoundingClientRect().width)}×${Math.round(element.getBoundingClientRect().height)}`);
    return {
      viewport: `${window.innerWidth}×${window.innerHeight}`,
      pageWiderBy: document.documentElement.scrollWidth - window.innerWidth,
      outside: [...new Set(outside.map(name))].slice(0, 12),
      sideways: [...new Set(sideways)].slice(0, 12),
      cut: [...new Set(cut)].slice(0, 12),
      rows: lines,
      smallTargets: [...new Set(smallTargets)].slice(0, 12),
      cards: [...main.querySelectorAll<HTMLElement>("[data-card]")].filter(shown).map((card) => `${card.dataset.card} ${Math.round(card.getBoundingClientRect().width)}×${Math.round(card.getBoundingClientRect().height)}`),
    };
  });
}

const browser = await chromium.launch();
const report: Record<string, unknown> = {};

/** 🔁️ One case, start to end; the dev server reloads the page whenever a source file changes, which ends the walk. */
async function walk(one: Case): Promise<void> {
  const context = await browser.newContext({ viewport: { width: one.width, height: one.height }, deviceScaleFactor: 1, isMobile: one.touch, hasTouch: one.touch, locale: one.locale === "de" ? "de-DE" : "en-GB", baseURL: origin });
  await context.addInitScript(
    (preferences) => localStorage.getItem("semio.quiz.architecture.preferences") === null && localStorage.setItem("semio.quiz.architecture.preferences", preferences),
    JSON.stringify({ locale: one.locale, theme: one.theme, textSize: one.textSize, showCursors: true, others: "submitted", animateIcons: false, iconsChosen: true, pets: "off", petsLiveliness: "calm", petsChosen: true }),
  );
  try {
    const page = await context.newPage();
    const problems: string[] = [];
    page.on("console", (message) => message.type() === "error" && problems.push(message.text()));
    page.on("pageerror", (error) => problems.push(String(error)));
    const front = (card: string): Locator => page.locator(`#quiz-main [data-card="${card}"]`).and(page.locator(":not([inert] *)"));
    const way = (name: string): Locator => page.locator(`header [data-quiz-nav="${name}"]`);
    const wanted = (screen: string): boolean => screens === undefined || screens.some((prefix) => screen.startsWith(prefix));
    const seen = async (screen: string): Promise<void> => {
      if (!wanted(screen)) return;
      await page.waitForTimeout(350);
      report[`${one.id} · ${screen}`] = await measure(page);
      const excess = await page.evaluate(() => Math.max(0, ...[...document.querySelectorAll<HTMLElement>("#quiz-main *")].filter((element) => element.closest("[inert]") === null && ["auto", "scroll"].includes(getComputedStyle(element).overflowY) && element.clientHeight > 200).map((element) => element.scrollHeight - element.clientHeight)));
      const tall = Math.min(9000, Math.floor(8_000_000 / one.width), one.height + excess + 8);
      if (excess > 0) await page.setViewportSize({ width: one.width, height: tall });
      await page.waitForTimeout(150);
      const sheet = Math.max(1500, one.width * 2);
      const height = excess > 0 ? tall : one.height;
      for (let top = 0, part = 0; top < height; top += sheet, part += 1) await page.screenshot({ path: resolve(out, `${one.id}-${screen}${height > sheet ? `-${"abcdefgh"[part]}` : ""}.png`), clip: { x: 0, y: top, width: one.width, height: Math.min(sheet, height - top) } });
      if (excess > 0) await page.setViewportSize({ width: one.width, height: one.height });
    };
    const overview = async (): Promise<void> => {
      await way("overview").click();
      await page.locator("[data-layered-overview]").waitFor();
      await page.locator("[data-layered-pane][data-opened]").waitFor({ state: "detached" });
    };

    await page.goto("/");
    await front("introduction").waitFor();
    await seen("01-first-visit");
    await front("introduction").locator('[data-overview-card-action="primary"]').click();
    await front("identity").waitFor();
    await front("identity").locator('input[type="radio"][value="pseudonym"]').check();
    await seen("02-identity");
    await front("identity").locator('input[type="radio"][value="anonymous"]').check();
    await front("identity").locator('[data-overview-card-action="primary"]').click();
    await page.locator("[data-layered-overview]").waitFor();
    await seen("03-overview");

    for (const quiz of QUIZZES) {
      if (!wanted(`run-${quiz}`) && !wanted(`results-${quiz}`) && !PAGES.some((id) => wanted(`page-${id}`))) continue;
      await page.locator(`[data-layered-card="${quiz}"] [data-overview-card-action="primary"]`).click();
      await front("run").waitFor();
      const steps = front("run").locator("nav button");
      const amount = await steps.count();
      for (let index = 0; index < amount; index++) {
        await steps.nth(index).click();
        await page.waitForTimeout(100);
        const task = front("task");
        await seen(`run-${quiz}-${index + 1}-open`);
        const selects = task.locator("select");
        const kind = (await task.locator("ol > [data-quiz-item]").count()) > 0 ? "sorting" : (await task.locator('[data-quiz-drop^="slot:"]').count()) > 0 ? "matching" : "classification";
        if (kind === "sorting") {
          const first = task.locator("ol > [data-quiz-item]").first();
          await first.locator("input").fill(one.locale === "de" ? "2,5 k" : "2.5 k");
          await seen(`run-${quiz}-${index + 1}-typing`);
          await first.locator("input").press("Enter");
          const rows = task.locator("ol > [data-quiz-item]");
          for (let place = 1; place < (await rows.count()); place++) {
            await rows.nth(place).locator("input").fill(`${place + 2} k`);
            await rows.nth(place).locator("input").press("Enter");
          }
        } else if (kind === "matching") {
          for (let row = 0; row < (await selects.count()); row++) {
            const value = await selects.nth(row).evaluate((select) => [...(select as HTMLSelectElement).options].find((option) => option.value !== "" && !option.disabled)?.value ?? "");
            await selects.nth(row).selectOption(value);
          }
        } else {
          const items = await task.locator("[data-quiz-item]").evaluateAll((elements) => elements.map((element) => (element as HTMLElement).dataset.quizItem ?? ""));
          for (const [place, item] of items.entries()) {
            const select = task.locator(`[data-quiz-item="${item}"] select`);
            const values = await select.evaluate((element) => [...(element as HTMLSelectElement).options].map((option) => option.value).filter((value) => value !== ""));
            if (place === 0) continue;
            await select.selectOption(values[place % values.length]!);
          }
          await seen(`run-${quiz}-${index + 1}-partly`);
          const select = task.locator(`[data-quiz-item="${items[0]}"] select`);
          await select.selectOption({ index: 1 });
        }
        await seen(`run-${quiz}-${index + 1}-answered`);
      }
      await front("run").locator('[data-overview-card-action="primary"]').click();
      await page.getByRole("alertdialog").waitFor();
      await seen(`run-${quiz}-confirm`);
      await page.getByRole("alertdialog").locator('[data-overview-card-action="primary"]').click();
      await front("results").waitFor();
      await page.waitForTimeout(600);
      await seen(`results-${quiz}`);
      await overview();
    }
    await seen("04-overview-played");
    for (const id of PAGES) {
      if (!wanted(`page-${id}`)) continue;
      await page.evaluate((hash) => (window.location.hash = hash), id);
      await page.locator(`[data-layered-pane="${id}"][data-opened]`).waitFor();
      await seen(`page-${id}`);
      await overview();
    }
    report[`${one.id} · problems`] = problems;
  } finally {
    await context.close();
  }
}

for (const one of CASES.filter((candidate) => only === undefined || only.split(",").some((prefix) => candidate.id.startsWith(prefix)))) {
  for (let attempt = 1; ; attempt += 1) {
    try {
      await walk(one);
      console.log(`walked ${one.id}`);
      break;
    } catch (error) {
      console.error(`[DEBUG] ${one.id}, attempt ${attempt}: ${String(error).split("\n").slice(0, 3).join(" | ")}`);
      if (attempt === 3) {
        report[`${one.id} · failed`] = String(error).split("\n").slice(0, 6);
        break;
      }
    }
  }
  writeFileSync(resolve(out, "report.json"), JSON.stringify(report, null, 2));
}
await browser.close();
