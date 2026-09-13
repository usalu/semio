/** 🎛️ wgpu RETAINED CONTROL COMMIT — does editing a generation's Form field reach the guest?
 *
 * Drives `?plugin=generation3d&mode=generate` on the coordinator's wgpu serve: waits for
 * `boot_shell leave`, clicks the generations window's `Add Generation` row so the Form has fields at
 * all, then clicks a Form field, types, and blurs — and reports every action the shell dispatched
 * plus every retained-hit registration it minted.
 *
 * The wgpu host ticks on input, so the probe nudges the pointer continuously; a probe that sits
 * still measures a runtime that was never asked to run (see `🐍️wgpu-boot-witness-probe.mjs`).
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-controls/commit-1 bun 🐍️wgpu-control-commit-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d&mode=generate";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-controls/commit");
const typed = process.env.SEMIO_PROBE_TEXT ?? "7";
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 6000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

const nudge = async (ticks) => {
  for (let tick = 0; tick < ticks; tick += 1) {
    await page.mouse.move(3 + (tick % 5), 3 + (tick % 5)).catch(() => {});
    await page.waitForTimeout(250);
  }
};
const seen = (needle) => lines.some((line) => line.includes(needle));
const waitFor = async (needle, ticks) => {
  for (let tick = 0; tick < ticks; tick += 1) {
    if (seen(needle)) return true;
    await page.mouse.move(3 + (tick % 5), 3 + (tick % 5)).catch(() => {});
    await page.waitForTimeout(250);
  }
  return false;
};
/** 🎯️ The control the host last resolved at `(x, y)`, or `null` — read off `os_host pointer hit`. */
const resolvedAt = (x) => {
  const prefix = `os_host pointer hit x=${x}`;
  const line = [...lines].reverse().find((entry) => entry.includes(prefix));
  if (!line) return null;
  const match = /hit=Some\(\((\w+), Some\("([^"]*)"\)/u.exec(line);
  return match ? { kind: match[1], controlId: match[2] } : line.includes("targets=0") ? "drained" : "none";
};

/** 🖱️ A real press, taken only once the host's registry actually answers at that point.
 *
 * 🩸️ The registry is drained once per frame build (`FrameBuildPhase::InputFrame`), so a press timed
 * blind lands on an EMPTY registry as often as not — measured as `targets=0 hit=None` on 82 of 187
 * probe samples. Moving first, and pressing only on a resolved hit, is the difference between
 * measuring the dispatch chain and measuring the drain. */
const press = async (x, y, tries = 24) => {
  const key = String(Math.round(x * 100) / 100).replace(/\.00$/u, "");
  let resolved = null;
  for (let attempt = 0; attempt < tries; attempt += 1) {
    await page.mouse.move(x, y);
    await page.waitForTimeout(160);
    resolved = resolvedAt(key);
    if (resolved && resolved !== "drained" && resolved !== "none") break;
    await page.mouse.move(x + 1, y + 1);
    await page.waitForTimeout(160);
  }
  await page.mouse.move(x, y);
  await page.waitForTimeout(120);
  await page.mouse.down();
  await page.waitForTimeout(140);
  await page.mouse.up();
  await page.waitForTimeout(500);
  await nudge(4);
  return resolved;
};

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));
const booted = await waitFor("boot_shell leave", 480);
await nudge(8);

/** 🪟️ The dock plan the shell printed, as `{windowId: {x, y, w, h}}`. */
const dock = () => {
  const line = [...lines].reverse().find((entry) => entry.includes("wgpu-shell dock plan"));
  if (!line) return {};
  return Object.fromEntries(
    [...line.matchAll(/([\w-]+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(\d+(?:\.\d+)?),(\d+(?:\.\d+)?)/gu)].map((match) => [match[1], { w: Number(match[2]), h: Number(match[3]), x: Number(match[4]), y: Number(match[5]) }]),
  );
};

const plan = dock();
const generations = plan["generation3d-generations"];
const form = plan["generation3d-generate-form"];
const steps = [];

/** 🪟️ The window kind the guest currently treats as active — a command is refused outright when the
 * active kind does not own it (`window kind procedural-main does not own action addGeneration`). */
const activeKind = () => {
  const line = [...lines].reverse().find((entry) => entry.includes("activeWindowKindId\":"));
  const match = line ? /"activeWindowKindId":"([^"]*)"/u.exec(line) : null;
  return match ? match[1] : null;
};

