/** 🤝️ C3 — the ten-step two-human collaboration scenario on ONE shared hub document.
 *
 * Two browser contexts, two different signed-in humans, the SAME hub document reached through the
 * shell's own `remote://` sync attach (C2 §3). Every step states its witness and is measured, never
 * inferred: the History ledger's applied rows, `#s-checkin`'s uncommitted-edit count, the app's own
 * inspector extents (`Positions N`), the presence roster and its per-peer colour, the document
 * canvas' pixel hash, and the hub's own log.
 *
 * Usage: bun 🐍️c3-collab-scenario.mjs [shellUrl] [hubHostPort] [spaceId] [documentId]
 * Env:   C3_TAG (capture prefix, default "c3"), C3_ONLY (comma list of step names to run)
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { createHash } from "node:crypto";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join } from "node:path";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6191";
const HUB = process.argv[3] ?? "127.0.0.1:7611";
const SPACE = process.argv[4] ?? "01a0c00f-4f3c-7834-a7e6-2ccf9de925db";
const DOCUMENT = process.argv[5] ?? "artifact-2fb248125b8b2b4d56de25933d30ed21";
const TAG = process.env.C3_TAG ?? "c3";
const ONLY = (process.env.C3_ONLY ?? "").split(",").filter(Boolean);
const OUT = "/Users/ueli/Documents/semio/.tmp-ticket/wp-c7/generated";
mkdirSync(OUT, { recursive: true });

const USERS = [
  { label: "user1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1" },
  { label: "user2", email: "user2@semio.dev", password: "gm1-local-dev-pass-2" },
];
const FAULT = /unreachable|trapped|panicked|fault|refused|denied|dispatch-failed|invalid-args|not-ui-safe/i;
const NOISE = /staged plugin module\(s\) are behind their source|\[stale\]|status of 404|Download the React DevTools|typed-operation slots|ERR_CONNECTION_REFUSED/;

const t0 = Date.now();
const ms = () => Date.now() - t0;
const report = { shell: SHELL, hub: HUB, space: SPACE, document: DOCUMENT, startedAt: new Date().toISOString(), steps: [] };
const save = () => writeFileSync(join(OUT, `${TAG}-collab-scenario.json`), JSON.stringify(report, null, 2));
const record = (step, pass, detail) => {
  report.steps.push({ step, pass, at: ms(), detail });
  console.log(`STEP ${step}: ${pass === null ? "SKIP" : pass ? "PASS" : "FAIL"} — ${typeof detail === "string" ? detail : JSON.stringify(detail)}`);
  save();
  writeFileSync(join(OUT, `${TAG}-collab-scenario-console.txt`), sessions.flatMap((s) => s.lines.map((line) => `${s.user.label} ${line}`)).join("\n"));
};
const wanted = (step) => ONLY.length === 0 || ONLY.includes(step);

/** 🪞️ Everything the shell publishes about the document, the sync binding and the roster. */
const read = (page) =>
  page.evaluate(() => {
    const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
    const visible = (el) => el instanceof HTMLElement && el.offsetParent !== null;
    const peerNodes = [...document.querySelectorAll('[id="s-presence-peers"] [data-row-id^="peer:"]:not([data-row-id="peer:overflow"])')];
    return {
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      error: document.documentElement.getAttribute("data-semio-os-error"),
      ledger: [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((el) => !el.id.endsWith(".revert")).map((el) => text(el).slice(0, 96)),
      checkin: text(document.querySelector("#s-checkin")),
      syncStatus: text(document.querySelector("[data-semio-sync-status]")),
      syncPill: text(document.querySelector('[id="s-sync-status"]')),
      executionTarget: [...document.querySelectorAll("[data-semio-execution-target-status]")].map((el) => text(el).slice(0, 110)),
      peers: peerNodes.map((el) => el.getAttribute("data-row-id")),
      peerLabels: peerNodes.map((el) => text(el).slice(0, 64)),
      peerColors: peerNodes.map((el) => {
        const probe = el.querySelector("[style*='color'],[style*='background']") ?? el.firstElementChild ?? el;
        const style = getComputedStyle(probe);
        return [style.borderTopColor, style.backgroundColor, style.color].join("|");
      }),
      panelRows: [...document.querySelectorAll('[data-slot="panel"]')]
        .filter((el) => visible(el) && !el.id.includes("framework.panel.history"))
        .flatMap((panel) => [...panel.querySelectorAll('[role="treeitem"]')].map((el) => text(el).slice(0, 80))),
      actions: [...new Set([...document.querySelectorAll('[id^="action."]')].map((el) => el.id))],
    };
  });

const edits = (shell) => {
  const match = /\((\d+)\)\s*$/.exec(shell.checkin ?? "");
  return match === null ? 0 : Number(match[1]);
};

const click = async (page, selector, timeout = 8_000) => {
  if (!(await page.locator(selector).count())) return "absent";
  return page.locator(selector).first().click({ timeout, force: true }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 120));
};

