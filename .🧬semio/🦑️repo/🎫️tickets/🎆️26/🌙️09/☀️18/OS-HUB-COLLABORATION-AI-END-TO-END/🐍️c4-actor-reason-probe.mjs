/** 🩻️ C4 — read the browser-actor child's own rejection reason on the live system.
 *
 * One browser context, one named human, the C3 attach chain, and then the ONE thing C3 could not
 * read: the typed reason the document's browser-actor child now sends with its `rejected` frame,
 * surfaced by the store worker on the execution-target status and rendered by the shell as
 * `data-semio-execution-target-diagnostic`.
 *
 * Usage: bun 🐍️c4-actor-reason-probe.mjs [user1|user2] [shellUrl] [hubHostPort] [spaceId] [documentId]
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join } from "node:path";

const WHO = process.argv[2] ?? "user1";
const SHELL = process.argv[3] ?? "http://127.0.0.1:6191";
const HUB = process.argv[4] ?? "127.0.0.1:7611";
const SPACE = process.argv[5] ?? "01a0c00f-4f3c-7834-a7e6-2ccf9de925db";
const DOCUMENT = process.argv[6] ?? "artifact-2fb248125b8b2b4d56de25933d30ed21";
const TAG = process.env.C4_TAG ?? "c4";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
mkdirSync(OUT, { recursive: true });

const USERS = {
  user1: { email: "user1@semio.dev", password: "gm1-local-dev-pass-1" },
  user2: { email: "user2@semio.dev", password: "gm1-local-dev-pass-2" },
};
const user = USERS[WHO];
const t0 = Date.now();
const ms = () => Date.now() - t0;
const out = { who: WHO, email: user.email, statuses: [], notes: [] };
const save = () => writeFileSync(join(OUT, `${TAG}-actor-reason-${WHO}.json`), JSON.stringify(out, null, 2));

const click = async (page, selector, timeout = 8_000) => {
  if (!(await page.locator(selector).count())) return "absent";
  return page.locator(selector).first().click({ timeout, force: true }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 110));
};

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
const page = await context.newPage();
const lines = [];
page.on("console", (message) => lines.push(`${ms()} ${message.type()} ${message.text().slice(0, 700)}`));
page.on("pageerror", (error) => lines.push(`${ms()} pageerror ${String(error).slice(0, 300)}\n${(error.stack ?? "<no stack>").slice(0, 1_600)}`));
page.on("websocket", (ws) => {
  lines.push(`${ms()} ws-open ${ws.url().slice(0, 160)}`);
  ws.on("close", () => lines.push(`${ms()} ws-close ${ws.url().slice(0, 160)}`));
});
// A dedicated worker that stops answering and a dedicated worker that DIED look identical from the
// page: both leave the last status row on screen forever. Playwright reports the page's own workers,
// so the store worker's death is a recorded event rather than an inference.
page.on("worker", (worker) => {
  lines.push(`${ms()} worker-open ${worker.url().slice(0, 400)}`);
  worker.on("close", () => lines.push(`${ms()} worker-close ${worker.url().slice(0, 400)}`));
});

await page.goto(`${SHELL}/?plugin=gis2d`, { waitUntil: "domcontentloaded", timeout: 180_000 });
for (let attempt = 0; attempt < 200; attempt += 1) {
  await page.waitForTimeout(1_000);
  const ready = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"));
  if (ready && attempt > 6) break;
}

await page.locator('[data-semio-hub-sign-in=""]').first().click();
const form = page.locator("[data-semio-hub-workspace]");
await form.waitFor({ state: "visible", timeout: 30_000 });
await form.locator('input[type="email"]').fill(user.email);
await form.locator('input[type="password"]').fill(user.password);
await form.locator('button[type="submit"][aria-label="Sign in"]').click();
await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 }).catch((error) => out.notes.push(`sign-in wait: ${String(error).split("\n")[0]}`));
await page.waitForTimeout(3_000);
await click(page, '[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]');
await page.waitForTimeout(2_000);

for (let attempt = 0; attempt < 6; attempt += 1) {
  if (await page.locator('[id="framework.sync.remote.path"]').count()) break;
  if (await page.locator('[id="framework.sync.remote"]').count()) await click(page, '[id="framework.sync.remote"]');
  else if (await page.locator('[id="ui.utilities.group.sync"]').count()) await click(page, '[id="ui.utilities.group.sync"]');
  else await click(page, '[data-slot="panel-tab-button"][id="s-sync-status"], [id="s-sync-status"]');
  await page.waitForTimeout(1_500);
}
const input = page.locator('[id="framework.sync.remote.path"]');
out.cardFound = (await input.count()) > 0;
if (out.cardFound) {
  await input.fill(`${HUB}/${SPACE}/${DOCUMENT}`);
  await page.waitForTimeout(400);
  const attach = input.locator("xpath=ancestor::*[@data-slot='popover-content'][1]").locator('button:has([data-icon="link"])');
  out.attachPressed = (await attach.count()) ? await attach.first().click({ force: true, timeout: 8_000 }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 90)) : "absent";
}

// The execution target and the DOCUMENT BOOTSTRAP are two different live regions: the actor can be
// active while the document's own cold pair has never arrived, and only the bootstrap region says so.
// It is polled with the status, because it is cleared the moment the snapshot is replaced.
const readStatus = () =>
  page.evaluate(() =>
    [...document.querySelectorAll("[data-semio-bootstrap-status]"), ...document.querySelectorAll("[data-semio-execution-target-status]")].map((el) => ({
      text: (el.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 160),
      diagnostic: el.getAttribute("data-semio-execution-target-diagnostic"),
      stage: el.getAttribute("data-semio-execution-target-stage"),
      bootstrap: el.hasAttribute("data-semio-bootstrap-status") || undefined,
      progress: (() => {
        const bar = el.querySelector("progress");
        return bar ? `${bar.value}/${bar.max}` : null;
      })(),
    })),
  );

// Terminal states, read from the product's own vocabulary (`DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1`,
// `📇️directory/🧬️schema/🟦️.ts:1693`): `verifying` is the only non-terminal code, every other code is
// an `alert`, and SUCCESS has no code at all — `reduceExecutionTargetUiState` DELETES the row on
// `execution-target-cleared`. So "the actor reached active" is: a `verifying` notice was seen and the
// notice is then gone, with no diagnostic and no alert ever rendered.
const VERIFYING = /Verifying document component|Dokumentkomponente wird/u;
const RENDERER_UNAVAILABLE = /this renderer is unavailable|dieser Renderer ist nicht verf/u;
const deadline = Date.now() + Number(process.env.C4_WAIT_MS ?? 90_000);
let seen = null;
let sawVerifying = false;
let sawRendererUnavailable = false;
let cleared = false;
while (Date.now() < deadline) {
  await page.waitForTimeout(1_500);
  const rows = await readStatus();
  const stamped = JSON.stringify(rows);
  if (stamped !== JSON.stringify(out.statuses.at(-1)?.rows)) out.statuses.push({ at: ms(), rows });
  if (rows.some((row) => VERIFYING.test(row.text))) sawVerifying = true;
  if (rows.some((row) => row.diagnostic && !row.bootstrap)) {
    seen = rows.find((row) => row.diagnostic && !row.bootstrap);
    break;
  }
  // `renderer-unavailable` is NOT terminal: `documentOpen` emits it the moment the browser-actor
  // lease is admitted (`🏪️store/👷️worker/🟦️.ts:2771`), i.e. BEFORE the component is fetched, and the
  // same code replaces it with `verifying` and then clears it on the first rendered UI patch. Only
  // the three refusals below end the run.
  const alert = rows.find((row) => !row.bootstrap && row.text && !VERIFYING.test(row.text) && !RENDERER_UNAVAILABLE.test(row.text));
  if (alert) {
    seen = alert;
    break;
  }
  if (rows.some((row) => RENDERER_UNAVAILABLE.test(row.text))) sawRendererUnavailable = true;
  if ((sawVerifying || sawRendererUnavailable) && rows.length === 0) {
    cleared = true;
    break;
  }
  // A status row that stops changing is either a thread that stopped running or a promise that will
  // never settle; they are indistinguishable from the page. Asking each of the page's own workers to
  // run one expression and one timer separates them: an answer means the thread is alive, and a
  // timer that resolves means `setTimeout` still fires there, so an armed deadline that never closes
  // the child is a missing deadline and not a wedged event loop.
  if (stamped === JSON.stringify(out.statuses.at(-1)?.rows) && ms() - (out.statuses.at(-1)?.at ?? 0) > Number(process.env.C4_STALL_MS ?? 25_000) && !out.stall) {
    out.stall = { at: ms(), rows, workers: [] };
    for (const worker of page.workers()) {
      const answered = await worker.evaluate(() => ({ now: Date.now(), self: typeof self, timers: typeof setTimeout })).then((value) => value).catch((error) => `unreachable: ${String(error).split("\n")[0].slice(0, 120)}`);
      const timer = await worker.evaluate(() => new Promise((resolve) => setTimeout(() => resolve("timer-fired"), 500))).catch((error) => `timer-unreachable: ${String(error).split("\n")[0].slice(0, 120)}`);
      out.stall.workers.push({ url: worker.url().slice(-90), answered, timer });
      lines.push(`${ms()} worker-probe ${worker.url().slice(-70)} answered=${JSON.stringify(answered)} timer=${JSON.stringify(timer)}`);
    }
    save();
  }
  save();
}
out.diagnostic = seen?.diagnostic ?? null;
out.alertText = seen?.text ?? null;
out.sawVerifying = sawVerifying;
out.sawRendererUnavailable = sawRendererUnavailable;
out.cleared = cleared;
out.final = await readStatus();
out.canvas = await page.evaluate(() => {
  const canvas = document.querySelector("canvas");
  return canvas ? `${canvas.width}x${canvas.height}` : "no-canvas";
});
out.active = cleared && out.diagnostic === null && out.alertText === null;
out.syncPill = await page.evaluate(() => (document.querySelector('[id="s-sync-status"]')?.textContent ?? "").replace(/\s+/g, " ").trim());
// The execution target and the DOCUMENT BOOTSTRAP are two different live regions: the actor can be
// active while the document's own cold pair has never arrived, and only the bootstrap region says so.
out.bootstrap = await page.evaluate(() =>
  [...document.querySelectorAll("[data-semio-bootstrap-status]")].map((el) => ({
    text: (el.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 200),
    progress: (() => {
      const bar = el.querySelector("progress");
      return bar ? `${bar.value}/${bar.max}` : null;
    })(),
  })),
);
save();
writeFileSync(join(OUT, `${TAG}-actor-reason-${WHO}-console.txt`), lines.join("\n"));
await page.screenshot({ path: join(OUT, `${TAG}-actor-reason-${WHO}.png`), fullPage: false }).catch(() => {});
console.log(`WHO ${WHO} card=${out.cardFound} attach=${out.attachPressed}`);
console.log(`STATUS ${JSON.stringify(out.final)}`);
console.log(`DIAGNOSTIC ${out.diagnostic ?? "<none>"}`);
console.log(`ALERT ${out.alertText ?? "<none>"}`);
console.log(`ACTIVE ${out.active} (verifying=${out.sawVerifying} rendererUnavailableSeen=${out.sawRendererUnavailable} cleared=${out.cleared} canvas=${out.canvas})`);
console.log(`SYNC ${out.syncPill}`);
console.log(`BOOTSTRAP ${JSON.stringify(out.bootstrap)}`);
if (out.stall) for (const worker of out.stall.workers) console.log(`STALL-WORKER ${worker.url} answered=${JSON.stringify(worker.answered)} timer=${JSON.stringify(worker.timer)}`);
// Stage timeline: the load phase now names the stage it is in (`data-semio-execution-target-stage`,
// relayed from the child's own frames), so a stall is reported as the stage it stalled in.
let previousAt = 0;
for (const entry of out.statuses) {
  const stage = entry.rows.map((row) => `${row.bootstrap ? "BOOTSTRAP:" + row.text.slice(0, 40) : row.stage ?? "-"} ${row.progress ?? ""}`.trim()).join(" | ");
  console.log(`STAGE ${String(entry.at).padStart(7)}ms (+${String(entry.at - previousAt).padStart(6)}) ${stage || "<cleared>"}`);
  previousAt = entry.at;
}
for (const row of lines.filter((line) => /browser actor|execution-target|integrity/i.test(line)).slice(0, 25)) console.log(`  ${row}`);
await browser.close();
