#!/usr/bin/env bun
/** 🤏️ U4 (ticket 26/09/18) — the three UX-completeness items wired live, judged inside ONE running `s` session:
 *
 * 1. PINCH — real multi-touch (CDP `Input.dispatchTouchEvent` on a `hasTouch` context, so Chrome itself mints
 *    `pointerType: "touch"` events) on the dag window's canvas: a spread zooms by whole engine notches about the
 *    centroid, a two-finger drag pans, and the finger left down after the other lifts stays latched (no marquee,
 *    no selection, no camera change). Read from the surface's `data-viewport-camera-json` mirror.
 * 2. DIAGRAM KEYS — the dag `NodeGraphHost` (`role="application"`): arrows move focus, Enter/Shift+Enter select
 *    through the guest (read back from `data-selection-json`), Escape clears; each step is spoken by the polite
 *    live region, in en and then in de. The ARIA snapshot of the surface is captured.
 * 3. CONTRAST — Settings → Theme → Appearances: a paint set equal to its foreground raises the localized inline
 *    `role="alert"` warning with the measured ratio (en `1.00`, de `1,00`) and `aria-invalid` on the swatch.
 *
 * Usage: bun 🐍️u4-pinch-diagram-contrast-probe.mjs <baseUrl> [--tag <tag>] [--out <ticket-relative dir>]
 */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { FAULT, NOISE, awaitBeacon, click, dismissIntroduction, openPalette, windowIds } from "./🐍️s6-all-kinds-sweep.mjs";

const argv = process.argv.slice(2);
const baseUrl = argv[0] ?? "http://127.0.0.1:6070/";
const valueOf = (flag, fallback) => (argv.includes(flag) ? argv[argv.indexOf(flag) + 1] : fallback);
const tag = valueOf("--tag", "adhoc");
const outDir = valueOf("--out", "🗑️generated");
const out = fileURLToPath(new URL(`./${outDir}/u4-probe-${tag}.txt`, import.meta.url));
const shotPath = (name) => fileURLToPath(new URL(`./${outDir}/u4-${tag}-${name}.png`, import.meta.url));
const log = (...parts) => console.log("[u4]", ...parts);
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

/** 🚀️ Opens one program from the command palette and waits until `readySelector` exists. */
async function openProgram(page, query, itemIds, readySelector) {
  await openPalette(page);
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  if ((await input.count()) === 0) return "command palette never opened";
  await input.fill(query);
  await page.waitForTimeout(1_500);
  let clicked = null;
  for (const id of itemIds) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    await item.waitFor({ state: "visible", timeout: 8_000 }).catch(() => undefined);
    if ((await item.count()) === 0) continue;
    await item.click({ force: true });
    clicked = id;
    break;
  }
  if (clicked === null) {
    await page.keyboard.press("Escape");
    return `no palette row ${itemIds.join(" | ")}`;
  }
  const deadline = Date.now() + 120_000;
  while (Date.now() < deadline) {
    if ((await page.locator(readySelector).count()) > 0) {
      await page.waitForTimeout(5_000);
      return `opened via ${clicked}`;
    }
    await page.waitForTimeout(500);
  }
  return `${clicked} clicked, ${readySelector} never appeared`;
}

/** 🤏️ One symmetric horizontal spread about the centre of `box`, from `from` to `to` px separation. */
async function spreadAt(cdp, box, from, to, ids) {
  const cx = box.x + box.width / 2;
  const cy = box.y + box.height / 2;
  await touch(cdp, "touchStart", [[ids[0], cx - from / 2, cy], [ids[1], cx + from / 2, cy]]);
  for (let i = 1; i <= 10; i += 1) {
    const half = (from + ((to - from) * i) / 10) / 2;
    await touch(cdp, "touchMove", [[ids[0], cx - half, cy], [ids[1], cx + half, cy]]);
    await sleep(30);
  }
  await touch(cdp, "touchEnd", []);
}

