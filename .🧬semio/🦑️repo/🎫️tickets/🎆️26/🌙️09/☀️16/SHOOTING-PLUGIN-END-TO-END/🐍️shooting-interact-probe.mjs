/** 🎛️ Shooting interaction probe: boots the shooting react playground (6019), then (1) records the boot
 * state (shell beacon, scene/icon window hosts, Artifact tree shot rows), (2) clicks the Catalogue panel's
 * "PNG Rectangle" row (`addShot`) and checks the Artifact tree gains a shot, (3) presses mod+z and checks
 * the shot is removed again (undo proves the mutation landed in the document store), (4) flips the scene
 * window's Shadow toggle measure (`setShadowEnabled`, `pressed` contract), (5) selects the other shot in
 * the Artifact tree (`setShotSelection`, config lane) and checks the Inspection panel follows.
 * Every step records the console delta, guest/host fault lines and a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=shooting-interact-1 bun 🐍️shooting-interact-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6019/?plugin=shooting";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "shooting-interact");
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 12);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not a framework-reserved|do not decode/.test(l)).map((l) => l.slice(0, 400));
const historyLines = (from) => lines.slice(from).filter((l) => /history patch applied|patchCursor/.test(l)).map((l) => l.slice(0, 200));
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from), history: historyLines(from).slice(0, 6) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1500)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};
const state = () => page.evaluate(() => {
  const body = document.body.innerText.replace(/\s+/g, " ");
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => { const st = parse(el.getAttribute("data-status-json")); return { id: el.getAttribute("data-surface-id"), phase: st?.phase, fault: st?.fault?.code ?? null, canvases: el.querySelectorAll("canvas").length, textLength: (el.innerText ?? "").length }; });
  const treeRows = [...document.querySelectorAll('[role="treeitem"]')].map((el) => el.innerText.replace(/\s+/g, " ").trim().slice(0, 60));
  const shotRows = treeRows.filter((t) => /^Shot \d+|Overview/.test(t));
  const toggles = [...document.querySelectorAll('[role="checkbox"], [role="switch"], button[aria-pressed]')].map((el) => ({ id: el.id, label: (el.getAttribute("aria-label") ?? el.innerText ?? "").slice(0, 40), pressed: el.getAttribute("aria-checked") ?? el.getAttribute("aria-pressed") ?? el.dataset.state }));
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    hosts,
    treeItems: treeRows.length,
    shotRows,
    treeRows: treeRows.slice(0, 24),
    toggles: toggles.slice(0, 24),
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    bodyHead: body.slice(0, 600),
  };
});
const clickRow = async (label) => {
  const row = page.locator(`[role="treeitem"]:has-text("${label}")`).first();
  if (!(await row.count())) return `absent:${label}`;
  return row.click({ timeout: 8000, force: true }).then(() => `ok:${label}`).catch((e) => String(e).slice(0, 120));
};

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length && s.shotRows.length > 0 && i > 8) break; if (s.error) break; }
await note("boot", s, 0);

// ── open the Artifact + Catalogue panels (collapsed by default in this layout) ──
{
  const from = lines.length;
  const opened = [];
  for (const id of ["framework.panel.artifact", "framework.panel.catalogue", "framework.panel.inspection"]) {
    const toggle = page.locator(`[id="${id}"]`).first();
    if (!(await toggle.count())) { opened.push(`absent:${id}`); continue; }
    const pressed = await toggle.getAttribute("aria-pressed");
    if (pressed === "true") { opened.push(`already:${id}`); continue; }
    opened.push(await toggle.click({ timeout: 8000, force: true }).then(() => `ok:${id}`).catch((e) => String(e).slice(0, 120)));
    await page.waitForTimeout(800);
  }
  let after = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (after.shotRows.length > 0) break; }
  await note("open-panels", { opened, ...after }, from);
  s = after;
}
let before = s;

// ── Catalogue "PNG Rectangle" → addShot ───────────────────────────────────
{
  const from = lines.length;
  const clicked = await clickRow("PNG Rectangle");
  await page.waitForTimeout(2500);
  // 🗂️ Artifact and Catalogue share the left panel group — switch back to the Artifact tree to count shots.
  const artifactTab = await page.locator('[id="framework.panel.artifact"]').first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  let after = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (after.shotRows.length >= 3) break; }
  const created = lines.slice(from).filter((l) => /history patch applied.*createShot/.test(l)).length;
  await note("catalogue-add-shot", { clicked, artifactTab, created, shotRows: after.shotRows, added: after.shotRows.length === 3, ...after }, from);
  before = after;
}

// ── undo through the keybinding ──────────────────────────────────────────
{
  const from = lines.length;
  // ↩️ The shell also journals its own panel-tab switches, so the first undo may revert the tab switch
  // made above — undo until the createShot edit is gone (at most three steps).
  // 🖱️ Focus the shell WITHOUT touching the scene canvas (a canvas click journals a `set-camera` edit).
  await page.locator('[id="framework.panel.artifact"]').first().focus().catch(() => {});
  const ensureArtifactPanel = async () => {
    const toggle = page.locator('[id="framework.panel.artifact"]').first();
    if ((await toggle.getAttribute("aria-pressed")) !== "true") await toggle.click({ timeout: 4000, force: true }).catch(() => {});
    await page.waitForTimeout(800);
  };
  let after = null;
  let presses = 0;
  for (let attempt = 0; attempt < 4; attempt++) {
    await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
    presses += 1;
    await page.waitForTimeout(1500);
    await ensureArtifactPanel();
    for (let i = 0; i < settleSeconds; i++) { await page.waitForTimeout(500); after = await state(); if (after.shotRows.length === before.shotRows.length - 1) break; }
    if (after.shotRows.length === before.shotRows.length - 1) break;
  }
  const undone = lines.slice(from).filter((l) => /history patch applied.*"Undo"/.test(l)).length;
  await note("undo", { shotsBefore: before.shotRows.length, presses, undoPatches: undone, undone: after.shotRows.length === before.shotRows.length - 1, shotRows: after.shotRows, hosts: after.hosts }, from);
  before = after;
}

// ── scene window Shadow toggle measure → setShadowEnabled ────────────────
{
  const from = lines.length;
  const options = page.locator('[id="framework.window.shootingScene.measures.unfold"]').first();
  let optionsOpened = "absent";
  if (await options.count()) optionsOpened = await options.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const toggle = page.locator('[id="shooting-scene/shooting.measure.shadow"]').first();
  let clicked = `absent (options ${optionsOpened})`;
  if (await toggle.count()) clicked = await toggle.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(3000);
  const after = await state();
  const dispatched = lines.slice(from).filter((l) => /setShadowEnabled|change-scene-shadow|changeSceneShadow/.test(l)).map((l) => l.slice(0, 220));
  let published = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); published = await page.evaluate(() => document.getElementById("shooting-scene/shooting.measure.shadow")?.getAttribute("data-published-value") ?? null); if (published === "false") break; }
  await note("measure-shadow-toggle", { clicked, dispatched: dispatched.slice(0, 4), published, hosts: after.hosts }, from);
  await page.keyboard.press("Escape").catch(() => {});
}

// ── Artifact tree shot row → setShotSelection (config) ───────────────────
{
  const from = lines.length;
  const toggle = page.locator('[id="framework.panel.artifact"]').first();
  if ((await toggle.getAttribute("aria-pressed")) !== "true") await toggle.click({ timeout: 4000, force: true }).catch(() => {});
  await page.waitForTimeout(1000);
  before = await state();
  const target = before.shotRows.find((r) => /Overview Png/.test(r)) ?? before.shotRows[1] ?? before.shotRows[0];
  const clicked = target ? await clickRow(target) : "absent";
  await page.waitForTimeout(3000);
  const after = await state();
  const selection = lines.slice(from).filter((l) => /set-shot-selection|setShotSelection/.test(l)).map((l) => l.slice(0, 220));
  const inspector = await page.evaluate(() => [...document.querySelectorAll('[id^="shooting-play-inspector"] input')].map((el) => `${el.id}=${el.value}`));
  await note("tree-select-shot", { clicked, target, selection: selection.slice(0, 3), inspector, shotRows: after.shotRows, hosts: after.hosts }, from);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE", report.steps.length);
await browser.close();
