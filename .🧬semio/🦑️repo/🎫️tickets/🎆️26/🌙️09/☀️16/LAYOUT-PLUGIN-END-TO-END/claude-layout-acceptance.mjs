/** 🧪️ Layout acceptance probe: drives the React renderer through boot, demo document, frame/page mutations, undo/redo,
 * selection, exports, reload and tab lifecycle, writing one `{id, ok, detail, evidence}` row and one screenshot per step.
 * Usage: bun claude-layout-acceptance.mjs   (env: SEMIO_PROBE_URL, SEMIO_PROBE_RUN, SEMIO_PROBE_STEPS=A1,A3,...)
 * Selector sources: `🏛️ShellHost/🟦️.tsx` (ready beacon), `🪟️window-panes/🟦️.tsx` (action rows/args/execute),
 * `🪟️Window/🟦️.tsx` (engagement pane), `🌳️Tree/🍃️item/🟦️.tsx` (treeitem), `🧭️PanelTabBar/🟦️.tsx` (data-tab-id).
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync, readFileSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6079/?plugin=layout";
const run = process.env.SEMIO_PROBE_RUN ?? new Date().toISOString().replace(/[:.]/g, "-");
const outDir = join(dirname(fileURLToPath(import.meta.url)), "🗑️generated", "claude", `acceptance-${run}`);
const wanted = process.env.SEMIO_PROBE_STEPS ? new Set(process.env.SEMIO_PROBE_STEPS.split(",").map((s) => s.trim().toUpperCase())) : null;
const ART = "framework.panel.artifact";
const INSPECT = "framework.panel.inspection";
const SOFT = new Set(["A10"]);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const rows = [];
const log = (message) => console.log(`[DEBUG] ${message}`);
const stamp = (prefix, text) => lines.push(`${Date.now() - t0} ${prefix} ${text}`);
const attach = (p, tag) => {
  p.on("console", (m) => stamp(`${tag}${m.type()}`, m.text().slice(0, m.type() === "error" ? 6000 : 800)));
  p.on("pageerror", (e) => stamp(`${tag}pageerror`, String(e).slice(0, 2000)));
};
const FAULT = /panic|fault|refused|error/i;
const SET_ACTIVE = /setActiveExample refused/i;
const faultsFrom = (from) => lines.slice(from).filter((l) => FAULT.test(l) && !SET_ACTIVE.test(l));
const pageErrorsFrom = (from) => lines.slice(from).filter((l) => / pageerror /.test(l));
const consoleSummary = () => ({
  total: lines.length,
  faults: lines.filter((l) => FAULT.test(l) && !SET_ACTIVE.test(l)).length,
  setActiveExampleRefused: lines.filter((l) => SET_ACTIVE.test(l)).length,
  faultLines: lines.filter((l) => FAULT.test(l) && !SET_ACTIVE.test(l)).slice(0, 40).map((l) => l.slice(0, 400)),
});
const flush = () => {
  writeFileSync(join(outDir, "report.json"), JSON.stringify({ url, run, steps: rows, console: consoleSummary() }, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
};
const summary = () => {
  const hard = rows.filter((r) => !SOFT.has(r.id));
  const pass = rows.filter((r) => r.ok).length;
  const fail = hard.filter((r) => !r.ok).length;
  console.log(`[acceptance] pass=${pass} fail=${fail}`);
  return fail;
};

let browser = null;
let context = null;
let page = null;
const finish = async (code) => {
  flush();
  const fail = summary();
  await browser?.close().catch(() => {});
  process.exit(code ?? (fail === 0 && rows.length > 0 ? 0 : 1));
};
process.once("SIGINT", () => { void finish(130); });
process.once("SIGTERM", () => { void finish(143); });
const deadline = setTimeout(() => { log("global deadline 600s"); rows.push({ id: "deadline", ok: false, detail: "global deadline of 600s exceeded", evidence: {} }); void finish(1); }, 600000);
deadline.unref?.();

const state = (p = page) => p.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const html = document.documentElement;
  const items = [...document.querySelectorAll('[role="treeitem"]')];
  return {
    ready: html.getAttribute("data-semio-os-ready"),
    error: html.getAttribute("data-semio-os-error"),
    hosts: [...document.querySelectorAll("[data-surface-id]")].map((el) => { const st = parse(el.getAttribute("data-status-json")); return { id: el.getAttribute("data-surface-id"), phase: st?.phase ?? null, fault: st?.fault?.code ?? null, canvases: el.querySelectorAll("canvas").length, status: st }; }),
    frameIds: items.filter((el) => /^frame-/.test(el.id)).map((el) => el.id).sort(),
    pageIds: items.filter((el) => el.id.startsWith("layout-document.page.")).map((el) => el.id.replace("layout-document.page.", "")).sort(),
    selected: items.filter((el) => el.getAttribute("aria-selected") === "true").map((el) => el.id),
    treeItems: items.length,
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
    notices: [...document.querySelectorAll('[role="alert"],[role="status"],[data-slot*="toast"],[data-sonner-toast]')].map((el) => el.innerText.replace(/\s+/g, " ").trim().slice(0, 160)).filter(Boolean),
  };
});
const ownFrames = (ids) => ids.filter((id) => !/inherited/.test(id));
const sleep = (ms) => page.waitForTimeout(ms);

const waitReady = async (p, label, seconds = 180) => {
  let s = null;
  for (let i = 0; i < seconds; i++) {
    await p.waitForTimeout(1000);
    s = await state(p).catch(() => null);
    if (s?.error) return { ok: false, s, reason: `data-semio-os-error=${s.error}` };
    if (s?.ready === label && s.hosts.length) return { ok: true, s, seconds: i + 1 };
  }
  return { ok: false, s, reason: `timeout waiting for data-semio-os-ready=${label}` };
};

const panelActive = async (tabId) => page.evaluate((id) => {
  const tab = [...document.querySelectorAll(`[data-tab-id="${id}"]`)].find((el) => el.offsetParent !== null);
  return tab ? tab.getAttribute("data-active") === "true" : null;
}, tabId);
const setPanel = async (tabId, open) => {
  const active = await panelActive(tabId);
  if (active === null) return "absent";
  if (active === open) return "unchanged";
  const tab = page.locator(`[data-tab-id="${tabId}"]:visible`).first();
  await tab.click({ timeout: 8000, force: true }).catch(() => {});
  await sleep(700);
  return "toggled";
};
const readTree = async () => {
  const wasOpen = await panelActive(ART);
  if (wasOpen === false) await setPanel(ART, true);
  let s = null;
  for (let i = 0; i < 16; i++) { s = await state(); if (s.treeItems > 0 && s.pageIds.length) break; await sleep(500); }
  if (s && s.frameIds.length === 0) {
    for (const id of ["layout-document.frames", "layout-document.pages"]) {
      const group = page.locator(`[role="treeitem"][id="${id}"][aria-expanded="false"]`).first();
      if (await group.count()) await group.click({ timeout: 4000, force: true }).catch(() => {});
    }
    await sleep(800);
    s = await state();
  }
  if (wasOpen === false) await setPanel(ART, false);
  return { ...s, panel: wasOpen === null ? "absent" : "ok" };
};

const optionLabels = { rect: "Rectangle", text: "Text Frame", image: "Image Frame" };
const fillArg = async (key, value) => {
  const trigger = page.locator(`button[id="${key}"], [role="combobox"][id="${key}"]`).first();
  if (await trigger.count()) {
    await trigger.click({ timeout: 6000, force: true }).catch(() => {});
    const option = page.locator('[role="option"]').filter({ hasText: new RegExp(`^\\s*(${optionLabels[value] ?? value})\\s*$`, "i") }).first();
    if (await option.count()) { await option.click({ timeout: 6000 }).catch(() => {}); return `${key}=${value}:select`; }
    await page.keyboard.press("Escape");
    return `${key}:option-absent`;
  }
  const input = page.locator(`input[id="${key}"], [id="${key}"] input, textarea[id="${key}"]`).first();
  if (await input.count()) {
    await input.fill(String(value), { timeout: 6000 }).catch((e) => stamp("probe", `fill ${key} ${String(e).slice(0, 100)}`));
    await input.press("Tab").catch(() => {});
    return `${key}=${value}:input`;
  }
  const group = page.locator(`[id="${key}"] [value="${value}"], [id="${key}"] button`).filter({ hasText: new RegExp(optionLabels[value] ?? String(value), "i") }).first();
  if (await group.count()) { await group.click({ timeout: 6000, force: true }).catch(() => {}); return `${key}=${value}:segmented`; }
  return `${key}:absent`;
};
const confirmDialog = async () => {
  const button = page.locator('[role="alertdialog"] button, [role="dialog"] button').filter({ hasText: /confirm|ok|yes|export|execute|delete|best.tigen|ja|l.schen/i }).first();
  if (await button.count()) { await button.click({ timeout: 3000 }).catch(() => {}); return true; }
  return false;
};
const runAction = async (actionId, args = {}) => {
  await setPanel(ART, false);
  let s = await state();
  const engagement = s.engagements.find((id) => /blueprint/i.test(id)) ?? s.engagements[0] ?? null;
  const detail = { actionId, engagement, filled: [], found: false, executed: "none" };
  if (!engagement) return detail;
  if (!s.actionRows.includes(`action.${actionId}`)) {
    await page.locator(`[id="${engagement}.toggle"]`).first().click({ timeout: 8000, force: true }).catch(() => {});
    await sleep(1200);
    s = await state();
  }
  const rowId = `action.${actionId}`;
  if (!s.actionRows.includes(rowId)) { detail.availableActions = s.actionRows; return detail; }
  detail.found = true;
  const execute = page.locator(`[id$=".action.${actionId}.execute"]`).first();
  if (!(await execute.count())) {
    await page.locator(`[id="${rowId}"]`).first().click({ timeout: 8000, force: true }).catch((e) => { detail.rowClick = String(e).slice(0, 100); });
    await sleep(1200);
  }
  if (await execute.count()) {
    for (const [key, value] of Object.entries(args)) detail.filled.push(await fillArg(key, value));
    await sleep(300);
    const disabled = await execute.isDisabled().catch(() => false);
    detail.executeDisabled = disabled;
    if (!disabled) { await execute.click({ timeout: 8000 }).then(() => { detail.executed = "execute"; }).catch((e) => { detail.executed = String(e).slice(0, 100); }); }
    else detail.executed = "disabled";
  } else detail.executed = "row-direct";
  await sleep(300);
  detail.confirmed = await confirmDialog();
  return detail;
};
const keys = async (chord) => {
  await page.evaluate(() => { const el = document.activeElement; if (el && el !== document.body) el.blur(); });
  await page.keyboard.press(chord);
  await sleep(2500);
};
const settleTree = async (predicate, seconds = 12) => {
  let t = null;
  for (let i = 0; i < seconds * 2; i++) { t = await readTree(); if (predicate(t)) return t; await sleep(500); }
  return t;
};

const ctx = { frames: [], pages: [], newFrame: null, pageAdded: false, tree: null };
const downloadTo = async (name, trigger) => {
  const pending = page.waitForEvent("download", { timeout: 90000 }).then(async (d) => {
    const target = join(outDir, `${name}-${d.suggestedFilename() || "download"}`);
    await d.saveAs(target);
    return { suggested: d.suggestedFilename(), target, failure: await d.failure() };
  }).catch((e) => ({ error: String(e).slice(0, 200) }));
  const action = await trigger();
  const result = await pending;
  return { action, ...result };
};
const magic = { exportPdf: { head: "%PDF", ext: ".pdf" }, exportSvg: { head: "<svg", ext: ".svg" }, exportPng: { head: "\x89PNG", ext: ".png" } };
const checkDownload = (actionId, dl) => {
  if (dl.error || !dl.target) return { ok: false, why: dl.error ?? "no download", action: dl.action };
  const bytes = readFileSync(dl.target);
  const text = bytes.toString("latin1");
  const spec = magic[actionId];
  const head = actionId === "exportSvg" ? text.slice(0, 2048) : text.slice(0, 8);
  const ok = bytes.length > 0 && (actionId === "exportSvg" ? head.includes(spec.head) : head.startsWith(spec.head)) && dl.suggested.toLowerCase().endsWith(spec.ext) && !dl.failure;
  const out = { ok, suggested: dl.suggested, bytes: bytes.length, magicOk: ok, file: dl.target };
  if (actionId === "exportPdf") out.pdfPages = (text.match(/\/Type\s*\/Page(?![s\w])/g) ?? []).length;
  return out;
};

const steps = {
  async A1() {
    const r = await waitReady(page, "layout");
    const s = r.s ?? (await state());
    const blueprint = s.hosts.some((h) => /blueprint/i.test(h.id));
    const preview = s.hosts.some((h) => /preview/i.test(h.id));
    return { ok: r.ok && !s.error && blueprint && preview, detail: r.ok ? `ready after ${r.seconds}s; blueprint=${blueprint} preview=${preview}` : r.reason, evidence: { hosts: s.hosts.map(({ status, ...h }) => h), error: s.error } };
  },
  async A2() {
    const t = await readTree();
    ctx.tree = t;
    ctx.frames = t.frameIds;
    ctx.pages = t.pageIds;
    const needPages = ["page-1", "page-2"].filter((id) => !t.pageIds.includes(id));
    const needFrames = ["frame-1", "frame-text-1", "frame-image-1"].filter((id) => !t.frameIds.includes(id));
    const statuses = t.hosts.map((h) => ({ id: h.id, phase: h.phase, fault: h.fault, status: h.status }));
    return { ok: needPages.length === 0 && needFrames.length === 0 && t.hosts.every((h) => !h.fault), detail: `pages=${t.pageIds} frames=${t.frameIds}${needPages.length || needFrames.length ? ` missing=${[...needPages, ...needFrames]}` : ""}`, evidence: { pageIds: t.pageIds, frameIds: t.frameIds, statuses } };
  },
  async A3() {
    const before = ownFrames((await readTree()).frameIds);
    const a = await runAction("addFrame", { kind: "rect" });
    const t = await settleTree((x) => ownFrames(x.frameIds).length === before.length + 1);
    const added = ownFrames(t.frameIds).filter((id) => !before.includes(id));
    ctx.newFrame = added[0] ?? null;
    ctx.frames = t.frameIds;
    const expected = `frame-${before.filter((id) => /^frame-\d+$/.test(id)).length + 1}`;
    return { ok: added.length === 1, detail: `before=${before.length} after=${ownFrames(t.frameIds).length} new=${added} expectedGuess=${expected}`, evidence: { action: a, added, frameIds: t.frameIds } };
  },
  async A4() {
    const before = ownFrames((await readTree()).frameIds);
    await keys("Control+z");
    const t = await settleTree((x) => ownFrames(x.frameIds).length === before.length - 1);
    return { ok: ownFrames(t.frameIds).length === before.length - 1, detail: `before=${before.length} after=${ownFrames(t.frameIds).length}`, evidence: { frameIds: t.frameIds } };
  },
  async A5() {
    const before = ownFrames((await readTree()).frameIds);
    await keys("Control+Shift+z");
    const t = await settleTree((x) => ownFrames(x.frameIds).length === before.length + 1);
    ctx.frames = t.frameIds;
    return { ok: ownFrames(t.frameIds).length === before.length + 1, detail: `before=${before.length} after=${ownFrames(t.frameIds).length}`, evidence: { frameIds: t.frameIds } };
  },
  async A6() {
    const frameId = ctx.newFrame;
    if (!frameId) return { ok: false, detail: "no new frame from A3", evidence: {} };
    await setPanel(ART, true);
    const row = page.locator(`[role="treeitem"][id="${frameId}"]`).first();
    if (await row.count()) await row.click({ timeout: 8000, force: true }).catch(() => {});
    await sleep(1500);
    await setPanel(ART, false);
    const inspector = await setPanel(INSPECT, true);
    await sleep(1200);
    const field = (key) => page.locator(`[id="layout-play-inspector.patchFrame.${key}.input"]`).first();
    const read = async () => {
      const out = {};
      for (const key of ["x", "y", "width", "height"]) out[key] = (await field(key).count()) ? Number(await field(key).inputValue()) : null;
      return out;
    };
    const initial = await read();
    if (Object.values(initial).every((v) => v === null)) {
      await setPanel(INSPECT, false);
      return { ok: false, detail: `unmapped: inspector inputs layout-play-inspector.patchFrame.*.input absent (inspector=${inspector}); patchFrame is not an Actions pane row`, evidence: { inspector, initial } };
    }
    const set = async (key, value) => {
      const input = field(key);
      await input.fill(String(value), { timeout: 6000 }).catch((e) => stamp("probe", `inspector fill ${key} ${String(e).slice(0, 100)}`));
      await input.press("Enter").catch(() => {});
      await input.blur().catch(() => {});
      await sleep(900);
    };
    const target = { x: 100, y: 150, width: 80, height: 30 };
    for (const [key, value] of Object.entries(target)) await set(key, value);
    await sleep(1500);
    const patched = await read();
    const geometryOk = Object.entries(target).every(([key, value]) => patched[key] === value);
    const from = lines.length;
    await set("width", "NaN");
    const afterNaN = await read();
    const nanFault = lines.slice(from).filter((l) => /pageerror|panic|trapped|unreachable/i.test(l));
    const nanRefused = afterNaN.width === patched.width && nanFault.length === 0;
    const notices = (await state()).notices;
    await setPanel(INSPECT, false);
    return { ok: geometryOk && nanRefused, detail: `patched=${JSON.stringify(patched)} geometryOk=${geometryOk} afterNaN=${JSON.stringify(afterNaN)} nanRefused=${nanRefused}`, evidence: { frameId, initial, patched, afterNaN, nanFault, notices } };
  },
  async A7() {
    const before = (await readTree()).pageIds;
    const a = await runAction("addPage", {});
    const t = await settleTree((x) => x.pageIds.length === before.length + 1);
    const added = t.pageIds.filter((id) => !before.includes(id));
    ctx.pageAdded = added.length === 1;
    ctx.pages = t.pageIds;
    let back = { found: false };
    if (ctx.pageAdded) {
      back = await runAction("setActivePage", { pageId: "page-1" });
      if (!back.found) {
        await setPanel(ART, true);
        const row = page.locator('[role="treeitem"][id="layout-document.page.page-1"]').first();
        if (await row.count()) { await row.click({ timeout: 8000, force: true }).catch(() => {}); back = { found: true, via: "treeitem-click" }; }
        await setPanel(ART, false);
      }
    }
    return { ok: added.length === 1 && added[0] === "page-3", detail: `pagesBefore=${before} pagesAfter=${t.pageIds} added=${added} setActivePage=${back.found ? back.via ?? back.executed : "unmapped"}`, evidence: { action: a, added, setActivePage: back, pageIds: t.pageIds } };
  },
  async A8() {
    const ev = {};
    await setPanel(ART, true);
    const row = page.locator('[role="treeitem"][id="frame-1"]').first();
    if (!(await row.count())) return { ok: false, detail: "frame-1 tree item absent", evidence: {} };
    await row.click({ timeout: 8000, force: true });
    await sleep(1500);
    ev.afterClick = (await state()).selected;
    const clickSelected = ev.afterClick.includes("frame-1");
    await setPanel(ART, false);
    await keys("Control+a");
    await setPanel(ART, true);
    ev.afterSelectAll = (await state()).selected;
    await setPanel(ART, false);
    const selectAllOk = ev.afterSelectAll.length > ev.afterClick.length || ev.afterSelectAll.length > 1;
    await keys("Escape");
    await setPanel(ART, true);
    ev.afterClear = (await state()).selected.filter((id) => /^frame-/.test(id));
    const clearOk = ev.afterClear.length === 0;
    await row.click({ timeout: 8000, force: true }).catch(() => {});
    await sleep(1500);
    const framesBefore = ownFrames((await state()).frameIds);
    await setPanel(ART, false);
    let del = await runAction("deleteSelection", {});
    ev.deleteVia = del.found ? "action" : "context-menu";
    if (!del.found) {
      await setPanel(ART, true);
      await row.click({ button: "right", timeout: 8000, force: true }).catch(() => {});
      await sleep(800);
      const item = page.locator('[role="menuitem"]').filter({ hasText: /delete|l.schen|remove/i }).first();
      if (await item.count()) { await item.click({ timeout: 5000 }).catch(() => {}); ev.deleteVia = "context-menu:clicked"; await confirmDialog(); } else ev.deleteVia = "unmapped";
    }
    ev.delete = del;
    const afterDelete = await settleTree((x) => !x.frameIds.includes("frame-1"), 8);
    const deleted = !afterDelete.frameIds.includes("frame-1");
    await keys("Control+z");
    const restored = await settleTree((x) => x.frameIds.includes("frame-1"), 10);
    const undone = restored.frameIds.includes("frame-1");
    ev.framesBefore = framesBefore;
    ev.framesAfterDelete = afterDelete.frameIds;
    ev.framesRestored = restored.frameIds;
    return { ok: clickSelected && selectAllOk && clearOk && deleted && undone, detail: `click=${clickSelected} selectAll=${selectAllOk}(${ev.afterSelectAll.length}) clear=${clearOk} deleteVia=${ev.deleteVia} deleted=${deleted} undoRestored=${undone}`, evidence: ev };
  },
  async A9() {
    const ev = {};
    let ok = true;
    for (const actionId of ["exportPdf", "exportSvg", "exportPng"]) {
      const dl = await downloadTo(actionId, () => runAction(actionId, {}));
      const checked = checkDownload(actionId, dl);
      if (actionId === "exportPdf") {
        const expected = ctx.pageAdded ? 3 : 2;
        checked.expectedPages = expected;
        checked.ok = checked.ok && (ctx.pageAdded ? checked.pdfPages === expected : checked.pdfPages >= expected);
      }
      ev[actionId] = checked;
      ok = ok && checked.ok;
    }
    return { ok, detail: Object.entries(ev).map(([k, v]) => `${k}:${v.ok ? "ok" : "FAIL"}:${v.bytes ?? 0}B${v.pdfPages !== undefined ? ` pages=${v.pdfPages}` : ""}`).join(" "), evidence: ev };
  },
  async A10() {
    const before = await readTree();
    await page.reload({ waitUntil: "domcontentloaded" });
    const r = await waitReady(page, "layout");
    if (!r.ok) return { ok: false, detail: r.reason, evidence: {} };
    const after = await readTree();
    return { ok: true, detail: `observed: frames ${before.frameIds.length}->${after.frameIds.length} pages ${before.pageIds.length}->${after.pageIds.length}; newFrameSurvived=${ctx.newFrame ? after.frameIds.includes(ctx.newFrame) : null} page3Survived=${after.pageIds.includes("page-3")}`, evidence: { before: { frameIds: before.frameIds, pageIds: before.pageIds }, after: { frameIds: after.frameIds, pageIds: after.pageIds } } };
  },
  async A11() {
    const second = await context.newPage();
    attach(second, "tab2:");
    await second.goto(url, { waitUntil: "domcontentloaded" });
    const r = await waitReady(second, "layout");
    await second.screenshot({ path: join(outDir, "A11-second-tab.png") }).catch(() => {});
    await second.close();
    await page.bringToFront();
    await sleep(1000);
    const before = ownFrames((await readTree()).frameIds);
    await runAction("addFrame", { kind: "rect" });
    const t = await settleTree((x) => ownFrames(x.frameIds).length === before.length + 1);
    const grew = ownFrames(t.frameIds).length === before.length + 1;
    return { ok: r.ok && grew, detail: `secondTabReady=${r.ok}${r.ok ? "" : ` (${r.reason})`} firstTabAddFrame=${grew} frames ${before.length}->${ownFrames(t.frameIds).length}`, evidence: { secondTab: { ok: r.ok, seconds: r.seconds }, frameIds: t.frameIds } };
  },
};

try {
  browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--enable-unsafe-swiftshader"] });
  context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, acceptDownloads: true });
  page = await context.newPage();
  attach(page, "");
  await page.goto(url, { waitUntil: "domcontentloaded" });
  for (const id of Object.keys(steps)) {
    if (wanted && id !== "A1" && !wanted.has(id)) continue;
    const from = lines.length;
    let row;
    try {
      const r = await steps[id]();
      row = { id, ok: Boolean(r.ok), detail: r.detail, evidence: r.evidence };
    } catch (e) {
      row = { id, ok: false, detail: `exception: ${String(e).slice(0, 400)}`, evidence: {} };
    }
    const pageErrors = pageErrorsFrom(from);
    if (pageErrors.length) { row.ok = false; row.detail += ` | pageerrors=${pageErrors.length}`; }
    row.evidence = { ...row.evidence, faultLines: faultsFrom(from).slice(0, 12).map((l) => l.slice(0, 300)), pageErrors: pageErrors.slice(0, 5).map((l) => l.slice(0, 300)), tMs: Date.now() - t0 };
    rows.push(row);
    log(`${id} ${row.ok ? "PASS" : "FAIL"} ${row.detail}`);
    await page.screenshot({ path: join(outDir, `${id}.png`) }).catch(() => {});
    flush();
    if (id === "A1" && !row.ok) break;
  }
} catch (e) {
  rows.push({ id: "harness", ok: false, detail: String(e).slice(0, 400), evidence: {} });
}
clearTimeout(deadline);
await finish();