async function board2dJourney(page, cdp) {
  const result = { open: await openProgram(page, "puzzle", ["spawn.puzzle"], "[data-board-camera-json]") };
  const host = page.locator("[data-board-camera-json]").last();
  if ((await host.count()) === 0) return result;
  const canvas = host.locator("canvas").first();
  const box = await canvas.boundingBox();
  const camera = async () => JSON.parse((await host.getAttribute("data-board-camera-json")) || "null");
  result.before = await camera();
  await spreadAt(cdp, box, 80, 240, [11, 12]);
  await sleep(2_500);
  result.after = await camera();
  result.zoomRatio = result.before && result.after ? result.after.zoom / result.before.zoom : null;
  result.expectedZoomRatio = 3;
  await page.screenshot({ path: shotPath("board2d-after-pinch") }).catch(() => undefined);
  return result;
}

async function world3dJourney(page, cdp) {
  const result = { open: await openProgram(page, "block3d", ["spawn.block.s.block.block3d@1/*#editor"], "[data-viewport-camera-json]:not([data-gesture-surface])") };
  const host = page.locator("[data-viewport-camera-json][data-orbit-view-gizmo]").last();
  if ((await host.count()) === 0) return result;
  const box = await host.boundingBox();
  const pose = async () => JSON.parse((await host.getAttribute("data-viewport-camera-json")) || "null");
  const distance = (value) => (value?.position && value?.target ? Math.hypot(value.position[0] - value.target[0], value.position[1] - value.target[1], value.position[2] - value.target[2]) : null);
  result.before = await pose();
  await spreadAt(cdp, box, 80, 240, [21, 22]);
  await sleep(2_500);
  result.after = await pose();
  result.distanceBefore = distance(result.before);
  result.distanceAfter = distance(result.after);
  result.distanceRatio = result.distanceBefore && result.distanceAfter ? result.distanceAfter / result.distanceBefore : null;
  result.expectedDistanceRatio = 1 / 3;
  await page.screenshot({ path: shotPath("world3d-after-pinch") }).catch(() => undefined);
  return result;
}

async function openDag(page) {
  const before = await windowIds(page);
  await openPalette(page);
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  if ((await input.count()) === 0) return { windowIds: [], detail: "command palette never opened" };
  await input.fill("dag");
  const item = page.locator('[data-slot="command-item"][data-command-item-id="spawn.dag"]').first();
  await item.waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
  if ((await item.count()) === 0) return { windowIds: [], detail: "no spawn.dag row" };
  await item.click({ force: true });
  const deadline = Date.now() + 120_000;
  while (Date.now() < deadline) {
    if ((await page.locator('[data-gesture-surface="dag"]').count()) > 0) {
      await page.waitForTimeout(4_000);
      return { windowIds: (await windowIds(page)).filter((id) => !before.includes(id)), detail: null };
    }
    await page.waitForTimeout(500);
  }
  return { windowIds: (await windowIds(page)).filter((id) => !before.includes(id)), detail: "no dag gesture surface appeared" };
}

const readCamera = (page) => page.evaluate(() => {
  const surface = document.querySelector('[data-gesture-surface="dag"]')?.parentElement;
  const raw = surface?.getAttribute("data-viewport-camera-json");
  return raw ? JSON.parse(raw) : null;
});

const readGraph = (page) => page.evaluate(() => {
  const host = document.querySelector('[data-gesture-surface="dag"]')?.closest('[role="application"]');
  if (!host) return null;
  const selection = JSON.parse(host.getAttribute("data-selection-json") ?? "{}");
  return {
    role: host.getAttribute("role"),
    tabIndex: host.getAttribute("tabindex"),
    roleDescription: host.getAttribute("aria-roledescription"),
    label: host.getAttribute("aria-label"),
    describedBy: host.getAttribute("aria-describedby"),
    help: document.getElementById(host.getAttribute("aria-describedby") ?? "")?.textContent ?? null,
    focusedNode: host.getAttribute("data-diagram-focused-node"),
    announcement: host.querySelector("[data-diagram-announcement]")?.textContent ?? null,
    liveRole: host.querySelector("[data-diagram-announcement]")?.getAttribute("role") ?? null,
    livePoliteness: host.querySelector("[data-diagram-announcement]")?.getAttribute("aria-live") ?? null,
    selectedIds: selection.selectedIds ?? null,
    sessionSelection: JSON.parse(document.querySelector('[data-gesture-surface="dag"]')?.parentElement?.getAttribute("data-session-selection-json") ?? "null"),
    hoverTarget: selection.hoverTarget ?? null,
    marquee: document.querySelectorAll('[data-gesture-surface="dag"] ~ *, [data-slot="selection-marquee"]').length,
    lang: document.documentElement.lang,
  };
});