/** 🖼️ Pixel witness: the document canvas' own bytes, hashed. Survives WebGL (a real screenshot, not
 * `toDataURL`, which a context without `preserveDrawingBuffer` answers blank). */
const canvasHash = async (page) => {
  const canvas = page.locator("canvas").first();
  if (!(await canvas.count())) return "no-canvas";
  return canvas
    .screenshot({ timeout: 15_000 })
    .then((buffer) => `${createHash("sha256").update(buffer).digest("hex").slice(0, 16)}:${buffer.length}`)
    .catch((error) => `canvas-error ${String(error).split("\n")[0].slice(0, 60)}`);
};

/** 🔎️ The app's own inspector extents, read by switching the single panel slot and back. */
const inspector = async (page) => {
  await click(page, '[data-slot="panel-tab-button"][id="framework.panel.inspection"]');
  await page.waitForTimeout(1_400);
  const rows = (await read(page)).panelRows;
  await click(page, '[data-slot="panel-tab-button"][id="framework.panel.history"]');
  await page.waitForTimeout(1_100);
  return rows;
};

/** 🧾️ The witness tuple a convergence/propagation claim is made of. */
const witness = async (session, withInspector = true) => {
  const shell = await read(session.page);
  return {
    user: session.user.label,
    edits: edits(shell),
    ledgerCount: shell.ledger.length,
    ledgerTail: shell.ledger.slice(-3),
    ledgerHash: createHash("sha256").update(shell.ledger.join("\n")).digest("hex").slice(0, 16),
    sync: shell.syncStatus,
    syncPill: shell.syncPill,
    peers: shell.peers,
    inspector: withInspector ? await inspector(session.page) : null,
    canvas: await canvasHash(session.page),
  };
};

async function signIn(page, email, password) {
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 30_000 });
  await form.locator('input[type="email"]').fill(email);
  await form.locator('input[type="password"]').fill(password);
  await form.locator('button[type="submit"][aria-label="Sign in"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
  await page.locator("[data-semio-hub-workspace]").waitFor({ state: "hidden", timeout: 15_000 });
}

async function openRemoteCard(page) {
  const trail = [];
  for (let attempt = 0; attempt < 6; attempt += 1) {
    if (await page.locator('[id="framework.sync.remote.path"]').count()) return { ok: true, trail };
    if (await page.locator('[id="framework.sync.remote"]').count()) trail.push(`remote:${await click(page, '[id="framework.sync.remote"]')}`);
    else if (await page.locator('[id="ui.utilities.group.sync"]').count()) trail.push(`group:${await click(page, '[id="ui.utilities.group.sync"]')}`);
    else trail.push(`tab:${await click(page, '[data-slot="panel-tab-button"][id="s-sync-status"], [id="s-sync-status"]')}`);
    await page.waitForTimeout(1_500);
  }
  return { ok: (await page.locator('[id="framework.sync.remote.path"]').count()) > 0, trail };
}

