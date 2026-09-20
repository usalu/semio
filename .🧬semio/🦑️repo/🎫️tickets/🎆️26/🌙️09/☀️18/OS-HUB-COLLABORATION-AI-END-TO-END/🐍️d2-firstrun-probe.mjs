// 🎓️ D2 — observe the hub first-run walkthrough in a REAL served shell against a REAL hub.
// Reuses AU3's boot recipe verbatim (footer connection badge → hub workspace); the only new part is
// asserting the introduction info box is there and shooting it. Fresh browser context every run, so
// `ui.introduction.seen.os.hub` starts empty and the tour must auto-start.
// Usage: node 🐍️d2-firstrun-probe.mjs <uiOrigin> <hubOrigin> <screenshotPath>
import { chromium } from "playwright";

const ui = process.argv[2] ?? "http://127.0.0.1:7502";
const hub = process.argv[3] ?? "http://127.0.0.1:7501";
const shot = process.argv[4] ?? "🗑️generated/d2-firstrun-live.png";

const console_ = [];
const rows = [];
let failures = 0;
function check(name, actual, expected) {
  const ok = actual === expected;
  if (!ok) failures += 1;
  rows.push(`${ok ? "PASS" : "FAIL"} ${name} — got ${JSON.stringify(actual)}, want ${JSON.stringify(expected)}`);
}

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
try {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const page = await context.newPage();
  page.on("console", (m) => console_.push(`${m.type()} ${m.text().slice(0, 300)}`));
  page.on("pageerror", (e) => console_.push(`pageerror ${String(e).slice(0, 300)}`));

  await page.goto(ui, { waitUntil: "domcontentloaded" });
  await page.waitForSelector("[data-semio-hub-connection]", { timeout: 180_000 });
  await page.click('[data-semio-hub-sign-in=""]');
  await page.waitForSelector("[data-semio-hub-workspace]", { timeout: 30_000 });

  // 🎓️ The tour must auto-start on this fresh profile, unprompted.
  await page.waitForSelector('[data-slot="introduction-info-box-title"]', { timeout: 30_000 });
  const title = (await page.textContent('[data-slot="introduction-info-box-title"]'))?.trim();
  check("1 the walkthrough auto-starts in the hub workspace", typeof title === "string" && title.length > 0, true);
  check("2 it opens on the sign-in step (signed out)", title, "Sign in");
  check("3 the help affordance is present", await page.locator('[id="os.hub.firstRun.replay"]').count(), 1);
  const body = (await page.textContent('[data-slot="introduction-body-paragraph"]'))?.trim() ?? "";
  check("4 the step body rendered", body.startsWith("Enter the email and password"), true);

  await page.screenshot({ path: shot, fullPage: false });
  rows.push(`shot ${shot}`);

  // 🎓️ Dismiss → gone, and the shared introduction key is written.
  await page.click('[id="ui.introduction.skip"]');
  await page.waitForSelector('[data-slot="introduction-info-box-title"]', { state: "detached", timeout: 15_000 });
  check("5 skip closes the walkthrough", await page.locator('[data-slot="introduction-info-box-title"]').count(), 0);
  const seen = await page.evaluate(() => {
    try {
      return window.localStorage.getItem("ui.introduction.seen.os.hub");
    } catch {
      return "<storage denied>";
    }
  });
  check("6 dismissal persisted into the shared introduction key", seen, "true");

  // 🎓️ The help affordance replays it from the beginning even though the flag is now set.
  await page.click('[id="os.hub.firstRun.replay"]');
  await page.waitForSelector('[data-slot="introduction-info-box-title"]', { timeout: 15_000 });
  check("7 the help affordance replays from the first step", (await page.textContent('[data-slot="introduction-info-box-title"]'))?.trim(), "What a hub gives you");

  // 📱️ Phone width: the centered screen step must stay inside the viewport.
  await page.setViewportSize({ width: 375, height: 812 });
  await page.waitForTimeout(500);
  const box = await page.locator('[data-slot="introduction-info-box-content"]').boundingBox();
  check("8 the info box fits a 375px viewport", Boolean(box) && box.x >= 0 && box.x + box.width <= 375, true);
  await page.screenshot({ path: shot.replace(/\.png$/, "-phone.png"), fullPage: false });

  await context.close();
} catch (error) {
  failures += 1;
  rows.push(`FAIL probe threw — ${String(error).slice(0, 600)}`);
} finally {
  await browser.close();
}

console.log(rows.join("\n"));
console.log(`\n${failures === 0 ? "ALL PASS" : `${failures} FAILED`}`);
if (console_.length) console.log(`\n--- console (last 25) ---\n${console_.slice(-25).join("\n")}`);
process.exit(failures === 0 ? 0 : 1);