async function touch(cdp, type, points) {
  await cdp.send("Input.dispatchTouchEvent", { type, touchPoints: points.map(([id, x, y]) => ({ id, x, y, radiusX: 4, radiusY: 4, force: 1 })) });
}

async function pinchJourney(page, cdp) {
  const box = await page.locator('[data-gesture-surface="dag"]').first().boundingBox();
  const cx = box.x + box.width / 2;
  const cy = box.y + box.height / 2;
  const result = { box };
  const events = await page.evaluate(() => {
    const seen = [];
    const surface = document.querySelector('[data-gesture-surface="dag"]');
    for (const type of ["pointerdown", "pointermove", "pointerup", "pointercancel"]) surface.addEventListener(type, (event) => seen.push(`${type}:${event.pointerType}:${event.pointerId}`), { capture: true });
    window.__u4PointerLog = seen;
    return true;
  });
  result.pointerLogArmed = events;

  result.cameraBefore = await readCamera(page);
  const selectionBefore = (await readGraph(page))?.selectedIds;
  await touch(cdp, "touchStart", [[1, cx - 50, cy]]);
  await touch(cdp, "touchStart", [[1, cx - 50, cy], [2, cx + 50, cy]]);
  for (let i = 1; i <= 10; i += 1) {
    const half = 50 + i * 10;
    await touch(cdp, "touchMove", [[1, cx - half, cy], [2, cx + half, cy]]);
    await sleep(30);
  }
  await touch(cdp, "touchEnd", []);
  await sleep(800);
  result.cameraAfterSpread = await readCamera(page);
  result.spreadExpected = { fingerScale: 300 / 100, wholeNotchesIn: Math.floor(Math.log(3) / Math.log(1.1) + 1e-12), zoomRatioIfUnclamped: 1.1 ** Math.floor(Math.log(3) / Math.log(1.1) + 1e-12) };
  result.spreadZoomRatio = result.cameraBefore && result.cameraAfterSpread ? result.cameraAfterSpread.zoom / result.cameraBefore.zoom : null;

  const beforePan = await readCamera(page);
  await touch(cdp, "touchStart", [[3, cx - 60, cy - 40], [4, cx + 60, cy - 40]]);
  for (let i = 1; i <= 8; i += 1) {
    await touch(cdp, "touchMove", [[3, cx - 60 + i * 5, cy - 40 + i * 10], [4, cx + 60 + i * 5, cy - 40 + i * 10]]);
    await sleep(30);
  }
  await touch(cdp, "touchEnd", []);
  await sleep(800);
  const afterPan = await readCamera(page);
  result.pan = { fingerTravel: { x: 40, y: 80 }, before: beforePan, after: afterPan, deltaWorld: beforePan && afterPan ? { x: afterPan.x - beforePan.x, y: afterPan.y - beforePan.y, zoomRatio: afterPan.zoom / beforePan.zoom } : null, expectedWorldY: beforePan ? -80 / beforePan.zoom : null };

  const beforeLatch = await readCamera(page);
  await touch(cdp, "touchStart", [[5, cx - 80, cy + 60], [6, cx + 80, cy + 60]]);
  await touch(cdp, "touchEnd", [[6, cx + 80, cy + 60]]);
  let marqueeSeen = 0;
  for (let i = 1; i <= 8; i += 1) {
    await touch(cdp, "touchMove", [[5, cx - 80 + i * 25, cy + 60 - i * 20]]);
    marqueeSeen = Math.max(marqueeSeen, await page.locator('[data-slot="selection-marquee"]').count());
    await sleep(30);
  }
  await touch(cdp, "touchEnd", []);
  await sleep(1_200);
  const afterLatch = await readCamera(page);
  result.latch = { before: beforeLatch, after: afterLatch, cameraUnchanged: JSON.stringify(beforeLatch) === JSON.stringify(afterLatch), marqueeSeenWhileLatched: marqueeSeen, selectionBefore, selectionAfter: (await readGraph(page))?.selectedIds };
  result.pointerLog = await page.evaluate(() => window.__u4PointerLog.slice(0, 80));
  result.touchPointerEvents = result.pointerLog.filter((line) => line.includes(":touch:")).length;
  return result;
}

