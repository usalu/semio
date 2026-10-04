/** 🧬️ Ticket tool of work package P1: which mutations of the home overview reach an observer like the pets' survey watch (class, hidden, inert, open and child lists of the whole document) while the pointer sweeps — how many callbacks hold only records behind an inert or hidden ancestor, and where the others happen.
 *
 * Usage (from the repository root): node ".../p1_mutation_probe.mjs" [--url http://127.0.0.1:6245/] [--mode off|still|calm|lively] [--seconds 10] [--rest]
 * (`--rest` leaves the pointer on the navbar instead of sweeping.)
 */
import { chromium } from "playwright";

const option = (name, fallback) => {
  const at = process.argv.indexOf(`--${name}`);
  return at < 0 ? fallback : process.argv[at + 1];
};
const url = option("url", "http://127.0.0.1:6245/");
const mode = option("mode", "off");
const seconds = Number(option("seconds", "10"));
const browser = await chromium.launch({ channel: "chromium" });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1, locale: "en-US" });
await context.addInitScript(() => {
  const state = { on: false, calls: 0, seenCalls: 0, records: 0, unseen: 0, inLayer: 0, where: {} };
  const describe = (node) => {
    const element = node.nodeType === 1 ? node : node.parentElement;
    if (element === null) return "?";
    const marks = [...element.attributes].map((attribute) => attribute.name).filter((name) => name.startsWith("data-")).slice(0, 2);
    const anchor = element.closest("[data-layered-card], [data-layered-pane], [data-card], header, footer, [data-presence-layer]");
    const where = anchor === null ? "page" : anchor.matches("[data-layered-card]") ? `card ${anchor.getAttribute("data-layered-card")}` : anchor.matches("[data-layered-pane]") ? "pane" : anchor.matches("[data-card]") ? `data-card ${anchor.getAttribute("data-card")}` : anchor.tagName.toLowerCase();
    return `${where} › ${element.tagName.toLowerCase()}${marks.length > 0 ? `[${marks.join(",")}]` : ""}`;
  };
  const start = () => {
    new MutationObserver((records) => {
      if (!state.on) return;
      state.calls += 1;
      let seen = false;
      for (const record of records) {
        state.records += 1;
        const element = record.target.nodeType === 1 ? record.target : record.target.parentElement;
        if (element?.closest(".pet-layer") != null) {
          state.inLayer += 1;
          continue;
        }
        const above = record.type === "attributes" ? element?.parentElement : element;
        if (above?.closest("[inert], [hidden]") != null) {
          state.unseen += 1;
          continue;
        }
        seen = true;
        const key = `${record.type === "attributes" ? `@${record.attributeName}` : "children"} ${describe(record.target)}`;
        state.where[key] = (state.where[key] ?? 0) + 1;
      }
      if (seen) state.seenCalls += 1;
    }).observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["class", "hidden", "inert", "open"] });
  };
  if (document.documentElement !== null) start();
  else new MutationObserver((_, watch) => document.documentElement !== null && (watch.disconnect(), start())).observe(document, { childList: true });
  window.__p1mutations = {
    start: () => Object.assign(state, { on: true, calls: 0, seenCalls: 0, records: 0, unseen: 0, inLayer: 0, where: {} }),
    stop: () => ((state.on = false), state),
  };
});
const page = await context.newPage();
await page.goto(url);
const intro = page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]');
await intro.waitFor({ timeout: 60_000 });
await intro.click();
await page.locator('#quiz-main [data-card="identity"] input[type="radio"][value="anonymous"]').check();
await page.locator('#quiz-main [data-card="identity"] [data-overview-card-action="primary"]').click();
await page.locator('[data-layered-overview][data-mode="strip"]').waitFor({ timeout: 60_000 });
await page.evaluate((mode) => {
  const key = Object.keys(localStorage)
    .find((name) => name.startsWith("semio.quiz.") && name.endsWith(".learner"))
    .replace(/learner$/u, "preferences");
  localStorage.setItem(key, JSON.stringify({ ...JSON.parse(localStorage.getItem(key) ?? "{}"), pets: mode, petsChosen: true }));
}, mode);
await page.mouse.move(720, 20);
await page.reload();
await page.locator('[data-layered-overview][data-mode="strip"]').waitFor({ timeout: 60_000 });
await page.waitForTimeout(7000);
await page.evaluate(() => window.__p1mutations.start());
const begun = Date.now();
const resting = process.argv.includes("--rest");
if (resting) await page.waitForTimeout(seconds * 1000);
while (!resting && Date.now() - begun < seconds * 1000) {
  const t = (Date.now() - begun) / 1000;
  await page.mouse.move(1440 * (0.5 + 0.46 * Math.sin((2 * Math.PI * t) / 3.7)), 900 * (0.5 + 0.4 * Math.sin((2 * Math.PI * t) / 2.3 + 0.7)));
}
const seen = await page.evaluate(() => window.__p1mutations.stop());
process.stdout.write(`${mode}, ${seconds} s ${resting ? "with the pointer resting on the navbar" : "sweeping"}: callbacks ${seen.calls}, of which with a record outside inert/hidden subtrees ${seen.seenCalls}; records ${seen.records}: behind inert/hidden ${seen.unseen}, inside the pet layer ${seen.inLayer}\n`);
for (const [key, count] of Object.entries(seen.where)
  .sort((a, b) => b[1] - a[1])
  .slice(0, 25))
  process.stdout.write(`  ${count} ${key}\n`);
await browser.close();
