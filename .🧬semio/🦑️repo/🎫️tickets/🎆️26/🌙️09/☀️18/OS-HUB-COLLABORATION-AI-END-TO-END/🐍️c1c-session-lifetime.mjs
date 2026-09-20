/** ⏳️ Slice C1c — isolates the one identity check that failed: after a second human signs in in a
 * SEPARATE browser context, the first shell stopped holding its authority. This probe signs ONE human
 * in and then samples the badge every 2 s for 90 s with no second context at all, printing every
 * `/auth/*` response and every console error in between, so the answer is either "the session decays on
 * its own" (a lifetime/refresh defect) or "only a second context does it" (a shared-worker defect).
 *
 * Usage: bun 🐍️c1c-session-lifetime.mjs <uiOrigin> <hubOrigin> [samples]
 */
import { chromium } from "playwright";

const uiOrigin = process.argv[2] ?? "http://127.0.0.1:7502";
const hubOrigin = process.argv[3] ?? "http://127.0.0.1:7501";
const samples = Number(process.argv[4] ?? 45);
const USER1 = { email: "user1@semio.dev", password: "collab e2e first human phrase" };

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
const page = await context.newPage();
const started = Date.now();
const since = () => `${((Date.now() - started) / 1000).toFixed(1)}s`;
page.on("console", (message) => {
  if (message.type() === "error" || message.text().includes("session") || message.text().includes("hub")) {
    console.log(`[${since()}] console:${message.type()} ${message.text()}`.slice(0, 300));
  }
});
page.on("response", (response) => {
  const url = response.url();
  if (!url.includes("/auth/")) return;
  console.log(`[${since()}] wire ${response.status()} ${response.request().method()} ${url.replace(uiOrigin, "").replace(hubOrigin, "hub")}`);
});

try {
  await page.goto(`${uiOrigin}/`, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.waitForTimeout(3_000);
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 30_000 });
  await form.locator('input[type="email"]').fill(USER1.email);
  await form.locator('input[type="password"]').fill(USER1.password);
  await form.locator('button[type="submit"][aria-label="Sign in"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
  console.log(`[${since()}] signed in`);
  const mint = await page.evaluate(() => {
    try {
      return globalThis.sessionStorage.getItem("semio.os.hub-session-capability.v1");
    } catch {
      return "storage-unavailable";
    }
  });
  console.log(`[${since()}] stored capability: ${String(mint).slice(0, 200)}`);

  let flipped = -1;
  for (let index = 0; index < samples; index += 1) {
    await page.waitForTimeout(2_000);
    const offers = await page.locator('[data-semio-hub-sign-in=""]').count();
    const stored = await page.evaluate(() => {
      try {
        return globalThis.sessionStorage.getItem("semio.os.hub-session-capability.v1") === null ? "cleared" : "held";
      } catch {
        return "storage-unavailable";
      }
    });
    console.log(`[${since()}] sample ${index}: signInAffordance=${offers} capability=${stored}`);
    if (offers > 0 && flipped < 0) {
      flipped = index;
      console.log(`[${since()}] FLIPPED to signed-out at sample ${index}`);
    }
  }
  console.log(flipped < 0 ? "c1c-session-lifetime: the shell held its authority for the whole window" : `c1c-session-lifetime: authority lost after ${flipped * 2}s with no second context`);
} finally {
  await browser.close();
}
