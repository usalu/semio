/** 🧭️ C10 journey helpers shared by the collaboration matrix and the session-12 probes: open a Space (retrying the
 * measured hard-load miss), open an artifact from its row, wait for a mounted document, witness the document as the human
 * sees it (window-body text) and as the hub holds it (admin `headSeq`), and drive one pinned document verb / undo. */
import { activate, clickRowAction, dialog, read, rowIds, submitDialog, waitNewRow, waitRow } from "./c11-lib.mjs";
import { DEFAULT_ARGS, DEFAULT_VERBS, FAULT, NOISE, clickUncovered, fillStagedArgument, readShell, submitStagedVerb, unfoldActionsRail } from "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

/** 🧩️ Kind ids whose schema does not name its plugin (the catalog names the dialect only). */
export const PLUGIN_BY_KIND = { "2d.drawing": "draw", "text.document": "writer", "2d.puzzle": "puzzle", "3d.puzzle": "puzzle", "5d.puzzle": "puzzle", "2d.block": "block", "3d.block": "block", "5d.block": "block", "3d.wfcgrid3d": "wfc", "animate.presentation": "animate" };
export const faultsSince = (session, cursor) => session.lines.slice(cursor).filter((line) => FAULT.test(line) && !NOISE.test(line)).map((line) => line.slice(0, 240));
export const origin = (session) => new URL(session.url).origin;
export const pause = (session, ms) => session.page.waitForTimeout(ms);

/** ⏳️ Polls `probe` until it answers truthy or the budget ends; answers the last value either way. */
export async function until(probe, deadlineMs, stepMs = 500) {
  const deadline = Date.now() + deadlineMs;
  let value = await probe();
  while (!value && Date.now() < deadline) {
    await new Promise((resolve) => setTimeout(resolve, stepMs));
    value = await probe();
  }
  return value;
}

/** 🪟️ A document is usable once its window renders a surface, no execution-target notice remains and its Actions rail has rows. */
/** 🪟️ The shell's own chrome windows (Home, the Space index) — a document is mounted once a window OTHER than these shows. */
const SHELL_WINDOWS = new Set(["framework.window.table", "s-home-main"]);

export async function awaitMounted(session, deadlineMs) {
  const mounted = await until(async () => {
    const shell = await readShell(session.page);
    const status = await read(session.page);
    const documentWindows = shell.windowIds.filter((id) => !SHELL_WINDOWS.has(id));
    const painted = await session.page.evaluate(() => [...document.querySelectorAll('[data-slot="window-body"]')].some((body) => body.querySelectorAll("*").length > 20));
    return documentWindows.length > 0 && painted && status.executionTarget.length === 0 && !new URL(session.page.url()).pathname.endsWith("/") ? { ...shell, windowIds: documentWindows } : null;
  }, deadlineMs, 1_000);
  if (!mounted) {
    const status = await read(session.page);
    const shell = await readShell(session.page);
    const dom = await session.page.evaluate(() => ({ url: location.pathname, windows: [...document.querySelectorAll('[data-slot="window"]')].map((element) => `${element.id}|${element.getAttribute("data-window-kind") ?? ""}|${element.getAttribute("data-state") ?? ""}`).slice(0, 12), bodies: [...document.querySelectorAll('[data-slot="window-body"]')].map((body) => body.querySelectorAll("*").length) }));
    throw new Error(`not mounted within ${deadlineMs} ms: ${JSON.stringify({ executionTarget: status.executionTarget, diagnostics: status.diagnostics, windows: status.windows, shellWindowIds: shell.windowIds, dom, error: status.error })}`);
  }
  await unfoldActionsRail(session.page);
  await session.page.locator('[data-slot="panel-tab-button"][id="framework.panel.history"]').first().click({ force: true }).catch(() => undefined);
  await pause(session, 1_000);
  return readShell(session.page);
}

/** 🏘️ Opens the Space app at `/spaces/<id>`. A hard load sometimes lands on Home with the URL still naming the space
 * (measured 1 of 3 on hub 8021, `probe-s12-reopen.mjs`, routed to the shell owner), so a load is retried up to three
 * times and every miss is recorded in the report (`routeMisses`). */
