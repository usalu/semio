#!/usr/bin/env bun
/** 🗂️ S15 — the hub-catalog document journey inside `s` (adapted from S12's probe, ticket 26/09/18), instrumented for the
 * execution-target lane: every `/_semio/hub` request (path, status, bytes) and every execution-target status notice
 * (`[data-semio-execution-target-status]`, stage, progress) the shell shows while it fetches the document's actor
 * from the hub catalog.
 *
 * Original S12 header:
 *
 * S11 §5.4 proved the last hop fails when the space index is SPAWNED from the command palette: the
 * host's `os.create-space-artifact` route runs only when exactly one open document session belonging
 * to the shell's OWN base session is scoped to the space index, and a palette spawn is a different
 * instance. The shell mounts the real one on a bare `/spaces/{id}` route
 * (`🏛️ShellHost/🟦️.tsx:6602`), and the gesture that navigates there is the space row in the hub
 * workspace's own Space Browser (`🏘️SpaceBrowser/🟦️.tsx:267`). This probe takes that route and no
 * other.
 *
 * Journey: sign in → space browser → OPEN the space (navigation, not a spawn) → the space index
 * mounts as the base session → create a document (`createArtifact`, kind picked from the hub's own
 * catalog) → open the row → mutate it → undo. Every phase is reported separately.
 *
 * Usage: bun 🐍️s12-hub-document.mjs <baseUrl> <tag> [kindFilter] [verb]
 */
import { chromium } from "playwright";
import { mutateUndoRedo as sweepMutateUndoRedo, readShell as sweepReadShell } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6072/";
const tag = process.argv[3] ?? "s12";
const kindFilter = process.argv[4] ?? "note";
const verb = process.argv[5] ?? "addBlock";
const generated = fileURLToPath(new URL("./generated/", import.meta.url));
const EMAIL = process.env.S6_SIGN_IN_EMAIL ?? "user1@semio.dev";
const PASSWORD = process.env.S6_SIGN_IN_PASSWORD ?? "gm1-local-dev-pass-1";
const log = (...parts) => console.log("[s15-hub]", ...parts);

const windowIds = (page) => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).filter((id) => typeof id === "string"));

/** 🧯️ Every transient notice the shell raises, recorded as it appears — it auto-dismisses after
 * 4 000 ms, so polling for it loses the very refusal this slice made visible. */
const installNoticeRecorder = (page) =>
  page.addInitScript(() => {
    const seen = [];
    globalThis.__s12Notices = seen;
    const scan = () => {
      for (const element of document.querySelectorAll("[data-semio-transient-notice]")) {
        const row = { code: element.getAttribute("data-notice-code") ?? "", text: (element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 160) };
        if (!seen.some((other) => other.code === row.code && other.text === row.text)) seen.push(row);
      }
      for (const element of document.querySelectorAll("[data-semio-artifact-creation]")) {
        const row = { code: `creation:${element.getAttribute("data-semio-artifact-creation") ?? ""}`, text: (element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 160) };
        if (!seen.some((other) => other.code === row.code && other.text === row.text)) seen.push(row);
      }
      for (const element of document.querySelectorAll("[data-semio-execution-target-status]")) {
        const progress = element.querySelector("progress");
        const row = { code: `execution-target:${element.getAttribute("data-semio-execution-target-stage") ?? element.getAttribute("role")}`, text: `${(element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 120)}${progress ? ` [${progress.getAttribute("value")}/${progress.getAttribute("max")}]` : ""}` };
        if (!seen.some((other) => other.code === row.code && other.text === row.text)) seen.push(row);
      }
      for (const element of document.querySelectorAll("[data-semio-plugin-install]")) {
        const progress = element.querySelector("[data-semio-plugin-install-progress]");
        const row = { code: `plugin-install:${element.getAttribute("data-plugin-install-ids") ?? ""}`, text: `${(element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 120)}${progress ? ` [${progress.getAttribute("value")}/${progress.getAttribute("max")}]` : ""}` };
        if (!seen.some((other) => other.code === row.code && other.text === row.text)) seen.push(row);
      }
      for (const element of document.querySelectorAll('[role="dialog"]')) {
        const row = { code: "dialog", text: (element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 160) };
        if (!seen.some((other) => other.code === row.code && other.text === row.text)) seen.push(row);
      }
    };
    const start = () => {
      scan();
      new MutationObserver(scan).observe(document.body, { subtree: true, childList: true, attributes: true });
    };
    if (document.body) start();
    else document.addEventListener("DOMContentLoaded", start);
  });

