// 🧯️ Tool-run intake probe (ticket 26/09/09, lane concurrent-patch-intake). Drives one app's Tool runs panel
// through N rounds of Start → mid-run Abort/Finalize → Dismiss and keeps EVERY console line, so the host's own
// `[DEBUG] intake concurrent entry` / `[DEBUG] intake duplicate surfaces in one turn` instrumentation and any
// `plugin-ui.intake-rejected:intake:Foreign or busy instance surface owner` are counted against the pill's
// visible state. Works on both doors:
//   SEMIO_PROBE_URL=http://127.0.0.1:6024/?plugin=generation3d SEMIO_PROBE_ARM=none bun 🐍️tool-run-intake-probe.mjs
//   SEMIO_PROBE_URL=http://127.0.0.1:6013/?plugin=puzzle3d SEMIO_PROBE_ARM=fill bun 🐍️tool-run-intake-probe.mjs
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6024/?plugin=generation3d";
const arm = process.env.SEMIO_PROBE_ARM ?? "none";
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT_SECONDS ?? 240);
const midRunMs = Number(process.env.SEMIO_PROBE_MIDRUN_MS ?? 0);
const settleMs = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? 9000);
const cycles = Number(process.env.SEMIO_PROBE_CYCLES ?? 1);
const fillCount = process.env.SEMIO_PROBE_COUNT ?? "2000";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "intake-toolrun");
mkdirSync(outDir, { recursive: true });

