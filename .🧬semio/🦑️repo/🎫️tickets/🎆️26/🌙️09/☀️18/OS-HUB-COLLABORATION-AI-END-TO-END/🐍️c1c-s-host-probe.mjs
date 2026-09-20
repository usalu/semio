/** 🪐️ Slice C1c — the signed-in `s` HOST, observed. S2 brought the first cold `s` boot up and its
 * foreign-kind probe stops at `s.home.session-identity-required`; this probe drives the other half —
 * a real human signing in inside `s` — and states, from the running page, whether Home then publishes
 * the surface it refused before. That is the runtime proof of C1c's ShellHost re-assembly fix
 * (the session-refresh key carrying the signed-in human, `🏛️ShellHost/🟦️.tsx`).
 *
 * 🪪️ Check 1 was rewritten by S3 (2026-09-20) when the product contract changed under it. It used to
 * assert the DEFECT — Home refusing its surface without a signed-in human — which S2 §3.4 removed and
 * S3 §3 finished by clearing the `DuplicateSiblingKey` underneath it. It now asserts the behaviour:
 * the landing window publishes a live, EMPTY surface signed out, with the shell's sign-in call to
 * action beside it and no space rows; the rows arriving after the sign-in are check 2's business.
 *
 * Usage: bun 🐍️c1c-s-host-probe.mjs <uiOrigin> <hubOrigin> [users]
 */
import { chromium } from "playwright";
import { fileURLToPath } from "node:url";

const uiOrigin = process.argv[2] ?? "http://127.0.0.1:6071";
const hubOrigin = process.argv[3] ?? "http://127.0.0.1:7501";
const users = Number(process.argv[4] ?? 1);
const IDENTITY_FAULT = "s.home.session-identity-required";
const ACCOUNTS = [
  { email: "user1@semio.dev", password: "collab e2e first human phrase" },
  { email: "user2@semio.dev", password: "collab e2e second human phrase" },
];

let failures = 0;
const check = (label, actual, expected) => {
  const ok = JSON.stringify(actual) === JSON.stringify(expected);
  if (!ok) failures += 1;
  console.log(`${ok ? "PASS" : "FAIL"} ${label}: ${JSON.stringify(actual)}${ok ? "" : ` (expected ${JSON.stringify(expected)})`}`);
};

const census = (page) =>
  page.evaluate((fault) => ({
    identityFault: document.body.innerText.includes(fault),
    tableHosts: document.querySelectorAll(".semio-table-host").length,
    spaceRows: document.querySelectorAll('[data-row-id^="space:"]').length,
    homeSurface: document.querySelector('[id="s-home-main"]') !== null,
    commandTab: document.querySelector('[id="framework.category.command"]') !== null,
    presence: document.querySelector('[id="s-presence-peers"]') !== null,
    signInOffered: document.querySelectorAll('[data-semio-hub-sign-in=""]').length,
    ids: [...document.querySelectorAll("[id]")].map((element) => element.id).filter((id) => id.startsWith("s-")),
  }), IDENTITY_FAULT);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const shells = [];
