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


const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
const page = await context.newPage();
const lines = [];
page.on("console", (m) => lines.push(`${ms()} ${m.type()} ${m.text().slice(0, 600)}`));
await page.goto(`${SHELL}/?plugin=gis2d`, { waitUntil: "domcontentloaded", timeout: 180_000 });
for (let i = 0; i < 200; i++) { await page.waitForTimeout(1000); const r = await read(page); if (r.error || (r.ready && i > 6)) break; }
for (const toggle of await page.locator('[id$=".engagement.toggle"]').all()) { await toggle.click({ timeout: 6000, force: true }).catch(() => {}); await page.waitForTimeout(700); }
console.log("ids", JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[id]")].map((e) => e.id).filter((id) => /ommit|heckpoint/.test(id)))));
console.log("click", await click(page, '[id="action.commitCheckpoint"]'));
await page.waitForTimeout(3000);
console.log("exec", await click(page, '[id$=".action.commitCheckpoint.execute"]'));
await page.waitForTimeout(8000);
writeFileSync(join(OUT, `${TAG}-console.txt`), lines.join("\n"));
await browser.close();