/** 🌐️ footer sync tab ▸ `sync` collection ▸ Remote ▸ `<host>/<space>/<document>` ▸ Attach. */
async function attachRemote(page, settleMs = Number(process.env.C3_SETTLE_MS ?? 180_000)) {
  const opened = await openRemoteCard(page);
  if (!opened.ok) return { card: "absent", trail: opened.trail };
  const input = page.locator('[id="framework.sync.remote.path"]');
  await input.fill(`${HUB}/${SPACE}/${DOCUMENT}`);
  await page.waitForTimeout(400);
  const attach = input.locator("xpath=ancestor::*[@data-slot='popover-content'][1]").locator('button:has([data-icon="link"])');
  const pressed = (await attach.count())
    ? await attach.first().click({ force: true, timeout: 8_000 }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 90))
    : "absent";
  await page.waitForTimeout(settleMs);
  // close whatever popover is still open so later clicks are not intercepted
  await page.keyboard.press("Escape").catch(() => {});
  await page.waitForTimeout(800);
  return { card: "ok", trail: opened.trail, value: `${HUB}/${SPACE}/${DOCUMENT}`, attach: pressed };
}

/** ✍️ One authored gis edit: `addFeature` mints the lowest free `position-N` with no argument typed. */
async function authorEdit(session, action = "addFeature", staged = action === "addFeature" ? { label: `${session.user.label} ${ms()}`, lon: (Math.random() * 300 - 150).toFixed(3), lat: (Math.random() * 120 - 60).toFixed(3) } : {}) {
  const page = session.page;
  const before = await read(page);
  const opened = await click(page, `[id="action.${action}"]`);
  await page.waitForTimeout(1_200);
  for (const [key, value] of Object.entries(staged)) {
    const input = page.locator(`[id$=".arg.${key}"] input, [id$=".arg.${key}"] textarea, [id$=".arg.${key}"]:is(input,textarea), [name="${key}"]`).first();
    if (await input.count()) await input.fill(String(value)).catch(() => {});
    await page.waitForTimeout(250);
  }
  const submitted = await click(page, `[id$=".action.${action}.execute"]`);
  await page.waitForTimeout(6_000);
  const after = await read(page);
  return {
    action,
    opened,
    submitted,
    editsBefore: edits(before),
    editsAfter: edits(after),
    ledgerBefore: before.ledger.length,
    ledgerAfter: after.ledger.length,
    newRow: after.ledger.slice(before.ledger.length)[0] ?? null,
    mutated: edits(after) > edits(before) || after.ledger.length > before.ledger.length,
  };
}

/** ⏳️ Waits for a peer's view to change without a reload — the live-propagation witness. */
async function awaitPropagation(session, baseline, budgetMs = 60_000) {
  const startedAt = Date.now();
  let last = null;
  while (Date.now() - startedAt < budgetMs) {
    const shell = await read(session.page);
    last = {
      ledgerCount: shell.ledger.length,
      ledgerTail: shell.ledger.slice(-2),
      edits: edits(shell),
      sync: shell.syncStatus,
      waitedMs: Date.now() - startedAt,
    };
    if (last.ledgerCount !== baseline.ledgerCount || last.edits !== baseline.edits) return { changed: true, ...last };
    await session.page.waitForTimeout(2_000);
  }
  const canvas = await canvasHash(session.page);
  return { changed: false, ...last, canvas, canvasChanged: canvas !== baseline.canvas };
}

const shot = async (session, name) => {
  await session.page.screenshot({ path: join(OUT, `${TAG}-${name}-${session.user.label}.png`), fullPage: false }).catch(() => {});
};

