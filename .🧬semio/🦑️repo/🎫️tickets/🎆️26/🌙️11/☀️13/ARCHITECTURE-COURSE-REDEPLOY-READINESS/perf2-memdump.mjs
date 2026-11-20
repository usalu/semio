/** 🔬️ Allocator breakdown of the renderer of one learner tab after the intro or the home screen: a Chrome memory-infra dump (malloc, V8, Skia, Blink GC, partition alloc, fonts, images) per process, largest allocators first. Env: SITE, STAGE (intro|home). */
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/.cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test/");
const { chromium } = require("@playwright/test");
const site = process.env.SITE ?? "http://127.0.0.1:6069", stage = process.env.STAGE ?? "intro";
const browser = await chromium.launch();
const root = await browser.newBrowserCDPSession();
const page = await (await browser.newContext({ viewport: { width: 1280, height: 800 }, locale: "en" })).newPage();
await page.goto(site + "/", { waitUntil: "load" });
await page.locator('[data-slot="introduction-close"]').waitFor();
if (stage === "home" || stage === "run") { await page.locator('[data-slot="introduction-close"]').click(); await page.locator('[data-slot="introduction-info-box"]').waitFor({ state: "detached" }); }
if (stage === "run") {
  await page.locator('[data-slot="introduction-info-box"]').waitFor({ state: "detached" }).catch(() => {});
  const learner = page.locator('[data-course-grid][data-grid-active] > [data-layered-overview] [data-layered-card="learner"] [data-overview-card-action="primary"]');
  await learner.click();
  const dialog = page.getByRole("dialog").filter({ has: page.locator('input[type="radio"][value="anonymous"]') });
  await dialog.locator('input[type="radio"][value="name"]').click();
  await dialog.locator('input[type="text"]').fill("Dump Walker");
  await dialog.locator('[data-overview-card-action="primary"]').click();
  await page.goto(site + "/?preview=1#quizzes", { waitUntil: "commit" });
  const card = page.locator('[data-course-grid][data-grid-active] > [data-layered-overview] [data-layered-card="physics"] [data-overview-card-action="primary"]').first();
  await card.waitFor();
  await card.evaluate((element) => element.click());
  await page.locator('#course-main [data-card="run"]').first().waitFor();
}
await page.waitForTimeout(3000);
const events = [];
root.on("Tracing.dataCollected", (e) => events.push(...e.value));
const complete = new Promise((done) => root.once("Tracing.tracingComplete", done));
await root.send("Tracing.start", { traceConfig: { recordMode: "recordUntilFull", includedCategories: ["-*", "disabled-by-default-memory-infra"], memoryDumpConfig: { triggers: [] } } });
await root.send("Tracing.requestMemoryDump", { deterministic: true, levelOfDetail: "light" });
await root.send("Tracing.end");
await complete;
const dumps = events.filter((e) => e.ph === "v" && e.args?.dumps);
for (const dump of dumps) {
  const total = dump.args.dumps.process_totals;
  if (!dump.args.dumps.allocators) continue;
  const rows = Object.entries(dump.args.dumps.allocators ?? {}).filter(([name]) => !name.includes("/")).map(([name, a]) => [name, Math.round(Number(BigInt("0x" + (a.attrs?.size?.value ?? a.attrs?.effective_size?.value ?? "0"))) / 1048576 * 10) / 10]).sort((a, b) => b[1] - a[1]).slice(0, 10);
  console.log(dump.pid, "resident", total ? Math.round(parseInt(total.resident_set_size ?? "0", 16) / 1048576) : "?", JSON.stringify(rows));
}
await browser.close();