const lines = [];
const report = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`+${Date.now() - t0}ms ${msg.type()} ${msg.text().slice(0, 700)}`));
page.on("pageerror", (error) => lines.push(`+${Date.now() - t0}ms pageerror ${String(error).slice(0, 700)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });

const panelReady = () =>
  page.evaluate(() => document.querySelectorAll('[id$="framework.toolRun.ready"], [id*="framework.toolRun."]').length > 0);
const deadline = Date.now() + bootSeconds * 1000;
await page.waitForTimeout(4000);
await page.getByText("Skip", { exact: true }).first().click({ force: true, timeout: 3000 }).catch(() => {});
await page.evaluate(() => document.getElementById("framework.panelTab.framework.panel.toolRun")?.click());
if (arm === "fill") {
  await page.getByRole("button", { name: "Tool", exact: true }).first().click({ timeout: 20000 }).catch(() => {});
  await page.waitForTimeout(2500);
  await page.getByRole("button", { name: "Fill", exact: true }).first().click({ timeout: 20000 }).catch(() => {});
  await page.waitForTimeout(2500);
  await page
    .getByRole("spinbutton")
    .first()
    .fill(fillCount, { timeout: 20000 })
    .then(() => page.getByRole("spinbutton").first().press("Enter"))
    .catch(() => {});
  await page.waitForTimeout(2500);
}
while (Date.now() < deadline && !(await panelReady())) await page.waitForTimeout(1000);
await page.evaluate(() => document.getElementById("framework.panelTab.framework.panel.toolRun")?.click());
await page.waitForTimeout(6000);
report.push(`panel ready at +${Date.now() - t0}ms`);

const sample = () =>
  page.evaluate(() => {
    const text = (el) => (el ? (el.textContent ?? "").replace(/\s+/g, " ").trim() : null);
    const group = [...document.querySelectorAll("[id]")].filter((el) => /framework\.toolRun\.\d+$/.test(el.id)).at(-1) ?? null;
    const ready = document.querySelector('[id$="framework.toolRun.ready"]');
    const scope = group ?? ready;
    const bar = group?.querySelector('[role="progressbar"],progress') ?? null;
    return {
      groupId: group?.id ?? null,
      status: (text(scope) ?? "").slice(0, 120),
      progress: bar ? bar.getAttribute("aria-valuetext") ?? text(bar) : null,
      buttons: [...(scope?.querySelectorAll("button") ?? [])].map((node) => ({
        label: (node.textContent ?? "").replace(/\s+/g, " ").trim(),
        disabled: node.disabled || node.getAttribute("aria-disabled") === "true",
      })),
    };
  });

const press = async (label) => {
  const clicked = await page.evaluate((label) => {
    const group = [...document.querySelectorAll("[id]")].filter((el) => /framework\.toolRun\.\d+$/.test(el.id)).at(-1);
    const scope = group ?? document.querySelector('[id$="framework.toolRun.ready"]');
    const node = scope ? [...scope.querySelectorAll("button")].find((candidate) => (candidate.textContent ?? "").trim() === label) : undefined;
    if (!node) return false;
    node.click();
    return true;
  }, label);
  report.push(`+${Date.now() - t0}ms press ${label} ${clicked ? "ok" : "NOT FOUND"}`);
  return clicked;
};

const faults = () => lines.filter((line) => line.includes("intake-rejected") || line.includes("intake-blocked")).length;

report.push(`initial ${JSON.stringify(await sample())}`);
const rounds = [];
for (let cycle = 0; cycle < cycles; cycle += 1) {
  for (const action of ["Abort", "Finalize"]) {
    report.push(`===== cycle ${cycle + 1} round ${action}`);
    let startedOk = await press("Start");
    if (!startedOk && arm === "none") {
      // 🔁️ generation3d dismisses its whole run group and shows no ready group (its `previewEval` is
      // dispatched by the document, not by an active tool), so the next run is armed the way a user arms
      // it: pick the next example, which re-dispatches `toolRunStart`.
      const picked = await page.evaluate(() => {
        const select = document.getElementById("playground.navbar.fixture");
        if (!select) return false;
        select.click();
        return true;
      });
      await page.waitForTimeout(1200);
      if (picked) {
        const items = await page.$$('[role="option"],[role="menuitem"]');
        const target = items[(cycle + 2) % Math.max(1, items.length)];
        await target?.click().catch(() => {});
      }
      report.push(`+${Date.now() - t0}ms re-armed by example switch picked=${picked}`);
      startedOk = picked;
    }
    const armDeadline = Date.now() + 8000;
    let armed = false;
    while (Date.now() < armDeadline) {
      const now = await sample();
      if (now.buttons.some((button) => button.label === action && !button.disabled)) {
        armed = true;
        if (midRunMs > 0) await page.waitForTimeout(midRunMs);
        break;
      }
      await page.waitForTimeout(25);
    }
    const midRun = await sample();
    report.push(`mid-run armed=${armed} ${JSON.stringify(midRun)}`);
    const before = faults();
    const pressedAt = Date.now();
    const pressedOk = await press(action);
    let last = "";
    const timeline = [];
    while (Date.now() - pressedAt < settleMs) {
      const now = await sample();
      const key = JSON.stringify({ status: now.status, buttons: now.buttons.map((button) => `${button.label}${button.disabled ? "!" : ""}`) });
      if (key !== last) {
        timeline.push(`  +${Date.now() - pressedAt}ms ${key}`);
        last = key;
      }
      await page.waitForTimeout(100);
    }
    report.push(`== ${action} timeline`, ...timeline);
    const final = await sample();
    rounds.push({
      cycle: cycle + 1,
      action,
      startedOk,
      armed,
      pressedOk,
      midRunStatus: midRun.status,
      firstFeedbackMs: timeline.length > 1 ? Number(timeline[1].trim().split("ms")[0].slice(1)) : null,
      terminal: final.status,
      intakeFaults: faults() - before,
    });
    await press("Dismiss");
    await page.waitForTimeout(2500);
  }
}

const summary = {
  url,
  arm,
  rounds,
  intakeRejected: lines.filter((line) => line.includes("intake-rejected")).length,
  intakeBlocked: lines.filter((line) => line.includes("intake-blocked")).length,
  duplicateSurfaceTurns: lines.filter((line) => line.includes("intake duplicate surfaces")).length,
  concurrentEntries: lines.filter((line) => line.includes("intake concurrent entry")).length,
  pageerrors: lines.filter((line) => line.includes(" pageerror ")).length,
  consoleErrors: lines.filter((line) => line.includes(" error ")).length,
};
report.push("== summary", JSON.stringify(summary, null, 2));
report.push("== notable console", ...lines.filter((line) => /intake|toolRun|pageerror|duplicate surfaces|concurrent entry/i.test(line)).slice(0, 600));
await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "report.txt"), report.join("\n"));
console.log(["== summary", JSON.stringify(summary, null, 2)].join("\n"));
await browser.close();
