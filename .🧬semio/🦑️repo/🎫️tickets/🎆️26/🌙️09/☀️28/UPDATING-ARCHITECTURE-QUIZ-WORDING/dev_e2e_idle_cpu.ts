/** 🔥️ Measures how busy one page of the quiz site keeps the browser's main thread while nobody touches it: the share of
 * wall time Chromium spends in tasks (script, layout, style) on the introduction, on the overview and inside a run.
 * `bun dev_e2e_idle_cpu.ts [site origin] [seconds]` against a running stack (default the dev site on 6061). */
import { chromium, type CDPSession, type Page } from "playwright";

const origin = process.argv[2] ?? "http://127.0.0.1:6061";
const seconds = Number(process.argv[3] ?? "10");

async function metrics(session: CDPSession): Promise<Record<string, number>> {
  const { metrics: rows } = await session.send("Performance.getMetrics");
  return Object.fromEntries(rows.map((row) => [row.name, row.value]));
}

async function busy(page: Page, session: CDPSession, where: string): Promise<void> {
  const before = await metrics(session);
  await page.waitForTimeout(seconds * 1_000);
  const after = await metrics(session);
  const share = (name: string): string => `${(((after[name]! - before[name]!) / seconds) * 100).toFixed(1)} %`;
  console.log(`[DEBUG] ${where}: tasks ${share("TaskDuration")}, script ${share("ScriptDuration")}, layout ${share("LayoutDuration")}, style ${share("RecalcStyleDuration")}; layouts ${after.LayoutCount! - before.LayoutCount!}, style recalcs ${after.RecalcStyleCount! - before.RecalcStyleCount!}, nodes ${after.Nodes}`);
}

const browser = await chromium.launch();
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "en-GB" })).newPage();
const session = await page.context().newCDPSession(page);
await session.send("Performance.enable");
await page.goto(origin);
await page.locator('#quiz-main [data-card="introduction"]').waitFor();
await busy(page, session, "introduction");
await page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]').click();
await page.locator('#quiz-main [data-card="identity"] [data-overview-card-action="primary"]').click();
await page.locator("[data-layered-overview]").waitFor();
await page.waitForTimeout(3_000);
await busy(page, session, "overview at rest");
await page.locator('[data-layered-card="physics"] [data-overview-card-action="primary"]').click();
await page.locator('#quiz-main [data-card="run"]').waitFor();
await page.waitForTimeout(2_000);
await busy(page, session, "inside a run");
await browser.close();