try {
  for (let index = 0; index < users; index += 1) {
    const account = ACCOUNTS[index];
    const label = `user${index + 1}`;
    const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
    const page = await context.newPage();
    page.on("pageerror", (error) => console.log(`[${label}] pageerror ${error.message}`.slice(0, 220)));
    await page.goto(`${uiOrigin}/`, { waitUntil: "domcontentloaded", timeout: 300_000 });
    // 🐢️ `s` boots ~60 plugin modules; the landing app's own surface is the readiness signal.
    await page.waitForFunction(
      (fault) => document.body.innerText.includes(fault) || document.querySelector('[id="s-home-main"]') !== null || document.querySelectorAll(".semio-table-host").length > 0,
      IDENTITY_FAULT,
      { timeout: 300_000 },
    );
    // 🫧️ The surface id can appear a beat before the plugin's own fault renders into it, so the
    // "before" state is read after it settles rather than at the first paint.
    await page.waitForTimeout(8_000);
    const before = await census(page);
    console.log(`[${label}] before sign-in: ${JSON.stringify(before)}`);
    await page.screenshot({ path: fileURLToPath(new URL(`./🗑️generated/c1c-s-host-${label}-before.png`, import.meta.url)) });
    if (index === 0) {
      check("1 the s host renders its hub badge signed out", before.signInOffered, 1);
      // 🪪️ NEW CONTRACT (ticket 26/09/18, S2 §3.4 + S3 §3): signed out is a STATE, not a fault. The
      // landing window publishes a LIVE, empty surface for a visitor with no identity — the ordinary
      // first paint of a hub-configured shell — next to the shell's own sign-in call to action. This
      // check used to assert the defect (`identityFault: true`, `homeSurface: false`); it now asserts
      // the product behaviour, and the space table filling in is check 2's business.
      check("1 Home publishes a live signed-out surface, not a fault", [before.identityFault, before.homeSurface || before.tableHosts > 0], [false, true]);
      check("1 the signed-out surface offers the sign-in call to action", before.signInOffered > 0, true);
      check("1 a signed-out human is shown no spaces", before.spaceRows, 0);
    }

    // 🎓️ The `s` host runs its introduction on a first visit and its veil
    // (`[data-slot="introduction-veil"]`, `pointer-events-auto`) sits over the whole shell — the
    // playground variants pass `suppressAutoIntroduction`, `s` does not. A human dismisses it before
    // touching anything, so the probe does exactly that rather than synthesising a click past it.
    // A dismissed tour can be followed by the next one, so the probe does what a human does: skip,
    // try the real control, repeat until the click lands.
    const clickThroughIntroduction = async (locator, budgetMs) => {
      const deadline = Date.now() + budgetMs;
      let skips = 0;
      for (;;) {
        try {
          await locator.click({ timeout: 4_000 });
          return skips;
        } catch (error) {
          if (Date.now() >= deadline) throw error;
          if ((await page.locator('[data-slot="introduction-veil"]').count()) > 0) skips += 1;
          await page.keyboard.press("Escape").catch(() => undefined);
          await page.waitForTimeout(300);
        }
      }
    };
    console.log(`[${label}] introduction steps skipped before the badge: ${await clickThroughIntroduction(page.locator('[data-semio-hub-sign-in=""]').first(), 120_000)}`);
    const form = page.locator("[data-semio-hub-workspace]");
    await form.waitFor({ state: "visible", timeout: 60_000 });
    await form.locator('input[type="email"]').fill(account.email);
    await form.locator('input[type="password"]').fill(account.password);
    await clickThroughIntroduction(form.locator('button[type="submit"][aria-label="Sign in"]'), 60_000);
    await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 });
    await clickThroughIntroduction(page.locator("[data-semio-hub-workspace] button[aria-label]").first(), 60_000);
    // ⏳️ Re-assembly is a fresh plugin round trip, not a paint: wait for the fault to clear on the
    // SAME page rather than sampling once.
    await page
      .waitForFunction((fault) => !document.body.innerText.includes(fault), IDENTITY_FAULT, { timeout: 180_000 })
      .catch(() => undefined);
    // 🪐️ Clearing the fault is the re-assembly; PUBLISHING `s-home-main` is a further guest round
    // trip plus the directory bootstrap, so the surface is waited for separately and its absence is
    // reported as its own fact rather than folded into the fault check.
    await page
      .waitForFunction(() => document.querySelector('[id="s-home-main"]') !== null || document.querySelectorAll(".semio-table-host").length > 0, undefined, { timeout: 120_000 })
      .catch(() => undefined);
    const after = await census(page);
    console.log(`[${label}] after sign-in: ${JSON.stringify(after)}`);
    await page.screenshot({ path: fileURLToPath(new URL(`./🗑️generated/c1c-s-host-${label}-after.png`, import.meta.url)) });
    if (index === 0) {
      check("2 the human is signed in inside s", after.signInOffered, 0);
      check("2 Home no longer answers session-identity-required", after.identityFault, false);
      check("2 the host app surface is published", after.homeSurface || after.tableHosts > 0, true);
      check("2 the command palette tab is present", after.commandTab, true);
    }
    shells.push({ label, page, after });
  }

  if (users > 1) {
    const [one, two] = shells;
    check("3 both shells are signed in at once", [one.after.signInOffered, two.after.signInOffered], [0, 0]);
    const rosterOf = (shell) => shell.page.locator('[id="s-presence-peers"] [data-row-id^="peer:"]:not([data-row-id="peer:overflow"])').count();
    console.log(`[presence] user1 roster=${await rosterOf(one)} user2 roster=${await rosterOf(two)}`);
    for (const shell of shells) await shell.page.screenshot({ path: fileURLToPath(new URL(`./🗑️generated/c1c-s-host-presence-${shell.label}.png`, import.meta.url)) });
  }
} finally {
  await browser.close();
}
console.log(failures === 0 ? "c1c-s-host: all checks passed" : `c1c-s-host: ${failures} check(s) failed`);
process.exit(failures === 0 ? 0 : 1);
