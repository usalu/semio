/** 🔬️ Timeline trace of the first quiz start (or the second with QUIZ=1): total milliseconds per browser activity (style, layout, script evaluation, compile, paint, GC) inside the main-thread tasks over 40 ms. */
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/.cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test/");
const { chromium } = require("@playwright/test");
const site = process.env.SITE ?? "http://127.0.0.1:6069", ids = ["physics", "ventilation"], which = Number(process.env.QUIZ ?? 0);
const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 1280, height: 800 }, locale: "en" });
const page = await context.newPage();
page.setDefaultTimeout(120_000);
await page.goto(site + "/", { waitUntil: "commit" });
await page.locator('[data-slot="introduction-close"]').click();
await page.locator('[data-slot="introduction-info-box"]').waitFor({ state: "detached" });
const learner = page.locator('[data-course-grid][data-grid-active] > [data-layered-overview] [data-layered-card="learner"] [data-overview-card-action="primary"]');
await learner.click();
const dialog = page.getByRole("dialog").filter({ has: page.locator('input[type="radio"][value="anonymous"]') });
await dialog.locator('input[type="radio"][value="name"]').click();
await dialog.locator('input[type="text"]').fill("Trace Walker");
await dialog.locator('[data-overview-card-action="primary"]').click();
await page.goto(site + "/?preview=1#quizzes", { waitUntil: "commit" });
const open = async (id) => { const card = page.locator(`[data-course-grid][data-grid-active] > [data-layered-overview] [data-layered-card="${id}"] [data-overview-card-action="primary"]`).first(); await card.waitFor(); await page.waitForTimeout(2500); await card.evaluate((e) => e.click()); await page.locator('#course-main [data-card="run"]').first().waitFor(); await page.waitForTimeout(1500); };
if (which > 0) { await open(ids[0]); await page.evaluate(() => (window.location.hash = "quizzes")); await page.locator('[data-course-grid="quizzes"][data-grid-active]').waitFor(); }
const cdp = await context.newCDPSession(page);
const events = [];
cdp.on("Tracing.dataCollected", (e) => events.push(...e.value));
const complete = new Promise((done) => cdp.once("Tracing.tracingComplete", done));
await cdp.send("Tracing.start", { traceConfig: { includedCategories: ["devtools.timeline", "v8", "disabled-by-default-devtools.timeline"] } });
await open(ids[which]);
await cdp.send("Tracing.end"); await complete;
const main = events.filter((e) => e.ph === "X" && e.dur !== undefined);
const tasks = main.filter((e) => e.name === "RunTask" && e.dur > 40_000);
const total = {};
for (const task of tasks) {
  const inside = main.filter((e) => e.tid === task.tid && e.ts >= task.ts && e.ts + e.dur <= task.ts + task.dur && e !== task);
  for (const e of inside) { total[e.name] = (total[e.name] ?? 0) + e.dur / 1000; }
}
console.log("tasks over 40ms:", tasks.map((t) => Math.round(t.dur / 1000)));
console.log(Object.entries(total).sort((a, b) => b[1] - a[1]).slice(0, 14).map(([k, v]) => `${Math.round(v)}ms ${k}`).join("\n"));
await browser.close();
