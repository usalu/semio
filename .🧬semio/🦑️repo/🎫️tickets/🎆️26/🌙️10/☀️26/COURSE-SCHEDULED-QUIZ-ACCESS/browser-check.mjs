import fs from "node:fs";
import path from "node:path";
import { chromium, expect } from "@playwright/test";

const base = "http://127.0.0.1:6067/";
const generated = path.resolve(import.meta.dirname, "🗑️generated");
fs.mkdirSync(generated, { recursive: true });
const course = JSON.parse(fs.readFileSync("🎓️teaching/🏛️architecture/📚️course/🔣️.json", "utf8"));
const browser = await chromium.launch({ headless: true, channel: "msedge" });
const results = [];
const card = (page, id) => page.locator(`[data-course-grid][data-grid-active] > [data-layered-overview] [data-layered-card="${id}"]`).and(page.locator(":not([inert] *)"));
const pane = (page, id) => page.locator(`[data-layered-pane="${id}"]`);
async function setup(unlocks, viewport = { width: 1440, height: 900 }, instant) {
  console.log(`[DEBUG] Browser setup: ${viewport.width}px, scheduled=${Boolean(unlocks)}, clock=${instant ?? "real"}`);
  const context = await browser.newContext({ viewport, locale: "en", reducedMotion: "reduce" });
  const page = await context.newPage();
  if (instant !== undefined) await page.clock.install({ time: new Date(instant) });
  const errors = [];
  page.on("pageerror", error => errors.push(String(error)));
  if (unlocks) await page.route(url => decodeURIComponent(url.pathname) === "/🔣️.json", route => route.fulfill({ contentType: "text/javascript", body: `export default ${JSON.stringify({ ...course, unlocks })};` }));
  await page.goto(base);
  try { await page.getByRole("button", { name: "Skip", exact: true }).click(); }
  catch (error) {
    console.log("[DEBUG] Page errors: " + JSON.stringify(errors));
    console.log("[DEBUG] Page text: " + (await page.locator("body").innerText()).slice(0,7000));
    await page.screenshot({ path: path.join(generated, "boot-failure.png") });
    throw error;
  }
  await expect(page.locator(".course-introduction")).toHaveCount(0);
  return { context, page, errors };
}
try {
  for (const viewport of [{ width: 1440, height: 900 }, { width: 375, height: 812 }]) {
    const { context, page, errors } = await setup(undefined, viewport);
    await page.evaluate(() => { location.hash = "quizzes"; });
    await expect(page.locator('[data-course-grid="quizzes"][data-grid-active]')).toBeVisible();
    for (const topic of course.topics.filter(topic => topic.id !== "energy")) {
      const host = card(page, topic.id);
      await expect(host.locator("a[href]")).toHaveCount(0);
      await expect(host.locator(".course-access-preview[inert]")).toHaveCount(1);
      await expect(pane(page, topic.id)).not.toHaveAttribute("data-opened", "");
      await page.evaluate(id => { location.hash = id; }, topic.id);
      await expect(page.locator('[data-course-grid="quizzes"][data-grid-active]')).toBeVisible();
      await expect(pane(page, topic.id)).not.toHaveAttribute("data-opened", "");
    }
    await page.evaluate(() => { location.hash = "energy"; });
    await expect(page.locator('[data-course-grid="energy"][data-grid-active]')).toBeVisible();
    await card(page, "physics").getByRole("link").click();
    await expect(pane(page, "physics")).toHaveAttribute("data-opened", "");
    await page.evaluate(() => { location.hash = "quizzes"; });
    await page.screenshot({ path: path.join(generated, `locked-${viewport.width}.png`), fullPage: true });
    await page.goto(base + "?preview=1#parametric");
    await expect(pane(page, "parametric")).toHaveAttribute("data-opened", "");
    await expect(pane(page, "parametric").locator("[data-course-locked]")).toHaveCount(0);
    expect(errors).toEqual([]);
    results.push(`Current configuration, deep links and author preview at ${viewport.width}px passed`);
    await context.close();
  }
  {
    const instant = Date.now();
    const release = instant + 90_000;
    const { context, page, errors } = await setup({ ...course.unlocks, parametric: release, "physics/power-or-energy": release }, undefined, instant);
    await page.evaluate(() => { location.hash = "quizzes"; });
    await expect(card(page, "parametric").locator("[data-course-locked]")).toHaveCount(1);
    await page.evaluate(() => { location.hash = "physics"; });
    await pane(page, "physics").locator('[data-card="quiz"] [data-overview-card-action="primary"]').click();
    await page.getByRole("dialog").locator("input[value=anonymous]").check();
    await page.getByRole("dialog").locator("[data-overview-card-action=primary]").click();
    await page.evaluate(() => { location.hash = "physics"; });
    await pane(page, "physics").locator("[data-card=quiz] [data-overview-card-action=primary]").click();
    await expect(page.locator('[data-card="run"]')).toBeVisible();
    const question = page.locator('.course-run > [data-course-locked="physics/power-or-energy"]');
    await expect(question).toBeVisible();
    await expect(question.locator(".course-access-preview[inert]")).toHaveCount(1);
    await expect(page.locator('[data-card="run"] nav button').first()).toBeDisabled();
    await page.clock.fastForward(90_001);
    await expect(question).toHaveCount(0);
    await expect(page.locator('[data-card="run"] nav button').first()).toBeEnabled();
    await page.evaluate(() => { location.hash = "quizzes"; });
    await page.locator('header [data-course-nav="overview"]').click();
    await page.evaluate(() => { location.hash = "quizzes"; });
    await expect(card(page, "parametric").locator("a[href]")).toHaveCount(1);
    expect(errors).toEqual([]);
    results.push("Authored topic and question dates flow through material, course projection, session and run UI; release timers unlock both without reload or focus");
    await context.close();
  }
  fs.writeFileSync(path.join(generated, "browser-results.json"), JSON.stringify(results, null, 2));
  console.log("[DEBUG] " + results.join("\n[DEBUG] "));
} finally { await browser.close(); }
