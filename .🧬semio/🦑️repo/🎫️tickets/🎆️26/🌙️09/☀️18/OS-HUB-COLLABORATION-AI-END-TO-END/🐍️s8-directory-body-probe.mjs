/** 📇️ S8 — what the hub's directory frontier actually contains for the signed-in user, read off the
 * live shell's own requests (`/directory/event-page/v1` and `/directory/spaces`), so "Home lists no
 * studios" can be attributed to the projection or to the hub's own contents.
 *
 * Usage: bun 🐍️s8-directory-body-probe.mjs [shellUrl] [email] [password] [settleMs]
 */
import { chromium } from "playwright";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6190";
const EMAIL = process.argv[3] ?? "user1@semio.dev";
const PASSWORD = process.argv[4] ?? "gm1-local-dev-pass-1";
const SETTLE = Number(process.argv[5] ?? 30_000);
const say = (...parts) => console.log("[s8]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const bodies = [];
page.on("response", async (response) => {
  const url = response.url();
  if (!/\/directory\/spaces|\/auth\/sessions\/me/u.test(url)) return;
  try { bodies.push(`${url.replace(SHELL, "")} → ${response.status()} ${(await response.text()).slice(0, 2500)}`); }
  catch (error) { bodies.push(`${url} → body unavailable: ${String(error).slice(0, 120)}`); }
});

try {
  await page.goto(`${SHELL}/`, { waitUntil: "domcontentloaded", timeout: 300_000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().waitFor({ state: "visible", timeout: 180_000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const workspace = page.locator("[data-semio-hub-workspace]");
  await workspace.waitFor({ state: "visible", timeout: 60_000 });
  await workspace.locator('input[type="email"]').fill(EMAIL);
  await workspace.locator('input[type="password"]').fill(PASSWORD);
  await workspace.locator('button[type="submit"][aria-label="Sign in"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 });
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click().catch(() => undefined);
  await page.waitForTimeout(SETTLE);
  for (const body of bodies) say(body);
} catch (error) {
  say(`ABORTED ${String(error).slice(0, 300)}`);
} finally {
  await browser.close();
}