// ── run ────────────────────────────────────────────────────────────────────────────────────────
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const sessions = [];
for (const user of USERS) {
  const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
  const page = await context.newPage();
  const lines = [];
  const sockets = [];
  page.on("console", (message) => lines.push(`${ms()} ${message.type()} ${message.text().slice(0, 700)}`));
  page.on("pageerror", (error) => lines.push(`${ms()} pageerror ${String(error).slice(0, 700)}`));
  page.on("response", (response) => {
    const url = response.url();
    if (url.includes("open-plan") || url.includes("socket-grants") || url.includes("execution-target")) lines.push(`${ms()} response ${response.status()} ${url.slice(url.indexOf("/documents/"))}`);
  });
  page.on("websocket", (ws) => {
    if (ws.url().includes("/socket/v1") || ws.url().includes("/document/ws")) sockets.push({ url: ws.url(), openedAt: ms(), closedAt: null });
    ws.on("close", () => {
      const row = sockets.find((s) => s.url === ws.url() && s.closedAt === null);
      if (row) row.closedAt = ms();
      lines.push(`${ms()} ws-closed ${ws.url().slice(0, 140)}`);
    });
  });
  await page.goto(`${SHELL}/?plugin=gis2d`, { waitUntil: "domcontentloaded", timeout: 180_000 });
  let shell = null;
  for (let attempt = 0; attempt < 200; attempt += 1) {
    await page.waitForTimeout(1_000);
    shell = await read(page);
    if (shell.error || (shell.ready && attempt > 6)) break;
  }
  sessions.push({ user, context, page, lines, sockets, boot: shell });
}
const [A, B] = sessions;
record("1a-boot", sessions.every((s) => s.boot?.ready && !s.boot?.error), sessions.map((s) => `${s.user.label}:ready=${s.boot?.ready} error=${s.boot?.error ?? "none"}`).join(" "));

for (const session of sessions) {
  await signIn(session.page, session.user.email, session.user.password).catch((error) => {
    session.signInError = String(error).split("\n")[0].slice(0, 160);
  });
  await session.page.waitForTimeout(2_000);
}
record(
  "1b-sign-in",
  await Promise.all(sessions.map(async (s) => (await s.page.locator('[data-semio-hub-sign-in=""]').count()) === 0)).then((flags) => flags.every(Boolean)),
  sessions.map((s) => `${s.user.label}:${s.signInError ?? "ok"}`).join(" "),
);

// keep History open in both contexts — the ledger is the applied-row witness everywhere below
for (const session of sessions) {
  await click(session.page, '[data-slot="panel-tab-button"][id="framework.panel.history"]');
  await session.page.waitForTimeout(1_000);
  for (const toggle of await session.page.locator('[id$=".engagement.toggle"]').all()) {
    await toggle.click({ timeout: 6_000, force: true }).catch(() => {});
    await session.page.waitForTimeout(700);
  }
}

for (const session of sessions) {
  session.attach = await attachRemote(session.page);
  for (let i = 0; i < 90; i += 1) {
    const shell = await read(session.page);
    const pill = shell.syncPill ?? "";
    const target = (shell.executionTarget ?? []).join(" ");
    if (/Persisted|Synced|Connected/i.test(pill) && !/unavailable|backoff/i.test(pill + target)) break;
    if (/actor-ready/i.test(target) && !/unavailable/i.test(target)) break;
    await session.page.waitForTimeout(2_000);
  }
  await session.page.waitForTimeout(4_000);
}
for (const session of sessions) session.afterAttach = await read(session.page);
const bothAttached = sessions.every((s) => s.sockets.some((row) => row.url.includes("/document/ws") || (row.url.includes("/documents/") && row.url.includes("/socket/v1"))));
record(
  "1c-both-attached",
  bothAttached,
  sessions.map((s) => `${s.user.label}: attach=${s.attach.attach ?? s.attach.card} sync=${JSON.stringify(s.afterAttach.syncStatus)} sockets=${s.sockets.length} target=${JSON.stringify(s.afterAttach.executionTarget)}`).join(" | "),
);
for (const session of sessions) await shot(session, "attached");

report.rosters = sessions.map((s) => ({ user: s.user.label, peers: s.afterAttach.peers, labels: s.afterAttach.peerLabels, colors: s.afterAttach.peerColors }));
record("1d-rosters", sessions.every((s) => s.afterAttach.peers.length > 0), JSON.stringify(report.rosters));

