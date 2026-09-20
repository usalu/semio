/** 🤝️ C2 — TWO signed-in humans, two browser contexts, ONE hub document.
 *
 * Reaches the shared document through the shell's own `remote://` sync attach (the path C1c fixed and
 * could not observe while `features.openPlan` was false), not through the Home/Space tables, whose
 * directory projection is blocked on the receipt defect in §2.4 of the slice report.
 *
 * Usage: bun 🐍️c2-shared-document.mjs <shellUrl> <hubHostPort> <spaceId> <documentId> [phase]
 */
import { fileURLToPath } from "node:url";
import { mkdirSync, writeFileSync } from "node:fs";
import { chromium } from "playwright";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6191";
const HUB = process.argv[3] ?? "127.0.0.1:7611";
const SPACE = process.argv[4] ?? "01a0c00f-4f3c-7834-a7e6-2ccf9de925db";
const DOCUMENT = process.argv[5] ?? "artifact-2fb248125b8b2b4d56de25933d30ed21";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
mkdirSync(OUT, { recursive: true });

const USERS = [
  { label: "user1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1" },
  { label: "user2", email: "user2@semio.dev", password: "gm1-local-dev-pass-2" },
];
const FAULT = /unreachable|trapped|panicked|fault|refused|denied|dispatch-failed|invalid-args|not-ui-safe/i;
const NOISE = /staged plugin module\(s\) are behind their source|\[stale\]|status of 404|Download the React DevTools|typed-operation slots|ERR_CONNECTION_REFUSED/;

const report = { shell: SHELL, hub: HUB, space: SPACE, document: DOCUMENT, startedAt: new Date().toISOString(), steps: [] };
const save = () => writeFileSync(`${OUT}c2-shared-document.json`, JSON.stringify(report, null, 2));
const record = (step, pass, detail) => {
  report.steps.push({ step, pass, detail });
  console.log(`STEP ${step}: ${pass ? "PASS" : "FAIL"} — ${typeof detail === "string" ? detail : JSON.stringify(detail)}`);
  save();
};

const read = (page) =>
  page.evaluate(() => {
    const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
    const visible = (el) => el instanceof HTMLElement && el.offsetParent !== null;
    return {
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      error: document.documentElement.getAttribute("data-semio-os-error"),
      ledger: [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((el) => !el.id.endsWith(".revert")).map((el) => text(el).slice(0, 72)),
      checkin: text(document.querySelector("#s-checkin")),
      syncStatus: text(document.querySelector("[data-semio-sync-status]")),
      executionTarget: [...document.querySelectorAll("[data-semio-execution-target-status]")].map((el) => text(el).slice(0, 90)),
      syncPill: text(document.querySelector('[id="s-sync-status"]')),
      peers: [...document.querySelectorAll('[id="s-presence-peers"] [data-row-id^="peer:"]:not([data-row-id="peer:overflow"])')].map((el) => el.getAttribute("data-row-id")),
      peerColors: [...document.querySelectorAll('[id="s-presence-peers"] [data-row-id^="peer:"]:not([data-row-id="peer:overflow"])')].map((el) => getComputedStyle(el.firstElementChild ?? el).borderTopColor),
      panelRows: [...document.querySelectorAll('[data-slot="panel"]')]
        .filter((el) => visible(el) && !el.id.includes("framework.panel.history"))
        .flatMap((panel) => [...panel.querySelectorAll('[role="treeitem"]')].map((el) => text(el).slice(0, 72))),
      actions: [...new Set([...document.querySelectorAll('[id^="action."]')].map((el) => el.id))],
    };
  });

const edits = (shell) => {
  const match = /\((\d+)\)\s*$/.exec(shell.checkin ?? "");
  return match === null ? 0 : Number(match[1]);
};

const click = async (page, selector, timeout = 8_000) => {
  if (!(await page.locator(selector).count())) return "absent";
  return page
    .locator(selector)
    .first()
    .click({ timeout, force: true })
    .then(() => "ok")
    .catch((error) => String(error).split("\n")[0].slice(0, 120));
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
    if (await page.locator('[id="framework.sync.remote"]').count()) {
      trail.push(`remote:${await click(page, '[id="framework.sync.remote"]')}`);
    } else if (await page.locator('[id="ui.utilities.group.sync"]').count()) {
      trail.push(`group:${await click(page, '[id="ui.utilities.group.sync"]')}`);
    } else {
      trail.push(`tab:${await click(page, '[data-slot="panel-tab-button"][id="s-sync-status"], [id="s-sync-status"]')}`);
    }
    await page.waitForTimeout(1_500);
  }
  return { ok: (await page.locator('[id="framework.sync.remote.path"]').count()) > 0, trail };
}