async function keyboardJourney(page, localeTag) {
  const host = page.locator('[data-gesture-surface="dag"]').first().locator('xpath=ancestor::*[@role="application"][1]');
  await host.focus();
  const steps = [];
  const record = async (key) => {
    await page.keyboard.press(key);
    await page.waitForTimeout(key.includes("Enter") || key === "Escape" ? 2_500 : 600);
    const graph = await readGraph(page);
    steps.push({ key, focused: graph?.focusedNode ?? null, announcement: graph?.announcement ?? null, sessionSelection: graph?.sessionSelection ?? null, sceneSelection: graph?.selectedIds ?? null });
  };
  await record("ArrowRight");
  await record("ArrowRight");
  await record("Enter");
  await record("ArrowDown");
  await record("Shift+Enter");
  await record("Escape");
  const aria = await host.ariaSnapshot().catch((error) => `ariaSnapshot failed: ${String(error).slice(0, 120)}`);
  return { locale: localeTag, activeIsHost: await host.evaluate((element) => document.activeElement === element), surface: await readGraph(page), steps, aria: String(aria).slice(0, 1_500) };
}

/** 🌐️ Seats the shell locale through the live Settings → General → Language combobox (the s6 helper
 * predates that tree). */
async function seatShellLocale(page, wanted) {
  if ((await page.locator('[id="framework.settings.general"]').count()) === 0) await click(page, '[id="framework.settings"]');
  await page.waitForTimeout(2_000);
  await click(page, '[id="framework.settings.general"]');
  await page.waitForTimeout(1_500);
  const combo = page.locator('[id="framework.settings.language"][role="combobox"]').first();
  if ((await combo.count()) === 0) return `${wanted}:no-language-combobox`;
  await combo.click({ force: true });
  await page.waitForTimeout(800);
  const options = await page.locator('[role="option"]').allTextContents();
  const option = page.locator('[role="option"]').filter({ hasText: wanted === "de" ? /deutsch|german/iu : /english|englisch/iu }).first();
  if ((await option.count()) === 0) return `${wanted}:no-option ${JSON.stringify(options.slice(0, 8))}`;
  await option.click({ force: true });
  await page.waitForTimeout(4_000);
  return `${wanted}=${await page.evaluate(() => document.documentElement.lang || null)}`;
}

/** 🪟️ Brings the dag window back in front after a Settings detour (the panel shares the dock). */
async function refocusDag(page, ids) {
  if ((await page.locator('[id="framework.settings.theme"], [id="framework.settings.general"]').count()) > 0) await click(page, '[id="framework.settings"]');
  await page.waitForTimeout(1_500);
  for (const id of ids) {
    if ((await page.locator('[data-gesture-surface="dag"]').count()) > 0) break;
    await click(page, `[data-window-id="${id}"]`);
    await page.waitForTimeout(2_000);
  }
  return (await page.locator('[data-gesture-surface="dag"]').count()) > 0 ? "dag visible" : "dag surface absent";
}

