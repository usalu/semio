/** 👥️ PR1 — the presence roster over the hub, sampled over time in BOTH contexts at once.
 *
 * C3 §3.4 left three unexplained facts: `user1`'s roster listed both peers while `user2`'s listed
 * only itself, in another run the asymmetry reversed, and read at the END of a run the roster was
 * EMPTY in both contexts although both document sockets were still open. A single reading at attach
 * time cannot tell a join-replay gap from a lease decay, so this probe reads both rosters on a fixed
 * cadence for a fixed window and prints the whole series — then closes ONE context and keeps reading
 * the other, so removal latency is measured rather than assumed.
 *
 * Usage: bun 🐍️pr1-presence-probe.mjs [shellUrl] [hubHostPort] [spaceId] [documentId]
 * Env:   PR1_TAG (capture prefix, default "pr1"), PR1_WINDOW_MS (default 120000),
 *        PR1_SAMPLE_MS (default 5000), PR1_CLOSE_WINDOW_MS (default 40000)
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join } from "node:path";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6195";
const HUB = process.argv[3] ?? "127.0.0.1:7611";
const SPACE = process.argv[4] ?? "01a0c00f-4f3c-7834-a7e6-2ccf9de925db";
const DOCUMENT = process.argv[5] ?? "artifact-2fb248125b8b2b4d56de25933d30ed21";
const TAG = process.env.PR1_TAG ?? "pr1";
const WINDOW_MS = Number(process.env.PR1_WINDOW_MS ?? 120_000);
const SAMPLE_MS = Number(process.env.PR1_SAMPLE_MS ?? 5_000);
const CLOSE_WINDOW_MS = Number(process.env.PR1_CLOSE_WINDOW_MS ?? 40_000);
const ATTACH_SETTLE_MS = Number(process.env.PR1_ATTACH_SETTLE_MS ?? 45_000);
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
mkdirSync(OUT, { recursive: true });

const USERS = [
  { label: "user1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1" },
  { label: "user2", email: "user2@semio.dev", password: "gm1-local-dev-pass-2" },
];

const t0 = Date.now();
const ms = () => Date.now() - t0;
const report = { shell: SHELL, hub: HUB, space: SPACE, document: DOCUMENT, windowMs: WINDOW_MS, sampleMs: SAMPLE_MS, startedAt: new Date().toISOString(), samples: [], verdicts: [] };
const save = () => writeFileSync(join(OUT, `${TAG}-presence-probe.json`), JSON.stringify(report, null, 2));
const verdict = (name, pass, detail) => {
  report.verdicts.push({ name, pass, at: ms(), detail });
  console.log(`VERDICT ${name}: ${pass === null ? "SKIP" : pass ? "PASS" : "FAIL"} — ${detail}`);
  save();
};

/** 🪞️ The roster exactly as the shell's own chrome renders it, plus the live socket count. */
const roster = (page) =>
  page.evaluate(() => {
    const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
    const nodes = [...document.querySelectorAll('[id="s-presence-peers"] [data-row-id^="peer:"]:not([data-row-id="peer:overflow"])')];
    return {
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      peers: nodes.map((el) => el.getAttribute("data-row-id")),
      labels: nodes.map((el) => el.getAttribute("aria-label") ?? text(el)),
      kinds: nodes.map((el) => el.getAttribute("data-presence-kind")),
      colors: nodes.map((el) => {
        const probe = el.querySelector("[style*='border']") ?? el.firstElementChild ?? el;
        return getComputedStyle(probe).borderTopColor;
      }),
      syncPill: text(document.querySelector('[id="s-sync-status"]')),
    };
  });

const click = async (page, selector, timeout = 8_000) => {
  if (!(await page.locator(selector).count())) return "absent";
  return page.locator(selector).first().click({ timeout, force: true }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 120));
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

async function attachRemote(page, settleMs = ATTACH_SETTLE_MS) {
  const opened = await openRemoteCard(page);
  if (!opened.ok) return { card: "absent", trail: opened.trail };
  const input = page.locator('[id="framework.sync.remote.path"]');
  await input.fill(`${HUB}/${SPACE}/${DOCUMENT}`);
  await page.waitForTimeout(400);
  const attach = input.locator("xpath=ancestor::*[@data-slot='popover-content'][1]").locator('button:has([data-icon="link"])');
  const pressed = (await attach.count()) ? await attach.first().click({ force: true, timeout: 8_000 }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 90)) : "absent";
  await page.waitForTimeout(settleMs);
  await page.keyboard.press("Escape").catch(() => {});
  await page.waitForTimeout(800);
  return { card: "ok", trail: opened.trail, attach: pressed };
}

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const sessions = [];
for (const user of USERS) {
  const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
  const page = await context.newPage();
  const lines = [];
  const sockets = [];
  page.on("console", (message) => lines.push(`${ms()} ${message.type()} ${message.text().slice(0, 500)}`));
  page.on("pageerror", (error) => lines.push(`${ms()} pageerror ${String(error).slice(0, 500)}`));
  page.on("response", (response) => {
    const url = response.url();
    if (!url.includes("/_semio/hub") && !url.includes(HUB)) return;
    lines.push(`${ms()} hub ${response.status()} ${response.request().method()} ${url.replace(/^https?:\/\/[^/]+/, "")}`);
  });
  page.on("websocket", (ws) => {
    if (!ws.url().includes("/socket/v1")) return;
    sockets.push({ url: ws.url(), openedAt: ms(), closedAt: null });
    ws.on("close", () => {
      const row = sockets.find((s) => s.url === ws.url() && s.closedAt === null);
      if (row) row.closedAt = ms();
    });
  });
  await page.goto(`${SHELL}/?plugin=gis2d`, { waitUntil: "domcontentloaded", timeout: 180_000 });
  let shell = null;
  for (let attempt = 0; attempt < 200; attempt += 1) {
    await page.waitForTimeout(1_000);
    shell = await roster(page);
    if (shell.ready && attempt > 6) break;
  }
  sessions.push({ user, context, page, lines, sockets, boot: shell });
}
const [A, B] = sessions;
console.log(`BOOT ${sessions.map((s) => `${s.user.label}:ready=${s.boot?.ready}`).join(" ")}`);

