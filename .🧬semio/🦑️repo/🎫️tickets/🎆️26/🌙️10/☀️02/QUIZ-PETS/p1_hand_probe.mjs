/** 🖐️ Ticket tool of work package P1: what lies under the pets of the home overview and which cursor it shows when the root element alone carries the hand (`cursor: grab` inherited from the document element, the way the pets' stylesheet marks a pet that can be picked up) — whether any element under a pet asks for a cursor of its own and would hide the hand.
 *
 * For every pet on stage and nine points of its drawn box: the element the browser hit-tests there (the layer takes no
 * pointer events), whether a press there would be the page's (a control: the hand never shows over one) and the
 * cursor that element computes under the probe's own rule `[data-p1="grab"] { cursor: grab !important }`.
 *
 * Usage (from the repository root): node ".../p1_hand_probe.mjs" [--url http://127.0.0.1:6245/] [--mode calm] [--rounds 5]
 */
import { chromium } from "playwright";

const option = (name, fallback) => {
  const at = process.argv.indexOf(`--${name}`);
  return at < 0 ? fallback : process.argv[at + 1];
};
const url = option("url", "http://127.0.0.1:6245/");
const mode = option("mode", "calm");
const rounds = Number(option("rounds", "5"));
const browser = await chromium.launch({ channel: "chromium" });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1, locale: "en-US", bypassCSP: true });
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
const seen = {};
for (let round = 0; round < rounds; round++) {
  await page.waitForTimeout(4000);
  const found = await page.evaluate(() => {
    const sheet = document.createElement("style");
    sheet.textContent = '[data-p1="grab"] { cursor: grab !important; }';
    document.head.append(sheet);
    document.documentElement.setAttribute("data-p1", "grab");
    const controls = 'a[href], button, input, select, textarea, summary, [role="button"], [role="link"], [role="checkbox"], [role="radio"], [role="tab"], [role="menuitem"], [role="option"], [role="slider"], [role="switch"], [tabindex]:not([tabindex="-1"]), [contenteditable]:not([contenteditable="false"]), label, [data-quiz-grip], [data-quiz-drop], [data-layered-card]';
    const rows = [];
    for (const pet of document.querySelectorAll(".pet-layer svg.pet")) {
      const box = pet.getBoundingClientRect();
      for (const fx of [0.25, 0.5, 0.75])
        for (const fy of [0.25, 0.5, 0.75]) {
          const under = document.elementFromPoint(box.left + box.width * fx, box.top + box.height * fy);
          if (under === null) continue;
          const control = under.closest(controls) !== null;
          rows.push(`${control ? "control" : "free"} ${under.tagName.toLowerCase()}${under.classList.length > 0 ? `.${[...under.classList].slice(0, 2).join(".")}` : ""} → ${getComputedStyle(under).cursor}`);
        }
    }
    document.documentElement.removeAttribute("data-p1");
    sheet.remove();
    return rows;
  });
  for (const row of found) seen[row] = (seen[row] ?? 0) + 1;
}
for (const [row, count] of Object.entries(seen).sort((a, b) => b[1] - a[1])) process.stdout.write(`${count} ${row}\n`);
await browser.close();
