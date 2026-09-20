#!/usr/bin/env bun
/** 🔬️ S6 — how late is "one refresh late"? (ticket 26/09/18, S5 §4.3's named residual.)
 *
 * S5 measured that a spawned program's first typed operation lands in the document but the window
 * "repaints one refresh late", and ruled out two host-side causes. That sentence has two very
 * different readings and the fix differs per reading:
 *
 *   (a) LATENCY — the window repaints on its own, some hundreds of ms after the verb settles, with
 *       no further input. Then the host's refresh lane is simply behind the guest's publication.
 *   (b) STALLED — the window does NOT repaint until the NEXT dispatch of any kind. Then a refresh
 *       is being dropped, not delayed.
 *
 * This probe distinguishes them: it runs one verb and then samples the canvas every 250 ms for 12 s
 * WITHOUT touching the page, and does the same for the undo and the redo. A transition inside the
 * quiet window is (a); no transition until the next keystroke is (b).
 *
 * Usage: bun 🐍️s6-repaint-latency.mjs <baseUrl> <pluginId> [verbId]
 */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6071/";
const pluginId = process.argv[3] ?? "dag";
const wantedVerb = process.argv[4] ?? null;
const generated = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const QUIET_MS = Number(process.env.S6_QUIET_MS ?? 12_000);
const log = (...parts) => console.log("[s6lat]", ...parts);

const windowIds = (page) => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).filter((id) => typeof id === "string"));

/** 📖️ What the DOCUMENT reads, not what the chrome reads. S5's probe (and this probe's first run)
 * judged mutate/undo/redo on `textContent` of the window bodies AND the engagement pane — but
 * unfolding the Actions rail injects ~50 static row labels into that same text, and a graph window's
 * own body is SVG whose text never changes at all. `dag addNode` therefore read "nothing moved" for
 * both the verb and its undo, which is a blind oracle, not a broken undo.
 *
 * Two things move when the document moves, and nothing else does: the window's published MEASURES
 * (`SET_SPAWNED_WINDOW_MEASURES` — `2 nodes`, `1 layer · 0 selected`) and the element COUNT of the
 * window body (an SVG node, a list row, a canvas child). The action pane is excluded by name. */
const docDigest = (page) =>
  page.evaluate(() => {
    const clean = (value) => (value ?? "").replace(/\s+/gu, " ").trim();
    const bodies = [...document.querySelectorAll('[data-slot="window-body"]')].map((body) => {
      const pane = body.querySelector('[data-slot="window-action-pane"], [data-slot="window-action-pane-overlay"]');
      const paneElements = pane ? pane.querySelectorAll("*").length + 1 : 0;
      const paneText = pane ? clean(pane.textContent).length : 0;
      return `${body.querySelectorAll("*").length - paneElements}#${clean(body.textContent).length - paneText}#${clean(body.textContent).slice(0, 200)}`;
    });
    const measures = [...document.querySelectorAll('[data-slot="window-measures-body"], [data-slot="window-measure-tree-row"]')].map((row) => clean(row.textContent)).join(" · ");
    return `M[${measures}] B[${bodies.join(" || ")}]`;
  });
const canvasText = docDigest;

async function awaitBeacon(page, deadline) {
  while (Date.now() < deadline) {
    const beacon = await page.evaluate(() => {
      const data = document.documentElement.dataset;
      return data.semioOsReady !== undefined ? `ready:${data.semioOsReady}` : data.semioOsError !== undefined ? `error:${data.semioOsError}` : null;
    });
    if (beacon !== null) return beacon;
    await page.waitForTimeout(1000);
  }
  return null;
}
async function dismissIntroduction(page) {
  const deadline = Date.now() + 60_000;
  while (Date.now() < deadline) {
    if ((await page.locator('[data-slot="introduction-veil"]').count()) === 0) return;
    const skip = page.locator('[data-slot="introduction-veil"] button', { hasText: /skip/i }).first();
    if ((await skip.count()) > 0) await skip.click({ force: true }).catch(() => undefined);
    else await page.keyboard.press("Escape").catch(() => undefined);
    await page.waitForTimeout(500);
  }
}
async function openPalette(page) {
  await dismissIntroduction(page);
  await page.locator('[data-slot="navbar"]').first().click({ force: true, position: { x: 4, y: 4 } }).catch(() => undefined);
  await page.keyboard.press("Meta+p");
  await page.locator("[role='dialog'] [data-slot='command-input']").first().waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
}
async function signIn(page, email, password) {
  const badge = page.locator('[data-semio-hub-sign-in=""]').first();
  if ((await badge.count()) === 0) return "no sign-in badge";
  await badge.click({ force: true }).catch(() => undefined);
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 60_000 }).catch(() => undefined);
  await form.locator('input[type="email"]').fill(email);
  await form.locator('input[type="password"]').fill(password);
  await form.locator('button[type="submit"][aria-label="Sign in"]').click({ force: true }).catch(() => undefined);
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
  await page.locator("[data-semio-hub-workspace] button[aria-label]").first().click({ force: true }).catch(() => undefined);
  await page.goBack({ waitUntil: "commit" }).catch(() => undefined);
  await page.waitForFunction(() => document.querySelector('[id="s-home-main"]') !== null, undefined, { timeout: 180_000 }).catch(() => undefined);
  await page.waitForTimeout(4_000);
  return null;
}
async function enterStudio(page) {
  await openPalette(page);
  const item = page.locator('[data-slot="command-item"]').filter({ hasText: /s\s*·\s*studio/iu }).first();
  if ((await item.count()) === 0) return "no studio palette entry";
  const before = await windowIds(page);
  await item.click({ force: true }).catch(() => undefined);
  const deadline = Date.now() + 180_000;
  while (Date.now() < deadline) {
    if ((await windowIds(page)).some((id) => !before.includes(id))) {
      await page.waitForTimeout(6_000);
      return null;
    }
    await page.waitForTimeout(500);
  }
  return "studio never opened";
}
async function spawnProgram(page, id) {
  const before = await windowIds(page);
  await openPalette(page);
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.fill(id);
  const item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${id}"]`).first();
  await item.waitFor({ state: "visible", timeout: 20_000 }).catch(() => undefined);
  if ((await item.count()) === 0) return [];
  await item.click({ force: true }).catch(() => undefined);
  const deadline = Date.now() + 60_000;
  while (Date.now() < deadline) {
    const opened = (await windowIds(page)).filter((entry) => !before.includes(entry));
    if (opened.length > 0) {
      await page.waitForTimeout(3_000);
      return opened;
    }
    await page.waitForTimeout(250);
  }
  return [];
}

