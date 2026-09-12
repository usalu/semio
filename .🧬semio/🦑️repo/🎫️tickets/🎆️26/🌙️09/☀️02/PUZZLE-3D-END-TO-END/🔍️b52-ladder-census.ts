/** 🪜️ Wave B52 — WHAT STATE the reconcile retirement ladder sits in on a live guest after one gesture.
 *
 * Native measurement put the freeze in the document retirement ladder: one `UiNodeRecord` (6 416 bytes)
 * is wider than the reconciler's 4 096-byte copy grant, so `PagedList::release_empty_page` is contract-
 * bound to refuse the page, the terminal never empties, the closing instance never completes and the
 * actor answers `more-work` for ever (see `📓️2026-09-13-wave-B52-reconcile-ladder-livelock.md` §2).
 *
 * This probe reads the guest's own census for that shape. The reactor prints
 * `[DEBUG] reactor more-work streak=… sources=… effects=… patches=[slots=… ready=… terminals=…
 * deferred=… output_fault=… reserve_refusal=…]` once per more-work turn when runtime diagnostics are
 * armed (`⚛️reactor/🔄️turn/🦀️.rs:1272`), so one orbit plus a watch window answers, without any tap:
 *
 * - `terminals=[g…:c--]` — close requested, no fault, NOT terminal-empty: the ladder shape above.
 * - `reserve_refusal=<surface>:<predicate>` — wave B48's refusal evidence, naming which predicate holds
 *   a surface that keeps being deferred instead of re-rendered.
 * - `ui.surface-render-uncommitted` / `ui.dirty-surface-deferred-capacity` — wave B48's named faults.
 * - `sources=` and `effects=` separate "busy with work" from "spinning on nothing".
 *
 * Run: `bun 🔍️b52-ladder-census.ts [--port=6013] [--label=wasm60] [--boot=240] [--watch=30]`.
 * Output: `🗑️generated/b52-<stamp>-<label>.md`. Ticket 26/09/02/PUZZLE-3D-END-TO-END. */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const OUT = join(import.meta.dir, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const arg = (name: string, fallback: string) => process.argv.find((a) => a.startsWith(`--${name}=`))?.slice(name.length + 3) ?? fallback;
const label = arg("label", "run");
const port = arg("port", "6013");
const bootSeconds = Number(arg("boot", "240"));
const watchSeconds = Number(arg("watch", "30"));
const stamp = `${new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19)}-${label}`;
const mdPath = join(OUT, `b52-${stamp}.md`);

const t0 = Date.now();
const lines: string[] = [];
const log = (m: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(2)}s] ${m}`;
  lines.push(row);
  console.log(row);
};

const tape: { ms: number; text: string }[] = [];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.addInitScript(() => {
  try {
    globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
  } catch {
    /* a sandboxed tab throws on localStorage */
  }
});
page.on("console", (msg) => tape.push({ ms: Date.now() - t0, text: msg.text().slice(0, 2400) }));
page.on("pageerror", (error) => tape.push({ ms: Date.now() - t0, text: `pageerror: ${String(error).slice(0, 300)}` }));

const PANE = "puzzle3d-main-perspective";
const pane = async () =>
  page
    .evaluate((id) => {
      const host = (document.querySelector(`[id="${id}"] [data-camera-json]`) ?? document.querySelector(`[id="framework.window.${id}"] [data-camera-json]`)) as HTMLElement | null;
      return { published: host?.getAttribute("data-camera-json") ?? null, viewport: host?.getAttribute("data-viewport-camera-json") ?? null };
    }, PANE)
    .catch(() => ({ published: null, viewport: null }));

/** 🧮️ Every DISTINCT census the tape carries since `from`, most frequent first, plus the ladder facts
 * the report quotes: the deepest more-work streak, the refusal predicates named, and whether any close
 * ladder is sitting on a terminal that never empties. */
const census = (from: number) => {
  const states = new Map<string, number>();
  const refusals = new Map<string, number>();
  const terminals = new Map<string, number>();
  const sources = new Map<string, number>();
  let streak = 0;
  let effectless = 0;
  for (const row of tape.slice(from)) {
    const patches = /patches=\[(.+?)\] pending=\[/.exec(row.text);
    if (patches) states.set(patches[1], (states.get(patches[1]) ?? 0) + 1);
    const refusal = /reserve_refusal=([^ \]]+)/.exec(row.text);
    if (refusal) refusals.set(refusal[1], (refusals.get(refusal[1]) ?? 0) + 1);
    const terminal = /terminals=\[([^\]]*)\]/.exec(row.text);
    if (terminal) terminals.set(terminal[1], (terminals.get(terminal[1]) ?? 0) + 1);
    const source = /sources=(\[[^\]]*\])/.exec(row.text);
    if (source) sources.set(source[1], (sources.get(source[1]) ?? 0) + 1);
    const found = /more-work streak=(\d+)/.exec(row.text);
    if (found) streak = Math.max(streak, Number(found[1]));
    if (/effects=0/.test(row.text)) effectless += 1;
  }
  const top = (map: Map<string, number>, n: number) => [...map.entries()].sort((a, b) => b[1] - a[1]).slice(0, n);
  return { streak, effectless, states: top(states, 4), refusals: top(refusals, 6), terminals: top(terminals, 6), sources: top(sources, 4) };
};

const report = (name: string, from: number) => {
  const read = census(from);
  log(`${name} maxStreak=${read.streak} effectlessTurns=${read.effectless} distinctCensus=${read.states.length}`);
  for (const [state, count] of read.sources) log(`${name} sources ${state} x${count}`);
  for (const [state, count] of read.terminals) log(`${name} terminals=[${state}] x${count}`);
  for (const [state, count] of read.refusals) log(`${name} reserve_refusal=${state} x${count}`);
  for (const [state, count] of read.states) log(`${name} census x${count} ${state.slice(0, 1200)}`);
  const faults = tape.slice(from).filter((row) => /surface-render-uncommitted|dirty-surface-deferred-capacity|did not publish|stopped without publishing|pageerror|stall budget/.test(row.text));
  log(`${name} namedFaults=${faults.length}`);
  for (const row of faults.slice(0, 12)) log(`${name} | +${row.ms}ms ${row.text.slice(0, 400)}`);
};

log(`goto http://127.0.0.1:${port}/`);
await page.goto(`http://127.0.0.1:${port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });
const bootStarted = Date.now();
while (!(await pane()).published && Date.now() - bootStarted < bootSeconds * 1000) await page.waitForTimeout(1000);
const booted = await pane();
log(`boot published=${String(booted.published).slice(0, 120)} waitedMs=${Date.now() - bootStarted}`);
report("boot", 0);

const box = (await page.locator(`[id="${PANE}"] canvas, [id="framework.window.${PANE}"] canvas`).first().boundingBox()) ?? (await page.locator("canvas").last().boundingBox());
if (!box) log("orbit SKIPPED no canvas");
else {
  const from = tape.length;
  const before = await pane();
  const cx = box.x + box.width * 0.5;
  const cy = box.y + box.height * 0.35;
  await page.keyboard.down("Alt").catch(() => {});
  await page.mouse.move(cx, cy);
  await page.mouse.down({ button: "right" });
  await page.mouse.move(cx + 160, cy + 70, { steps: 20 });
  await page.mouse.up({ button: "right" });
  await page.keyboard.up("Alt").catch(() => {});
  const rig = await pane();
  const watchStarted = Date.now();
  while (Date.now() - watchStarted < watchSeconds * 1000 && (await pane()).published === before.published) await page.waitForTimeout(500);
  const after = await pane();
  log(`orbit rigMoved=${rig.viewport !== before.viewport} publishedMoved=${after.published !== before.published} waitedMs=${Date.now() - watchStarted}`);
  log(`orbit publishedBefore=${String(before.published).slice(0, 140)}`);
  log(`orbit publishedAfter=${String(after.published).slice(0, 140)}`);
  report("orbit", from);
}

log(`tape lines=${tape.length}`);
writeFileSync(mdPath, `# Wave B52 ladder census — ${stamp}\n\n\`\`\`\n${lines.join("\n")}\n\`\`\`\n`);
log(`wrote ${mdPath}`);
await browser.close();
