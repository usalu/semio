#!/usr/bin/env bun
/** 🔬️ S2 — foreign-kind open inside the REAL `s` host (ticket 26/09/18).
 *
 * G9 §5 item 3: every live proof on this ticket so far ran in a single-plugin playground that happens to
 * mount the same `ShellHost`. This drives the actual `s` hub: from the 🪐️space home/studio, open an
 * artifact of a plugin that is NOT space, watch the lazy install fire, dispatch one mutation and one undo
 * inside it, then do the same for two more plugins — all in ONE session, no reload between them.
 *
 * Selectors are the ones `🧪️tests/🔬️catalog-smoke/🟦️.ts` already proves against this shell (command
 * palette `spawn.<pluginId>` item, `[id="<windowId>.windowControls.close"]`), so this probe adds only the
 * mutate/undo half rather than a second, drifting way to open a window.
 *
 * Usage: bun 🐍️s2-s-host-foreign-kind-probe.mjs <baseUrl> [pluginId...]
 */
import { chromium } from "playwright";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6070/";
const wanted = process.argv.length > 3 ? process.argv.slice(3) : ["draw", "note", "layout"];
const TIMEOUT_MS = Number(process.env.S2_TIMEOUT_MS ?? 300_000);
const SPAWN_MS = Number(process.env.S2_SPAWN_MS ?? 60_000);

const log = (...parts) => console.log(`[s2]`, ...parts);

/** 🚦️ The shell's own readiness beacon — the same dataset keys catalog-smoke polls. */
async function awaitBeacon(page, deadline) {
  while (Date.now() < deadline) {
    const beacon = await page.evaluate(() => {
      const data = document.documentElement.dataset;
      if (data.semioOsReady !== undefined) return `ready:${data.semioOsReady}`;
      if (data.semioOsError !== undefined) return `error:${data.semioOsError}`;
      if (data.semioOsNotFound !== undefined) return `not-found:${data.semioOsNotFound}`;
      return null;
    });
    if (beacon !== null) return beacon;
    await page.waitForTimeout(1000);
  }
  return null;
}

const windowIds = (page) => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).filter((id) => typeof id === "string"));

const readProbe = (page) => page.evaluate(() => window.__semioOsCatalogProbe ?? null);

/** 🎓️ The `s` host runs its introduction on a first visit and its veil (`[data-slot="introduction-veil"]`,
 * `pointer-events-auto`) sits over the whole shell — the playground variants pass
 * `suppressAutoIntroduction`, `s` does not. A human skips it before touching anything; until S3 this
 * probe did not, and every spawn came back `command palette never opened` because the veil owned the
 * keyboard. A dismissed tour can be followed by the next one, so this skips until the veil is gone. */
async function dismissIntroduction(page, budgetMs = 60_000) {
  const deadline = Date.now() + budgetMs;
  let skips = 0;
  while (Date.now() < deadline) {
    if ((await page.locator('[data-slot="introduction-veil"]').count()) === 0) return skips;
    const skip = page.locator('[data-slot="introduction-veil"] button', { hasText: /skip|überspringen/i }).first();
    if ((await skip.count()) > 0) await skip.click({ force: true }).catch(() => undefined);
    else await page.keyboard.press("Escape").catch(() => undefined);
    skips += 1;
    await page.waitForTimeout(500);
  }
  return skips;
}

/** ⌨️ The palette chord is **`Meta+p`**, not `Meta+K` — measured live by S3 on the real `s` host
 * (`🐍️s3-journey-diagnose.mjs`: `Meta+KeyK` opens nothing, `Meta+p` opens the dialog), and it is the
 * chord the shipped studio e2e uses too (`🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:72`). Every
 * `command palette never opened` row this probe ever reported was this one wrong key. */