const notices = (page) => page.evaluate(() => globalThis.__s12Notices ?? []);

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
  await page.waitForTimeout(6_000);
  return null;
}

/** 🏘️ The space rows the hub workspace lists, and the gesture that OPENS one — the only lane that
 * makes the shell mount the space index as its own base session. */
const spaceRows = (page) => page.evaluate(() => [...document.querySelectorAll("[data-semio-hub-workspace] li[data-space-id]")].map((row) => ({ id: row.getAttribute("data-space-id"), text: (row.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 80) })));

async function ensureSpace(page) {
  let rows = await spaceRows(page);
  const own = new RegExp(process.env.S12_SPACE ?? "S15 Space", "u");
  if (rows.some((row) => own.test(row.text))) return rows;
  const name = page.locator('[data-element-alias="os.hub.spaces.createName"], #os\\.hub\\.spaces\\.createName').first();
  if ((await name.count()) === 0) return rows;
  await name.fill(`S15 Space ${Date.now() % 100000}`).catch(() => undefined);
  await page.locator('#os\\.hub\\.spaces\\.createSubmit, [id="os.hub.spaces.createSubmit"]').first().click({ force: true }).catch(() => undefined);
  const deadline = Date.now() + 90_000;
  while (Date.now() < deadline) {
    rows = await spaceRows(page);
    if (rows.some((row) => own.test(row.text))) return rows;
    await page.waitForTimeout(2_000);
  }
  return rows;
}

const indexRows = (page) =>
  page.evaluate(() =>
    [...document.querySelectorAll('[data-slot="window-body"] tr, [data-slot="window-body"] [role="row"], [data-slot="window-body"] [data-row-id]')]
      .map((element) => ({ id: element.getAttribute("data-row-id"), text: (element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 120) }))
      .filter((row) => row.text.length > 0)
      .slice(0, 40),
  );

const readShell = (page) =>
  page.evaluate(() => {
    const text = (element) => (element?.innerText ?? "").replace(/\s+/gu, " ").trim();
    const checkin = document.querySelector("#s-checkin");
    return {
      uri: `${window.location.pathname}${window.location.search}`,
      windowIds: [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).filter((id) => typeof id === "string"),
      surfaces: [...document.querySelectorAll("[data-surface-id]")].map((element) => element.getAttribute("data-surface-id")),
      actions: [...new Set([...document.querySelectorAll('[data-slot="window-action-pane"] [id^="action."]')].map((element) => element.id))].filter((id) => !id.startsWith("action.category.") && !/\.arg\./u.test(id)),
      ledger: [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((element) => !element.id.endsWith(".revert")).map((element) => `${element.id}:${text(element).slice(0, 40)}`),
      checkin: checkin === null ? null : text(checkin),
    };
  });

const editCount = (shell) => {
  const match = /\((\d+)\)\s*$/u.exec(shell.checkin ?? "");
  return match === null ? (shell.checkin === null ? -1 : 0) : Number(match[1]);
};

const click = async (page, selector) => {
  const control = page.locator(selector).first();
  if ((await control.count()) === 0) return "absent";
  return control.click({ force: true }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 70));
};

/** 🎛️ Presses only the engagement chips that are actually FOLDED — the chip is a toggle and the
 * shell persists its state per window, so pressing blindly closes a rail that is already open
 * (`🐍️s6-all-kinds-sweep.mjs` carries the same rule and the measurement behind it). */
