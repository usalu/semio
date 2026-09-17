/** 🧪️ Fem3d interactive probe: boots the react lane, opens the Artifact panel, picks a node row, reads the
 * model window's selection record, arms the Transform utility and reads the gumball descriptor, edits one
 * inspector number field (a `patchNode` → `replace-node` round trip) and counts the results window's
 * re-solves, stages a `translateSelection` through the model window's action pane, drives the Results
 * panel transport and samples the results scene, then switches to the House example.
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6087/?plugin=fem3d SEMIO_PROBE_OUT=fem3d-panels-1 bun 🐍️fem3d-panels-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6087/?plugin=fem3d";
const bootSeconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 120);
const treeNs = process.env.SEMIO_PROBE_TREE_NS ?? "fem3d-play-artifact";
const nodeId = process.env.SEMIO_PROBE_NODE ?? "n00_l1";
const modelSurface = process.env.SEMIO_PROBE_MODEL_SURFACE ?? "window:fem3d-model";
const resultsSurface = process.env.SEMIO_PROBE_RESULTS_SURFACE ?? "window:fem3d-results";
const modelWindow = process.env.SEMIO_PROBE_MODEL_WINDOW ?? "fem3dModel";
const houseSeconds = Number(process.env.SEMIO_PROBE_HOUSE_SECONDS ?? 90);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "fem3d-panels");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, msg.type() === "error" ? 6000 : 1200)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 1500)}`));
const report = {};
const flush = () => { writeFileSync(join(outDir, "console.txt"), lines.join("\n")); writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2)); };
const note = (key, value) => { report[key] = value; lines.push(`${Date.now() - t0} probe ${key} ${JSON.stringify(value).slice(0, 800)}`); console.log(`[DEBUG] ${key} ${JSON.stringify(value).slice(0, 700)}`); flush(); };
const shot = (name) => page.screenshot({ path: join(outDir, `${name}.png`), type: "png" }).catch(() => {});
// 📊 A re-solve is observed as the results scene changing (the guest prints no solve counter).
const solves = () => lines.filter((line) => line.includes("probe results-scene ")).length;
const resultsHash = async () => (await sceneOf(resultsSurface)).hash;
const guest = (from) => lines.slice(from).filter((line) => /\[DEBUG\] fem3d|trapped|panicked|action failed|shell fault|faults=|pageerror/.test(line)).map((line) => line.slice(0, 300)).slice(-12);
const state = () => page.evaluate(() => ({
  ready: document.documentElement.getAttribute("data-semio-os-ready"),
  error: document.documentElement.getAttribute("data-semio-os-error"),
  hosts: [...document.querySelectorAll("[data-surface-id]")].map((el) => ({ id: el.getAttribute("data-surface-id"), canvases: el.querySelectorAll("canvas").length })),
  tabs: [...document.querySelectorAll('button[role="tab"], [role="tablist"] button')].map((b) => b.textContent?.trim()).filter(Boolean).slice(0, 24),
  combobox: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null,
}));
const rows = (ns) => page.evaluate((ns) => [...document.querySelectorAll(`[id^="panel:${ns}/"]`)].map((e) => ({ id: e.id.replace(`panel:${ns}/`, ""), selected: e.getAttribute("aria-selected"), text: e.textContent?.trim().replace(/\s+/g, " ").slice(0, 70) })), ns);
const clickRow = async (id) => {
  const row = page.locator(`[id="${id}"]`).first();
  await row.waitFor({ state: "attached", timeout: 8000 });
  await row.evaluate((el) => el.scrollIntoView({ block: "center" }));
  await page.waitForTimeout(300);
  const box = await row.boundingBox();
  if (!box) throw new Error(`row ${id} has no box`);
  await page.mouse.click(box.x + Math.min(90, box.width / 2), box.y + box.height / 2);
  await page.waitForTimeout(2500);
};
const openTab = async (name) => { const tab = page.getByRole("button", { name, exact: true }).first(); if (await tab.count()) { await tab.click(); await page.waitForTimeout(2500); return "ok"; } return "absent"; };
const selectionOf = (surface) => page.evaluate((s) => { const el = document.querySelector(`[data-surface-id="${s}"]`); const parse = (raw) => { try { return raw ? JSON.parse(raw) : null; } catch { return raw?.slice(0, 400) ?? null; } }; return { ...parse(el?.getAttribute("data-selection-json")), guest: parse(el?.getAttribute("data-guest-selection-json")) }; }, surface);
const sceneOf = (surface) => page.evaluate((s) => { const el = document.querySelector(`[data-surface-id="${s}"]`); const attrs = {}; for (const a of el?.attributes ?? []) if (a.name.startsWith("data-")) attrs[a.name] = a.value.length; return { attrs, hash: [...(el?.outerHTML ?? "")].reduce((h, c) => (h * 31 + c.charCodeAt(0)) >>> 0, 7), text: (el?.innerText ?? "").replace(/\s+/g, " ").slice(0, 160) }; }, surface);
const pane = (id) => page.evaluate((windowId) => {
  const el = document.getElementById(`framework.window.${windowId}.engagement`);
  const scoped = [...(el ?? document).querySelectorAll('[id^="action."]')].map((row) => row.id);
  return { found: Boolean(el), folded: el?.getAttribute("data-folded") ?? null, rows: (scoped.length ? scoped : [...document.querySelectorAll('[id^="action."]')].map((row) => row.id)).slice(0, 60) };
}, id);
const unfold = async (id) => {
  let opened = await pane(id);
  if (opened.rows.length === 0) {
    const toggle = page.locator(`[id="framework.window.${id}.engagement.toggle"]`).first();
    if (await toggle.count()) await toggle.click({ timeout: 8000 }).catch((e) => lines.push(`toggle ${String(e).slice(0, 120)}`));
    await page.waitForTimeout(2500);
    opened = await pane(id);
  }
  return opened;
};

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < bootSeconds; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length >= 2 && i > 8) break; if (s.error) break; }
note("boot", { ...s, solves: solves(), guest: guest(0) });
await shot("1-boot");

// ── artifact tree pick ────────────────────────────────────────────────────
{
  const from = lines.length;
  const tab = await openTab(process.env.SEMIO_PROBE_ARTIFACT_TAB ?? "Artifact");
  const before = await rows(treeNs);
  const target = before.find((row) => row.id === nodeId || row.id.endsWith(`/${nodeId}`) || row.id.endsWith(`.${nodeId}`));
  let picked = "absent";
  if (target) { await clickRow(`panel:${treeNs}/${target.id}`); picked = target.id; }
  const after = await rows(treeNs);
  note("tree-pick", { tab, rowsBefore: before.length, sample: before.slice(0, 12), picked, selectedRows: after.filter((row) => row.selected === "true").map((row) => row.id), modelSelection: await selectionOf(modelSurface), resultsSelection: await selectionOf(resultsSurface), guest: guest(from) });
  await shot("2-tree-pick");
}

// ── transform utility arms the gumball ────────────────────────────────────
{
  const from = lines.length;
  const unfoldBar = page.locator(`[id="framework.window.${modelWindow}.utilityBar.unfold"]`).first();
  if (await unfoldBar.count()) await unfoldBar.click({ timeout: 8000 }).catch(() => {});
  await page.waitForTimeout(1500);
  const button = page.locator(`[id="${process.env.SEMIO_PROBE_UTILITY ?? "transform"}"]`).first();
  let armed = "absent";
  if (await button.count()) armed = await button.click({ timeout: 8000 }).then(() => "clicked").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(3000);
  const selection = await selectionOf(modelSurface);
  const options = await page.evaluate(() => [...document.querySelectorAll('[role="checkbox"], [aria-pressed]')].map((el) => ({ id: el.id, label: el.getAttribute("aria-label") ?? el.textContent?.trim().slice(0, 30), pressed: el.getAttribute("aria-pressed") ?? el.getAttribute("aria-checked") })).slice(0, 24));
  note("transform-utility", { armed, gumballActive: selection?.gumballActive ?? null, gumballLiveDispatch: selection?.guest?.gumballLiveDispatch ?? null, guestGumballConfig: selection?.guest?.gumballConfig ?? null, transformMode: selection?.transformMode ?? null, gumballTarget: selection?.gumballTarget ?? null, gumballConfig: selection?.gumballConfig ?? null, ids: selection?.ids ?? null, options, guest: guest(from) });
  await shot("3-transform");
}

// ── inspector edit → patchNode → re-solve ─────────────────────────────────
{
  const from = lines.length;
  const solvesBefore = solves();
  const tab = "auto-opened-on-pick";
  const fields = await page.evaluate(() => [...document.querySelectorAll('input[type="number"], input[inputmode="decimal"], input[type="text"]')].map((el) => ({ id: el.id, name: el.getAttribute("name"), label: el.getAttribute("aria-label"), value: el.value })).slice(0, 24));
  const field = page.locator('input[type="number"]').nth(2);
  let edited = "absent";
  const resultsBefore = await sceneOf(resultsSurface);
  if (await field.count()) {
    const current = Number(await field.inputValue().catch(() => "0")) || 0;
    edited = await field.fill(String(current + 0.25)).then(async () => { await field.press("Enter"); return `filled:${current}->${current + 0.25}`; }).catch((e) => String(e).slice(0, 120));
  }
  await page.waitForTimeout(6000);
  const resultsAfter = await sceneOf(resultsSurface);
  note("inspector-edit", { tab, fields, edited, solvesBefore, solvesAfter: solves(), resultsMoved: resultsBefore.hash !== resultsAfter.hash, modelSelection: await selectionOf(modelSurface), guest: guest(from) });
  await shot("4-inspector");
}

// ── translateSelection through the model window's action pane ────────────
{
  const from = lines.length;
  const solvesBefore = solves();
  const artifactTab = page.locator('[id="framework.panel.artifact"]').first();
  if (await artifactTab.count()) await artifactTab.click({ timeout: 8000 }).catch(() => {});
  await page.waitForTimeout(1200);
  const opened = await unfold(modelWindow);
  const rowId = "action.translateSelection";
  let clicked = "absent";
  if (opened.rows.includes(rowId)) clicked = await page.locator(`[id="${rowId}"]`).first().click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const filled = [];
  for (const [key, value] of Object.entries({ ids: JSON.stringify([nodeId]), dx: "0.5", dy: "0", dz: "0" })) {
    const input = page.locator(`[id="${key}"], [name="${key}"], input[aria-label="${key}"]`).first();
    if (await input.count()) { await input.fill(String(value)).catch((e) => filled.push(`${key}:${String(e).slice(0, 60)}`)); filled.push(`${key}=${value}`); } else filled.push(`${key}:absent`);
  }
  const submit = page.locator(`[id="framework.window.${modelWindow}.action.translateSelection.execute"]`).first();
  let submitted = "none";
  if (await submit.count()) submitted = await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  else { await page.keyboard.press("Enter"); submitted = "enter"; }
  await page.waitForTimeout(6000);
  note("translate-selection", { opened: { found: opened.found, rows: opened.rows.length, sample: opened.rows.slice(0, 40) }, clicked, filled, submitted, solvesBefore, solvesAfter: solves(), modelSelection: await selectionOf(modelSurface), guest: guest(from) });
  await shot("5-translate");
}

// ── results transport ─────────────────────────────────────────────────────
{
  const from = lines.length;
  const panelTab = page.locator(`[id="${process.env.SEMIO_PROBE_RESULTS_TAB_ID ?? "fem3d.panel.results"}"]`).first();
  const tab = (await panelTab.count()) ? await panelTab.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 80)) : "absent";
  await page.waitForTimeout(2500);
  const buttons = await page.evaluate(() => [...document.querySelectorAll("button")].map((b) => b.getAttribute("aria-label") ?? b.textContent?.trim()).filter(Boolean).filter((t) => /play|pause|phase|loop|speed|wiedergabe/i.test(t)).slice(0, 12));
  // 🪟 Focus the results pane first: the panel binds to the FOCUSED window, so its play button only
  // reads (and tags) the results window once that pane is the active one.
  const resultsTitle = page.locator('[data-surface-id="' + resultsSurface + '"]').first();
  const focused = (await resultsTitle.count()) ? await resultsTitle.click({ position: { x: 20, y: 20 }, timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 80)) : "absent";
  await page.waitForTimeout(1500);
  const play = page.getByRole("button", { name: /^play$|^play \/ pause$|wiedergabe/i }).first();
  let played = "absent";
  if (await play.count()) played = await play.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const caption = () => page.evaluate(() => (document.body.innerText.match(/phase [0-9.]+ · ▶ [0-9.]+ Hz/) ?? [null])[0]);
  const a = await sceneOf(resultsSurface);
  const captionA = await caption();
  await page.waitForTimeout(1500);
  const b = await sceneOf(resultsSurface);
  const captionB = await caption();
  await page.waitForTimeout(4000);
  const c = await sceneOf(resultsSurface);
  const captionC = await caption();
  const ticks = lines.filter((line) => line.includes('"actionId":"resultAnimationTick"') && line.includes("performInvocation {")).length;
  const saturated = lines.filter((line) => line.includes("revision capacity")).length;
  // ⏸ Pause before leaving: the chain parks the clock (the resting phase is the frame on screen), and
  // the house switch below is measured on a still window, the way a user switches examples.
  const pause = page.getByRole("button", { name: /^pause$/i }).first();
  const paused = (await pause.count()) ? await pause.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent";
  await page.waitForTimeout(2500);
  const ticksAfterPause = lines.filter((line) => line.includes('"actionId":"resultAnimationTick"') && line.includes("performInvocation {")).length - ticks;
  const restingPhase = await page.evaluate(() => [...document.querySelectorAll('[role="slider"]')].map((s) => s.getAttribute("aria-valuenow")).slice(0, 1)[0] ?? null);
  note("results-transport", { tab, buttons, focused, played, animates: a.hash !== b.hash || b.hash !== c.hash, captions: [captionA, captionB, captionC], ticks, paused, ticksAfterPause, restingPhase, saturated, solves: solves(), guest: guest(from) });
  await shot("6-results");
}

// ── house example ─────────────────────────────────────────────────────────
{
  const from = lines.length;
  const solvesBefore = solves();
  const combo = page.locator('[role="combobox"]').first();
  let picked = "absent";
  if (await combo.count()) {
    await combo.click({ timeout: 5000 }).catch((e) => (picked = String(e).slice(0, 100)));
    await page.waitForTimeout(500);
    const option = page.locator('[role="option"]').filter({ hasText: process.env.SEMIO_PROBE_EXAMPLE ?? "House" }).first();
    if (await option.count()) picked = await option.click({ timeout: 5000 }).then(() => "ok").catch((e) => String(e).slice(0, 100));
    else { picked = "no-option"; await page.keyboard.press("Escape"); }
  }
  const started = Date.now();
  let solved = false;
  const resultsBefore = await resultsHash();
  for (let i = 0; i < houseSeconds; i++) { await page.waitForTimeout(1000); if ((await resultsHash()) !== resultsBefore) { solved = true; lines.push(`${Date.now() - t0} probe results-scene changed`); break; } }
  await page.waitForTimeout(4000);
  await openTab(process.env.SEMIO_PROBE_ARTIFACT_TAB ?? "Artifact");
  const tree = await rows(treeNs);
  note("house", { picked, solved, secondsToSolve: (Date.now() - started) / 1000, solvesAfter: solves(), model: await sceneOf(modelSurface), results: await sceneOf(resultsSurface), treeRows: tree.length, sample: tree.filter((row) => /raft|wall|roof|slab/.test(row.id)).map((row) => row.text).slice(0, 10), guest: guest(from) });
  await shot("7-house");
}

flush();
console.log("[DEBUG] PANELS DONE");
await browser.close();