/** ⏱️ The measurement: sample the canvas every 250 ms for {@link QUIET_MS} with NO input in between,
 * and report the first sample that differs from the baseline. `null` means the canvas never moved
 * while the page was left alone — reading (b), a dropped refresh rather than a slow one. */
async function quietSamples(page, baseline, label) {
  const started = Date.now();
  const transitions = [];
  let previous = baseline;
  while (Date.now() - started < QUIET_MS) {
    const now = await canvasText(page);
    if (now !== previous) {
      transitions.push({ atMs: Date.now() - started, from: previous.slice(0, 90), to: now.slice(0, 90) });
      previous = now;
    }
    await page.waitForTimeout(250);
  }
  log(`${label}: ${transitions.length} transition(s) in ${QUIET_MS} ms of quiet`);
  return { label, transitions, final: previous };
}

const browser = await chromium.launch({ headless: process.env.S6_HEADED !== "1", args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const refusals = [];
page.on("console", (message) => {
  const text = message.text();
  if (/refused:|dropped action|rejected/iu.test(text)) refusals.push(text.slice(0, 240));
});
const result = { baseUrl, pluginId, phases: [] };
try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  result.beacon = await awaitBeacon(page, Date.now() + 300_000);
  if (process.env.S6_SIGN_IN_EMAIL) result.signIn = await signIn(page, process.env.S6_SIGN_IN_EMAIL, process.env.S6_SIGN_IN_PASSWORD ?? "");
  result.studio = await enterStudio(page);
  result.windowIds = await spawnProgram(page, pluginId);
  log(`spawned ${JSON.stringify(result.windowIds)}`);
  const toggles = page.locator('[id$=".engagement.toggle"]');
  const toggleCount = await toggles.count();
  for (let index = 0; index < toggleCount; index += 1) await toggles.nth(index).click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(1_500);
  const rows = page.locator('[data-slot="window-action-pane"] [id^="action."]');
  const count = await rows.count();
  const candidates = [];
  for (let index = 0; index < count; index += 1) {
    const id = ((await rows.nth(index).getAttribute("id")) ?? "").replace(/^action\./u, "");
    if (!id || id.startsWith("category.") || id.includes(".arg.")) continue;
    const label = ((await rows.nth(index).textContent().catch(() => "")) ?? "").trim();
    candidates.push({ index, id, staged: label.endsWith("…") || label.endsWith("...") });
  }
  const target = (wantedVerb ? candidates.find((entry) => entry.id === wantedVerb) : undefined) ?? candidates.find((entry) => /^add|^create|^insert/u.test(entry.id));
  if (!target) throw new Error(`no usable rail row among ${candidates.length}`);
  result.verb = target.id;
  log(`verb ${target.id} (staged=${target.staged}) of ${candidates.length} rows`);

  const baseline = await canvasText(page);
  await rows.nth(target.index).scrollIntoViewIfNeeded().catch(() => undefined);
  await rows.nth(target.index).click({ force: true }).catch(() => undefined);
  if (target.staged) {
    await page.waitForTimeout(1_000);
    const execute = page.locator(`[data-slot="window-action-pane"] [id$=".action.${target.id}.execute"]`).first();
    if ((await execute.count()) > 0) await execute.click({ force: true }).catch(() => undefined);
  }
  result.phases.push(await quietSamples(page, baseline, "after-verb"));
  const afterVerb = result.phases.at(-1).final;

  await page.keyboard.press("Meta+KeyZ");
  result.phases.push(await quietSamples(page, afterVerb, "after-undo"));
  const afterUndo = result.phases.at(-1).final;

  await page.keyboard.press("Meta+Shift+KeyZ");
  result.phases.push(await quietSamples(page, afterUndo, "after-redo"));
  const afterRedo = result.phases.at(-1).final;

  result.verdict = {
    verbMovedCanvas: afterVerb !== baseline,
    undoMovedCanvas: afterUndo !== afterVerb,
    redoMovedCanvas: afterRedo !== afterUndo,
    undoRestored: afterUndo === baseline,
    redoReapplied: afterRedo === afterVerb,
  };
  log(`verdict ${JSON.stringify(result.verdict)}`);
} catch (error) {
  result.fatal = String(error);
  log(`FATAL ${result.fatal}`);
} finally {
  result.refusals = [...new Set(refusals)].slice(0, 20);
  await browser.close();
}
const out = `${generated}s6-repaint-${pluginId}.txt`;
writeFileSync(out, JSON.stringify(result, null, 2));
console.log(`=== S6 REPAINT LATENCY ${pluginId} → ${out} ===`);
console.log(JSON.stringify(result, null, 2).slice(0, 6000));
process.exit(result.fatal ? 1 : 0);