// 🪟️ Focus the generations window first: its own `addGeneration` belongs to ITS window kind, and the
// shell boots with `procedural-main` active, which owns no such action.
const focus = [];
if (generations) {
  for (const y of [32, 38, 44, 50]) {
    const x = generations.x + generations.w * 0.5;
    const resolved = await press(x, y, 8);
    focus.push({ point: [Math.round(x), y], resolved, activeKind: activeKind() });
    if (activeKind() === "generation3d-generations") break;
  }
  await nudge(8);
}

// ➕️ A generation has to exist before the Form has any field to edit.
if (generations) {
  // 🌳️ Four rows of one 24 px band each: two section headers and two items. The `Add Generation`
  // item is the last of them — press every band, so a reordered tree still finds it.
  for (let band = 0; band < 6; band += 1) {
    const before = lines.length;
    const x = generations.x + generations.w * 0.5;
    const y = generations.y + 12 + band * 24;
    const resolved = await press(x, y);
    const slice = lines.slice(before);
    steps.push({ step: `generations-band-${band}`, point: [Math.round(x), Math.round(y)], resolved, dispatched: slice.filter((line) => /addGeneration|does not own action|dispatch_action/u.test(line)).slice(0, 4) });
    if (slice.some((line) => line.includes("addGeneration"))) break;
  }
  await nudge(16);
}

// 🎛️ Sweep the Form window's own column for a retained control, one row band at a time, pressing and
// typing at each — the registry's rects are not exposed to a probe, so the gesture itself is the
// search. A press that focuses an Input is visible as a `FocusChanged`/`route_retained` line.
const attempts = [];
if (form) {
  for (let row = 0; row < 18; row += 1) {
    const x = form.x + form.w * 0.62;
    const y = form.y + 40 + row * 34;
    const before = lines.length;
    const resolved = await press(x, y, 8);
    await page.keyboard.type(typed, { delay: 60 });
    await page.waitForTimeout(200);
    await page.keyboard.press("Tab");
    await page.waitForTimeout(300);
    await nudge(4);
    const slice = lines.slice(before);
    attempts.push({
      row,
      point: [Math.round(x), Math.round(y)],
      resolved,
      newLines: slice.length,
      dispatched: slice.filter((line) => /updateGenerationValues|renameGeneration|dispatch_action|reserve_action|publish_retained_action/u.test(line)).slice(0, 6),
    });
    if (slice.some((line) => line.includes("updateGenerationValues"))) break;
  }
}

await page.screenshot({ path: join(outDir, "shot.png"), type: "png" }).catch(() => {});
const bodyHits = Object.fromEntries([...new Set(lines.filter((line) => line.includes("retained body hits")).map((line) => line.replace(/^\d+ log /u, "")))].map((line) => [line, lines.filter((entry) => entry.includes(line)).length]));
const counts = Object.fromEntries(
  ["boot_shell leave", "os_host handle_event", "os_host pointer hit", "dispatch_normalized_event", "updateGenerationValues", "addGeneration", "renameGeneration", "FocusChanged", "retained body hits", "panicked", "surface fault"].map((needle) => [needle, lines.filter((line) => line.includes(needle)).length]),
);
const verdict = { url, bodyHits, focus, activeKind: activeKind(), booted, bootAtMs: booted ? Number((lines.find((line) => line.includes("boot_shell leave")) ?? "0").split(" ", 1)[0]) : null, seconds: Math.round(at() / 1000), consoleLines: lines.length, plan, steps, attempts, counts };
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "verdict.json"), JSON.stringify(verdict, null, 2));
console.log(JSON.stringify(verdict, null, 2).slice(0, 8000));
await browser.close();
