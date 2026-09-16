/** 🎛️ Forms interaction probe: boots the forms react playground (6058), then (1) records the boot state
 * (shell beacon, window hosts, Artifact tree step/question count), (2) unfolds a window's Actions pane
 * and submits one document action (default `add-step`, arg-less) through the pane's own form, checking
 * the Artifact tree gains a step, (3) presses mod+z and checks the step is removed again (undo proves
 * the mutation landed in the document store), (4) drives the Try window's `nextStep` view action.
 * Every step records the console delta, guest/host fault lines and a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=forms-interact-1 bun 🐍️forms-interact-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6058/?plugin=forms";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "forms-interact");
const actionId = process.env.SEMIO_PROBE_ACTION ?? "add-step";
const actionArgs = JSON.parse(process.env.SEMIO_PROBE_ACTION_ARGS ?? "{}");
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 12);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{/.test(l)).map((l) => l.slice(0, 400));
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1500)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};
const text = () => page.evaluate(() => document.body.innerText.replace(/\s+/g, " "));
const state = () => page.evaluate(() => {
  const body = document.body.innerText.replace(/\s+/g, " ");
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => { const st = parse(el.getAttribute("data-status-json")); return { id: el.getAttribute("data-surface-id"), phase: st?.phase, fault: st?.fault?.code ?? null, textLength: (el.innerText ?? "").length }; });
  const stepRows = [...document.querySelectorAll('[role="treeitem"]')].map((el) => el.innerText.replace(/\s+/g, " ").trim().slice(0, 60));
  const stepMatches = body.match(/\d+ questions/g) ?? [];
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    hosts,
    treeItems: stepRows.length,
    stepCount: stepMatches.length,
    stepLabels: stepMatches.slice(0, 12),
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
    bodyHead: body.slice(0, 600),
  };
});

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length && s.stepCount > 0 && i > 8) break; if (s.error) break; }
await note("boot", s, 0);

// ── actions pane + one document action ────────────────────────────────────
let before = null;
{
  const from = lines.length;
  before = await state();
  const engagement = before.engagements.find((id) => new RegExp(process.env.SEMIO_PROBE_WINDOW ?? "blueprint", "i").test(id)) ?? before.engagements[0];
  let toggled = "absent";
  if (engagement) {
    const toggle = page.locator(`[id="${engagement}.toggle"]`).first();
    if (await toggle.count()) toggled = await toggle.click({ timeout: 8000, force: true }).then(() => `ok:${engagement}`).catch((e) => String(e).slice(0, 120));
    else toggled = `no-toggle:${engagement}`;
  }
  await page.waitForTimeout(1200);
  const opened = await state();
  const rowId = `action.${actionId}`;
  let clicked = "absent";
  if (opened.actionRows.includes(rowId)) clicked = await page.locator(`[id="${rowId}"]`).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const filled = [];
  for (const [key, value] of Object.entries(actionArgs)) {
    const input = page.locator(`[id="${key}"], [name="${key}"], input[aria-label="${key}"]`).first();
    if (await input.count()) { await input.fill(String(value)).catch((e) => filled.push(`${key}:${String(e).slice(0, 60)}`)); filled.push(`${key}=${value}`); } else filled.push(`${key}:absent`);
  }
  let submitted = "none";
  const camel = actionId.replace(/-([a-z])/g, (_, c) => c.toUpperCase());
  const submit = page.locator(`[id$=".action.${actionId}.execute"], [id$=".action.${camel}.execute"]`).first();
  if (await submit.count()) submitted = await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  else submitted = "no-execute-control";
  let after = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (after.stepCount === before.stepCount + 1) break; }
  await note("action", { engagement, toggled, rowId, clicked, filled, submitted, stepsBefore: before.stepCount, ...after }, from);
}

// ── undo through the keybinding ──────────────────────────────────────────
{
  const from = lines.length;
  await page.mouse.click(400, 500).catch(() => {});
  await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
  let after = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (after.stepCount === before.stepCount) break; }
  await note("undo", { stepsBefore: before.stepCount, ...after }, from);
}

// ── Try window: next step ────────────────────────────────────────────────
{
  const from = lines.length;
  const bodyBefore = await text();
  const next = page.locator('[id$="nextStep"], [id*="next-step"], button:has-text("Next")').first();
  let clicked = "absent";
  if (await next.count()) clicked = await next.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(3000);
  const bodyAfter = await text();
  await note("try-next", { clicked, changed: bodyBefore !== bodyAfter, ...(await state()) }, from);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE", report.steps.length);
await browser.close();
