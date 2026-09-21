/** 🔁️ S8 — which side of the typed-operation drain never resolves.
 *
 * Signs in to the real `s` host, waits for Home's directory bootstrap to admit its mounted
 * `applyDirectoryEventPage` operation, then reads the in-page `__s8Drain` hook the PluginRuntime
 * drain publishes (one record per poll stage, each carrying the turn/thunk scheduler state at that
 * moment). The last record names the await that never returned.
 *
 * Usage: bun 🐍️s8-drain-hook-probe.mjs [shellUrl] [email] [password] [settleMs]
 */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6071";
const EMAIL = process.argv[3] ?? "user1@semio.dev";
const PASSWORD = process.argv[4] ?? "gm1-local-dev-pass-1";
const SETTLE = Number(process.argv[5] ?? 60_000);
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const say = (...parts) => console.log("[s8]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const seen = new Map();
page.on("console", (message) => {
  const text = message.text();
  const wide = /S8PROBE|units with no change|publication attempt|typed-operation slots|directory-bootstrap|plugin_exchange actionId=applyDirectoryEventPage/u.test(text);
  if (!wide) return;
  const key = text.replace(/\d+/gu, "N").slice(0, 200);
  const count = (seen.get(key) ?? 0) + 1;
  seen.set(key, count);
  if (count <= 12) console.log(`[console:${message.type()}] ${text.slice(0, 2000)}`);
});
page.on("pageerror", (error) => console.log(`[pageerror] ${String(error).slice(0, 300)}`));

const drain = () => page.evaluate(() => (globalThis.__s8Drain ?? []).map((row) => ({ ...row })));

try {
  if (process.env.S8_DIAGNOSTICS === "1") await page.addInitScript(() => { try { globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch { /* storage-blocked realm */ } });
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
  await page.waitForTimeout(SETTLE);

  const rows = await drain();
  say(`drain records: ${rows.length}`);
  for (const row of rows) say(`  t=${row.t} inst=${row.instanceId} actor=${row.actorId} poll=${row.poll} ${row.stage}${row.detail === null ? "" : ` (${row.detail})`} turnBusy=${row.turnBusy} turnPending=${row.turnPending} thunkBusy=${row.thunkBusy} thunkPending=${row.thunkPending} cont=${row.continuations}`);

  const surface = await page.evaluate(() => ({
    bootstrap: [...document.querySelectorAll("[data-directory-bootstrap]")].map((element) => ({ kind: element.getAttribute("data-directory-bootstrap"), code: element.getAttribute("data-directory-bootstrap-code"), text: (element.textContent ?? "").slice(0, 200) })),
    homeRows: [...document.querySelectorAll('[data-window-id="s-home-main"] [data-row-id], [id="s-home-main"] [data-row-id]')].map((element) => element.getAttribute("data-row-id")),
    home: document.querySelector('[data-window-id="s-home-main"], [id="s-home-main"]')?.innerText?.replace(/\s+/gu, " ").slice(0, 600) ?? null,
  }));
  say(`STEP home: ${JSON.stringify(surface)}`);
  await page.screenshot({ path: `${OUT}s8-drain-home.png`, fullPage: false });
} catch (error) {
  say(`ABORTED ${String(error).slice(0, 400)}`);
  await page.screenshot({ path: `${OUT}s8-drain-aborted.png` }).catch(() => undefined);
} finally {
  await browser.close();
}