/** 🌐️ Drives the shell's own sync card: footer sync tab ▸ `sync` collection ▸ Remote ▸
 * `<host>/<space>/<document>` ▸ Attach. */
async function attachRemote(page) {
  const opened = await openRemoteCard(page);
  if (!opened.ok) return { card: "absent", trail: opened.trail };
  const input = page.locator('[id="framework.sync.remote.path"]');
  await input.fill(`${HUB}/${SPACE}/${DOCUMENT}`);
  await page.waitForTimeout(400);
  const attach = input.locator("xpath=ancestor::*[@data-slot='popover-content'][1]").locator('button:has([data-icon="link"])');
  const pressed = (await attach.count()) ? await attach.first().click({ force: true, timeout: 8_000 }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 90)) : "absent";
  await page.waitForTimeout(45_000);
  return { card: "ok", trail: opened.trail, value: `${HUB}/${SPACE}/${DOCUMENT}`, attach: pressed };
}

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const sessions = [];
for (const user of USERS) {
  const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
  const page = await context.newPage();
  const lines = [];
  const sockets = [];
  page.on("console", (message) => lines.push(`${message.type()} ${message.text().slice(0, 600)}`));
  page.on("pageerror", (error) => lines.push(`pageerror ${String(error).slice(0, 600)}`));
  page.on("response", (response) => {
    const url = response.url();
    if (url.includes("open-plan") || url.includes("socket-grants") || url.includes("execution-target")) lines.push(`response ${response.status()} ${url.slice(url.indexOf("/documents/"))}`);
  });
  page.on("websocket", (ws) => {
    sockets.push(ws.url());
    ws.on("close", () => lines.push(`ws-closed ${ws.url()}`));
  });
  await page.goto(`${SHELL}/?plugin=gis2d`, { waitUntil: "domcontentloaded", timeout: 180_000 });
  let shell = null;
  for (let attempt = 0; attempt < 180; attempt += 1) {
    await page.waitForTimeout(1_000);
    shell = await read(page);
    if (shell.error || (shell.ready && attempt > 6)) break;
  }
  sessions.push({ user, context, page, lines, sockets, boot: shell });
}
record("boot", sessions.every((s) => s.boot?.ready && !s.boot?.error), sessions.map((s) => `${s.user.label}:ready=${s.boot?.ready} error=${s.boot?.error ?? "none"}`).join(" "));

for (const session of sessions) {
  await signIn(session.page, session.user.email, session.user.password);
  await session.page.waitForTimeout(2_000);
}
record(
  "sign-in",
  await Promise.all(sessions.map(async (s) => (await s.page.locator('[data-semio-hub-sign-in=""]').count()) === 0)).then((flags) => flags.every(Boolean)),
  "both contexts hold a verified hub session authority",
);

for (const session of sessions) {
  session.attach = await attachRemote(session.page);
  await session.page.waitForTimeout(4_000);
}
for (const session of sessions) session.afterAttach = await read(session.page);
record(
  "attach",
  sessions.every((s) => s.sockets.some((url) => url.includes("/documents/") && url.includes("/socket/v1"))),
  sessions.map((s) => `${s.user.label}: ${JSON.stringify(s.attach)} sync=${JSON.stringify(s.afterAttach.syncStatus)} sockets=${JSON.stringify(s.sockets.filter((u) => !u.includes("?token=") && !u.endsWith("/bridge")))}`).join(" | "),
);

for (const session of sessions) {
  await session.page.screenshot({ path: `${OUT}c2-shared-${session.user.label}-attached.png` });
}

report.afterAttach = sessions.map((s) => ({ user: s.user.label, edits: edits(s.afterAttach), ledger: s.afterAttach.ledger.length, panelRows: s.afterAttach.panelRows.slice(0, 8), peers: s.afterAttach.peers, actions: s.afterAttach.actions.filter((id) => /Feature|undo|redo/.test(id)) }));
save();

const faults = sessions.flatMap((s) => s.lines.filter((line) => FAULT.test(line) && !NOISE.test(line)).map((line) => `${s.user.label}: ${line}`));
report.faults = faults.slice(0, 60);
console.log(`FAULTS ${faults.length}`);
for (const fault of faults.slice(0, 20)) console.log(`  ${fault}`);
save();
writeFileSync(`${OUT}c2-shared-document-console.txt`, sessions.flatMap((s) => s.lines.map((line) => `${s.user.label} ${line}`)).join("\n"));
await browser.close();