for (const session of sessions) {
  await signIn(session.page, session.user.email, session.user.password).catch((error) => {
    session.signInError = String(error).split("\n")[0].slice(0, 160);
  });
  await session.page.waitForTimeout(2_000);
}
console.log(`SIGNIN ${sessions.map((s) => `${s.user.label}:${s.signInError ?? "ok"}`).join(" ")}`);

// 🕰️ A attaches FIRST and is left to settle, so B is a genuine LATE joiner — the exact shape C3's
// asymmetry needs: A already has a live socket and a published roster row before B ever connects.
A.attach = await attachRemote(A.page);
report.attachGapMs = 0;
const aAttachedAt = ms();
await A.page.waitForTimeout(20_000);
B.attach = await attachRemote(B.page);
report.attachGapMs = ms() - aAttachedAt;
console.log(`ATTACH ${sessions.map((s) => `${s.user.label}:${s.attach.attach ?? s.attach.card} sockets=${s.sockets.length}`).join(" ")} lateJoinerGapMs=${report.attachGapMs}`);

const sample = async (phase) => {
  const rows = [];
  for (const session of sessions) {
    if (session.closed) {
      rows.push({ user: session.user.label, closed: true });
      continue;
    }
    rows.push({ user: session.user.label, ...(await roster(session.page)) });
  }
  const row = { at: ms(), phase, rows };
  report.samples.push(row);
  console.log(`SAMPLE ${String(row.at).padStart(7)} ${phase} ${rows.map((r) => (r.closed ? `${r.user}:CLOSED` : `${r.user}:[${r.peers.map((p) => p.replace("peer:", "").slice(0, 12)).join(",")}] colors=[${r.colors.join(",")}] kinds=[${r.kinds.join(",")}] pill="${r.syncPill}"`)).join(" | ")}`);
  save();
  return row;
};

const windowStart = ms();
while (ms() - windowStart < WINDOW_MS) {
  await sample("both-attached");
  await A.page.waitForTimeout(SAMPLE_MS);
}

// 🧹️ Close ONE context and keep reading the other: removal latency, measured.
B.closed = true;
await B.context.close();
const closedAt = ms();
report.closedAt = closedAt;
let droppedAt = null;
while (ms() - closedAt < CLOSE_WINDOW_MS) {
  const row = await sample("after-close-b");
  const a = row.rows.find((r) => r.user === "user1");
  if (droppedAt === null && a && !a.closed && a.peers.length <= 1) droppedAt = row.at;
  await A.page.waitForTimeout(SAMPLE_MS);
}
report.removalLatencyMs = droppedAt === null ? null : droppedAt - closedAt;

// ── verdicts ───────────────────────────────────────────────────────────────────────────────────
const attachedSamples = report.samples.filter((row) => row.phase === "both-attached");
const bothEverywhere = attachedSamples.filter((row) => row.rows.every((r) => r.peers.length === 2));
verdict(
  "symmetric-roster-for-the-whole-window",
  attachedSamples.length > 0 && bothEverywhere.length === attachedSamples.length,
  `${bothEverywhere.length}/${attachedSamples.length} samples list BOTH peers in BOTH contexts; per-sample sizes ${JSON.stringify(attachedSamples.map((row) => row.rows.map((r) => r.peers.length)))}`,
);
const everNonEmpty = attachedSamples.filter((row) => row.rows.some((r) => r.peers.length > 0));
const decayed = everNonEmpty.length > 0 && attachedSamples.slice(-1)[0].rows.every((r) => r.peers.length === 0);
verdict("no-decay-while-the-socket-is-open", everNonEmpty.length > 0 && !decayed, `non-empty in ${everNonEmpty.length}/${attachedSamples.length} samples; last sample sizes ${JSON.stringify(attachedSamples.slice(-1)[0]?.rows.map((r) => r.peers.length) ?? [])}`);
const colourSamples = attachedSamples.filter((row) => row.rows.some((r) => r.peers.length === 2));
const colourDistinct = colourSamples.every((row) => row.rows.every((r) => r.peers.length < 2 || new Set(r.colors).size === r.colors.length));
verdict("distinct-colour-per-session", colourSamples.length > 0 && colourDistinct, `${colourSamples.length} samples carried a two-peer roster; every one distinct=${colourDistinct}; colours ${JSON.stringify(colourSamples.map((row) => row.rows.map((r) => r.colors)))}`);
verdict("removal-within-bound-after-close", report.removalLatencyMs !== null, `user1 roster dropped user2 after ${report.removalLatencyMs ?? "never (within " + CLOSE_WINDOW_MS + " ms)"} ms`);
verdict("sockets-stayed-open", sessions.every((s) => s.sockets.some((row) => row.url.includes("/documents/") && row.url.includes("/socket/v1"))), JSON.stringify(sessions.map((s) => ({ user: s.user.label, sockets: s.sockets }))));

report.finishedAt = new Date().toISOString();
save();
for (const session of sessions) writeFileSync(join(OUT, `${TAG}-console-${session.user.label}.txt`), session.lines.join("\n"));
await A.context.close().catch(() => {});
await browser.close();
console.log(`DONE samples=${report.samples.length} report=${TAG}-presence-probe.json`);
