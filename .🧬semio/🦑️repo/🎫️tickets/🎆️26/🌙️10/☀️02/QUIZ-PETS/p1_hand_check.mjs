/** 🫳️ Ticket tool of work package P1: the hand on the real home overview — the pointer is moved onto a perched pet over free space and off it again to the navbar, ten times, traced: what the document element says, which element carries the cursor and what it computes, and how many elements the browser restyles for the passes; then the same passes at the same places with the pets off, as the control (the overview itself restyles while the pointer crosses it).
 *
 * Usage (from the repository root): node ".../p1_hand_check.mjs" [--url http://127.0.0.1:6245/] [--browser gpu|shell]
 */
import { chromium } from "playwright";

const option = (name, fallback) => {
  const at = process.argv.indexOf(`--${name}`);
  return at < 0 ? fallback : process.argv[at + 1];
};
const url = option("url", "http://127.0.0.1:6245/");
const browser = await chromium.launch(option("browser", "gpu") === "gpu" ? { channel: "chromium" } : {});
const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1, locale: "en-US" });
const page = await context.newPage();
const cdp = await browser.newBrowserCDPSession();
await page.goto(url);
const intro = page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]');
await intro.waitFor({ timeout: 60_000 });
await intro.click();
await page.locator('#quiz-main [data-card="identity"] input[type="radio"][value="anonymous"]').check();
await page.locator('#quiz-main [data-card="identity"] [data-overview-card-action="primary"]').click();
await page.locator('[data-layered-overview][data-mode="strip"]').waitFor({ timeout: 60_000 });

/** 🎛️ Chooses the pets' liveliness in the local store and reloads. */
async function choose(mode) {
  await page.evaluate((mode) => {
    const key = Object.keys(localStorage)
      .find((name) => name.startsWith("semio.quiz.") && name.endsWith(".learner"))
      .replace(/learner$/u, "preferences");
    localStorage.setItem(key, JSON.stringify({ ...JSON.parse(localStorage.getItem(key) ?? "{}"), pets: mode, petsChosen: true, petsPlay: true }));
  }, mode);
  await page.mouse.move(720, 20);
  await page.reload();
  await page.locator(`.quiz-app[data-pets="${mode}"]`).waitFor({ timeout: 60_000 });
  if (mode !== "off") await page.locator(".pet-layer svg.pet").first().waitFor({ timeout: 90_000 });
  await page.waitForTimeout(8000);
}

/** 🔁️ Ten passes onto `spot` and off it to the navbar, traced: the states seen and the style recalculations. */
async function passes(spot) {
  const complete = new Promise((done) => cdp.once("Tracing.tracingComplete", done));
  await cdp.send("Tracing.start", { transferMode: "ReturnAsStream", traceConfig: { recordMode: "recordAsMuchAsPossible", includedCategories: ["devtools.timeline", "blink.user_timing"] } });
  await page.evaluate(() => performance.mark("p1-sweep-start"));
  const states = [];
  const look = () =>
    page.evaluate(() => ({
      root: document.documentElement.getAttribute("data-pet-cursor"),
      marked: [...document.querySelectorAll('[style*="cursor"]')].filter((element) => element.style.getPropertyValue("cursor") === "grab").map((element) => `${element.tagName.toLowerCase()}${element.classList.length > 0 ? `.${[...element.classList].slice(0, 2).join(".")}` : ""} → ${getComputedStyle(element).cursor}`),
    }));
  for (let pass = 0; pass < 10; pass++) {
    await page.mouse.move(spot.x, spot.y, { steps: 2 });
    await page.waitForTimeout(120);
    states.push(await look());
    await page.mouse.move(spot.x + 300, 20, { steps: 2 });
    await page.waitForTimeout(120);
    states.push(await look());
  }
  await page.evaluate(() => performance.mark("p1-sweep-end"));
  await cdp.send("Tracing.end");
  const { stream } = await complete;
  const chunks = [];
  for (;;) {
    const read = await cdp.send("IO.read", { handle: stream, size: 16 * 1024 * 1024 });
    chunks.push(read.base64Encoded ? Buffer.from(read.data, "base64") : Buffer.from(read.data, "utf8"));
    if (read.eof) break;
  }
  await cdp.send("IO.close", { handle: stream });
  const events = JSON.parse(Buffer.concat(chunks).toString("utf8")).traceEvents;
  const start = events.find((event) => event.name === "p1-sweep-start");
  const end = events.find((event) => event.name === "p1-sweep-end");
  const styles = events.filter((event) => event.name === "UpdateLayoutTree" && event.ph === "X" && event.pid === start.pid && event.tid === start.tid && event.ts >= start.ts && event.ts <= end.ts);
  const counts = styles.map((event) => event.args?.elementCount ?? 0);
  return { states, recalcs: styles.length, ms: styles.reduce((sum, event) => sum + event.dur / 1000, 0), whole: counts.filter((count) => count > 1000).length, largest: Math.max(0, ...counts), longest: Math.max(0, ...styles.map((event) => event.dur / 1000)) };
}

await choose("calm");
const spot = await page.evaluate(() => {
  const controls = 'a[href], button, input, select, textarea, summary, [role="button"], [role="link"], [tabindex]:not([tabindex="-1"]), label, [data-quiz-grip], [data-quiz-drop], [data-layered-card]';
  for (const pet of document.querySelectorAll('.pet-layer svg.pet[data-pet-footing="perch"]')) {
    const box = pet.getBoundingClientRect();
    const x = box.left + box.width / 2;
    const y = box.top + box.height * 0.6;
    const under = document.elementFromPoint(x, y);
    if (under !== null && under.closest(controls) === null) return { x, y, pet: pet.getAttribute("data-pet") };
  }
  return null;
});
if (spot === null) {
  process.stdout.write("no perched pet over free space\n");
  await browser.close();
  process.exit(1);
}
const calm = await passes(spot);
await choose("off");
const off = await passes(spot);
const on = calm.states.filter((_, index) => index % 2 === 0);
const away = calm.states.filter((_, index) => index % 2 === 1);
process.stdout.write(`pet ${spot.pet} at ${Math.round(spot.x)},${Math.round(spot.y)}; on the pet: ${JSON.stringify(on[0])}\n`);
process.stdout.write(`calm: root says grab on ${on.filter((state) => state.root === "grab").length}/10 passes onto the pet, nothing marked on ${away.filter((state) => state.root === null && state.marked.length === 0).length}/10 passes off it\n`);
for (const [name, seen] of [
  ["calm", calm],
  ["off (control)", off],
])
  process.stdout.write(`${name}: ${seen.recalcs} style recalculations, ${seen.ms.toFixed(1)} ms, ${seen.whole} of more than 1000 elements (largest ${seen.largest}), longest ${seen.longest.toFixed(2)} ms\n`);
await browser.close();