async function unfoldActionsRail(page) {
  const rows = () => page.locator('[data-slot="window-action-pane"] [id^="action."]').count();
  const pressed = new Set();
  const deadline = Date.now() + 25_000;
  while ((await rows()) === 0 && Date.now() < deadline) {
    let ids = await page.evaluate(() => [...document.querySelectorAll('[id$=".engagement.toggle"]')].filter((toggle) => toggle.closest('[data-folded="true"]') !== null).map((toggle) => toggle.id));
    if (ids.length === 0) ids = (await page.evaluate(() => [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((toggle) => toggle.id))).filter((id) => !pressed.has(id));
    for (const id of ids) {
      pressed.add(id);
      await page.locator(`[id="${id}"]`).first().click({ force: true }).catch(() => undefined);
    }
    await page.waitForTimeout(700);
  }
  return pressed.size;
}

async function raiseHistory(page) {
  const open = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"]')].some((element) => element instanceof HTMLElement && element.offsetParent !== null && /framework\.panel\.history/u.test(element.id)));
  if (open) return "already-open";
  return click(page, '[data-slot="panel-tab-button"][id="framework.panelTab.framework.panel.history"], [id="framework.panelTab.framework.panel.history"]');
}

async function neutralDispatch(page) {
  await click(page, '[data-slot="panel-tab-button"][id="framework.panelTab.framework.panel.history"]');
  await page.waitForTimeout(900);
  await click(page, '[data-slot="panel-tab-button"][id="framework.panelTab.framework.panel.history"]');
  await page.waitForTimeout(900);
}

/** 💾️ `S15_PROFILE_DIR=<dir>` runs in one persistent browser profile, so a second run is a later session on the
 * same device: its storage and HTTP cache are the ones the first run left. */
const persistent = process.env.S15_PROFILE_DIR;
const launchArgs = ["--use-angle=metal", ...(process.env.S15_NETLOG ? [`--log-net-log=${process.env.S15_NETLOG}`, "--net-log-capture-mode=Default"] : [])];
const browser = persistent ? null : await chromium.launch({ headless: process.env.S12_HEADED !== "1", args: launchArgs });
const context = persistent
  ? await chromium.launchPersistentContext(persistent, { headless: process.env.S12_HEADED !== "1", args: launchArgs, viewport: { width: 1600, height: 1000 } })
  : await browser.newContext({ viewport: { width: 1600, height: 1000 } });
const page = context.pages()[0] ?? (await context.newPage());
await installNoticeRecorder(page);
/** 🚫️ `S15_BLOCK_PLUGIN=<dir>` answers 404 for every locally staged module of that plugin — the served lane then holds
 * no local copy of it, exactly like a plugin that was never staged on this device. */
if (process.env.S15_BLOCK_PLUGIN) {
  const blocked = `/🔌️plugin-modules/${process.env.S15_BLOCK_PLUGIN}/`;
  await context.route((url) => decodeURIComponent(url.pathname).includes(blocked), (route) => route.fulfill({ status: 404, body: "not staged on this device" }));
  /** 📡️ A device that never staged the plugin never announces it either: the dev watch stream's connect-time
   * snapshot is answered from the activation receipt without that plugin (`S15_BLOCK_PLUGIN_ID`). */
  if (process.env.S15_BLOCK_PLUGIN_ID) {
    const receipt = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/react/dev/s/activation/🔣️receipt.json", "utf8"));
    const snapshot = { kind: "snapshot", plugins: receipt.plugins.filter((row) => row.pluginId !== process.env.S15_BLOCK_PLUGIN_ID).map((row) => ({ pluginId: row.pluginId, rebuiltAt: row.rebuiltAt })) };
    await context.route((url) => decodeURIComponent(url.pathname) === "/🔌️plugin-modules/watch", (route) => route.fulfill({ status: 200, headers: { "content-type": "text/event-stream", "cache-control": "no-store" }, body: `retry: 600000\ndata: ${JSON.stringify(snapshot)}\n\n` }));
  }
}
page.setDefaultNavigationTimeout(180_000);
const faults = [];
const shellLines = [];
const hubRequests = [];
const moduleTraffic = { phase: "first", first: { requests: 0, bytes: 0 }, reopen: { requests: 0, bytes: 0 } };
page.on("requestfinished", async (request) => {
  const url = decodeURIComponent(request.url());
  if (!/\/_semio\/hub\/trusted-catalog\/plugin-modules\/[0-9a-f]{64}\//u.test(url)) return;
  const sizes = await request.sizes().catch(() => ({ responseBodySize: 0 }));
  const response = await request.response().catch(() => null);
  const fromCache = response !== null && response.status() === 200 && sizes.responseBodySize === 0;
  moduleTraffic[moduleTraffic.phase].requests += 1;
  moduleTraffic[moduleTraffic.phase].bytes += sizes.responseBodySize;
  if (fromCache) moduleTraffic[moduleTraffic.phase].cached = (moduleTraffic[moduleTraffic.phase].cached ?? 0) + 1;
});
page.on("response", (response) => {
  const url = decodeURIComponent(response.url());
  if (/\/_semio\/hub\/|plugin-modules/u.test(url)) hubRequests.push(`${response.status()} ${response.request().method()} ${url.replace(/^https?:\/\/[^/]+/u, "")} ${response.headers()["content-length"] ?? "?"}`.slice(0, 220));
});
page.on("pageerror", (error) => faults.push(`pageerror: ${String(error)}`.slice(0, 240)));
const consoleAll = [];
page.on("console", (message) => {
  const text = message.text();
  if (process.env.S15_CONSOLE_ALL === "1" && !/\[vite\]|transform freshness/u.test(text)) consoleAll.push(`${message.type()} ${text}`.slice(0, 500));
  if (/refused:|dropped|rejected|\[os-shell\]|program load failed|program module unavailable|plugin install|no surface registered|descriptor/iu.test(text)) shellLines.push(`${message.type()} ${text}`.slice(0, 400));
});

const result = { baseUrl, tag, kindFilter, verb, beacon: null, signIn: null, spaces: [], openedSpace: null, uriAfterOpen: null, indexWindows: [], indexSurfaces: [], rowsBefore: [], railRowIds: [], stagedControls: [], kindOptions: [], filledName: null, filledKind: null, submitted: null, createOutcome: null, rowsAfter: [], opened: null, openedWindows: [], documentVerb: null, edits: [], ledgerTail: [], notices: [], shellLines: [], faults: [] };
try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  result.beacon = await awaitBeacon(page, Date.now() + 300_000);
  log(`beacon ${result.beacon}`);
  await dismissIntroduction(page);
  result.censusAtBoot = await page.evaluate(() => (window.__semioOsCatalogProbe?.plugins ?? []).filter((row) => row.status !== "loaded").map((row) => `${row.pluginId}:${row.status}`)).catch(() => null);
  log(`not loaded at boot ${JSON.stringify(result.censusAtBoot)}`);
  result.signIn = await signIn(page);
  log(`sign-in ${result.signIn ?? "ok"}`);

  result.spaces = await ensureSpace(page);
  log(`spaces (${result.spaces.length}) ${JSON.stringify(result.spaces.slice(0, 4))}`);
  await page.screenshot({ path: `${generated}s15-hub-document-${tag}-spaces.png` }).catch(() => undefined);
  if (result.spaces.length === 0) throw new Error("hub workspace listed no space to open");

  const spaceId = (result.spaces.find((row) => new RegExp(process.env.S12_SPACE ?? "S15 Space", "u").test(row.text)) ?? result.spaces[0]).id;
  const openOutcome = await click(page, `[data-semio-hub-workspace] li[data-space-id="${spaceId}"] button`);
  result.openedSpace = `${spaceId} ${openOutcome}`;
  log(`open space → ${result.openedSpace}`);

  const deadline = Date.now() + 180_000;
  let shell = await readShell(page);
  while (Date.now() < deadline) {
    shell = await readShell(page);
    if (shell.uri.includes(`/spaces/${spaceId}`) && shell.windowIds.some((id) => id !== "s-home-main")) break;
    await page.waitForTimeout(2_000);
  }
  result.uriAfterOpen = shell.uri;
  result.indexWindows = shell.windowIds;
  result.indexSurfaces = shell.surfaces;
  log(`uri ${result.uriAfterOpen} windows ${JSON.stringify(result.indexWindows)}`);
  await page.waitForTimeout(8_000);
  result.rowsBefore = await indexRows(page);
  log(`rows before (${result.rowsBefore.length})`);
  await page.screenshot({ path: `${generated}s15-hub-document-${tag}-index.png` }).catch(() => undefined);

  await unfoldActionsRail(page);
  result.railRowIds = (await readShell(page)).actions.map((id) => id.replace(/^action\./u, ""));
  log(`rail (${result.railRowIds.length}) ${result.railRowIds.slice(0, 16).join(", ")}`);

  const noticeCursor = (await notices(page)).length;
  const createRow = page.locator('[data-slot="window-action-pane"] [id="action.createArtifact"]').first();
  if ((await createRow.count()) === 0) result.createOutcome = "no action.createArtifact row";
  else {
    await createRow.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(2_000);
    if (process.env.S15_DUMP_STAGED === "1") {
      result.stagedDom = await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-action-pane"] *')]
        .filter((element) => element.id || element.getAttribute("role") || element.hasAttribute("aria-expanded") || element.hasAttribute("data-state"))
        .map((element) => `${element.tagName.toLowerCase()}#${element.id}|role=${element.getAttribute("role") ?? ""}|exp=${element.getAttribute("aria-expanded") ?? ""}|state=${element.getAttribute("data-state") ?? ""}|vis=${element.offsetParent !== null}|${(element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 40)}`)
        .filter((line) => /createArtifact|CREATE|Create Artifact|kind|Kind|arg\./u.test(line))
        .slice(0, 80));
    }
    result.expandedStaged = await page.evaluate(() => {
      const collapsed = [...document.querySelectorAll('[data-slot="window-action-pane"] [aria-expanded="false"]')].filter((element) => /create artifact/iu.test(element.textContent ?? ""));
      for (const element of collapsed) element.click();
      return collapsed.length;
    }).catch(() => -1);
    await page.waitForTimeout(800);
    result.stagedControls = await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-action-pane"] [id*=".arg."]')].map((element) => `${element.id}|${element.tagName.toLowerCase()}|${element.getAttribute("role") ?? ""}`));
    const name = page.locator('[data-slot="window-action-pane"] [id$=".arg.name"]:is(input,textarea), [data-slot="window-action-pane"] [id$=".arg.name"] :is(input,textarea)').first();
    result.filledName = (await name.count()) > 0 ? await name.fill(`S12 Hub Document ${Date.now() % 100000}`).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 60)) : "absent";
    const kind = page.locator('[data-slot="window-action-pane"] [role="combobox"][id$=".arg.kindChoice"], [data-slot="window-action-pane"] select[id$=".arg.kindChoice"], [data-slot="window-action-pane"] [id$=".arg.kindChoice"] :is([role="combobox"],select,button)').first();
    if ((await kind.count()) > 0) {
      await kind.click({ force: true }).catch(() => undefined);
      await page.waitForTimeout(1_500);
      result.kindOptions = await page.evaluate(() => [...document.querySelectorAll('[role="option"]')].map((element) => (element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 60)));
      if (process.env.S15_DUMP_STAGED === "1") result.kindPopup = await page.evaluate(() => [...document.querySelectorAll('[role="listbox"], [role="option"], [role="menu"], [role="menuitem"], [role="dialog"], [data-radix-popper-content-wrapper], [cmdk-item], [data-slot*="select"], [data-slot*="combobox"]')].map((element) => `${element.tagName.toLowerCase()}|role=${element.getAttribute("role") ?? ""}|slot=${element.getAttribute("data-slot") ?? ""}|vis=${element.getClientRects().length > 0}|${(element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 50)}`).slice(0, 40));
      // 🏷️ The hub labels every creation kind "Editor" (it sends the app ROLE label, not the kind's
      // own — measured 2026-09-22 on 7651: two kinds, two identical labels), so a label filter cannot
      // pick one. The catalog's own ORDER is a contract (`parseSpaceArtifactCreationCatalogV1` rejects
      // a non-ascending `kindId` list), so the index is the addressable thing.
      const optionCount = await page.locator('[role="option"]').count();
      const wantedIndex = process.env.S12_KIND_INDEX === undefined ? optionCount - 1 : Number(process.env.S12_KIND_INDEX);
      const option = page.locator('[role="option"]').nth(Math.max(0, Math.min(optionCount - 1, wantedIndex)));
      result.filledKind = (await option.count()) > 0 ? await option.click({ force: true }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 60)) : "no-option";
    } else result.filledKind = "absent";
    result.stagedState = await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-action-pane"] [id*="createArtifact"]')].map((element) => `${element.id}|${element.tagName.toLowerCase()}|disabled=${element.hasAttribute("disabled") || element.getAttribute("aria-disabled") === "true"}|${(element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 40)}`));
    result.submitted = "absent";
    for (const selector of ['[data-slot="window-action-pane"] [id$=".action.createArtifact.execute"]', '[id$=".action.createArtifact.execute"]', '[id$="createArtifact.execute"]']) {
      const outcome = await click(page, selector);
      if (outcome !== "absent") { result.submitted = outcome; break; }
    }
    log(`name=${result.filledName} kind=${result.filledKind} options=${JSON.stringify(result.kindOptions)} submit=${result.submitted}`);
    await page.waitForTimeout(3_000);
    // 🗨️ The guest answers an empty name or kind with `Effect::OpenDialog` rather than a refusal
    // (`🌱create-artifact/🦀️.rs:24`), so a dialog on screen IS the diagnosis that the staged payload
    // arrived empty — and it is also the lane the collaboration e2e drives, so the probe finishes it.
    result.dialog = await page.evaluate(() => {
      const dialog = document.querySelector('[role="dialog"]');
      return dialog === null ? null : { text: (dialog.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 200), ids: [...dialog.querySelectorAll("[id]")].map((element) => element.id).slice(0, 20) };
    });
    if (result.dialog !== null) {
      log(`dialog ${JSON.stringify(result.dialog)}`);
      await page.locator('[role="dialog"] #name, [role="dialog"] input[id$="name"]').first().fill(`S12 Dialog Document ${Date.now() % 100000}`).catch(() => undefined);
      const dialogKind = page.locator('[role="dialog"] [role="combobox"], [role="dialog"] select').first();
      if ((await dialogKind.count()) > 0) {
        await dialogKind.click({ force: true }).catch(() => undefined);
        await page.waitForTimeout(1_500);
        const count = await page.locator('[role="option"]').count();
        result.dialogOptions = await page.evaluate(() => [...document.querySelectorAll('[role="option"]')].map((element) => (element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 40)));
        const index = process.env.S12_KIND_INDEX === undefined ? count - 1 : Number(process.env.S12_KIND_INDEX);
        await page.locator('[role="option"]').nth(Math.max(0, Math.min(count - 1, index))).click({ force: true }).catch(() => undefined);
      }
      result.dialogSubmitted = await click(page, '[role="dialog"] button[type="submit"], [role="dialog"] [id$="submit"], [role="dialog"] [id$="Submit"]');
      log(`dialog submit ${result.dialogSubmitted} options ${JSON.stringify(result.dialogOptions)}`);
    }
    if (process.env.S15_CANCEL_OPEN === "1") {
      const cancel = page.locator("[data-semio-execution-target-cancel]").first();
      await cancel.waitFor({ state: "visible", timeout: 120_000 }).catch(() => undefined);
      result.cancelStage = await page.evaluate(() => document.querySelector("[data-semio-execution-target-status]")?.getAttribute("data-semio-execution-target-stage") ?? null);
      result.cancelClicked = await cancel.click({ timeout: 10_000 }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 80));
      await page.waitForTimeout(8_000);
      result.afterCancelWindows = await windowIds(page);
      log(`cancel at stage ${result.cancelStage}: ${result.cancelClicked}; windows ${JSON.stringify(result.afterCancelWindows)}`);
    }
    const rowDeadline = Date.now() + 120_000;
    while (Date.now() < rowDeadline) {
      result.rowsAfter = await indexRows(page);
      if (result.rowsAfter.length > result.rowsBefore.length) break;
      await page.waitForTimeout(3_000);
    }
    const raised = (await notices(page)).slice(noticeCursor);
    result.createOutcome = result.rowsAfter.length > result.rowsBefore.length
      ? `created (${result.rowsBefore.length} → ${result.rowsAfter.length} rows)`
      : raised.length > 0 ? `refused, notice: ${JSON.stringify(raised)}` : "dispatched, no notice, no new row";
  }
  log(`createArtifact → ${result.createOutcome}`);
  result.rowsAfter = await indexRows(page);
  await page.screenshot({ path: `${generated}s15-hub-document-${tag}-created.png` }).catch(() => undefined);

  const target = result.rowsAfter.filter((row) => row.id !== null).at(-1) ?? result.rowsAfter.at(-1);
  const afterCreation = await windowIds(page);
  if (!afterCreation.includes("framework.window.table") && afterCreation.length > 0) {
    result.openedWindows = afterCreation;
    result.opened = "opened by the creation saga";
  } else if (target === undefined) result.opened = "space index rendered no artifact row to open";
  else {
    const before = await windowIds(page);
    const locator = target.id !== null ? page.locator(`[data-slot="window-body"] [data-row-id="${target.id}"]`).first() : page.locator('[data-slot="window-body"] [role="row"]').last();
    await locator.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(1_200);
    await locator.dblclick({ force: true }).catch(() => undefined);
    await page.waitForTimeout(4_000);
    if ((await windowIds(page)).filter((id) => !before.includes(id)).length === 0) {
      await click(page, '[data-slot="window-action-pane"] [id="action.openArtifact"]');
      await page.waitForTimeout(1_500);
      for (const selector of ['[id$=".action.openArtifact.execute"]', '[id$="openArtifact.execute"]']) {
        if ((await click(page, selector)) !== "absent") break;
      }
    }
    const openDeadline = Date.now() + 150_000;
    while (Date.now() < openDeadline) {
      const opened = (await windowIds(page)).filter((id) => !before.includes(id));
      if (opened.length > 0) { await page.waitForTimeout(6_000); result.openedWindows = (await windowIds(page)).filter((id) => !before.includes(id)); break; }
      await page.waitForTimeout(1_000);
    }
    result.opened = result.openedWindows.length > 0 ? "ok" : `row "${(target.text ?? "").slice(0, 40)}" opened no new window`;
  }
  log(`open → ${result.opened} ${JSON.stringify(result.openedWindows)}`);
  await page.screenshot({ path: `${generated}s15-hub-document-${tag}-opened.png` }).catch(() => undefined);

  if (result.openedWindows.length > 0) {
    result.breadcrumb = await page.evaluate(() => (document.querySelector('[data-slot="navbar"]')?.textContent ?? "").replace(/\s+/gu, " ").slice(0, 160));
    const sweepRefusals = [];
    const sweep = await sweepMutateUndoRedo(page, sweepRefusals, kindFilter);
    result.sweep = { railRows: sweep.railRows, verb: sweep.mutation, detail: sweep.mutationDetail, edits: sweep.edits, applied: sweep.applied, mutated: sweep.mutated, undone: sweep.undone, redone: sweep.redone, redoDiffersFromUndo: sweep.redoDiffersFromUndo, undoLane: sweep.undoLane, redoLane: sweep.redoLane, attempts: sweep.attempts, refusals: sweepRefusals.slice(0, 5) };
    result.ledgerAfterSweep = (await sweepReadShell(page)).ledger.slice(-6);
    log(`sweep ${JSON.stringify(result.sweep)}`);
    await raiseHistory(page);
    await unfoldActionsRail(page);
    result.edits.push(editCount(await readShell(page)));
    result.documentVerb = await click(page, `[data-slot="window-action-pane"] [id="action.${verb}"]`);
    await page.waitForTimeout(1_500);
    for (const selector of [`[id$=".action.${verb}.execute"]`, `[id$="${verb}.execute"]`]) {
      if ((await click(page, selector)) !== "absent") break;
    }
    await neutralDispatch(page);
    result.edits.push(editCount(await readShell(page)));
    await click(page, '[data-slot="window-action-pane"] [id="action.undo"]');
    await neutralDispatch(page);
    result.edits.push(editCount(await readShell(page)));
    await click(page, '[data-slot="window-action-pane"] [id="action.redo"]');
    await neutralDispatch(page);
    const final = await readShell(page);
    result.edits.push(editCount(final));
    result.ledgerTail = final.ledger.slice(-6);
    log(`verb ${verb} → ${result.documentVerb} edits ${JSON.stringify(result.edits)}`);
  }
  await page.screenshot({ path: `${generated}s15-hub-document-${tag}-mutated.png` }).catch(() => undefined);
  /** ♻️ `S15_REOPEN=1`: reload and open the same document again — the module now comes from this device (the
   * recorded verified bundle and the immutable content-addressed responses), not a second download. */
  if (process.env.S15_REOPEN === "1" && result.openedWindows.length > 0) {
    moduleTraffic.phase = "reopen";
    result.installedRecord = await page.evaluate(() => Object.keys(localStorage).filter((key) => key.includes("semio.hub-plugin-modules.v1")).map((key) => (localStorage.getItem(key) ?? "").slice(0, 400)));
    await page.reload({ waitUntil: "commit" });
    result.reopenBeacon = await awaitBeacon(page, Date.now() + 300_000);
    await page.waitForTimeout(6_000);
    const reopenTarget = (await indexRows(page)).filter((row) => row.id !== null).at(-1);
    if (reopenTarget) {
      const before = await windowIds(page);
      const row = page.locator(`[data-slot="window-body"] [data-row-id="${reopenTarget.id}"]`).first();
      await row.click({ force: true }).catch(() => undefined);
      await page.waitForTimeout(1_200);
      await row.dblclick({ force: true }).catch(() => undefined);
      const reopenDeadline = Date.now() + 150_000;
      while (Date.now() < reopenDeadline) {
        const opened = (await windowIds(page)).filter((id) => !before.includes(id));
        if (opened.length > 0) { await page.waitForTimeout(6_000); result.reopenedWindows = (await windowIds(page)).filter((id) => !before.includes(id)); break; }
        await page.waitForTimeout(1_000);
      }
    }
    log(`reopen → ${JSON.stringify(result.reopenedWindows ?? [])}`);
    await page.screenshot({ path: `${generated}s15-hub-document-${tag}-reopened.png` }).catch(() => undefined);
  }
} catch (error) {
  result.fatal = String(error).slice(0, 300);
  log(`FATAL ${result.fatal}`);
}
result.notices = await notices(page).catch(() => []);
result.creationProgress = await page.evaluate(() => [...document.querySelectorAll("[data-semio-artifact-creation-catalog], [data-semio-artifact-creation]")].map((element) => `${element.getAttribute("data-semio-artifact-creation-catalog") ?? element.getAttribute("data-semio-artifact-creation")}|${(element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 120)}`)).catch(() => []);
result.shellLines = shellLines.slice(0, 40);
result.hubRequests = hubRequests.filter((line) => /\/_semio\/hub\/.*(execution-target|open-plan|artifact-creations|socket-grants|plugin-modules)/u.test(line)).slice(0, 120);
result.faults = faults.slice(0, 8);
result.moduleTraffic = moduleTraffic;
if (process.env.S15_CONSOLE_ALL === "1") result.consoleAll = consoleAll.filter((line) => /note|plugin|program|surface|descriptor|install|error/iu.test(line)).slice(-200);
writeFileSync(`${generated}s15-hub-document-${tag}.txt`, JSON.stringify(result, null, 2));
log(`=== ${generated}s15-hub-document-${tag}.txt ===`);
await (browser ?? context).close();
