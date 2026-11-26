import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { readFile } from "node:fs/promises";
import { chromium } from "playwright";
const browser = await chromium.launch({ channel: "chrome", headless: true });
const out = new URL("./🗑️generated/web2/", import.meta.url);
const cases = JSON.parse(await readFile("🎓️teaching/🏛️architecture/📚️course/🔣️.json", "utf8"));
const quizIds = cases.topics.flatMap((topic) => topic.quizzes);
const names = new Set(["physics", "bim-concepts", "excel-workbook", "grasshopper-data"]);
const transparent = "rgba(0, 0, 0, 0)";
const rootOf = (page) => page.locator(".course-app").first();
const errors = (page) => page.on("pageerror", (error) => console.error("[DEBUG]", error.message));
const startOptions = { locale: "en-US", reducedMotion: "reduce" };

const appearance = (element) => {
  const style = getComputedStyle(element);
  return { fill: style.backgroundColor, image: style.backgroundImage, blur: style.backdropFilter, level: element.closest("[data-level]")?.getAttribute("data-level") };
};

try {
  if (!process.argv.includes("--course-only")) for (const colorScheme of ["light", "dark"]) {
    const page = await browser.newPage({ ...startOptions, colorScheme, viewport: { width: 390, height: 844 } });
    errors(page);
    await page.goto("http://127.0.0.1:6173/", { waitUntil: "domcontentloaded" });
    const staffage = page.locator(".staffage-app").first();
    await staffage.waitFor({ timeout: 45000 });
    assert.equal((await staffage.evaluate(appearance)).level, "base");
    await page.getByRole("button", { name: "Display settings" }).click();
    const menu = page.locator("#staffage-settings");
    const menuPaint = await menu.evaluate(appearance);
    assert.equal(menuPaint.level, "menu");
    assert.notEqual(menuPaint.fill, transparent);
    assert.notEqual(menuPaint.blur, "none");
    const options = await menu.locator('[role="radio"][aria-checked="false"]').evaluateAll((nodes) => nodes.map((node) => getComputedStyle(node).backgroundColor));
    assert.ok(options.length >= 3 && options.every((fill) => fill === transparent));
    await page.screenshot({ path: fileURLToPath(new URL(`./${colorScheme}-staffage.png`, out)) });
    console.log("[DEBUG] staffage", colorScheme, JSON.stringify(menuPaint));
    await page.close();
  }
  if (process.argv.includes("--staffage-only")) process.exitCode = 0;
  else for (const colorScheme of ["light", "dark"]) {
    const page = await browser.newPage({ ...startOptions, colorScheme, viewport: { width: 1420, height: 960 } });
    errors(page);
    await page.addInitScript(({ colorScheme, challenges }) => {
      localStorage.setItem("semio.course.architecture.preferences", JSON.stringify({ locale: "en", theme: colorScheme, challenges, pets: "off", petsChosen: true }));
      localStorage.setItem("semio.course.architecture.introduced", "true");
    }, { colorScheme, challenges: Object.fromEntries(quizIds.map((quiz) => [quiz, quiz === "physics" ? "hard" : "medium"])) });
    await page.goto("http://127.0.0.1:6172/?preview=1#physics", { waitUntil: "domcontentloaded" });
    await page.locator('[data-page="quiz:physics"]').waitFor({ timeout: 45000 });
    const app = await rootOf(page).evaluate(appearance);
    assert.equal(app.level, "base");
    assert.notEqual(app.fill, transparent);
    for (const quiz of quizIds) {
      await page.evaluate((quiz) => { window.location.hash = quiz; }, quiz);
      const frame = page.locator(`[data-opened] [data-page="quiz:${quiz}"]`).first();
      await frame.waitFor();
      const structure = await frame.evaluate((frame) => {
        const cards = Array.from(frame.querySelectorAll("[data-overview-card]"));
        const flags = (element) => {
          const style = getComputedStyle(element);
          return { fill: style.backgroundColor, level: element.closest("[data-level]")?.getAttribute("data-level"), blur: style.backdropFilter };
        };
        return {
          frame: flags(frame),
          bodies: cards.map((card) => flags(card.querySelector("[data-window-frame-content]"))),
          chips: cards.flatMap((card) => Array.from(card.querySelectorAll("[data-window-frame-chip]"), flags)),
          gaps: cards.slice(1).map((card, i) => card.getBoundingClientRect().top - cards[i].getBoundingClientRect().bottom),
        };
      });
      assert.equal(structure.frame.fill, transparent, quiz);
      assert.equal(structure.frame.level, "base", quiz);
      assert.ok(structure.bodies.length >= 3, quiz);
      assert.ok(structure.bodies.every((body) => body.fill === transparent && body.level === "base"), JSON.stringify({ quiz, structure }));
      assert.ok(structure.chips.every((chip) => chip.level === "window" && chip.fill !== transparent && chip.blur !== "none"), JSON.stringify({ quiz, structure }));
      assert.ok(structure.gaps.every((gap) => Math.abs(gap) <= 1), JSON.stringify({ quiz, structure }));
      if (names.has(quiz)) await page.screenshot({ path: fileURLToPath(new URL(`./${colorScheme}-${quiz}.png`, out)) });
      console.log("[DEBUG] quiz", colorScheme, quiz, JSON.stringify({ base: app.fill, chip: structure.chips[0].fill, gaps: structure.gaps }));
    }
    await page.evaluate(() => { window.location.hash = "learner"; });
    const learner = page.locator('[data-opened] [data-page="learner"] [data-card="learner"] [data-overview-card-action="primary"]').first();
    await learner.click();
    const identity = page.getByRole("dialog").filter({ has: page.locator('input[value="anonymous"]') });
    const identityPaint = await identity.evaluate(appearance);
    assert.equal(identityPaint.level, "dialog");
    assert.notEqual(identityPaint.fill, transparent);
    assert.notEqual(identityPaint.blur, "none");
    await page.screenshot({ path: fileURLToPath(new URL(`./${colorScheme}-identity.png`, out)) });
    await identity.locator('input[value="anonymous"]').locator("xpath=ancestor::label[1]").click();
    await identity.locator('[data-overview-card-action="primary"]').click();
    await page.waitForFunction(() => JSON.parse(localStorage.getItem("semio.course.architecture.learner") ?? "null")?.id);
    const dialog = page.locator('[role="dialog"][data-level="dialog"]');
    if (await dialog.isVisible()) await page.keyboard.press("Escape");
    const taskKinds = new Set();
    const previewKinds = new Set();
    for (const quiz of ["physics", "demand", "excel-workbook", "grasshopper-data"]) {
      await page.evaluate((quiz) => { window.location.hash = quiz; }, quiz);
      const primary = page.locator(`[data-opened] [data-page="quiz:${quiz}"] [data-card="quiz-challenge"] [data-overview-card-action="primary"]`).first();
      await primary.click();
      const run = page.locator('[data-card="run"]');
      await run.waitFor({ timeout: 12000 });
      for (let index = 0; index < await run.locator("nav button").count(); index++) {
        await run.locator("nav button").nth(index).click();
        const rows = await page.locator(".course-sort, .course-chip, .course-value, .course-cell, .course-input").evaluateAll((elements) => elements.map((element) => {
          const kind = element.closest(".course-sort") ? "sorting" : element.closest(".course-chip") ? "classification" : "matching";
          return { kind, fill: getComputedStyle(element).backgroundColor };
        }));
        assert.ok(rows.length && rows.every(({ fill }) => fill === transparent), JSON.stringify(rows));
        for (const row of rows) taskKinds.add(row.kind);
        const previews = await page.locator('figure.course-mockup').evaluateAll((figures) => figures.map((figure) => ({
          kind: figure.dataset.mockup,
          surface: getComputedStyle(figure).backgroundColor,
          level: figure.closest("[data-level]")?.getAttribute("data-level"),
          chips: Array.from(figure.querySelectorAll("[data-glass-chip]"), (chip) => ({ level: chip.dataset.level, blur: getComputedStyle(chip).backdropFilter })),
        })));
        for (const preview of previews) {
          assert.equal(preview.surface, transparent);
          assert.equal(preview.level, "base");
          assert.ok(preview.chips.every((chip) => chip.level === "window" && chip.blur !== "none"));
          previewKinds.add(preview.kind);
        }
        if (quiz === "physics" || previews.length > 0 && index === 0) {
          await page.screenshot({ path: fileURLToPath(new URL(`./${colorScheme}-${quiz}-run-${index}.png`, out)) });
        }
        console.log("[DEBUG] task", colorScheme, quiz, index, rows.length, "transparent rows and fields", [...new Set(previews.map((item) => item.kind))]);
      }
    }
    assert.deepEqual([...taskKinds].sort(), ["classification", "matching", "sorting"]);
    assert.deepEqual([...previewKinds].sort(), ["excel", "grasshopper"]);
    await page.close();
  }
} finally {
  await browser.close();
}