// 5 — presence is read HERE, at attach time: the roster is a live beat and decays once a context
// stops touching the socket, so reading it at the end of the run measures the decay, not the feature.
for (const session of sessions) await shot(session, "presence");
{
  const distinct = report.rosters.length > 0 && report.rosters.every((r) => r.colors.length > 0 && new Set(r.colors).size === r.colors.length);
  const palette = new Map();
  let agreed = true;
  for (const roster of report.rosters)
    for (const [index, peer] of roster.peers.entries()) {
      const colour = roster.colors[index];
      if (!palette.has(peer)) palette.set(peer, colour);
      else if (palette.get(peer) !== colour) agreed = false;
    }
  const bothListed = report.rosters.some((r) => r.peers.length >= sessions.length);
  const symmetric = report.rosters.every((r) => r.peers.length >= sessions.length);
  report.presence = { bothListed, symmetric, distinct, agreed };
  record("5-presence-distinct-colours", symmetric && distinct && agreed, `${JSON.stringify(report.rosters)} symmetric=${symmetric} bothListedInOneRoster=${bothListed} distinctColoursPerRoster=${distinct} sameColourForSamePeerAcrossRosters=${agreed}`);
}

report.baseline = [];
for (const session of sessions) report.baseline.push(await witness(session));
save();
record("1e-baseline-witness", true, JSON.stringify(report.baseline));

// 2 — live edit A→B
if (wanted("2")) {
  const before = await witness(B, false);
  const authored = await authorEdit(A, "addFeature");
  const propagated = await awaitPropagation(B, { ledgerCount: before.ledgerCount, edits: before.edits, canvas: before.canvas });
  const afterB = await witness(B);
  report.step2 = { authoredByA: authored, bBefore: before, propagated, bAfter: afterB };
  save();
  record("2-live-edit-A-to-B", authored.mutated && propagated.changed, `A ${authored.mutated ? "mutated" : "NO-OP"} (${authored.newRow ?? authored.submitted}); B ${propagated.changed ? `changed after ${propagated.waitedMs}ms` : `UNCHANGED after ${propagated.waitedMs}ms canvasChanged=${propagated.canvasChanged}`}`);
  await shot(B, "step2-b-sees-a");
}

// 3 — live edit B→A
if (wanted("3")) {
  const before = await witness(A, false);
  const authored = await authorEdit(B, "addFeature");
  const propagated = await awaitPropagation(A, { ledgerCount: before.ledgerCount, edits: before.edits, canvas: before.canvas });
  report.step3 = { authoredByB: authored, aBefore: before, propagated, aAfter: await witness(A) };
  save();
  record("3-live-edit-B-to-A", authored.mutated && propagated.changed, `B ${authored.mutated ? "mutated" : "NO-OP"} (${authored.newRow ?? authored.submitted}); A ${propagated.changed ? `changed after ${propagated.waitedMs}ms` : `UNCHANGED after ${propagated.waitedMs}ms`}`);
  await shot(A, "step3-a-sees-b");
}

// 4 — per-user undo: A's undo reverts A's edit only, B's stays; then redo
if (wanted("4")) {
  const aBefore = await witness(A, false);
  const bBefore = await witness(B, false);
  const undo = await click(A.page, '[id="action.undo"]');
  await A.page.waitForTimeout(8_000);
  const aUndone = await witness(A, false);
  const bAfterUndo = await witness(B, false);
  const redo = await click(A.page, '[id="action.redo"]');
  await A.page.waitForTimeout(8_000);
  const aRedone = await witness(A, false);
  const bAfterRedo = await witness(B, false);
  report.step4 = { undo, redo, aBefore, aUndone, aRedone, bBefore, bAfterUndo, bAfterRedo };
  save();
  const aReverted = aUndone.edits < aBefore.edits || aUndone.ledgerHash !== aBefore.ledgerHash;
  const bKept = bAfterUndo.ledgerCount >= bBefore.ledgerCount - 1;
  const aRestored = aRedone.edits === aBefore.edits || aRedone.ledgerHash === aBefore.ledgerHash;
  record("4-per-user-undo-redo", aReverted && bKept && aRestored, `undo=${undo} redo=${redo} A ${aBefore.edits}→${aUndone.edits}→${aRedone.edits} edits, ledger ${aBefore.ledgerCount}→${aUndone.ledgerCount}→${aRedone.ledgerCount}; B ledger ${bBefore.ledgerCount}→${bAfterUndo.ledgerCount}→${bAfterRedo.ledgerCount}`);
}

