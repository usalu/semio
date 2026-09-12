/** 🚪️ Runtime probe for the COST of the predecessor close ladder on a ⌘⌥V role switch
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 *
 * `🗑️generated/journey-5/console.txt` measured the defect: after a long editor session the switch sat
 * for 87 s inside `retireInstanceLifecycle`, threw `plugin-ui.lifecycle-close-budget-exhausted`, and
 * only THEN published the viewer. This probe reproduces that shape deliberately — boot, converge, pick
 * three examples so the retained window transients, tessellation cache and history are all real — and
 * reports the two numbers that matter:
 *
 *   • SWITCH  — chord → `surface-switch publish` (when the successor's session starts). This is what
 *               the user waits for, and the whole point of retiring in the background.
 *   • RETIRE  — chord → `surface-switch retire` / `retire-failed` (when the predecessor's ladder
 *               finally reports). It may be long; it may never arrive; neither may delay SWITCH.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=close-ladder/switch-1 bun 🐍️close-ladder-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "close-ladder/switch");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 240);
const exampleWait = Number(process.env.SEMIO_PROBE_EXAMPLE_WAIT ?? 120);
const switchWait = Number(process.env.SEMIO_PROBE_SWITCH_WAIT ?? 180);
const examples = Number(process.env.SEMIO_PROBE_EXAMPLES ?? 3);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1400)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1400)}`));

const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map((el) => {
    let meshes = 0; try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); meshes = Array.isArray(v) ? v.length : 0; } catch {}
    const st = parse(el.getAttribute("data-status-json"));
    return { surfaceId: el.getAttribute("data-surface-id"), meshes, phase: st?.phase ?? null, ratio: st?.progress?.ratio ?? null, nodeStatuses: st && !st.phase ? Object.values(st).map((v) => v?.status ?? "?") : undefined };
  });
  const pressed = (id) => document.getElementById(id)?.getAttribute("aria-pressed") ?? null;
  return {
    hosts,
    meshes: hosts.reduce((n, h) => n + h.meshes, 0),
    windows: [...document.querySelectorAll("[data-window-instance-id]")].map((e) => e.getAttribute("data-window-instance-id")),
    roles: { editor: pressed("playground.navbar.roles.editor"), viewer: pressed("playground.navbar.roles.viewer") },
    rolesBusy: document.getElementById("playground.navbar.roles")?.getAttribute("aria-busy") ?? null,
  };
});

const converged = (s) => {
  const main = s.hosts.find((h) => h.nodeStatuses);
  const preview = s.hosts.find((h) => h.surfaceId && h.surfaceId.includes("preview"));
  const nodes = main?.nodeStatuses ?? [];
  return Boolean(preview) && preview.phase === "idle" && preview.ratio === 1 && nodes.length > 0 && nodes.every((st) => st === "ok" || st === "error");
};

const waitFor = async (label, seconds, predicate) => {
  let last = null; let stable = 0;
  for (let i = 0; i < seconds; i++) {
    await page.waitForTimeout(1000);
    last = await snap();
    if (predicate(last)) { stable += 1; if (stable >= 3) break; } else stable = 0;
  }
  console.log(`[DEBUG] ${label} t=${((Date.now() - t0) / 1000).toFixed(0)}s ok=${predicate(last)} ${JSON.stringify({ roles: last.roles, busy: last.rolesBusy, windows: last.windows, meshes: last.meshes })}`);
  await page.screenshot({ path: join(outDir, `${label}.png`) });
  return last;
};

const steps = [];
const record = (label, snapshot) => { steps.push({ label, t: Date.now() - t0, ...snapshot }); };

await page.goto(url, { waitUntil: "domcontentloaded" });
record("boot", await waitFor("1-boot", bootWait, converged));

// 🏋️ Accumulate a REAL session: each example pick retains another document generation's window
// transients, another tessellation generation and another history frame — the retained families the
// close ladder then has to walk.
for (let index = 0; index < examples; index += 1) {
  const combo = page.locator('[role="combobox"]').first();
  if (await combo.count()) {
    await combo.click({ timeout: 5000 });
    await page.waitForTimeout(400);
    const option = page.locator('[role="option"]').nth(index + 1);
    if (await option.count()) await option.click({ timeout: 5000 });
  }
  record(`example-${index + 1}`, await waitFor(`2-example-${index + 1}`, exampleWait, converged));
}

// 🚪️ The switch under measurement, on a QUIET shell: everything the close has to walk is retained,
// nothing is in flight, so the only thing between the chord and the viewer is the close ladder.
// 😴️ A converged DOM is not a quiet instance: the shell's quiesce pass refuses a switch while a typed
// operation is still in flight (`draining`), so settle first and retry the chord — a user does exactly
// that when the role group answers "Finishing the current operation…".
await page.waitForTimeout(20_000);
const chords = [];
for (let attempt = 0; attempt < Number(process.env.SEMIO_PROBE_CHORD_ATTEMPTS ?? 24); attempt += 1) {
  const pressedAt = Date.now() - t0;
  chords.push(pressedAt);
  await page.keyboard.press("Meta+Alt+V");
  // ⏳️ A refusal needs the whole quiesce budget to be certain, so give each attempt more than that.
  await page.waitForTimeout(14_000);
  if (lines.some((l) => Number(l.split(" ")[0]) >= pressedAt && l.includes("surface-switch seal "))) break;
  console.log(`[DEBUG] chord attempt ${attempt + 1} refused, retrying`);
  await page.waitForTimeout(6_000);
}
record("chord", await snap());
record("post-switch", await waitFor("3-post-switch", switchWait, (s) => s.roles.viewer === "true"));
// 🕰️ Keep watching after the successor mounted, so a background retirement that reports late is seen.
await page.waitForTimeout(90_000);
record("post-retire", await snap());

const trace = lines.filter((l) => l.includes("surface-switch "));
const at = (step) => {
  const hit = trace.find((l) => l.includes(`surface-switch ${step} `));
  return hit === undefined ? null : Number(hit.split(" ")[0]);
};
const sealAt = at("seal");
// 🕰️ The chord that actually opened the transaction is the last one pressed before the seal — every
// earlier press was refused `draining` while the editor still had work in flight.
const chordAt = sealAt === null ? (chords.at(-1) ?? 0) : (chords.filter((pressed) => pressed <= sealAt).at(-1) ?? chords[0] ?? 0);
const summary = {
  chordAt,
  chords,
  trace,
  switchMs: at("publish") === null ? null : at("publish") - chordAt,
  retireStartedMs: at("retire-started") === null ? null : at("retire-started") - chordAt,
  retireReportedMs: at("retire") === null ? (at("retire-failed") === null ? null : at("retire-failed") - chordAt) : at("retire") - chordAt,
  retireOutcome: at("retire") !== null ? "retire" : at("retire-failed") !== null ? "retire-failed" : "none",
  closeBudgetExhausted: lines.filter((l) => l.includes("close-budget-exhausted")),
  noActor: lines.filter((l) => l.includes("no actor for instance")).length,
  revoked: lines.filter((l) => l.includes("actor-activation.revoked")).length,
  pageErrors: lines.filter((l) => l.includes("pageerror")),
};
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "steps.json"), JSON.stringify(steps, null, 2));
writeFileSync(join(outDir, "summary.json"), JSON.stringify(summary, null, 2));
console.log(`[DEBUG] close-ladder probe switchMs=${summary.switchMs} retireReportedMs=${summary.retireReportedMs} (${summary.retireOutcome}) budgetExhausted=${summary.closeBudgetExhausted.length} noActor=${summary.noActor}`);
await browser.close();