export async function openSpace(session, spaceId, report = {}) {
  const create = session.page.locator('[data-ui-node-key="s-space-create-artifact"]').first();
  if (new URL(session.page.url()).pathname === `/spaces/${spaceId}` && (await create.count()) > 0) return;
  for (let attempt = 1; attempt <= 3; attempt += 1) {
    const cursor = session.lines.length;
    await session.page.goto(`${origin(session)}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
    if (await create.waitFor({ state: "attached", timeout: 60_000 }).then(() => true).catch(() => false) && (await settleAfterLoad(session, cursor, create))) return;
    const state = await read(session.page);
    report.routeMisses = [...(report.routeMisses ?? []), { user: session.user.label, spaceId, attempt, windows: state.windows, at: new Date().toISOString() }];
  }
  throw new Error(`the Space app never mounted at /spaces/${spaceId} in 3 loads`);
}

/** ⏳️ A hard load mounts the Space app, then the restored identity re-establishes the session and the route re-opens the
 * space ~5–7 s later ("space index opening failed: document closed", routed to U5 on 26/09/26): waits until the load has
 * been quiet for `quietMs` — no re-opening, no directory re-bootstrap — and the Space app is mounted again. */
export async function settleAfterLoad(session, cursor, create, quietMs = 12_000, deadlineMs = 90_000) {
  const started = Date.now();
  let quietSince = Date.now();
  let seen = cursor;
  while (Date.now() - started < deadlineMs) {
    const fresh = session.lines.slice(seen);
    seen = session.lines.length;
    if (fresh.some((line) => /space index opening failed|event-page\/v1\?after=0/u.test(line))) quietSince = Date.now();
    if (Date.now() - quietSince >= quietMs && (await create.count()) > 0) return true;
    await session.page.waitForTimeout(500);
  }
  return false;
}

export async function openRow(session, spaceId, artifactId, report = {}) {
  await openSpace(session, spaceId, report);
  await waitRow(session.page, "artifact", artifactId, 90_000);
  await clickRowAction(session.page, "artifact", artifactId, /^(open|öffnen)\b/iu);
}

/** 🗂️ The creation catalog's kinds as the Space app's picker offers them (the option's encoded choice carries the kind id). */
export async function creatableKinds(session) {
  await activate(session.page, "s-space-create-artifact");
  await dialog(session.page).waitFor({ state: "visible", timeout: 20_000 });
  const offered = await until(async () => {
    await session.page.locator('[id="kindChoice"]').focus();
    await session.page.keyboard.press("Enter");
    return session.page.locator('[role="option"]').first().waitFor({ state: "visible", timeout: 5_000 }).then(() => true).catch(() => false);
  }, 120_000, 1_000);
  if (!offered) throw new Error("the Space app's kind picker offered no artifact kind within 120 s");
  const options = await session.page.locator('[role="option"]').evaluateAll((elements) => elements.map((element) => ({ value: element.getAttribute("data-value") ?? "", label: (element.textContent ?? "").trim() })));
  await session.page.keyboard.press("Escape");
  await session.page.keyboard.press("Escape");
  await dialog(session.page).waitFor({ state: "hidden", timeout: 20_000 }).catch(() => undefined);
  return options.map((option) => ({ ...option, kindId: JSON.parse(option.value).kindId }));
}

export async function createArtifact(session, name, kind) {
  const before = await rowIds(session.page, "artifact");
  await activate(session.page, "s-space-create-artifact");
  await dialog(session.page).waitFor({ state: "visible", timeout: 20_000 });
  await session.page.locator("#name").fill(name);
  await session.page.locator('[id="kindChoice"]').focus();
  await session.page.keyboard.press("Enter");
  await session.page.locator(`[role="option"][data-value="${kind.value.replace(/"/gu, '\\"')}"]`).first().click();
  await submitDialog(session.page);
  return waitNewRow(session.page, "artifact", before, 240_000);
}

/** 📄️ The document as the human sees it: every window body's text plus its element / SVG-element / canvas counts, action
 * panes removed — the generic witness of an edit arriving (a note block is drawn as SVG and changes no text; a
 * command-history ledger also lists chrome and view verbs). */
export const docText = (session) =>
  session.page.evaluate(() =>
    [...document.querySelectorAll('[data-slot="window-body"]')]
      .map((body) => {
        const clone = body.cloneNode(true);
        clone.querySelectorAll('[data-slot="window-action-pane"], [data-slot="window-action-pane-overlay"], [data-slot="utility-bar"], [data-slot="canvas-presence-overlay"]').forEach((element) => element.remove());
        const keys = [...clone.querySelectorAll("[data-ui-node-key], [data-node-id], [data-row-id], svg [id]")].map((element) => element.getAttribute("data-ui-node-key") ?? element.getAttribute("data-node-id") ?? element.getAttribute("data-row-id") ?? element.id).sort().join("|");
        let hash = 0x811c9dc5;
        for (let index = 0; index < keys.length; index += 1) hash = Math.imul(hash ^ keys.charCodeAt(index), 0x01000193) >>> 0;
        return `${(clone.textContent ?? "").replace(/\s+/gu, " ").trim()} #${clone.querySelectorAll("*").length}/${clone.querySelectorAll("svg *").length}/${clone.querySelectorAll("canvas").length}/${hash.toString(16)}`;
      })
      .join(" ¦ "),
  );

/** 🗄️ The hub's own head sequence for the document (admin API), when this run was handed an admin capability file. */
export async function hubHead(artifactId) {
  const hub = process.env.S_MATRIX_HUB, file = process.env.S_MATRIX_ADMIN_FILE;
  if (!hub || !file) return null;
  const { readFileSync, writeFileSync } = await import("node:fs");
  const read = async () => {
    const { capability } = JSON.parse(readFileSync(file, "utf8"));
    return fetch(`${hub}/admin/api/documents`, { headers: { authorization: `Bearer ${capability}` } });
  };
  let response = await read();
  if (response.status === 401) {
    writeFileSync(file.replace(/admin-capability\.json$/u, "admin-request"), "");
    await new Promise((resolve) => setTimeout(resolve, 7_000));
    response = await read();
  }
  if (!response.ok) return `admin ${response.status}`;
  return (await response.json()).rows.find((row) => row.descriptor.documentId === artifactId)?.headSeq ?? null;
}

/** ⏳️ Waits until `session`'s document text differs from `before` (or equals `target` when given). */
export async function awaitText(session, predicate, deadlineMs) {
  const seen = await until(async () => {
    const now = await docText(session);
    return predicate(now) ? now : null;
  }, deadlineMs);
  return seen ?? { missed: await docText(session) };
}

/** ✏️ Dispatches the plugin's pinned document verb once (staged arguments filled); answers whether the author's own view
 * moved within 20 s. A verb whose staged form is still open from its last dispatch is submitted directly — clicking its
 * rail row again would fold the form away and dispatch nothing. */
export async function edit(session, plugin, args = undefined) {
  const verb = DEFAULT_VERBS[plugin];
  if (verb === undefined) throw new Error(`no pinned document verb for plugin ${plugin}`);
  const t0 = Date.now();
  const before = await docText(session);
  const staged = await session.page.locator(`[id$=".action.${verb}.execute"]`).first().isVisible().catch(() => false);
  const clicked = staged ? "staged" : await clickUncovered(session.page, `[data-slot="window-action-pane"] [id="action.${verb}"]`);
  const clickedMs = Date.now() - t0;
  await pause(session, 1_000);
  for (const [key, value] of Object.entries(args ?? DEFAULT_ARGS[`${plugin}.${verb}`] ?? DEFAULT_ARGS[verb] ?? {})) await fillStagedArgument(session.page, key, value);
  const filledMs = Date.now() - t0;
  const submitted = await submitStagedVerb(session.page, verb);
  const submittedMs = Date.now() - t0;
  const after = await awaitText(session, (now) => now !== before, 20_000);
  return { verb, clicked, submitted, timings: { clickedMs, filledMs, submittedMs, doneMs: Date.now() - t0 }, before, after: typeof after === "string" ? after : after.missed, applied: typeof after === "string" };
}

/** 👥️ The pinned arguments of `plugin`'s verb, made distinct per human: a text value gains the human's label, a numeric
 * one (a seed, a count) is offset for the second human, a live id stays live — so two humans' edits never write the same
 * value and each edit is a visible change. */
export function personalArgs(plugin, label) {
  const verb = DEFAULT_VERBS[plugin];
  const pinned = DEFAULT_ARGS[`${plugin}.${verb}`] ?? DEFAULT_ARGS[verb];
  if (pinned === undefined) return undefined;
  return Object.fromEntries(Object.entries(pinned).map(([key, value]) => [key, typeof value !== "string" || value.startsWith("@") ? value : /^-?\d+(\.\d+)?$/u.test(value) ? String(Number(value) + (label === "user2" ? 6 : 0)) : `${value} ${label}`]));
}

export async function undo(session, verb = "undo") {
  const before = await docText(session);
  const clicked = await clickUncovered(session.page, `[data-slot="window-action-pane"] [id="action.${verb}"]`);
  const after = await awaitText(session, (now) => now !== before, 20_000);
  return { clicked, before, after: typeof after === "string" ? after : after.missed, undone: typeof after === "string" };
}

export const short = (text) => (text.length <= 160 ? text : `${text.slice(0, 80)}…${text.slice(-70)}`);

/** 🏠️ Waits until `session`'s Home lists `spaceId`; a Home that is never told (the hub does not deliver a new member's
 * space live — routed hub defect) is reloaded once, and that miss is recorded in `report.homeMisses`. */
export async function awaitSharedSpace(session, spaceId, report = {}) {
  if (await waitRow(session.page, "space", spaceId, 45_000).then(() => true).catch(() => false)) return;
  report.homeMisses = [...(report.homeMisses ?? []), { user: session.user.label, spaceId, at: new Date().toISOString() }];
  await session.page.reload({ waitUntil: "domcontentloaded" });
  await waitRow(session.page, "space", spaceId, 120_000);
}
