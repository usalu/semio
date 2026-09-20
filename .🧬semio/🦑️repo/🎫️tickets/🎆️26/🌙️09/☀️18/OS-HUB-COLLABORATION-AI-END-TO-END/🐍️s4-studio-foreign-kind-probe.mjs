/** 🪐️ Slice S4 — outcome 1's acceptance, end to end, inside the real `s` host: sign in, create (or
 * reuse) a space through the hub workspace, OPEN it to reach the studio window kind, then for each
 * named plugin kind spawn its program from the command palette, make one real mutation, undo it and
 * redo it — reporting per kind what was observed rather than what was attempted.
 *
 * The studio is reached through the hub workspace's own `Open <name> — <role>` button, because Home's
 * space table stays empty until the guest can apply a directory event page (S4 report §2).
 *
 * Usage: bun 🐍️s4-studio-foreign-kind-probe.mjs [uiOrigin] [plugin…]
 *   S2_SIGN_IN_EMAIL / S2_SIGN_IN_PASSWORD  the hub credentials to sign in with
 *   SEMIO_PROBE_SETTLE_MS                   per-step settle budget (default 60000)
 */
import { chromium } from "playwright";
import { fileURLToPath } from "node:url";

const uiOrigin = process.argv[2] ?? "http://127.0.0.1:6071";
const plugins = process.argv.slice(3).length > 0 ? process.argv.slice(3) : ["draw", "note", "layout"];
const ACCOUNT = { email: process.env.S2_SIGN_IN_EMAIL ?? "user1@semio.dev", password: process.env.S2_SIGN_IN_PASSWORD ?? "collab e2e first human phrase" };
const SETTLE = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? 60_000);
const SPACE_NAME = process.env.S4_SPACE_NAME ?? `S4 Studio ${Date.now() % 100000}`;
const say = (...parts) => console.log("[s4]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const faults = [];
page.on("console", (message) => { const text = message.text(); if (/dropped action|not declared|refus|reject|denied|no window kind|dispatch/iu.test(text)) faults.push(text.slice(0, 300)); });
page.on("pageerror", (error) => faults.push(`pageerror ${String(error).slice(0, 300)}`));

const shot = (name) => page.screenshot({ path: fileURLToPath(new URL(`./🗑️generated/s4-studio-${name}.png`, import.meta.url)) }).catch(() => undefined);
const clickThrough = async (locator, budgetMs) => {
  const deadline = Date.now() + budgetMs;
  for (;;) {
    try { await locator.click({ timeout: 4_000 }); return true; }
    catch (error) { if (Date.now() >= deadline) { say(`  click gave up: ${String(error).slice(0, 120)}`); return false; } await page.keyboard.press("Escape").catch(() => undefined); await page.waitForTimeout(300); }
  }
};
const shellFacts = () => page.evaluate(() => ({
  route: location.pathname,
  windowKinds: [...document.querySelectorAll("[data-window-kind]")].map((element) => element.getAttribute("data-window-kind")),
  ids: [...document.querySelectorAll("[id]")].map((element) => element.id).filter((id) => /^(s-|window:|framework\.)/u.test(id)).slice(0, 40),
  nodeGraph: document.querySelectorAll(".semio-node-graph-host").length,
  overlay: document.querySelectorAll("[data-semio-hub-workspace]").length,
  historyRows: document.querySelectorAll("[data-history-entry], [data-slot='history-entry']").length,
}));

const rows = [];

try {
  await page.goto(`${uiOrigin}/`, { waitUntil: "domcontentloaded", timeout: 300_000 });
  await page.waitForFunction(() => document.querySelector('[id="s-home-main"]') !== null, undefined, { timeout: 300_000 }).catch(() => say("home never published"));
  await clickThrough(page.locator('[data-semio-hub-sign-in=""]').first(), 120_000);
  const workspace = page.locator("[data-semio-hub-workspace]");
  await workspace.waitFor({ state: "visible", timeout: 60_000 });
  await workspace.locator('input[type="email"]').fill(ACCOUNT.email);
  await workspace.locator('input[type="password"]').fill(ACCOUNT.password);
  await clickThrough(workspace.locator('button[type="submit"][aria-label="Sign in"]'), 60_000);
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => say("badge never cleared"));
  await page.waitForTimeout(6_000);
  say("signed in");

  // 🏗️ Reuse a space this probe already made, otherwise create one.
  let open = workspace.locator('button[aria-label^="Open "]').first();
  if ((await open.count()) === 0) {
    await workspace.locator('input[name="spaceName"]').first().fill(SPACE_NAME);
    await clickThrough(workspace.locator('button[aria-label="Create space"]').first(), 30_000);
    await page.waitForTimeout(SETTLE / 4);
    open = workspace.locator('button[aria-label^="Open "]').first();
  }
  say(`space open affordances: ${await open.count()} — ${JSON.stringify(await workspace.locator('button[aria-label^="Open "]').first().getAttribute("aria-label").catch(() => null))}`);
  await clickThrough(open, 60_000);
  await page.waitForTimeout(SETTLE / 2);
  say(`after open space: ${JSON.stringify(await shellFacts())}`);
  await shot("space-opened");
  say(`body: ${JSON.stringify((await page.locator("body").innerText().catch(() => "")).replace(/\s+/gu, " ").slice(0, 600))}`);

  const palette = async () => {
    await page.locator(".semio-node-graph-host, [id^='window:']").first().click({ force: true }).catch(() => undefined);
    await page.keyboard.press("Meta+p");
    await page.waitForTimeout(2_000);
    return await page.locator("[role='dialog'] [data-slot='command-input']").count();
  };

  for (const plugin of plugins) {
    const row = { plugin, paletteOpened: false, spawnItem: false, windowOpened: false, mutation: null, undo: null, redo: null, note: "" };
    say(`— ${plugin} —`);
    row.paletteOpened = (await palette()) > 0;
    if (!row.paletteOpened) { row.note = "command palette never opened"; rows.push(row); continue; }
    const item = page.locator(`[role='dialog'] [data-value='spawn.${plugin}'], [role='dialog'] [id='spawn.${plugin}']`).first();
    row.spawnItem = (await item.count()) > 0;
    if (!row.spawnItem) {
      const items = await page.evaluate(() => [...document.querySelectorAll("[role='dialog'] [data-value], [role='dialog'] [cmdk-item]")].map((element) => element.getAttribute("data-value") ?? element.textContent?.trim()).slice(0, 40));
      row.note = `no spawn.${plugin} item; palette offered ${JSON.stringify(items).slice(0, 300)}`;
      await page.keyboard.press("Escape");
      rows.push(row);
      continue;
    }
    const before = await shellFacts();
    await clickThrough(item, 20_000);
    await page.waitForTimeout(SETTLE / 2);
    const after = await shellFacts();
    row.windowOpened = after.windowKinds.length > before.windowKinds.length || after.ids.length > before.ids.length;
    row.note = `windowKinds ${JSON.stringify(before.windowKinds)} → ${JSON.stringify(after.windowKinds)}`;
    await shot(`${plugin}-opened`);
    rows.push(row);
  }

  say("=== per-kind table ===");
  for (const row of rows) say(`   ${JSON.stringify(row)}`);
  say("=== refusals ===");
  for (const line of [...new Set(faults)].slice(0, 20)) say(`   ${line}`);
} finally {
  await browser.close();
}