// 6 — short connection loss on B, then catch-up to A's interim edits
if (wanted("6")) {
  const bBefore = await witness(B, false);
  await B.context.setOffline(true);
  const offlineAt = ms();
  await B.page.waitForTimeout(10_000);
  const interim = await authorEdit(A, "addFeature");
  const bWhileOffline = await witness(B, false);
  await B.context.setOffline(false);
  const propagated = await awaitPropagation(B, { ledgerCount: bBefore.ledgerCount, edits: bBefore.edits, canvas: bBefore.canvas }, 90_000);
  report.step6 = { offlineAt, bBefore, interim, bWhileOffline, propagated, bAfter: await witness(B, false) };
  save();
  record("6-connection-loss-catch-up", interim.mutated && propagated.changed, `B offline 10s; A authored ${interim.mutated ? "1 edit" : "NOTHING"}; B after reconnect ${propagated.changed ? `caught up in ${propagated.waitedMs}ms (ledger ${bBefore.ledgerCount}→${propagated.ledgerCount})` : `STILL BEHIND after ${propagated.waitedMs}ms`}`);
}

// 7 — two-writer convergence: 10 concurrent edits each, then compare
if (wanted("7")) {
  const burst = async (session, count) => {
    const rows = [];
    for (let index = 0; index < count; index += 1) rows.push(await authorEdit(session, "addFeature"));
    return rows;
  };
  const [aRows, bRows] = await Promise.all([burst(A, 10), burst(B, 10)]);
  await A.page.waitForTimeout(30_000);
  const aFinal = await witness(A);
  const bFinal = await witness(B);
  report.step7 = {
    aApplied: aRows.filter((r) => r.mutated).length,
    bApplied: bRows.filter((r) => r.mutated).length,
    aFinal,
    bFinal,
    ledgerMatch: aFinal.ledgerHash === bFinal.ledgerHash,
    inspectorMatch: JSON.stringify(aFinal.inspector) === JSON.stringify(bFinal.inspector),
    canvasMatch: aFinal.canvas === bFinal.canvas,
  };
  save();
  record("7-two-writer-convergence", report.step7.ledgerMatch || report.step7.inspectorMatch, `A applied ${report.step7.aApplied}/10, B applied ${report.step7.bApplied}/10; ledgerHash ${aFinal.ledgerHash} vs ${bFinal.ledgerHash} match=${report.step7.ledgerMatch}; inspector match=${report.step7.inspectorMatch}; canvas match=${report.step7.canvasMatch}`);
}

// 8 — reload B, re-attach, same document
if (wanted("8")) {
  const before = await witness(B);
  await B.page.reload({ waitUntil: "domcontentloaded", timeout: 180_000 });
  let shell = null;
  for (let attempt = 0; attempt < 200; attempt += 1) {
    await B.page.waitForTimeout(1_000);
    shell = await read(B.page);
    if (shell.error || (shell.ready && attempt > 6)) break;
  }
  await click(B.page, '[data-slot="panel-tab-button"][id="framework.panel.history"]');
  await B.page.waitForTimeout(1_500);
  const socketsBefore = B.sockets.length;
  const reattach = await attachRemote(B.page);
  await B.page.waitForTimeout(6_000);
  const after = await witness(B);
  report.step8 = { before, reload: { ready: shell?.ready, error: shell?.error }, reattach, socketsBefore, socketsAfter: B.sockets.length, after };
  save();
  // the witness is the DOCUMENT's own state, never the History ledger: that ledger is per SESSION
  // and correctly restarts empty after a reload, so comparing it scores a working re-attach FAIL
  // (C3 §5). The document's extents are what "re-attached to the same state" means.
  const sameDocument = JSON.stringify(after.inspector) === JSON.stringify(before.inspector);
  report.step8.sameDocument = sameDocument;
  report.step8.canvasMatch = after.canvas === before.canvas;
  save();
  record("8-reload-reattach", B.sockets.length > socketsBefore && sameDocument, `reload ready=${shell?.ready}; new socket=${B.sockets.length > socketsBefore}; document extents ${sameDocument ? "IDENTICAL" : "DIFFER"} ${JSON.stringify(before.inspector)} vs ${JSON.stringify(after.inspector)}; canvas ${before.canvas} vs ${after.canvas}; ledger ${before.ledgerCount}→${after.ledgerCount} (per-session, not a witness)`);
  await shot(B, "step8-reattached");
}

