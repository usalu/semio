/** ✍️ Ticket tool of work package P1: what the pets write into their drawings per frame on the home overview — by attribute, and for every matrix written to a bone's group how far the drawing actually moved since the value it replaces (the largest change of a linear entry and of the translation) — with the pointer resting or sweeping.
 *
 * Usage (from the repository root): node ".../p1_write_probe.mjs" [--url http://127.0.0.1:6245/] [--mode calm|lively] [--seconds 5] [--sweep]
 */
import { chromium } from "playwright";

const option = (name, fallback) => {
  const at = process.argv.indexOf(`--${name}`);
  return at < 0 ? fallback : process.argv[at + 1];
};
const url = option("url", "http://127.0.0.1:6245/");
const mode = option("mode", "calm");
const seconds = Number(option("seconds", "5"));
const browser = await chromium.launch({ channel: "chromium" });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1, locale: "en-US" });
await context.addInitScript(() => {
  const state = { on: false, frames: 0, writes: {}, linear: [0, 0, 0, 0], shift: [0, 0, 0, 0], both: 0 };
  const original = Element.prototype.setAttribute;
  const bands = (value, limits) => limits.findIndex((limit) => value < limit);
  Element.prototype.setAttribute = function (name, value) {
    if (state.on && this.closest?.(".pet-layer") != null) {
      state.writes[name] = (state.writes[name] ?? 0) + 1;
      if (name === "transform" && this.tagName === "g" && String(value).startsWith("matrix(")) {
        const before = (this.getAttribute("transform") ?? "").match(/-?[\d.e-]+/gu)?.map(Number) ?? [];
        const after = String(value).match(/-?[\d.e-]+/gu)?.map(Number) ?? [];
        if (before.length === 6 && after.length === 6) {
          const linear = Math.max(...[0, 1, 2, 3].map((index) => Math.abs(after[index] - before[index])));
          const shift = Math.max(Math.abs(after[4] - before[4]), Math.abs(after[5] - before[5]));
          const a = bands(linear, [0.002, 0.005, 0.02]);
          const b = bands(shift, [0.05, 0.2, 0.5]);
          state.linear[a < 0 ? 3 : a] += 1;
          state.shift[b < 0 ? 3 : b] += 1;
          if (linear < 0.002 && shift < 0.05) state.both += 1;
        }
      }
    }
    return original.call(this, name, value);
  };
  const loop = () => {
    if (!state.on) return;
    state.frames += 1;
    requestAnimationFrame(loop);
  };
  window.__p1writes = {
    start: () => {
      Object.assign(state, { on: true, frames: 0, writes: {}, linear: [0, 0, 0, 0], shift: [0, 0, 0, 0], both: 0 });
      requestAnimationFrame(loop);
    },
    stop: () => {
      state.on = false;
      return state;
    },
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
await page.locator(".pet-layer svg.pet").first().waitFor({ timeout: 90_000 });
await page.waitForTimeout(6000);
await page.evaluate(() => window.__p1writes.start());
const begun = Date.now();
if (process.argv.includes("--sweep"))
  while (Date.now() - begun < seconds * 1000) {
    const t = (Date.now() - begun) / 1000;
    await page.mouse.move(1440 * (0.5 + 0.46 * Math.sin((2 * Math.PI * t) / 3.7)), 900 * (0.5 + 0.4 * Math.sin((2 * Math.PI * t) / 2.3 + 0.7)));
  }
else await page.waitForTimeout(seconds * 1000);
const seen = await page.evaluate(() => window.__p1writes.stop());
const frames = Math.max(seen.frames, 1);
process.stdout.write(`${mode}${process.argv.includes("--sweep") ? " sweeping" : " resting"}: frames ${seen.frames}; writes per frame ${Object.entries(seen.writes)
  .map(([name, count]) => `${name} ${(count / frames).toFixed(1)}`)
  .join(", ")}\n`);
const total = seen.linear.reduce((sum, count) => sum + count, 0) || 1;
process.stdout.write(`bone matrices: ${total} — largest linear change <0.002 ${seen.linear[0]}, <0.005 ${seen.linear[1]}, <0.02 ${seen.linear[2]}, more ${seen.linear[3]}; largest shift <0.05 px ${seen.shift[0]}, <0.2 ${seen.shift[1]}, <0.5 ${seen.shift[2]}, more ${seen.shift[3]}; both below 0.002 and 0.05 px: ${seen.both} (${((100 * seen.both) / total).toFixed(0)} %)\n`);
await browser.close();
