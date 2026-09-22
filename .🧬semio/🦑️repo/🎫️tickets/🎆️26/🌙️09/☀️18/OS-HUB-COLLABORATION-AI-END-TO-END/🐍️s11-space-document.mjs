#!/usr/bin/env bun
/** 🗂️ S11 — opening a HUB-BACKED DOCUMENT from inside the `s` shell (ticket 26/09/18, outcome 1's
 * last clause).
 *
 * The journey is the product's own, with no shortcut: sign in on the hub → enter a studio → open the
 * studio's SPACE INDEX (`s.space.space@1/*#editor`, the surface that lists a space's artifacts) →
 * read its rows → create one if the space is empty (`createArtifact`, the index's own verb) → open a
 * row (`openArtifact`) and wait for the document's own window.
 *
 * Every phase is reported separately, because each answers a different question and a failure in one
 * must not be readable as a failure of the others: `rowsBefore` says whether this hub's space already
 * holds documents, `created` whether the shell may add one, and `opened` whether a row becomes a live
 * document window inside `s`.
 *
 * Usage: bun 🐍️s11-space-document.mjs <baseUrl> <tag>
 */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6071/";
const tag = process.argv[3] ?? "s11";
const generated = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const EMAIL = process.env.S6_SIGN_IN_EMAIL ?? "user1@semio.dev";
const PASSWORD = process.env.S6_SIGN_IN_PASSWORD ?? "gm1-local-dev-pass-1";
const log = (...parts) => console.log("[s11]", ...parts);

const windowIds = (page) => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).filter((id) => typeof id === "string"));

async function awaitBeacon(page, deadline) {
  while (Date.now() < deadline) {
    const beacon = await page.evaluate(() => {
      const data = document.documentElement.dataset;
      if (data.semioOsReady !== undefined) return `ready:${data.semioOsReady}`;
      if (data.semioOsError !== undefined) return `error:${data.semioOsError}`;
      return null;
    });
    if (beacon !== null) return beacon;
    await page.waitForTimeout(1_000);
  }
  return null;
}

async function dismissIntroduction(page) {
  const deadline = Date.now() + 60_000;
  while (Date.now() < deadline) {
    if ((await page.locator('[data-slot="introduction-veil"]').count()) === 0) return;
    const skip = page.locator('[data-slot="introduction-veil"] button', { hasText: /skip|überspringen/iu }).first();
    if ((await skip.count()) > 0) await skip.click({ force: true }).catch(() => undefined);
    else await page.keyboard.press("Escape").catch(() => undefined);
    await page.waitForTimeout(500);
  }
}

async function openPalette(page) {
  await dismissIntroduction(page);
  await page.locator('[data-slot="navbar"]').first().click({ force: true, position: { x: 4, y: 4 } }).catch(() => undefined);
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  await page.locator("[role='dialog'] [data-slot='command-input']").first().waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
}

async function signIn(page) {
  const badge = page.locator('[data-semio-hub-sign-in=""]').first();
  if ((await badge.count()) === 0) return "no sign-in badge";
  await badge.click({ force: true }).catch(() => undefined);
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 60_000 }).catch(() => undefined);
  if ((await form.count()) === 0) return "hub workspace never opened";
  await form.locator('input[type="email"]').fill(EMAIL);
  await form.locator('input[type="password"]').fill(PASSWORD);
  await form.locator('button[type="submit"][aria-label="Sign in"]').click({ force: true }).catch(() => undefined);
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(6_000);
  return null;
}

async function spawnByCommandId(page, query, commandItemId, waitMs) {
  const before = await windowIds(page);
  await openPalette(page);
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  if ((await input.count()) === 0) return { windowIds: [], detail: "command palette never opened" };
  await input.fill(query);
  await page.waitForTimeout(2_000);
  const item = page.locator(`[data-slot="command-item"][data-command-item-id="${commandItemId}"]`).first();
  if ((await item.count()) === 0) {
    await page.keyboard.press("Escape");
    return { windowIds: [], detail: `no ${commandItemId} palette entry` };
  }
  await item.click({ force: true }).catch(() => undefined);
  const deadline = Date.now() + waitMs;
  while (Date.now() < deadline) {
    const opened = (await windowIds(page)).filter((id) => !before.includes(id));
    if (opened.length > 0) {
      await page.waitForTimeout(6_000);
      return { windowIds: (await windowIds(page)).filter((id) => !before.includes(id)), detail: null };
    }
    await page.waitForTimeout(500);
  }
  return { windowIds: [], detail: "no new window" };
}

