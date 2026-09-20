/** 🔬️ Slice B3a2 — is block2d's board surface re-published after `undo`?
 *
 * The full-bar run scored `mutated/undone/redone` all true off the ledger and the uncommitted-edit
 * count, but the board's own text (`block2d-play-board.counts`, a pure function of the snapshot —
 * `🪟️windows/📋️board/🦀️.rs:51`) read `7 Handle Kinds` after the mutation AND after undo. That is
 * either a genuinely stale projection or a refresh that simply lands later than the witness read,
 * so this polls the board text for a full budget after each history step instead of sampling once.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { writeFileSync } from "node:fs";

const VARIANT = process.env.SEMIO_BLOCK_VARIANT ?? "block2d";
const PORT = process.env.SEMIO_BLOCK_PORT ?? "6024";
const ACTION = process.env.SEMIO_BLOCK_ACTION ?? "addHandleKind";
const OUT = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/b3a2-${VARIANT}-undo-refresh.txt`;
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => { if (m.type() === "error") lines.push(`console ${Date.now() - t0} ${m.text().slice(0, 600)}`); });
page.on("pageerror", (e) => lines.push(`pageerror ${Date.now() - t0} ${String(e).slice(0, 600)}`));

const read = () => page.evaluate(() => ({
  board: [...document.querySelectorAll("[data-ui-node-key]")].filter((el) => el.closest('[data-slot="window-body"]') !== null && el.closest('[data-slot="window-engagement-overlay"]') === null && el.parentElement?.closest("[data-ui-node-key]") === null).map((el) => (el.innerText ?? "").replace(/\s+/g, " ").trim()).join(" · "),
  checkin: document.querySelector("#s-checkin")?.textContent ?? null,
  ledger: [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((el) => !el.id.endsWith(".revert")).slice(-1).map((el) => (el.innerText ?? "").replace(/\s+/g, " ").slice(0, 70))[0] ?? null,
}));
/** ⏱️ Sample the board for a full budget so a late refresh is distinguished from no refresh. */
const track = async (label, budgetMs) => {
  const seen = [];
  const deadline = Date.now() + budgetMs;
  while (Date.now() < deadline) {
    const state = await read();
    const key = JSON.stringify(state);
    if (seen.at(-1)?.key !== key) seen.push({ key, ms: Date.now() - t0, ...state });
    await page.waitForTimeout(500);
  }
  lines.push(`\n## ${label} (${budgetMs} ms of sampling, ${seen.length} distinct states)`);
  for (const state of seen) lines.push(`  ${state.ms}ms board="${state.board}" checkin="${state.checkin}" last-ledger="${state.ledger}"`);
  return seen.at(-1);
};

await page.goto(`http://127.0.0.1:${PORT}/?plugin=${VARIANT}`, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 90; i++) { await page.waitForTimeout(1000); if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))) break; }
await page.waitForTimeout(6000);

const click = async (selector) => page.locator(selector).first().click({ force: true, timeout: 8000 }).then(() => "ok").catch((e) => String(e).split("\n")[0].slice(0, 100));
lines.push(`history panel: ${await click('[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]')}`);
await page.waitForTimeout(1500);
lines.push(`rail: ${await click('[id$=".engagement.toggle"]')}`);
await page.waitForTimeout(2500);

await track("baseline", 3000);
lines.push(`\ndispatch action.${ACTION}: ${await click(`[id="action.${ACTION}"]`)}`);
await track("after mutation", 12_000);
lines.push(`\ndispatch action.undo: ${await click('[id="action.undo"]')}`);
await track("after undo", 20_000);
lines.push(`\ndispatch action.redo: ${await click('[id="action.redo"]')}`);
await track("after redo", 20_000);

writeFileSync(OUT, lines.join("\n"));
console.log(lines.join("\n"));
await browser.close();