async function openPalette(page) {
  await dismissIntroduction(page);
  // 🎯️ The chord is a shell binding, so the shell — not a plugin window's own editor — must own the
  // keyboard. Clicking the navbar puts focus on shell chrome without dispatching any app action.
  await page.locator('[data-slot="navbar"]').first().click({ force: true, position: { x: 4, y: 4 } }).catch(() => undefined);
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  await page.locator("[role='dialog'] [data-slot='command-input']").first().waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
}

/** 🧩️ Opens one foreign plugin's program through the shell's own command palette. */
async function spawnProgram(page, pluginId) {
  const before = await windowIds(page);
  await openPalette(page);
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  if ((await input.count()) === 0) return { windowId: null, detail: "command palette never opened" };
  await input.fill(pluginId);
  const item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${pluginId}"]`).first();
  await item.waitFor({ state: "visible", timeout: SPAWN_MS }).catch(() => undefined);
  if ((await item.count()) === 0) {
    await page.keyboard.press("Escape");
    return { windowId: null, detail: `no spawn.${pluginId} palette entry` };
  }
  await item.click();
  const deadline = Date.now() + SPAWN_MS;
  while (Date.now() < deadline) {
    const opened = (await windowIds(page)).find((id) => !before.includes(id));
    if (opened) return { windowId: opened, detail: null };
    await page.waitForTimeout(250);
  }
  return { windowId: null, detail: "no new window after spawn" };
}

/** 🎛️ A window's Actions rail lives inside its ENGAGEMENT pane and is FOLDED by default, so none of
 * its rows exist in the DOM until the pane is opened (`🪟️Window/🟦️.tsx:370-387`). Its toggle is
 * `framework.window.<segment>.engagement.toggle`; the rows are `Tree` labels
 * `tree.label.action.<actionId>` (`buildActionCategoryTree`). Every earlier reading of
 * "window exposed no action control" was this fold, not an empty app. */
async function unfoldActionsRail(page) {
  const toggles = page.locator('[id$=".engagement.toggle"]');
  const count = await toggles.count();
  for (let index = 0; index < count; index += 1) await toggles.nth(index).click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(1_500);
  return count;
}

/** 📖️ What the canvas currently reads — the engagement summary plus the window bodies. A document
 * mutation changes it, an undo restores it and a redo re-applies it, so this is what makes
 * "mutation + undo + redo" a measurement instead of three keystrokes that were sent. */
const canvasText = (page) =>
  page.evaluate(() => {
    const parts = [...document.querySelectorAll('[data-slot="window-body"], [data-slot="window-engagement-body"], [data-slot="mode-dock-stack"]')].map((element) => element.textContent ?? "");
    return (parts.length > 0 ? parts.join(" | ") : (document.body.innerText ?? "")).replace(/\s+/gu, " ").slice(0, 1200);
  });

/** ✏️ One REAL mutation + undo + redo, taken from whatever the opened window's own Actions rail
 * offers — never a hardcoded verb per plugin. The rail rows are `[data-slot="window-action-pane"]
 * [id^="action."]` (`buildActionCategoryTree`'s `action.<actionId>`); a row whose label ends with `…`
 * carries staged arguments and EXPANDS a form on its first click, whose Execute control is
 * `framework.window.<segment>.action.<actionId>.execute`.
 *
 * 🧾️ The verdict is the DOCUMENT, read off the window: a verb is a mutation when the canvas after the
 * REDO differs from the canvas after the UNDO. That is one statement covering all three — the verb
 * reached the spawned instance, its undo reached it, and its redo reached it — and it does not depend
 * on the window body being repainted on the same turn the verb settles (S5 §4.3's named residual).
 * Every refusal the shell prints for the verb is captured verbatim. */