async function contrastJourney(page, localeTag) {
  const result = { locale: localeTag };
  if ((await page.locator('[id="framework.settings.theme"]').count()) === 0) result.openSettings = await click(page, '[id="framework.settings"]');
  await page.waitForTimeout(2_500);
  result.themeTab = await click(page, '[id="framework.settings.theme"]');
  await page.waitForTimeout(2_000);
  const expand = async (id) => {
    const row = page.locator(`[id="${id}"][role="treeitem"], [id="${id}"][role="button"], button[id="${id}"]`).first();
    if ((await row.count()) === 0) return "absent";
    const expanded = await row.getAttribute("aria-expanded");
    const childPrefix = `${id}.`;
    const open = await page.evaluate((prefix) => [...document.querySelectorAll("[id]")].some((element) => element.id.startsWith(prefix) && element.getAttribute("role") === "treeitem" && element.getClientRects().length > 0), childPrefix);
    if (expanded === "true" || (expanded === null && open)) return "already-open";
    return row.click({ timeout: 8_000 }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 80));
  };
  for (const id of ["framework.settings.theme.appearances", "framework.settings.theme.appearances.light", "framework.settings.theme.appearances.light.chrome"]) {
    result[`expand:${id}`] = await expand(id);
    await page.waitForTimeout(1_200);
  }
  const inventory = await page.evaluate(() => ({
    colorInputs: [...document.querySelectorAll('input[type="color"]')].map((input) => ({ label: input.getAttribute("aria-label"), value: input.value, describedBy: input.getAttribute("aria-describedby") })).slice(0, 40),
    badges: [...document.querySelectorAll("[data-contrast-grade]")].map((badge) => ({ id: badge.id.replace(/^framework\.settings\.theme\.appearances\./u, ""), grade: badge.getAttribute("data-contrast-grade"), counterpart: badge.getAttribute("data-contrast-counterpart"), text: badge.textContent })).slice(0, 40),
    warnings: [...document.querySelectorAll("[data-contrast-warning]")].map((warning) => warning.textContent).slice(0, 20),
    themeIds: [...document.querySelectorAll('[id^="framework.settings.theme"]')].map((element) => element.id).slice(0, 40),
  }));
  result.inventoryBefore = { colorInputs: inventory.colorInputs.length, badges: inventory.badges, warnings: inventory.warnings };
  const fg = inventory.colorInputs.find((input) => input.label === "foreground");
  const target = inventory.colorInputs.find((input) => input.label && input.label !== "foreground" && input.describedBy);
  result.foreground = fg ?? null;
  result.target = target ?? null;
  if (!fg || !target) return result;
  const swatch = page.locator(`input[type="color"][aria-label="${target.label}"]`).first();
  await swatch.fill(fg.value);
  await page.waitForTimeout(2_500);
  result.after = await page.evaluate((label) => {
    const input = document.querySelector(`input[type="color"][aria-label="${label}"]`);
    const ids = (input?.getAttribute("aria-describedby") ?? "").split(" ").filter(Boolean);
    const warning = document.querySelector("[data-contrast-warning]");
    return {
      value: input?.value ?? null,
      ariaInvalid: input?.getAttribute("aria-invalid") ?? null,
      describedBy: ids.map((id) => ({ id, text: document.getElementById(id)?.textContent ?? null, role: document.getElementById(id)?.getAttribute("role") ?? null })),
      warning: warning ? { text: warning.textContent, role: warning.getAttribute("role"), id: warning.id } : null,
      badge: ids[0] ? { grade: document.getElementById(ids[0])?.getAttribute("data-contrast-grade") ?? null, ratio: document.getElementById(ids[0])?.getAttribute("data-contrast-ratio") ?? null } : null,
      lang: document.documentElement.lang,
    };
  }, target.label);
  await page.screenshot({ path: shotPath(`contrast-${localeTag}`) }).catch(() => undefined);
  await page.keyboard.press("Escape").catch(() => undefined);
  await page.waitForTimeout(1_000);
  return result;
}

