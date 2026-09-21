/** 🇩🇪️ S10 — the framework History panel's rows in German, for U3b's open locale gap.
 *
 * Signs in, enters a studio, spawns one kind and drives one document verb so the ledger has real
 * rows, raises History, then switches the shell locale through the command palette's own
 * `os.setLocale` verb and photographs the same panel again. Prints both row sets so the screenshot
 * is backed by text, not only by pixels.
 */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";
const SHELL = process.argv[2] ?? "http://127.0.0.1:6071/";
const KIND = process.argv[3] ?? "dag";
const VERB = process.argv[4] ?? "addNode";
const TAG = process.argv[5] ?? "1";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const say = (...p) => console.log("[s10]", ...p);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const notes = [];
page.on("pageerror", (e) => notes.push(`pageerror: ${String(e.message).slice(0, 160)}`));
const rows = () => page.evaluate(() => [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((e) => !e.id.endsWith(".revert")).map((e) => (e.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 60)));
const wins = () => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((e) => e.getAttribute("data-window-id")));
const click = async (sel) => { const l = page.locator(sel).first(); if ((await l.count()) === 0) return "absent"; return l.click({ force: true, timeout: 8_000 }).then(() => "ok").catch((e) => String(e).split("\n")[0].slice(0, 60)); };
const palette = async (query) => {
  await page.locator('[data-slot="navbar"]').first().click({ force: true, position: { x: 4, y: 4 } }).catch(() => undefined);
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
  if ((await input.count()) === 0) return null;
  await input.fill(query);
  await page.waitForTimeout(1_500);
  return input;
};
try {
  await page.goto(SHELL, { waitUntil: "commit", timeout: 300_000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().waitFor({ state: "visible", timeout: 180_000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const ws = page.locator("[data-semio-hub-workspace]");
  await ws.waitFor({ state: "visible", timeout: 60_000 });
  await ws.locator('input[type="email"]').fill("user1@semio.dev");
  await ws.locator('input[type="password"]').fill("gm1-local-dev-pass-1");
  await ws.locator('button[type="submit"][aria-label="Sign in"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(60_000);
  await palette("studio");
  await click('[data-slot="command-item"][data-command-item-id="spawn.space.s.space.studio@1/*#editor"]');
  await page.waitForTimeout(12_000);
  say(`studio windows ${JSON.stringify(await wins())}`);
  const before = await wins();
  await palette(KIND);
  await click(`[data-slot="command-item"][data-command-item-id="spawn.${KIND}"]`);
  await page.waitForTimeout(20_000);
  say(`spawned ${JSON.stringify((await wins()).filter((w) => !before.includes(w)))}`);
  const toggles = page.locator('[id$=".engagement.toggle"]');
  for (let i = 0; i < (await toggles.count()); i += 1) await toggles.nth(i).click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(2_000);
  say(`verb ${await click(`[data-slot="window-action-pane"] [id="action.${VERB}"]`)}`);
  await page.waitForTimeout(4_000);
  await click('[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
  await page.waitForTimeout(3_000);
  say(`EN rows ${JSON.stringify(await rows(), null, 1)}`);
  await page.screenshot({ path: `${OUT}s10-history-en-${TAG}.png` });
  // 🌐️ The locale lives on the SETTINGS surface's language tab. The palette's `os.setLocale` row is an
  // arg-bearing shell command whose chooser renders nothing headlessly (measured: `locale chooser
  // offers []`), so the settings surface is the honest lane for a locale toggle in a probe.
  say(`settings ${await click('[id="os.openSettings"], [data-slot="navbar"] [id*="settings" i], button:has-text("Settings")')}`);
  await page.waitForTimeout(3_000);
  const controls = await page.evaluate(() => [...document.querySelectorAll('[role="dialog"] [id], [data-slot="settings"] [id], [role="tab"], select, [role="combobox"]')].map((e) => `${e.id || e.getAttribute("role")}|${(e.textContent ?? "").trim().slice(0, 24)}`).slice(0, 30));
  say(`settings controls ${JSON.stringify(controls, null, 1)}`);
  const lang = page.locator('[role="tab"], [role="button"], button').filter({ hasText: /language|sprache/iu }).first();
  if ((await lang.count()) > 0) { await lang.click({ force: true }).catch(() => undefined); await page.waitForTimeout(2_000); }
  const sel = page.locator('select, [role="combobox"]').filter({ hasText: /english|deutsch|german/iu }).first();
  say(`locale control count=${await sel.count()}`);
  if ((await sel.count()) > 0) {
    const tag = await sel.evaluate((e) => e.tagName.toLowerCase());
    if (tag === "select") await sel.selectOption("de").catch(() => undefined);
    else { await sel.click({ force: true }).catch(() => undefined); await page.waitForTimeout(800); await page.locator('[role="option"]').filter({ hasText: /deutsch|german/iu }).first().click({ force: true }).catch(() => undefined); }
  }
  await page.waitForTimeout(6_000);
  await page.keyboard.press("Escape").catch(() => undefined);
  await page.waitForTimeout(2_000);
  await click('[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
  await page.waitForTimeout(3_000);
  say(`DE rows ${JSON.stringify(await rows(), null, 1)}`);
  await page.screenshot({ path: `${OUT}s10-history-de-${TAG}.png` });
  say(`NOTES ${JSON.stringify([...new Set(notes)].slice(0, 5))}`);
} catch (error) {
  say(`ABORTED ${String(error).slice(0, 300)}`);
  await page.screenshot({ path: `${OUT}s10-history-aborted-${TAG}.png` }).catch(() => undefined);
} finally { await browser.close(); }