async function mutateUndoRedo(page, windowId, refusals) {
  await unfoldActionsRail(page);
  const rows = page.locator('[data-slot="window-action-pane"] [id^="action."]');
  const count = await rows.count();
  if (count === 0) return { mutation: null, mutationDetail: "window exposed no Actions rail row after unfolding" };
  const candidates = [];
  for (let index = 0; index < count; index += 1) {
    const id = ((await rows.nth(index).getAttribute("id")) ?? "").replace(/^action\./u, "");
    if (!id || id.startsWith("category.") || id.includes(".arg.")) continue;
    // 🧹️ Destructive verbs, history verbs, and VIEW verbs (`setSelectionMode`, `setViewport`,
    // `reorganize`, …) are skipped: a view verb repaints the canvas without moving the document, so it
    // would be reported as the mutation and its undo would restore nothing. Measured on `forms`,
    // `raster`, `writer` and `flow`, whose rails all offer `setSelectionMode` before any document verb
    // (`🗑️generated/s5-foreign-5.txt`).
    if (/close|quit|delete|remove|reset|export|checkpoint|alternative|^undo$|^redo$/iu.test(id)) continue;
    if (/^set[A-Z]|selection|selectall|reorganize|viewport|zoom|^pan|^fit|^focus|^hover|granularity|^open|^toggle|^copy|^cut/iu.test(id)) continue;
    const label = ((await rows.nth(index).textContent().catch(() => "")) ?? "").trim();
    candidates.push({ index, id, staged: label.endsWith("…") || label.endsWith("...") });
  }
  let best = { mutation: null, mutationDetail: `none of ${candidates.length} rail rows moved the document`, canvasChangedByMutation: false, undoRestoredCanvas: false, redoReappliedCanvas: false };
  let tried = 0;
  for (const candidate of candidates) {
    if (tried >= 10) break;
    tried += 1;
    const cursor = refusals.length;
    const before = await canvasText(page);
    const row = rows.nth(candidate.index);
    await row.scrollIntoViewIfNeeded().catch(() => undefined);
    await row.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(1_200);
    if (candidate.staged) {
      const execute = page.locator(`[data-slot="window-action-pane"] [id$=".action.${candidate.id}.execute"]`).first();
      if ((await execute.count()) === 0) continue;
      await execute.click({ force: true }).catch(() => undefined);
      await page.waitForTimeout(2_500);
    }
    const mutated = await canvasText(page);
    await undoInWindow(page);
    const undone = await canvasText(page);
    await redoInWindow(page);
    const redone = await canvasText(page);
    const attempt = {
      mutation: candidate.id,
      mutationDetail: null,
      staged: candidate.staged,
      undoDispatched: true,
      redoDispatched: true,
      canvasChangedByMutation: mutated !== before,
      // 🧾️ The one claim that covers the whole round trip.
      documentMovedByRedoOverUndo: redone !== undone,
      undoRestoredCanvas: undone === before,
      redoReappliedCanvas: redone === mutated,
      refusals: refusals.slice(cursor, cursor + 3),
      railRowsTried: tried,
      railRows: candidates.length,
    };
    if (attempt.documentMovedByRedoOverUndo) return attempt;
    // 🧾️ Keep the STRONGEST attempt, not the first: a verb that at least moved the canvas beats one
    // that did nothing at all.
    if (best.mutation === null || (!best.canvasChangedByMutation && attempt.canvasChangedByMutation)) best = attempt;
  }
  return { ...best, railRowsTried: tried, railRows: candidates.length };
}

async function undoInWindow(page) {
  await page.keyboard.press(process.platform === "darwin" ? "Meta+KeyZ" : "Control+KeyZ");
  await page.waitForTimeout(600);
}

/** ↪️ The redo half, added by S3: an undo that cannot be redone is not a history, and outcome 1 asks
 * for the full round trip inside `s`. Same reserved shell chord the shell binds for redo. */
async function redoInWindow(page) {
  await page.keyboard.press(process.platform === "darwin" ? "Meta+Shift+KeyZ" : "Control+Shift+KeyZ");
  await page.waitForTimeout(600);
}

/** 🧾️ What the shell's own History panel currently offers, so "mutation + undo + redo" is judged by the
 * shell's recorded history rather than by the keystrokes having been sent. */