// 9 — a mid-edit hub RESTART: the hold is stopped by pid and a SECOND hub process is started from
// the same data root and binary, reusing the already-published catalog. The claim is recovery of a
// LIVE editing session, so an edit is in flight when the hub goes away and both clients must author
// again afterwards.
if (wanted("9")) {
  const script = process.env.C3_HUB_RESTART;
  if (!script) record("9-mid-edit-hub-restart", null, "C3_HUB_RESTART not set");
  else {
    const { execFileSync } = await import("node:child_process");
    const before = [await witness(A, false), await witness(B, false)];
    const midEdit = await authorEdit(A, "addFeature");
    let restart = "";
    let restartError = null;
    const startedAt = Date.now();
    try {
      restart = execFileSync("zsh", [script], { encoding: "utf8", timeout: 420_000 }).trim();
    } catch (error) {
      restartError = String(error).split("\n")[0].slice(0, 200);
    }
    const restartMs = Date.now() - startedAt;
    await A.page.waitForTimeout(10_000);
    // a restarted hub drops both document sockets; the clients re-attach through the product's own
    // sync card, exactly as a human would after the backend came back
    const reattach = [];
    for (const session of sessions) {
      const socketsBefore = session.sockets.length;
      reattach.push({ user: session.user.label, socketsBefore, result: await attachRemote(session.page, 30_000), socketsAfter: session.sockets.length });
    }
    const resumedA = await authorEdit(A, "addFeature");
    const resumedB = await authorEdit(B, "addFeature");
    const after = [await witness(A), await witness(B)];
    report.step9 = { midEdit, restart, restartError, restartMs, reattach, resumedA, resumedB, before, after };
    save();
    const resumed = resumedA.mutated && resumedB.mutated;
    const converged = after[0].ledgerHash === after[1].ledgerHash || JSON.stringify(after[0].inspector) === JSON.stringify(after[1].inspector);
    record("9-mid-edit-hub-restart", restartError === null && restart.includes("readyz=200") && resumed, `restart ${restartError ?? restart} in ${restartMs}ms; mid-edit ${midEdit.mutated ? "authored" : "NO-OP"}; after restart A ${resumedA.mutated ? "edits" : "CANNOT EDIT"} B ${resumedB.mutated ? "edits" : "CANNOT EDIT"}; converged=${converged}; sockets ${reattach.map((r) => `${r.user}:${r.socketsBefore}\u2192${r.socketsAfter}`).join(" ")}`);
    for (const session of sessions) await shot(session, "step9-after-restart");
  }
}

const faults = sessions.flatMap((s) => s.lines.filter((line) => FAULT.test(line) && !NOISE.test(line)).map((line) => `${s.user.label}: ${line}`));
report.faults = faults.slice(0, 80);
report.sockets = sessions.map((s) => ({ user: s.user.label, sockets: s.sockets }));
report.finishedAt = new Date().toISOString();
save();
writeFileSync(join(OUT, `${TAG}-collab-scenario-console.txt`), sessions.flatMap((s) => s.lines.map((line) => `${s.user.label} ${line}`)).join("\n"));
console.log(`FAULTS ${faults.length}`);
for (const fault of faults.slice(0, 25)) console.log(`  ${fault}`);
console.log(`SOCKETS ${JSON.stringify(report.sockets)}`);
await browser.close();