async function unfoldActionsRail(page) {
  const toggles = page.locator('[id$=".engagement.toggle"]');
  const count = await toggles.count();
  for (let index = 0; index < count; index += 1) await toggles.nth(index).click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(1_500);
  return count;
}

/** 🗂️ Every artifact row the space index is rendering, read off the table surface itself. */
const indexRows = (page) =>
  page.evaluate(() =>
    [...document.querySelectorAll('[data-slot="window-body"] tr, [data-slot="window-body"] [role="row"], [data-slot="window-body"] [data-row-id]')]
      .map((element) => ({ id: element.getAttribute("data-row-id"), text: (element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 120) }))
      .filter((row) => row.text.length > 0)
      .slice(0, 40),
  );

const browser = await chromium.launch({ headless: process.env.S11_HEADED !== "1", args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.setDefaultNavigationTimeout(180_000);
const faults = [];
const refusals = [];
page.on("pageerror", (error) => faults.push(`pageerror: ${String(error)}`.slice(0, 240)));
page.on("console", (message) => {
  const text = message.text();
  // 🧾️ `[os-shell]` warnings too: the host's OWN fail-closed gates on `os.create-space-artifact`
  // report through `console.warn` and match none of the refusal words, so a creation the host
  // dropped looked exactly like one that silently succeeded (measured 2026-09-22).
  if (/refused:|dropped|rejected|\[os-shell\]/iu.test(text)) refusals.push(text.slice(0, 240));
});

const result = { baseUrl, tag, beacon: null, signIn: null, studio: null, index: null, railToggles: null, railRowIds: [], rowsBefore: [], stagedControls: [], filledName: null, filledKind: null, submitted: null, created: null, rowsAfter: [], opened: null, openedWindows: [], refusals: [], faults: [] };
try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  result.beacon = await awaitBeacon(page, Date.now() + 300_000);
  log(`beacon ${result.beacon}`);
  result.signIn = await signIn(page);
  log(`sign-in ${result.signIn ?? "ok"}`);
  result.studio = await spawnByCommandId(page, "studio", "spawn.space.s.space.studio@1/*#editor", 180_000);
  log(`studio ${JSON.stringify(result.studio)}`);
  result.index = await spawnByCommandId(page, "space", "spawn.space.s.space.space@1/*#editor", 120_000);
  log(`space index ${JSON.stringify(result.index)}`);
  result.rowsBefore = await indexRows(page);
  log(`rows before (${result.rowsBefore.length}) ${JSON.stringify(result.rowsBefore.slice(0, 6))}`);

  result.railToggles = await unfoldActionsRail(page);
  result.railRowIds = await page.evaluate(() => [...new Set([...document.querySelectorAll('[data-slot="window-action-pane"] [id^="action."]')].map((element) => element.id.replace(/^action\./u, "")))].filter((id) => !id.startsWith("category.") && !/\.arg\./u.test(id)));
  log(`rail (${result.railRowIds.length}) ${result.railRowIds.slice(0, 20).join(", ")}`);
  const createCursor = refusals.length;
  const createRow = page.locator('[data-slot="window-action-pane"] [id="action.createArtifact"]').first();
  if ((await createRow.count()) === 0) result.created = "no action.createArtifact row";
  else {
    await createRow.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(1_500);
    result.stagedControls = await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-action-pane"] [id*=".arg."]')].map((element) => `${element.id}|${element.tagName.toLowerCase()}|${element.getAttribute("role") ?? ""}`));
    log(`staged controls ${JSON.stringify(result.stagedControls)}`);
    // 🌳️ The staged-arg ID sits on the rail's TREE ROW (`div[role="treeitem"]`), not on the control
    // inside it — measured 2026-09-22: `action.createArtifact.arg.name|div|treeitem`. A selector that
    // demands `:is(input,textarea)` ON the id therefore finds nothing and the form reads as absent
    // while it is fully rendered. Address the row, then the control within it.
    const name = page.locator('[data-slot="window-action-pane"] [id$=".arg.name"]:is(input,textarea), [data-slot="window-action-pane"] [id$=".arg.name"] :is(input,textarea)').first();
    result.filledName = (await name.count()) > 0 ? await name.fill(`S11 Hub Document ${Date.now() % 100000}`).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 60)) : "absent";
    const kind = page.locator('[data-slot="window-action-pane"] [role="combobox"][id$=".arg.kindChoice"], [data-slot="window-action-pane"] select[id$=".arg.kindChoice"], [data-slot="window-action-pane"] [id$=".arg.kindChoice"] :is([role="combobox"],select,button)').first();
    if ((await kind.count()) > 0) {
      await kind.click({ force: true }).catch(() => undefined);
      await page.waitForTimeout(800);
      const option = page.locator('[role="option"]').first();
      result.filledKind = (await option.count()) > 0 ? await option.click({ force: true }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 60)) : "no-option";
    } else result.filledKind = "absent";
    // 🚀️ The staged form's own commit control, addressed the way `🐍️s6-all-kinds-sweep.mjs` proved:
    // the rail names it `…action.<verb>.execute`, never `submit`.
    result.submitted = "absent";
    for (const selector of ['[data-slot="window-action-pane"] [id$=".action.createArtifact.execute"]', '[id$=".action.createArtifact.execute"]', '[id$="createArtifact.execute"]', '[data-slot="window-action-pane"] [id$=".execute"]']) {
      const control = page.locator(selector).first();
      if ((await control.count()) === 0) continue;
      result.submitted = await control.click({ force: true }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 60));
      break;
    }
    log(`name=${result.filledName} kind=${result.filledKind} submit=${result.submitted}`);
    await page.waitForTimeout(10_000);
    result.created = refusals.slice(createCursor).find((line) => /createArtifact/u.test(line)) ?? "dispatched, no refusal";
  }
  log(`createArtifact → ${result.created}`);
  result.rowsAfter = await indexRows(page);
  // 🧾️ The table surface's own text, because a row selector that matches nothing and a space that
  // really holds nothing are indistinguishable from the row count alone.
  result.indexBody = await page.evaluate(() =>
    [...document.querySelectorAll('[data-slot="window-body"]')].map((body) => (body.innerText ?? "").replace(/\s+/gu, " ").trim().slice(0, 400)),
  );
  log(`rows after (${result.rowsAfter.length}) body ${JSON.stringify(result.indexBody).slice(0, 500)}`);

  const target = result.rowsAfter.find((row) => row.id !== null) ?? result.rowsAfter[1] ?? result.rowsAfter[0];
  if (target === undefined) result.opened = "space index rendered no artifact row to open";
  else {
    const before = await windowIds(page);
    const locator = target.id !== null ? page.locator(`[data-slot="window-body"] [data-row-id="${target.id}"]`).first() : page.locator('[data-slot="window-body"] [role="row"]').nth(1);
    await locator.dblclick({ force: true }).catch(() => undefined);
    await page.waitForTimeout(3_000);
    const openRow = page.locator('[data-slot="window-action-pane"] [id="action.openArtifact"]').first();
    if ((await openRow.count()) > 0) {
      await openRow.click({ force: true }).catch(() => undefined);
      await page.waitForTimeout(1_200);
      const submit = page.locator('[data-slot="window-action-pane"] [id="action.openArtifact.submit"], [data-slot="window-action-pane"] button[type="submit"]').first();
      if ((await submit.count()) > 0) await submit.click({ force: true }).catch(() => undefined);
    }
    const deadline = Date.now() + 120_000;
    while (Date.now() < deadline) {
      const opened = (await windowIds(page)).filter((id) => !before.includes(id));
      if (opened.length > 0) {
        await page.waitForTimeout(5_000);
        result.openedWindows = (await windowIds(page)).filter((id) => !before.includes(id));
        break;
      }
      await page.waitForTimeout(500);
    }
    result.opened = result.openedWindows.length > 0 ? "ok" : `row "${target.text.slice(0, 40)}" opened no new window`;
  }
  log(`open → ${result.opened} ${JSON.stringify(result.openedWindows)}`);
} catch (error) {
  result.fatal = String(error).slice(0, 300);
  log(`FATAL ${result.fatal}`);
}
result.refusals = refusals.slice(0, 12);
result.faults = faults.slice(0, 8);
await page.screenshot({ path: `${generated}s11-space-document-${tag}.png`, fullPage: false }).catch(() => undefined);
writeFileSync(`${generated}s11-space-document-${tag}.txt`, JSON.stringify(result, null, 2));
log(`=== ${generated}s11-space-document-${tag}.txt ===`);
await browser.close();