const historyState = (page) =>
  page.evaluate(() => {
    const undo = document.querySelector('[data-action-id="shell.undo"], [id$=".undo"], button[aria-label="Undo"]');
    const redo = document.querySelector('[data-action-id="shell.redo"], [id$=".redo"], button[aria-label="Redo"]');
    const rows = document.querySelectorAll('[id="framework.panel.history"] [data-row-id], [data-slot="history-row"]');
    return { undoEnabled: undo instanceof HTMLButtonElement ? !undo.disabled : null, redoEnabled: redo instanceof HTMLButtonElement ? !redo.disabled : null, historyRows: rows.length };
  });

const browser = await chromium.launch({ headless: process.env.S2_HEADED !== "1", args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const errors = [];
page.on("pageerror", (error) => errors.push(`pageerror: ${String(error)}${process.env.S2_STACKS === "1" && error.stack ? `\nSTACK ${error.stack}` : ""}`));
/** 🚦️ A dispatch the shell refuses is a `console.warn`, never an error — a probe that filters on
 * `type() === "error"` sees none of them, which is how "the verb did nothing" read as silence. */
const refusals = [];
page.on("console", (message) => {
  const text = message.text();
  if (/refused:|dropped action|rejected/iu.test(text)) refusals.push(text.slice(0, 220));
  if (message.type() === "error") errors.push(`console: ${text}`);
  if (/install|activate|plugin|s2-identity/i.test(text)) log(`[page:${message.type()}]`, text.slice(0, 300));
});

/** 🪪️ Signs one human in through the shell's own hub badge, exactly the way `🐍️c1c-s-host-probe.mjs`
 * does — same selectors, no reload — so the two probes judge the same surface. */
async function signIn(page, email, password) {
  const badge = page.locator('[data-semio-hub-sign-in=""]').first();
  if ((await badge.count()) === 0) return "no sign-in badge";
  await badge.click({ force: true }).catch(() => undefined);
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 60_000 }).catch(() => undefined);
  if ((await form.count()) === 0) return "hub workspace never opened";
  await form.locator('input[type="email"]').fill(email);
  await form.locator('input[type="password"]').fill(password);
  await form.locator('button[type="submit"][aria-label="Sign in"]').click({ force: true }).catch(() => undefined);
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
  await page.locator("[data-semio-hub-workspace] button[aria-label]").first().click({ force: true }).catch(() => undefined);
  // 🏠️ Opening the hub workspace navigates a host-mode shell to `/hub` and leaves the overlay over the
  // whole shell, so a probe that signs in and then reaches for shell chrome is reaching THROUGH an
  // overlay. `applyShellUri` closes the workspace for any path that is not `/hub`, so popping that
  // history entry is the shell's own way back to the landing route — no reload, session kept
  // (measured by S3, `🐍️s3-journey-diagnose.mjs`: `route=/ homeSurface=1 overlay=0`).
  await page.goBack({ waitUntil: "commit" }).catch(() => undefined);
  await page
    .waitForFunction(() => document.querySelector('[id="s-home-main"]') !== null || document.querySelectorAll(".semio-table-host").length > 0, undefined, { timeout: 180_000 })
    .catch(() => undefined);
  await page.waitForTimeout(5_000);
  return null;
}

/** 🏗️ Home cannot spawn: `spawnApp` is declared by the STUDIO app (`s.space.studio@1/*#editor`), and a
 * dispatch from `s-home-main` is dropped with `no window kind declares it` (measured by S3). So the
 * journey for outcome 1 is Home → studio → palette → spawn, and this step is the middle leg.
 *
 * It takes the LOCAL path: `createStudio` mints an ephemeral studio and answers `Effect::Navigate`
 * to `/spaces/<id>` with no hub involved — `🎮️commands/🌱create-space/🦀️.rs` states that contract.
 * That verb was `InteractiveJobClassification::BatchOnlyPendingRewrite`, i.e. refused by
 * `validate_ui_dispatch_classification` in every shell, until ticket 26/09/18 S4 migrated it onto
 * Home's retained tool factory. A hub space is deliberately NOT used here: entering one reaches the
 * space INDEX, whose artifact creation needs the hub's `artifactAuthority` (503 `catalog-unavailable`
 * without a published trusted catalog). */