const browser = await chromium.launch({ headless: process.env.U4_HEADED !== "1", args: ["--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, hasTouch: true });
const page = await context.newPage();
page.setDefaultNavigationTimeout(300_000);
const cdp = await context.newCDPSession(page);
const faults = [];
page.on("pageerror", (error) => faults.push(`pageerror: ${String(error)}`.slice(0, 300)));
page.on("console", (message) => {
  const text = message.text();
  if (FAULT.test(text) && !NOISE.test(text)) faults.push(`${message.type()}: ${text}`.slice(0, 300));
});

const result = { baseUrl, tag, startedAt: new Date().toISOString() };
try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  result.beacon = await awaitBeacon(page, Date.now() + 300_000);
  log(`beacon ${result.beacon}`);
  if (result.beacon === null) throw new Error("shell never set a readiness beacon");
  await dismissIntroduction(page);
  await page.waitForTimeout(5_000);
  result.dag = await openDag(page);
  log(`dag ${JSON.stringify(result.dag)}`);
  if (result.dag.windowIds.length === 0 && (await page.locator('[data-gesture-surface="dag"]').count()) === 0) throw new Error(`dag did not open: ${result.dag.detail}`);
  await page.screenshot({ path: shotPath("dag-opened") }).catch(() => undefined);
  result.pinch = await pinchJourney(page, cdp);
  log(`pinch ${JSON.stringify({ before: result.pinch.cameraBefore, afterSpread: result.pinch.cameraAfterSpread, ratio: result.pinch.spreadZoomRatio, pan: result.pinch.pan.deltaWorld, latch: { unchanged: result.pinch.latch.cameraUnchanged, marquee: result.pinch.latch.marqueeSeenWhileLatched }, touchEvents: result.pinch.touchPointerEvents })}`);
  await page.screenshot({ path: shotPath("dag-after-pinch") }).catch(() => undefined);
  result.keyboardEn = await keyboardJourney(page, "en");
  log(`keys en ${JSON.stringify(result.keyboardEn.steps)}`);
  result.contrastEn = await contrastJourney(page, "en");
  log(`contrast en ${JSON.stringify(result.contrastEn.after ?? result.contrastEn)}`);
  result.seatDe = await seatShellLocale(page, "de");
  log(`locale ${result.seatDe}`);
  result.refocusDag = await refocusDag(page, result.dag.windowIds);
  result.keyboardDe = await keyboardJourney(page, "de");
  log(`keys de ${JSON.stringify(result.keyboardDe.steps)}`);
  result.contrastDe = await contrastJourney(page, "de");
  log(`contrast de ${JSON.stringify(result.contrastDe.after ?? result.contrastDe)}`);
  result.board2d = await board2dJourney(page, cdp);
  log(`board2d ${JSON.stringify(result.board2d)}`);
  result.world3d = await world3dJourney(page, cdp);
  log(`world3d ${JSON.stringify({ open: result.world3d.open, distanceBefore: result.world3d.distanceBefore, distanceAfter: result.world3d.distanceAfter, ratio: result.world3d.distanceRatio })}`);
} catch (error) {
  result.fatal = String(error).slice(0, 500);
  log(`FATAL ${result.fatal}`);
} finally {
  result.faults = faults.slice(0, 40);
  await browser.close();
}
const near = (value, expected, tolerance) => typeof value === "number" && Math.abs(value - expected) <= tolerance;
const keysPass = (run, pattern) => Boolean(run) && run.steps.length === 6 && run.steps.every((step) => typeof step.announcement === "string" && step.announcement.length > 0) && JSON.stringify(run.steps[4].sessionSelection) === JSON.stringify([run.steps[2].focused, run.steps[4].focused]) && run.steps[5].sessionSelection?.length === 0 && pattern.test(run.steps[0].announcement);
const contrastPass = (run, ratioText) => Boolean(run?.after?.warning) && run.after.warning.role === "alert" && run.after.warning.text.includes(ratioText) && run.after.ariaInvalid === "true";
result.verdicts = {
  dagPinchZoom: near(result.pinch?.spreadZoomRatio, result.pinch?.spreadExpected?.zoomRatioIfUnclamped ?? NaN, 1e-9) ? "PASS" : "FAIL",
  dagTwoFingerPanY: near(result.pinch?.pan?.deltaWorld?.y, result.pinch?.pan?.expectedWorldY ?? NaN, 1e-6) ? "PASS" : "FAIL",
  dagLatch: result.pinch?.latch?.cameraUnchanged && result.pinch.latch.marqueeSeenWhileLatched === 0 ? "PASS" : "FAIL",
  world3dPinchDolly: near(result.world3d?.distanceRatio, 1 / 3, 1e-3) ? "PASS" : result.world3d?.distanceRatio == null ? `UNAVAILABLE: ${result.world3d?.open ?? "not run"}` : "FAIL",
  board2dPinchZoom: near(result.board2d?.zoomRatio, 3, 1e-3) ? "PASS" : result.board2d?.zoomRatio == null ? `UNAVAILABLE: ${result.board2d?.open ?? "not run"}` : "FAIL",
  keyboardEn: keysPass(result.keyboardEn, / of /u) ? "PASS" : "FAIL",
  keyboardDe: keysPass(result.keyboardDe, / von /u) ? "PASS" : "FAIL",
  contrastEn: contrastPass(result.contrastEn, "1.00:1") ? "PASS" : "FAIL",
  contrastDe: contrastPass(result.contrastDe, "1,00:1") ? "PASS" : "FAIL",
};
writeFileSync(out, JSON.stringify(result, null, 2));
log(`verdicts ${JSON.stringify(result.verdicts)}`);
log(`=== U4 ${tag} → ${out} ===`);
process.exit(result.fatal || Object.values(result.verdicts).includes("FAIL") ? 1 : 0);
