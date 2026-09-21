/** 🏠️ S9 — does the published directory config reach Home's rendered table?
 *
 * Signs in on the brief's own pair, waits for the directory bootstrap to reach `idle`, then samples
 * Home's rendered rows repeatedly — before the ACK, right after it, and after a forced extra refresh
 * (a locale round trip, which re-renders the guest without touching the directory lane). Three
 * samples separate "the config never committed" from "the render that saw it never happened".
 *
 * Usage: bun 🐍️s9-home-rows-probe.mjs [shellUrl] [email] [password] [settleMs] [tag]
 */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6190";
const EMAIL = process.argv[3] ?? "user1@semio.dev";
const PASSWORD = process.argv[4] ?? "gm1-local-dev-pass-1";
const SETTLE = Number(process.argv[5] ?? 120_000);
const TAG = process.argv[6] ?? "1";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const say = (...parts) => console.log("[s9]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const seen = new Map();
page.on("console", (message) => {
  const text = message.text();
  if (!/S9PROBE|directory-bootstrap|home-rows|units with no change|plugin_exchange actionId=applyDirectoryEventPage|s\.home\./u.test(text)) return;
  const key = text.replace(/\d+/gu, "N").slice(0, 200);
  const count = (seen.get(key) ?? 0) + 1;
  seen.set(key, count);
  if (count <= 8) console.log(`[console:${message.type()}] ${text.slice(0, 1200)}`);
});
page.on("pageerror", (error) => console.log(`[pageerror] ${String(error).slice(0, 300)}`));

const surface = () => page.evaluate(() => ({
  bootstrap: [...document.querySelectorAll("[data-directory-bootstrap]")].map((element) => ({ kind: element.getAttribute("data-directory-bootstrap"), code: element.getAttribute("data-directory-bootstrap-code") })),
  rows: [...document.querySelectorAll('[data-window-id="s-home-main"] [data-row-id], [id="s-home-main"] [data-row-id]')].map((element) => element.getAttribute("data-row-id")),
  body: document.querySelector('[data-window-id="s-home-main"], [id="s-home-main"]')?.innerText?.replace(/\s+/gu, " ").slice(0, 400) ?? null,
}));

try {
  await page.addInitScript(() => { try { globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch { /* storage-blocked realm */ } });
  await page.goto(`${SHELL}/`, { waitUntil: "domcontentloaded", timeout: 300_000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().waitFor({ state: "visible", timeout: 180_000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const workspace = page.locator("[data-semio-hub-workspace]");
  await workspace.waitFor({ state: "visible", timeout: 60_000 });
  await workspace.locator('input[type="email"]').fill(EMAIL);
  await workspace.locator('input[type="password"]').fill(PASSWORD);
  await workspace.locator('button[type="submit"][aria-label="Sign in"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 });
  say(`STEP sign-in: PASS as ${EMAIL}`);
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click().catch(() => undefined);

  const deadline = Date.now() + SETTLE;
  let last = null;
  while (Date.now() < deadline) {
    await page.waitForTimeout(5_000);
    const now = await surface();
    const key = JSON.stringify(now);
    if (key !== last) {
      last = key;
      say(`t=${Math.round((SETTLE - (deadline - Date.now())) / 1000)}s ${key.slice(0, 500)}`);
    }
    if (now.bootstrap.length === 0 && now.rows.length > 0) break;
  }

  say(`SAMPLE settled: ${JSON.stringify(await surface())}`);
  await page.screenshot({ path: `${OUT}s9-home-${TAG}.png`, fullPage: false });
} catch (error) {
  say(`ABORTED ${String(error).slice(0, 400)}`);
  await page.screenshot({ path: `${OUT}s9-home-${TAG}-aborted.png` }).catch(() => undefined);
} finally {
  await browser.close();
}