async function enterStudio(page, budgetMs = 180_000) {
  const route = () => page.evaluate(() => window.location.pathname);
  const windows = () => windowIds(page);
  if ((await windows()).some((id) => /studio/iu.test(id))) return { route: await route(), detail: "already in a studio" };
  await openPalette(page);
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  if ((await input.count()) === 0) return { route: await route(), detail: "command palette never opened" };
  // 🔭️ Measured: the palette lists all three `🪐️space` apps under the SAME id `spawn.space`
  // (`Spawn semio · s · home`, `… s · space · index`, `… s · studio`), so the studio is identified by
  // its label, not by the id every other plugin is found with. `createStudio` is NOT in the palette
  // and Home publishes no `[data-action-id]` control at all, so this is the one reachable route.
  const item = page.locator('[data-slot="command-item"]').filter({ hasText: /s\s*·\s*studio/iu }).first();
  if ((await item.count()) === 0) {
    const offered = await page.evaluate(() => [...document.querySelectorAll('[data-slot="command-item"]')].map((element) => `${element.getAttribute("data-command-item-id")}|${element.textContent?.trim()?.slice(0, 40)}`).slice(0, 40));
    await page.keyboard.press("Escape");
    return { route: await route(), detail: `no studio palette entry; offered ${JSON.stringify(offered)}` };
  }
  const before = await windows();
  await item.click({ force: true }).catch(() => undefined);
  const deadline = Date.now() + budgetMs;
  while (Date.now() < deadline) {
    const opened = (await windows()).find((id) => !before.includes(id));
    if (opened) {
      await page.waitForTimeout(8_000);
      return { route: await route(), windowId: opened, detail: null };
    }
    await page.waitForTimeout(500);
  }
  return { route: await route(), detail: "studio spawn dispatched but no new window opened" };
}

const result = { baseUrl, beacon: null, shellPluginId: null, installedBefore: [], rows: [], errors: [] };
try {
  const deadline = Date.now() + TIMEOUT_MS;
  log(`navigating to ${baseUrl}`);
  await page.goto(baseUrl, { waitUntil: "commit", timeout: TIMEOUT_MS });
  result.beacon = await awaitBeacon(page, deadline);
  log(`beacon: ${result.beacon ?? "none"}`);
  // 🚦️ `error:s` is NOT a dead shell on a hub-backed host: `🪐️space`'s Home refuses its surface until a
  // human is signed in, and that refusal reaches the beacon. The chrome (badge, palette) is mounted and
  // signing in is exactly what clears it, so only a shell that never set ANY beacon is fatal here.
  if (result.beacon === null) throw new Error("shell never set a readiness beacon");

  if (process.env.S2_SIGN_IN_EMAIL) {
    result.signIn = await signIn(page, process.env.S2_SIGN_IN_EMAIL, process.env.S2_SIGN_IN_PASSWORD ?? "");
    result.homeSurfaceAfterSignIn = await page.evaluate(() => document.querySelector('[id="s-home-main"]') !== null || document.querySelectorAll(".semio-table-host").length > 0);
    log(`sign-in: ${result.signIn ?? "ok"}; host surface published: ${result.homeSurfaceAfterSignIn}`);
  }

  result.studio = await enterStudio(page);
  result.studioSurfaces = await page.evaluate(() => ({
    nodeGraph: document.querySelectorAll(".semio-node-graph-host").length,
    windowIds: [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")),
    title: (document.body.innerText ?? "").replace(/\s+/gu, " ").slice(0, 200),
  }));
  log(`studio: ${JSON.stringify(result.studio)} surfaces: ${JSON.stringify(result.studioSurfaces)}`);

  const probe = await readProbe(page);
  if (probe === null) throw new Error("shell exposed no window.__semioOsCatalogProbe");
  result.shellPluginId = probe.shellPluginId;
  result.installedBefore = probe.plugins.filter((row) => row.status === "loaded").map((row) => row.pluginId);
  log(`shell ${probe.shellPluginId}: ${probe.plugins.length} registry rows, ${result.installedBefore.length} already loaded, ${probe.programs.length} spawnable programs`);

  for (const pluginId of wanted) {
    const started = Date.now();
    const cursor = errors.length;
    const loadedBefore = result.installedBefore.includes(pluginId);
    const spawned = await spawnProgram(page, pluginId);
    const after = await readProbe(page);
    const status = after?.plugins.find((row) => row.pluginId === pluginId)?.status ?? "absent";
    const row = {
      pluginId,
      loadedBeforeOpen: loadedBefore,
      lazyInstalled: !loadedBefore && status === "loaded",
      statusAfterOpen: status,
      windowId: spawned.windowId,
      openDetail: spawned.detail,
      mutation: null,
      mutationDetail: null,
      undoDispatched: false,
      redoDispatched: false,
      canvasChangedByMutation: null,
      undoRestoredCanvas: null,
      redoReappliedCanvas: null,
      afterMutation: null,
      afterUndo: null,
      afterRedo: null,
      openMs: Date.now() - started,
      firstError: errors[cursor] ?? null,
    };
    if (spawned.windowId) {
      Object.assign(row, await mutateUndoRedo(page, spawned.windowId, refusals));
      row.afterRedo = await historyState(page);
    }
    row.firstError = errors[cursor] ?? null;
    result.rows.push(row);
    log(JSON.stringify(row));
  }
  // 🔗️ G9 §3's still-open question: `HubWorkspace` and the `/hub` route have only ever been observed in
  // the `animate` single-plugin playground, which by construction cannot enter `applyShellUri`'s
  // host-mode branch. This is that branch, inside the real `s` host, in the same session.
  const hub = { route: null, workspaceMounted: false, signInVisible: false, spacesVisible: false, badgeVisible: false, detail: null };
  try {
    hub.badgeVisible = (await page.locator("#s-presence-peers, [data-hub-connection], [id*='hubConnection']").count()) > 0;
    await page.goto(`${baseUrl.replace(/\/$/, "")}/hub`, { waitUntil: "commit", timeout: 60_000 }).catch(() => undefined);
    const beacon = await awaitBeacon(page, Date.now() + 120_000);
    hub.route = await page.evaluate(() => window.location.pathname);
    await page.waitForTimeout(2500);
    const text = await page.locator("body").innerText().catch(() => "");
    hub.workspaceMounted = /hub|space/i.test(text);
    hub.signInVisible = (await page.locator("input[type='email'], input[name='email']").count()) > 0 || /sign in|anmelden/i.test(text);
    hub.spacesVisible = /spaces|räume/i.test(text);
    hub.detail = `beacon ${beacon ?? "none"}; body starts: ${text.slice(0, 200).replace(/\s+/g, " ")}`;
  } catch (error) {
    hub.detail = `hub step failed: ${String(error)}`;
  }
  result.hub = hub;
  log(`hub: ${JSON.stringify(hub)}`);
} catch (error) {
  result.fatal = String(error);
  log(`FATAL ${result.fatal}`);
} finally {
  result.errors = errors.slice(0, 40);
  result.refusals = [...new Set(refusals)].slice(0, 20);
  await browser.close();
}

console.log("=== S2 FOREIGN-KIND PROBE RESULT ===");
console.log(JSON.stringify(result, null, 2));
process.exit(result.fatal || result.rows.some((row) => row.windowId === null) ? 1 : 0);
