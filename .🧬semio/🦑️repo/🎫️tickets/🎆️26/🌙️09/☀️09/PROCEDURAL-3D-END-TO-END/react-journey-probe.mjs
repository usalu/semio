/** 🧭️ ONE capped end-to-end user-journey probe for the React page `?plugin=generation3d` (nine journeys, one browser).
 *
 * Every row is `{ id, ok, status, ms, detail, screenshot }`; `status` is `pass` | `fail` | `missing` | `skipped`.
 * Selectors and published DOM lanes are reused from the ticket's own probes — see `📓️react-journey-probe-2026-11-25.md`.
 *
 * Usage: cd <ticket> && repo tools exec --light bun -- react-journey-probe.mjs
 * Env: SEMIO_PROBE_URL, SEMIO_PROBE_OUT (under 🗑️generated/journey), SEMIO_PROBE_ONLY (comma list of row numbers), SEMIO_PROBE_HEADED=1.
 * SEMIO_PROBE_MATH_DIR and SEMIO_PROBE_BREP_DIR optionally supply locally built component files at the installed extension URLs.
 * SEMIO_PROBE_MOUNT_MS sets the editor mount deadline. SEMIO_PROBE_BASELINE_MS sets the initial geometry deadline. SEMIO_PROBE_MAX_MS sets the overall cap.
 * SEMIO_PROBE_GEOMETRY_EXAMPLE=rectangle runs the same geometry and history assertions on Rectangle Wire Preview.
 * SEMIO_PROBE_CPU_PORT selects a loopback DevTools listener for the optional worker profile. Use with SEMIO_PROBE_WORKER_TRACE=1.
 * @see 🐍️journey-probe.mjs, 🐍️interaction-matrix-probe.mjs, 🐍️editor-verbs-keyboard-probe.mjs, 🐍️io-surface-probe.mjs
 */
import { chromium } from "playwright";
import { createHash } from "node:crypto";
import { inflateSync } from "node:zlib";
import { appendFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { gradeCameraFrames, gradeMeshes, meshStatsScript, oracleAtHeight, oracleForLabel } from "./🐍️example-oracle.mjs";
import { connectWorkerProfiler } from "./react-worker-profiler.mjs";

const URL_ = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const OUT = join(import.meta.dirname ?? import.meta.dir, "🗑️generated", "journey", process.env.SEMIO_PROBE_OUT ?? "react-journey");
const ONLY = (process.env.SEMIO_PROBE_ONLY ?? "").split(",").map((s) => s.trim()).filter(Boolean);
const HARD_MS = Number(process.env.SEMIO_PROBE_MAX_MS ?? 8 * 60 * 1000);
const MOD = process.platform === "darwin" ? "Meta" : "Control";
const MAIN = "window:procedural-main";
const PREVIEW = "window:procedural-preview";
const GEN_PREVIEW = "window:generation3d-generate-preview";
const VIEW_PREVIEW = "window:procedural-view-preview";
mkdirSync(OUT, { recursive: true });

const T0 = Date.now();
const left = () => HARD_MS - (Date.now() - T0);
const lines = [];
const rows = [];
const artifacts = new Map();
const log = (...a) => console.log("[DEBUG]", ...a);
let browser;
let browserServer;
let page;
let progressTimer;
let workerProfiler;
let profileStopper;
let shuttingDown = false;

const flush = () => {
  writeFileSync(join(OUT, "results.json"), JSON.stringify(rows, null, 2));
  writeFileSync(join(OUT, "console.txt"), lines.join("\n"));
  if (artifacts.size) writeFileSync(join(OUT, "served-artifacts.json"), JSON.stringify([...artifacts.values()], null, 2));
};
const finishBrowser = async (code) => {
  shuttingDown = true;
  clearInterval(progressTimer);
  clearTimeout(profileStopper);
  workerProfiler?.close();
  const forceExit = setTimeout(() => {
    void browserServer?.kill().catch(() => {});
    setTimeout(() => process.exit(code), 1000);
  }, 5000);
  try { await browserServer?.close(); }
  finally { clearTimeout(forceExit); process.exit(code); }
};
const watchdog = setTimeout(async () => {
  rows.push({ id: "watchdog", ok: false, status: "fail", ms: Date.now() - T0, detail: `hard ${HARD_MS} ms cap reached`, screenshot: null });
  flush();
  console.log(JSON.stringify(rows));
  await finishBrowser(2);
}, HARD_MS);

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
/** ⏳️ Poll `read` every `step` ms until `done(value)`; never a fixed sleep. Returns the last value and whether it was reached. */
const until = async (read, done, ms = 20000, step = 150) => {
  const end = Date.now() + Math.min(ms, Math.max(0, left() - 2000));
  let value;
  for (;;) {
    if (page?.isClosed() || browser?.isConnected() === false) throw new Error("browser disconnected during probe");
    try { value = await read(); } catch (error) {
      if (page?.isClosed() || browser?.isConnected() === false) throw error;
      value = undefined;
    }
    if (value !== undefined && done(value)) return { value, reached: true };
    if (Date.now() >= end) return { value, reached: false };
    await sleep(step);
  }
};

//#region PNG pixel reading (no third-party image library: zlib + PNG filters)
const decodePng = (buf) => {
  let pos = 8; let w = 0; let h = 0; let ct = 6; const idat = [];
  while (pos < buf.length) {
    const len = buf.readUInt32BE(pos); const type = buf.toString("latin1", pos + 4, pos + 8);
    if (type === "IHDR") { w = buf.readUInt32BE(pos + 8); h = buf.readUInt32BE(pos + 12); ct = buf[pos + 17]; }
    if (type === "IDAT") idat.push(buf.subarray(pos + 8, pos + 8 + len));
    pos += 12 + len;
  }
  const bpp = ct === 6 ? 4 : ct === 2 ? 3 : ct === 0 ? 1 : 4;
  const raw = inflateSync(Buffer.concat(idat)); const stride = w * bpp; const px = Buffer.alloc(stride * h);
  for (let y = 0; y < h; y += 1) {
    const f = raw[y * (stride + 1)]; const src = y * (stride + 1) + 1; const dst = y * stride;
    for (let x = 0; x < stride; x += 1) {
      const a = x >= bpp ? px[dst + x - bpp] : 0; const b = y > 0 ? px[dst - stride + x] : 0; const c = x >= bpp && y > 0 ? px[dst - stride + x - bpp] : 0;
      const v = raw[src + x];
      px[dst + x] = f === 0 ? v : f === 1 ? (v + a) & 255 : f === 2 ? (v + b) & 255 : f === 3 ? (v + ((a + b) >> 1)) & 255 : (v + (() => { const p = a + b - c; const pa = Math.abs(p - a); const pb = Math.abs(p - b); const pc = Math.abs(p - c); return pa <= pb && pa <= pc ? a : pb <= pc ? b : c; })()) & 255;
    }
  }
  return { w, h, bpp, px };
};
const luma = (img, i) => (img.px[i] * 299 + img.px[i + 1 % img.bpp] * 587 + img.px[i + 2 % img.bpp] * 114) / 1000;
/** 🎨️ Luma variance and a coarse 32x32 signature of a region; the signature compares two frames. */
const stats = (img) => {
  let n = 0; let s = 0; let ss = 0; const grid = new Float32Array(32 * 32); const cnt = new Uint32Array(32 * 32);
  for (let y = 0; y < img.h; y += 2) for (let x = 0; x < img.w; x += 2) {
    const l = luma(img, (y * img.w + x) * img.bpp); n += 1; s += l; ss += l * l;
    const g = Math.min(31, Math.floor((y / img.h) * 32)) * 32 + Math.min(31, Math.floor((x / img.w) * 32)); grid[g] += l; cnt[g] += 1;
  }
  for (let i = 0; i < grid.length; i += 1) grid[i] = cnt[i] ? grid[i] / cnt[i] : 0;
  const mean = s / Math.max(1, n);
  return { variance: ss / Math.max(1, n) - mean * mean, mean, grid };
};
const gridDiff = (a, b) => { let d = 0; for (let i = 0; i < a.length; i += 1) d += Math.abs(a[i] - b[i]); return d / a.length; };
const regionOf = async (selector) => {
  const box = await page.evaluate((sel) => { const el = document.querySelector(sel); const c = el?.querySelector("canvas") ?? el; const r = c?.getBoundingClientRect(); return r && r.width > 4 && r.height > 4 ? { x: Math.max(0, r.x), y: Math.max(0, r.y), width: r.width, height: r.height } : null; }, selector);
  if (!box) return null;
  return stats(decodePng(await page.screenshot({ clip: box, type: "png" })));
};
//#endregion

//#region DOM lanes (published by World3dHost / the flow surface; see 🐍️example-oracle.mjs)
const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map((el) => {
    const surfaceId = el.getAttribute("data-surface-id");
    const st = parse(el.getAttribute("data-status-json"));
    const meshes = parse(el.getAttribute("data-meshes-json"));
    const raw = el.getAttribute("data-meshes-json") ?? "";
    let h = 0x811c9dc5; for (let i = 0; i < raw.length; i += 1) { h ^= raw.charCodeAt(i); h = Math.imul(h, 0x01000193) >>> 0; }
    const probe = surfaceId ? window.__semioFlowGraphProbe?.[surfaceId] : undefined;
    let nodeIds = null; try { const ids = probe?.nodeIds?.(); if (Array.isArray(ids)) nodeIds = ids; } catch {}
    const nodeStatuses = st && typeof st === "object" && !st.phase ? Object.fromEntries(Object.entries(st).map(([k, v]) => [k, v?.status ?? "?"])) : null;
    const canvas = el.querySelector("canvas");
    const revision = el.getAttribute("data-world-geometry-revision");
    const drawnVertices = revision !== null && canvas?.dataset.worldGeometryRevision === revision ? Number(canvas.dataset.worldGeometryVertices) || 0 : 0;
    return {
      surfaceId, meshCount: Array.isArray(meshes) ? meshes.length : 0, drawnVertices, digest: raw.length ? `${h.toString(16)}:${raw.length}` : null,
      phase: st?.phase ?? null, ratio: st?.progress?.ratio ?? null, inFlight: st?.progress?.inFlight ?? null, computing: st?.computing ?? null, cancellable: st?.cancellable ?? null, fault: st?.fault?.code ?? null,
      unitsDone: st?.progress?.unitsDone ?? null, unitsTotal: st?.progress?.unitsTotal ?? null,
      nodeStatuses, nodeIds, selection: parse(el.getAttribute("data-selection-json")), camera: parse(el.getAttribute("data-viewport-camera-json")), sceneCamera: parse(el.getAttribute("data-camera-json")),
      hoverPaint: el.getAttribute("data-hover-paint-id"), w: el.offsetWidth, h: el.offsetHeight,
    };
  });
  const combo = document.querySelector('[role="combobox"]');
  const pane = document.querySelector('[data-slot="world-compute-status"]');
  const cancel = document.querySelector('[data-slot="world-compute-cancel"]');
  const pressed = (id) => document.getElementById(id)?.getAttribute("aria-pressed");
  return {
    hosts, example: combo?.innerText?.replace(/\s+/g, " ").trim() ?? null,
    windows: [...document.querySelectorAll('[data-slot="window"]')].map((e) => e.id),
    faults: [...document.querySelectorAll("[data-fault-code]")].map((e) => e.getAttribute("data-fault-code")),
    pane: pane ? { phase: pane.getAttribute("data-compute-phase"), ratio: pane.getAttribute("data-compute-ratio"), cancellable: pane.hasAttribute("data-compute-cancellable"), text: pane.innerText.replace(/\s+/g, " ").trim() } : null,
    cancel: cancel ? { action: cancel.getAttribute("data-cancel-action"), label: cancel.textContent?.trim() } : null,
    modes: { edit: pressed("playground.navbar.modes.edit"), generate: pressed("playground.navbar.modes.generate") },
    roles: { editor: pressed("playground.navbar.roles.editor"), viewer: pressed("playground.navbar.roles.viewer") },
    history: parse(document.querySelector("[data-history-json]")?.getAttribute("data-history-json")),
    text: document.body.innerText.replace(/\s+/g, " ").slice(0, 60000),
  };
});
const host = (s, id) => s?.hosts.find((h) => h.surfaceId === id) ?? null;
const previewHost = (s) => s.hosts.find((h) => h.surfaceId && h.surfaceId.endsWith("-preview")) ?? null;
const settled = (h) => Boolean(h) && h.phase === "idle" && h.ratio === 1 && h.computing !== true && !h.fault;
const geometryDrawn = (h) => settled(h) && h.meshCount > 0 && h.drawnVertices > 0;
const sliderContent = () => page.locator('[data-slot="panel"] [data-slot="slider-content"]:has([data-ui-node-key="procedural-play-inspector.value.input"])').first();
const sliderReadout = () => sliderContent().locator('[data-slot="slider-value"]');
const sliderValue = () => sliderContent().getByRole("slider").getAttribute("aria-valuenow");
const editSliderValue = async (value) => {
  await sliderReadout().press("Enter");
  const input = sliderContent().locator('input[type="number"]');
  await input.fill(String(value));
  await input.press("Enter");
};
/** 🧾️ The flow document as the surface publishes it: widget ids, values and synapses (read from the probe registry, then the attribute lane). */
const fixture = () => page.evaluate((id) => {
  const raw = window.__semioFlowGraphProbe?.[id]?.hostSnapshotJson?.() ?? document.querySelector(`[data-surface-id="${id}"]`)?.getAttribute("data-host-snapshot-json") ?? document.querySelector(`[data-surface-id="${id}"]`)?.getAttribute("data-fixture-json");
  try { const fx = JSON.parse(raw ?? "null"); if (!fx) return null;
    const widgets = (fx.widgets ?? []).map((w) => { const k = w && typeof w === "object" ? Object.keys(w)[0] : null; const inner = k && typeof w[k] === "object" && w.id === undefined ? w[k] : w; return { id: inner?.id ?? null, kind: w?.kind ?? k, value: inner?.value ?? null }; });
    const wires = (fx.synapses ?? []).map((x) => `${x.from}@${x.fromPort ?? x.from_port}->${x.to}@${x.toPort ?? x.to_port}`).sort();
    return { widgets, ids: widgets.map((w) => w.id).filter(Boolean).sort(), wires, values: Object.fromEntries(widgets.filter((w) => w.value !== null).map((w) => [w.id, w.value])) };
  } catch { return null; }
}, MAIN);
const handlePoint = (port) => page.evaluate(([surface, id]) => { const p = window.__semioFlowGraphProbe?.[surface]?.entity?.("handle", id) ?? null; return p?.visible ? (p.rect ? { x: p.rect.x + p.rect.width / 2, y: p.rect.y + p.rect.height / 2 } : p.point ?? null) : null; }, [MAIN, port]);
const entityPoint = (domain, id) => page.evaluate(([surface, d, e]) => { const p = window.__semioFlowGraphProbe?.[surface]?.entity?.(d, e) ?? null; return p ? (p.rect ? { x: p.rect.x + p.rect.width / 2, y: p.rect.y + p.rect.height / 2 } : p.point ?? null) : null; }, [MAIN, domain, id]);
const rectOf = (selector) => page.evaluate((sel) => { const r = document.querySelector(sel)?.getBoundingClientRect(); return r ? { x: r.x, y: r.y, width: r.width, height: r.height } : null; }, selector);
//#endregion

//#region UI gestures (real controls only)
const comboOptions = async () => {
  await page.locator('[role="combobox"]').first().click({ timeout: 6000 }); await sleep(300);
  const texts = (await page.locator('[role="option"]').allInnerTexts()).map((t) => t.replace(/\s+/g, " ").trim()).filter(Boolean);
  await page.keyboard.press("Escape"); await sleep(150);
  return texts;
};
const pickExample = async (text) => {
  await page.locator('[role="combobox"]').first().click({ timeout: 6000 }); await sleep(300);
  await page.locator('[role="option"]').filter({ hasText: new RegExp(`^\\s*${text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\s*$`) }).first().click({ timeout: 6000 });
};
const openPanel = async (id, marker) => {
  for (let i = 0; i < 4; i += 1) {
    if (await page.locator(`[data-slot="panel"] [id*="${marker}"]`).count()) return true;
    await page.locator(`[id="${id}"]`).first().click({ timeout: 6000 }).catch(() => {});
    await sleep(1200);
  }
  return (await page.locator(`[data-slot="panel"] [id*="${marker}"]`).count()) > 0;
};
const chord = async (keys) => {
  const map = { mod: MOD, alt: "Alt", shift: "Shift", ctrl: "Control" };
  await page.keyboard.press(keys.split("+").map((k, i, a) => map[k] ?? (i === a.length - 1 && k.length === 1 ? k : k)).join("+"));
};
const focusMain = async () => {
  const r = await rectOf(`[data-surface-id="${MAIN}"]`);
  if (r) await page.mouse.click(Math.round(r.x + r.width * 0.5), Math.round(r.y + r.height - 40));
};
const waitConverged = async (label, ms = 25000, expectLabel = null) => until(snap, (s) => {
  const previews = s.hosts.filter((h) => h.surfaceId?.endsWith("-preview"));
  if (!previews.length || !previews.every(settled)) return false;
  if (expectLabel && s.example !== expectLabel) return false;
  return true;
}, ms, 250);
const NOTICE_RE = /(Loading|Preparing) extension [^.…]+…|Extension [^.]+ is (ready|not available)\.|Erweiterung [^.…]+ (wird [a-z]+|ist [a-z]+)…?/g;
//#endregion

//#region row machinery
const shot = async (id) => { const p = join(OUT, `${id}.png`); await page.screenshot({ path: p }).catch(() => {}); return p; };
const row = async (id, fn) => {
  if (ONLY.length && !ONLY.includes(id.split("-")[0])) return;
  const t = Date.now();
  if (left() < 15000) { rows.push({ id, ok: false, status: "skipped", ms: 0, detail: "global probe budget exhausted", screenshot: null }); flush(); return; }
  let result;
  try { result = await fn(); } catch (e) { result = { ok: false, detail: { error: String(e?.stack ?? e).slice(0, 600) } }; }
  const status = result.status ?? (result.ok ? "pass" : "fail");
  const r = { id, ok: status === "pass", status, ms: Date.now() - t, detail: result.detail, screenshot: await shot(id) };
  rows.push(r); flush();
  log(`${id} ${status} ${r.ms}ms ${JSON.stringify(r.detail).slice(0, 500)}`);
};
//#endregion

const profilerPort = Number(process.env.SEMIO_PROBE_CPU_PORT ?? 0);
const profileDuration = Number(process.env.SEMIO_PROBE_CPU_DURATION_MS ?? 25000);
const captureTick = process.env.SEMIO_PROBE_CAPTURE_TICK === "1";
let tickCaptureOrdinal = 0;
const profilerArguments = profilerPort ? [`--remote-debugging-port=${profilerPort}`, "--remote-debugging-address=127.0.0.1"] : [];
browserServer = await chromium.launchServer({ host: "127.0.0.1", port: 0, headless: process.env.SEMIO_PROBE_HEADED !== "1", args: ["--js-flags=--max-old-space-size=1024", "--renderer-process-limit=1", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist", ...(process.platform === "darwin" ? ["--use-angle=metal"] : []), ...profilerArguments] });
browserServer.on("close", (code, signal) => {
  if (shuttingDown) return;
  lines.push(`${Date.now() - T0} pagecrash Chromium exited (${code ?? "no code"}, ${signal ?? "no signal"})`);
  flush();
});
browser = await chromium.connect(browserServer.wsEndpoint());
browser.on("disconnected", () => {
  if (shuttingDown) return;
  clearInterval(progressTimer);
  lines.push(`${Date.now() - T0} pagecrash Chromium browser disconnected`);
  flush();
});
const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, acceptDownloads: true });
const extensionModules = [
  ["🧮️flow-extension-math", "math", process.env.SEMIO_PROBE_MATH_DIR],
  ["🧊️flow-extension-brep", "brep", process.env.SEMIO_PROBE_BREP_DIR],
].filter(([, , directory]) => directory);
const componentFiles = new Map(extensionModules.flatMap(([extension, component, directory]) =>
  [".js", ".core.wasm", ".core2.wasm", ".core3.wasm"].map((suffix) => {
    const name = `semio_s_plugin_flow_${component}_component${suffix}`;
    return [`/🧩️extension-modules/${extension}/${name}`, join(directory, name)];
  })));
const probeOrigin = new URL(URL_).origin;
if (profilerPort) workerProfiler = await connectWorkerProfiler(profilerPort, OUT, probeOrigin, log);
const componentFile = (url) => url.origin === probeOrigin ? componentFiles.get(decodeURIComponent(url.pathname)) : undefined;
context.on("requestfinished", (request) => {
  const url = new URL(request.url());
  const path = decodeURIComponent(url.pathname);
  if (url.origin !== probeOrigin || !/^\/🧩️extension-modules\/(?:🧮️flow-extension-math|🧊️flow-extension-brep)\/[^/]+\.core\.wasm$/.test(path)) return;
  void request.sizes().then(({ responseBodySize }) => log("downloaded extension core", responseBodySize, path)).catch(() => {});
});
if (componentFiles.size) await context.route((url) => componentFile(url) !== undefined, async (route) => {
  const url = route.request().url();
  const file = componentFile(new URL(url));
  const body = readFileSync(file);
  await route.fulfill({ body, contentType: file.endsWith(".wasm") ? "application/wasm" : "text/javascript" });
  artifacts.set(url, { url, bytes: body.length, sha256: createHash("sha256").update(body).digest("hex") });
});
const observe = (suffix, rewrite) => context.route((url) => url.origin === probeOrigin && decodeURIComponent(url.pathname).endsWith(suffix), async (route) => {
  const response = await route.fetch();
  const before = await response.text();
  const body = rewrite(before);
  log("browser observation", { suffix, matched: body !== before });
  await route.fulfill({ response, body });
});
if (process.env.SEMIO_PROBE_PICKER_TRACE === "1") {
  await observe("/🧭️input-funnel/🟦️.ts", (source) => source.replace(/const outcome = directOutcome \?\? await dispatchProgramAction\([^\n]*\);/, `
    if (action.action === "setActiveExample") console.info("[DEBUG] PICK_INPUT " + JSON.stringify({ inputSeq: entry.provenance.inputSeq, causedBy: entry.provenance.causedBy, target: targetSession.instanceId, args: action.args }));
    $&
    if (action.action === "setActiveExample") console.info("[DEBUG] PICK_OUTCOME " + JSON.stringify(outcome));`));
  await observe("/📥️invocation-frames/🟦️.ts", (source) => source.replace("return response;", `
    if (actionId === "setActiveExample") console.info("[DEBUG] PICK_INVOCATION " + JSON.stringify({ frames: frames.map((frame) => frame.tag ?? frame.kind ?? Object.keys(frame)[0]), effects: (response.requestedEffects ?? []).map((effect) => typeof effect === "string" ? effect : Object.keys(effect)[0]), uiScope: response.uiScope, historyActions: response.historyPatch?.upserts?.map((entry) => entry.actionId) }));
    return response;`));
  await observe("/📥️document-load-effects/🟦️.ts", (source) => source
    .replace("const payload = effect.loadDocument;", `$& console.info("[DEBUG] PICK_LOAD " + JSON.stringify({ inputSeq: effectOwner.inputSeq, instance: baseSession.instanceId, keys: Object.keys(payload) }));`)
    .replace(/const loaded = await loadDocumentPair\([^\n]*\);/, `$& console.info("[DEBUG] PICK_LOADED " + JSON.stringify({ inputSeq: effectOwner.inputSeq, loaded }));`));
}
if (process.env.SEMIO_PROBE_HISTORY_TRACE === "1") {
  await observe("/🧭️input-funnel/🟦️.ts", (source) => source
    .replace("const command = action.commandAddress ?", `
      if (${process.env.SEMIO_PROBE_WORKER_TRACE === "1"} && action.action === "interactionSelect") console.info("[DEBUG] WORKER_SELECTION_INGRESS " + JSON.stringify({ action: action.action, windowId: dispatchWindowId, undeclared: undeclared?.message, args: action.args }, (_, value) => typeof value === "bigint" ? String(value) : value));
      $&`)
    .replace("const undoOutcome = action.commandAddress ?", `
      if (["patchFlowWidgets", "undo", "redo"].includes(action.action)) console.info("[DEBUG] HISTORY_INPUT " + JSON.stringify({ action: action.action, inputSeq: entry.provenance.inputSeq, target: targetSession.instanceId, canUndo: historyCanUndo, direct: directBrowserActor !== null, args: action.args }));
      $&`)
    .replace("if (undoOutcome !== undefined) return undoOutcome;", `if (undoOutcome !== undefined) {
      if (action.action === "undo") console.info("[DEBUG] HISTORY_ROUTE " + JSON.stringify({ kind: undoOutcome.kind, reason: undoOutcome.reason, message: undoOutcome.message }));
      return undoOutcome;
    }`)
    .replace(/const outcome = directOutcome \?\? await dispatchProgramAction\([^\n]*\);/, `$&
      if (["patchFlowWidgets", "undo", "redo"].includes(action.action) || (${process.env.SEMIO_PROBE_WORKER_TRACE === "1"} && action.action === "interactionSelect")) console.info("[DEBUG] HISTORY_OUTCOME " + JSON.stringify({ action: action.action, kind: outcome.kind, reason: outcome.reason, message: outcome.message }));`));
  await observe("/📥️invocation-frames/🟦️.ts", (source) => source.replace("return response;", `
    if (["patchFlowWidgets", "undo", "redo"].includes(actionId)) console.info("[DEBUG] HISTORY_INVOCATION " + JSON.stringify({ actionId, frames: frames.map((frame) => frame.tag ?? frame.kind ?? Object.keys(frame)[0]), effects: (response.requestedEffects ?? []).map((effect) => typeof effect === "string" ? effect : effect.dispatchAction?.action ?? Object.keys(effect)[0]), history: response.historyPatch !== undefined }));
    return response;`));
  await observe("/🚀️effect-dispatch/🟦️.ts", (source) => source
    .replace("if (!isCurrent()) return;", `
      if (${process.env.SEMIO_PROBE_WORKER_TRACE === "1"} && action === "flowEvalTick") console.info("[DEBUG] WORKER_TICK_INGRESS " + JSON.stringify({ action, args, current: isCurrent() }, (_, value) => typeof value === "bigint" ? String(value) : value));
      $&`)
    .replace("const isAppCommand = (baseSession.app.commands ?? []).some((command) => command.id === action);", `
      $&
      if (${captureTick} && action === "flowEvalTick" && args?.windowId === "procedural-preview") console.info("[DEBUG] WORKER_TICK_VIEW " + JSON.stringify({ args, viewState, sessionWindowId: baseSession.viewState.windowId, isAppCommand }, (_, value) => typeof value === "bigint" ? String(value) : value));
      if (action !== "flowEvalTick" || ${process.env.SEMIO_PROBE_WORKER_TRACE === "1"}) console.info("[DEBUG] HISTORY_FOLLOW " + JSON.stringify({ action, isAppCommand, causedBy: provenance?.causedBy, args, window: viewState.windowId, roster: action === "flowEvalTick" ? viewState.windowInstances : undefined }, (_, value) => typeof value === "bigint" ? String(value) : value));`)
    .replace("if (isCurrent()) await applyResponse(response, targetSession);", `
      if (action !== "flowEvalTick" || ${process.env.SEMIO_PROBE_WORKER_TRACE === "1"}) console.info("[DEBUG] HISTORY_FOLLOW_RESULT " + JSON.stringify({ action, window: args?.windowId, history: response.historyPatch !== undefined, uiScope: response.uiScope, current: isCurrent(), effects: (response.requestedEffects ?? []).map((effect) => typeof effect === "string" ? effect : effect.dispatchAction?.action ?? Object.keys(effect)[0]) }));
      $&`));
}
if (process.env.SEMIO_PROBE_EXTENSION_TRACE === "1") {
  await observe("/🔌️extension-runtime/🟦️.ts", (source) => source
    .replace(/raw = await invoke\.call\([^\n]*\);/, `
      console.info("[DEBUG] EXT_INVOKE " + JSON.stringify({ extensionId, capability, req: String(req) }));
      $&
      console.info("[DEBUG] EXT_ANSWER " + JSON.stringify({ extensionId, capability, req: String(req), bytes: raw.length }));`)
    .replace("outcome = { fault: encodePackValue(fault) };", `
      console.info("[DEBUG] EXT_FAULT " + JSON.stringify({ extensionId, capability, req: String(req), code: fault.code }));
      $&`)
    .replace("const response = await completion.complete(outcome);", `
      $&
      console.info("[DEBUG] EXT_DELIVERED " + JSON.stringify({ extensionId, capability, req: String(req), effects: response.requestedEffects?.length ?? 0 }));`));
}
if (process.env.SEMIO_PROBE_WORKER_TRACE === "1") {
  await observe("/🚚️handle-turn-drive/🟦️.ts", (source) => source
    .replace("const crossingStartedMs = performance.now();", `
      if (${captureTick} && inspected.actionId === "flowEvalTick") console.info("[DEBUG] WORKER_TICK_PACK " + JSON.stringify({ actorId, instanceId, inspected, commandIndex, events, pages }, (_, value) => typeof value === "bigint" ? String(value) : ArrayBuffer.isView(value) ? Array.from(value) : value));
      $&`)
    .replace("const previousIngress = terminal;", `
    if (["setContributions", "flowEvalTick", "noteShellCommand", "interactionSelect"].includes(inspected.actionId) && (continuations < 8 || continuations % 16 === 0)) {
      console.info("[DEBUG] WORKER_CONTINUATION " + JSON.stringify({ actorId, action: inspected.actionId, continuation: continuations + 1, status: wireTurnStatusTag(continued.status), effects: continued.effects.length, patches: continued.uiPatches.length }));
    }
    $&`));
}
if (captureTick) {
  await observe("/🧵️turn-dispatch/🟦️.ts", (source) => source.replace("raw = await owner.open(work.input, budget);", `
    if (owner.activation.actorId.startsWith("procedural#")) console.info("[DEBUG] WORKER_TICK_OPEN " + JSON.stringify({ actorId: owner.activation.actorId, input: work.input, budget }, (_, value) => typeof value === "bigint" ? String(value) : value));
    $&`));
  await observe("/🖼️surface-refresh/🟦️.ts", (source) => source.replace("const viewState = windowViewContext(request.viewState, binding.windowKey);", `
    $&
    if (viewState && String(binding.windowKey).includes("procedural-preview") && !globalThis.__g3dPreviewViewCaptured) {
      globalThis.__g3dPreviewViewCaptured = true;
      console.info("[DEBUG] WORKER_TICK_SURFACE " + JSON.stringify({ instanceId, binding, viewState, pack: Array.from(encodePackValue(viewContextWireValue(viewState))) }, (_, value) => typeof value === "bigint" ? String(value) : value));
    }`));
}
page = await context.newPage();
page.on("console", (m) => {
  const message = m.text();
  for (const kind of ["OPEN", "SURFACE", "VIEW", "PACK"]) {
    const prefix = `[DEBUG] WORKER_TICK_${kind} `;
    if (!captureTick || !message.startsWith(prefix)) continue;
    const value = JSON.parse(message.slice(prefix.length));
    if (kind === "VIEW") writeFileSync(join(OUT, `edit-tick-view-${tickCaptureOrdinal + 1}.json`), JSON.stringify(value, null, 2) + "\n");
    else if (kind === "PACK") {
      tickCaptureOrdinal += 1;
      writeFileSync(join(OUT, `tick-ingress-${tickCaptureOrdinal}.json`), JSON.stringify(value, null, 2) + "\n");
      writeFileSync(join(OUT, `tick-command-${tickCaptureOrdinal}.pack`), Uint8Array.from(value.events[value.commandIndex]));
    } else writeFileSync(join(OUT, `tick-${kind.toLowerCase()}.json`), JSON.stringify(value, null, 2) + "\n");
    log("exact tick capture", { kind, tickCaptureOrdinal });
    return;
  }
  lines.push(`${Date.now() - T0} ${m.type()} ${message.slice(0, 1200)}`);
  if (/^\[DEBUG\] (?:PICK|HISTORY|EXT|WORKER)_/.test(message)) log(Date.now() - T0, message);
  if (workerProfiler && message.startsWith("[DEBUG] WORKER_CONTINUATION ") && message.includes('"action":"setContributions"')) workerProfiler.start();
  if (workerProfiler && !profileStopper && message.startsWith("[DEBUG] WORKER_TICK_INGRESS ") && message.includes('"windowId":"procedural-preview"')) {
    workerProfiler.start();
    profileStopper = setTimeout(() => void workerProfiler.stop().catch((error) => log("worker profiler failed", String(error))), profileDuration);
  }
});
page.on("pageerror", (e) => lines.push(`${Date.now() - T0} pageerror ${String(e).slice(0, 1200)}`));
page.on("crash", () => lines.push(`${Date.now() - T0} pagecrash Chromium renderer stopped`));
const progressFile = join(OUT, "progress.ndjson");
writeFileSync(progressFile, "");
let samplingProgress = false;
progressTimer = setInterval(async () => {
  if (samplingProgress) return;
  samplingProgress = true;
  try {
    const state = await snap();
    const preview = host(state, PREVIEW);
    const progress = { ms: Date.now() - T0, example: state.example, preview, nodes: host(state, MAIN)?.nodeStatuses, pageErrors: pageErrors().slice(-3) };
    appendFileSync(progressFile, `${JSON.stringify(progress)}\n`);
    writeFileSync(join(OUT, "console.txt"), lines.join("\n"));
    log("preview progress", { ms: progress.ms, phase: preview?.phase, units: [preview?.unitsDone, preview?.unitsTotal], inFlight: preview?.inFlight, meshes: preview?.meshCount, vertices: preview?.drawnVertices, fault: preview?.fault });
  } catch (error) {
    appendFileSync(progressFile, `${JSON.stringify({ ms: Date.now() - T0, error: String(error) })}\n`);
  } finally {
    samplingProgress = false;
  }
}, 15000);
await page.addInitScript(() => {
  window.__journeyNotices = [];
  window.__journeyInput = [];
  const re = /(Loading|Preparing) extension [^.…]+…|Extension [^.]+ is (ready|not available)\.|Erweiterung [^.…]+ (wird [a-z]+|ist [a-z]+)…?/g;
  const scan = () => { const m = document.body?.innerText?.match(re); if (m) for (const t of m) if (!window.__journeyNotices.includes(t)) window.__journeyNotices.push(t); };
  new MutationObserver(scan).observe(document, { childList: true, subtree: true, characterData: true });
  addEventListener("pointermove", (e) => { window.__journeyInput.push(performance.now() - e.timeStamp); }, true);
});
const pageErrors = () => lines.filter((l) => /\bpage(?:error|crash)\b|OwnershipLimit|actor fault|no actor for instance|original-turn\.native-return-refused|panicked/i.test(l));

try {
  const response = await page.goto(URL_, { waitUntil: "domcontentloaded" });
  if (!response?.ok()) throw new Error(`Editor returned HTTP ${response?.status() ?? "no response"}`);
} catch (error) {
  const detail = String(error);
  lines.push(`goto ${detail}`);
  rows.push({ id: "editor-navigation", ok: false, status: "fail", ms: Date.now() - T0, detail, screenshot: null });
  flush();
  await finishBrowser(1);
}

//#region 1 boot
await row("1-boot", async () => {
  const boot = await until(snap, (s) => host(s, MAIN)?.nodeIds?.length > 0 && settled(host(s, PREVIEW)), 90000, 500);
  const s = boot.value ?? (await snap());
  const flowNodes = host(s, MAIN)?.nodeIds ?? [];
  const preview = await regionOf(`[data-surface-id="${PREVIEW}"]`);
  const flow = await regionOf(`[data-surface-id="${MAIN}"]`);
  const windows = await page.evaluate(() => [...document.querySelectorAll('[data-slot="window"]')].map((e) => { const r = e.getBoundingClientRect(); return { id: e.id, w: Math.round(r.width), h: Math.round(r.height), canvas: e.querySelectorAll("canvas").length, text: (e.innerText ?? "").trim().length }; }));
  const empty = windows.filter((w) => w.w < 8 || w.h < 8 || (w.canvas === 0 && w.text === 0));
  const detail = { reached: boot.reached, flowNodes: flowNodes.length, previewVariance: preview?.variance ?? null, flowVariance: flow?.variance ?? null, windows, emptyWindows: empty.map((w) => w.id), faults: s.faults, pageErrors: pageErrors().slice(0, 4), meshes: host(s, PREVIEW)?.meshCount };
  const edit = Boolean(boot.reached && flowNodes.length > 0 && (preview?.variance ?? 0) > 1 && (flow?.variance ?? 0) > 1 && empty.length === 0 && s.faults.length === 0 && detail.pageErrors.length === 0);
  const modes = {};
  await chord("mod+alt+ArrowRight");
  const gen = await until(snap, (x) => x.hosts.some((h) => h.surfaceId === GEN_PREVIEW), 20000);
  modes.generate = await page.evaluate(() => [...document.querySelectorAll('[data-slot="window"]')].map((e) => ({ id: e.id, canvas: e.querySelectorAll("canvas").length, text: (e.innerText ?? "").trim().length })));
  modes.generateMounted = gen.reached;
  modes.generatePreviewPainted = (await regionOf(`[data-surface-id="${GEN_PREVIEW}"]`))?.variance ?? null;
  await chord("mod+alt+ArrowLeft"); await until(snap, (x) => x.hosts.some((h) => h.surfaceId === PREVIEW), 20000);
  await chord("mod+alt+v");
  const view = await until(snap, (x) => x.hosts.some((h) => h.surfaceId === VIEW_PREVIEW), 20000);
  modes.viewerMounted = view.reached;
  modes.viewerPreviewPainted = (await regionOf(`[data-surface-id="${VIEW_PREVIEW}"]`))?.variance ?? null;
  await page.locator('[id="playground.navbar.roles.editor"]').first().click({ timeout: 6000 }).catch(() => {});
  const back = await until(snap, (x) => x.hosts.some((h) => h.surfaceId === MAIN), 20000);
  if (!back.reached) { await page.goto(URL_, { waitUntil: "domcontentloaded" }); await until(snap, (x) => host(x, MAIN)?.nodeIds?.length > 0, 60000, 500); }
  const generateOk = modes.generateMounted && modes.generate.every((w) => w.canvas > 0 || w.text > 0);
  const viewerOk = modes.viewerMounted && (modes.viewerPreviewPainted ?? 0) > 1;
  return { ok: edit && generateOk && viewerOk, detail: { ...detail, modes, edit, generateOk, viewerOk } };
});
//#endregion

//#region 2 eight examples
const examplesSeen = [];
await row("2-examples", async () => {
  const options = (await comboOptions()).filter((t) => !/^no example$/i.test(t));
  const results = [];
  let prevDigest = host(await snap(), PREVIEW)?.digest ?? null;
  for (const text of options) {
    if (left() < 60000) { results.push({ example: text, ok: false, reason: "budget" }); continue; }
    await pickExample(text);
    const c = await waitConverged(text, 22000, text);
    const s = c.value; const p = host(s, PREVIEW); const main = host(s, MAIN);
    const oracle = oracleForLabel(text);
    const stats = oracle ? await page.evaluate(meshStatsScript, oracle.previewMeshId) : null;
    const grade = gradeMeshes(oracle, stats?.find((mesh) => mesh.surfaceId === PREVIEW) ?? null);
    const bad = Object.entries(main?.nodeStatuses ?? {}).filter(([, st]) => st !== "ok");
    const changed = Boolean(p?.digest) && p.digest !== prevDigest;
    results.push({ example: text, ok: c.reached && changed && bad.length === 0 && grade.ok, oracle: Boolean(oracle), converged: c.reached, changed, badNodes: bad, meshes: p?.meshCount, mesh: grade, status: s?.pane?.text ?? p?.phase });
    if (p?.digest) prevDigest = p.digest;
    examplesSeen.push(text);
  }
  return { ok: results.length >= 1 && results.length === options.length && results.every((r) => r.ok), detail: { listed: options, count: results.length, withOracle: results.filter((r) => r.oracle).length, results } };
});
//#endregion

//#region 3 hover / selection / camera
await row("3-hover-select-camera", async () => {
  await pickExample("Box Shell Preview").catch(() => {});
  await waitConverged("box", 30000, "Box Shell Preview");
  const detail = {};
  const s0 = await snap();
  const main0 = host(s0, MAIN);
  const wires = (await fixture())?.wires ?? [];
  const src = wires[0]?.split("->")[0] ?? "shell@solid";
  const handle = src;
  const nodeId = handle.split("@")[0];
  const pt = (await until(() => handlePoint(handle), (v) => Boolean(v), 15000)).value;
  detail.handle = handle; detail.handlePoint = pt;
  if (pt) {
    await page.mouse.move(pt.x, pt.y);
    const g2p = await until(snap, (s) => { const p = host(s, PREVIEW)?.hoverPaint; return Boolean(p) && (p === handle || p.startsWith(handle + "#") || p.includes(nodeId)); }, 8000, 100);
    detail.graphToPreview = { ok: g2p.reached, hoverPaint: host(g2p.value, PREVIEW)?.hoverPaint ?? null };
  } else detail.graphToPreview = { ok: false, reason: "port handle not published by __semioFlowGraphProbe" };
  await page.mouse.move(2, 2);
  await until(snap, (s) => !host(s, PREVIEW)?.hoverPaint, 6000, 100);
  const pbox = await rectOf(`[data-surface-id="${PREVIEW}"] canvas`) ?? await rectOf(`[data-surface-id="${PREVIEW}"]`);
  let p2g = { ok: false, reason: "no preview canvas" };
  let hitPoint = null;
  if (pbox) {
    outer: for (const fy of [0.5, 0.4, 0.6, 0.3, 0.7, 0.2, 0.8]) for (const fx of [0.5, 0.4, 0.6, 0.3, 0.7, 0.2, 0.8]) {
      await page.mouse.move(pbox.x + fx * pbox.width, pbox.y + fy * pbox.height);
      const hit = await until(snap, (s) => { const h = host(s, MAIN)?.selection?.hoverTarget; return Boolean(h?.nodeId); }, 350, 70);
      if (hit.reached) { hitPoint = [pbox.x + fx * pbox.width, pbox.y + fy * pbox.height]; p2g = { ok: true, hoverTarget: host(hit.value, MAIN).selection.hoverTarget }; break outer; }
    }
    if (!p2g.ok) p2g = { ok: false, reason: "no preview point marked a node in the flow surface" };
  }
  detail.previewToGraph = p2g;
  if (hitPoint) {
    await page.mouse.click(hitPoint[0], hitPoint[1]);
    const sel = await until(snap, (s) => (host(s, PREVIEW)?.selection?.selectedIds ?? []).length > 0, 6000, 150);
    const selIds = host(sel.value, PREVIEW)?.selection?.selectedIds ?? [];
    const mainSel = await until(snap, (s) => (host(s, MAIN)?.selection?.selectedIds ?? []).length > 0, 6000, 150);
    detail.selectPreviewToGraph = { ok: sel.reached && mainSel.reached, preview: selIds, flow: host(mainSel.value, MAIN)?.selection?.selectedIds ?? [] };
  } else detail.selectPreviewToGraph = { ok: false, reason: "no geometry hit" };
  await page.keyboard.press("Escape"); await sleep(300);
  const nodePt = await entityPoint("node", nodeId) ?? pt;
  if (nodePt) {
    await page.mouse.click(nodePt.x, nodePt.y);
    const sel = await until(snap, (s) => JSON.stringify(host(s, PREVIEW)?.selection?.selectedIds ?? []).includes(nodeId) || JSON.stringify(host(s, MAIN)?.selection?.selectedIds ?? []).includes(nodeId), 6000, 150);
    detail.selectGraphToPreview = { ok: sel.reached, preview: host(sel.value, PREVIEW)?.selection?.selectedIds ?? [], flow: host(sel.value, MAIN)?.selection?.selectedIds ?? [] };
  } else detail.selectGraphToPreview = { ok: false, reason: "node point unavailable" };
  // 🖱️ Alt + right drag orbits, wheel zooms (resolveWorldOrbitRightMouseAction); the flow camera must not move.
  const before = await snap();
  const camB = host(before, PREVIEW)?.camera; const flowB = host(before, MAIN)?.camera ?? host(before, MAIN)?.sceneCamera ?? null;
  const flowNodeB = await entityPoint("node", nodeId);
  const pixB = await regionOf(`[data-surface-id="${PREVIEW}"]`);
  const ox = pbox.x + pbox.width * 0.6; const oy = pbox.y + pbox.height * 0.45;
  await page.keyboard.down("Alt"); await page.mouse.move(ox, oy); await page.mouse.down({ button: "right" }); await page.mouse.move(ox + 110, oy + 40, { steps: 14 }); await page.mouse.up({ button: "right" }); await page.keyboard.up("Alt");
  const orbit = await until(snap, (s) => JSON.stringify(host(s, PREVIEW)?.camera?.position) !== JSON.stringify(camB?.position), 6000, 150);
  const pixO = await regionOf(`[data-surface-id="${PREVIEW}"]`);
  const camO = host(orbit.value, PREVIEW)?.camera;
  await page.mouse.move(pbox.x + pbox.width / 2, pbox.y + pbox.height / 2); await page.mouse.wheel(0, -500);
  const zoom = await until(snap, (s) => JSON.stringify(host(s, PREVIEW)?.camera) !== JSON.stringify(camO), 6000, 150);
  const pixZ = await regionOf(`[data-surface-id="${PREVIEW}"]`);
  const flowA = host(zoom.value, MAIN)?.camera ?? host(zoom.value, MAIN)?.sceneCamera ?? null;
  const flowNodeA = await entityPoint("node", nodeId);
  const dist = (c) => (c ? Math.hypot(...c.position.map((v, i) => v - c.target[i])) : null);
  detail.camera = { orbitMoved: orbit.reached, orbitPixelDiff: pixB && pixO ? gridDiff(pixB.grid, pixO.grid) : null, zoomMoved: zoom.reached, distanceBefore: dist(camO), distanceAfter: dist(host(zoom.value, PREVIEW)?.camera), zoomPixelDiff: pixO && pixZ ? gridDiff(pixO.grid, pixZ.grid) : null, flowCameraUnchanged: JSON.stringify(flowB) === JSON.stringify(flowA) && JSON.stringify(flowNodeB) === JSON.stringify(flowNodeA), flowB, flowA, flowNodeB, flowNodeA };
  const ok = detail.graphToPreview.ok && detail.previewToGraph.ok && detail.selectPreviewToGraph.ok && detail.selectGraphToPreview.ok && detail.camera.orbitMoved && (detail.camera.orbitPixelDiff ?? 0) > 0.05 && detail.camera.zoomMoved && (detail.camera.zoomPixelDiff ?? 0) > 0.05 && detail.camera.flowCameraUnchanged;
  return { ok, detail };
});
//#endregion

//#region 4 edit a value, undo, redo
await row("4-edit-undo-redo", async () => {
  const widget = "height";
  const rectangle = process.env.SEMIO_PROBE_GEOMETRY_EXAMPLE === "rectangle";
  const example = rectangle ? "Rectangle Wire Preview" : "Hexagonal Mushroom Column";
  const exampleNode = rectangle ? "rect" : "column-preview";
  const heightLabel = rectangle ? /^Height\b/ : /^Column Height\b/;
  const mounted = await until(snap, (s) => Boolean(s.example && host(s, MAIN)?.nodeIds?.length), Number(process.env.SEMIO_PROBE_MOUNT_MS ?? 120000), 500);
  if (!mounted.reached) return { ok: false, detail: { reason: "Flow editor did not mount" } };
  if (mounted.value.example !== example) await pickExample(example);
  const loaded = await until(fixture, (f) => f?.ids.includes(exampleNode) && typeof f.values[widget] === "number", Number(process.env.SEMIO_PROBE_EXAMPLE_READY_MS ?? 90000));
  if (!loaded.reached) return { ok: false, detail: { reason: "height input never reached the graph", example, actualWidgetIds: loaded.value?.ids, actualHeight: loaded.value?.values?.[widget] } };
  const baselineDeadline = Date.now() + Number(process.env.SEMIO_PROBE_BASELINE_MS ?? 210000);
  const waitForBaseline = () => until(snap, (s) => geometryDrawn(host(s, PREVIEW)), Math.max(0, baselineDeadline - Date.now()), 500);
  const earlyBaseline = process.env.SEMIO_PROBE_BASELINE_FIRST === "1" ? await waitForBaseline() : null;
  if (earlyBaseline && !earlyBaseline.reached) return { ok: false, detail: { reason: "preview never rendered baseline geometry", example, baseline: host(earlyBaseline.value ?? await snap(), PREVIEW) } };
  const artifact = await openPanel("framework.panel.artifact", "procedural-play-graph");
  await page.locator('[data-slot="panel"] [role="treeitem"]').filter({ hasText: heightLabel }).first().click({ timeout: 8000 });
  const inspection = await openPanel("framework.panel.inspection", "procedural-play-inspector");
  const present = (await until(async () => (await sliderReadout().count()) > 0, Boolean, Number(process.env.SEMIO_PROBE_INSPECTOR_MS ?? 120000))).reached;
  if (!present) return { ok: false, detail: { artifact, inspection, selection: host(await snap(), MAIN)?.selection, reason: "inspector slider never mounted" } };
  const preview = `[data-surface-id="${PREVIEW}"]`;
  const pixels = () => regionOf(preview);
  const base = earlyBaseline ?? await waitForBaseline();
  const baseline = host(base.value ?? await snap(), PREVIEW);
  log("edit baseline", { drawn: base.reached, meshes: baseline?.meshCount, vertices: baseline?.drawnVertices, phase: baseline?.phase, units: [baseline?.unitsDone, baseline?.unitsTotal], fault: baseline?.fault });
  const oracle = oracleForLabel(example);
  const gradeCurrentGeometry = async (expected) => {
    const meshStats = expected ? await page.evaluate(meshStatsScript, expected.previewMeshId) : [];
    return gradeMeshes(expected, meshStats.find((mesh) => mesh.surfaceId === PREVIEW) ?? null);
  };
  const geometryGrade = base.reached ? await gradeCurrentGeometry(oracle) : null;
  if (!(await sliderReadout().isVisible())) await openPanel("framework.panel.inspection", "procedural-play-inspector");
  const baseDigest = baseline?.digest; const baseValue = await sliderValue(); const baseFx = (await fixture())?.values;
  const basePixels = base.reached ? await pixels() : null;
  const edited = String(Number(baseValue) + (rectangle ? 1 : 2));
  await editSliderValue(edited);
  const editValue = await until(fixture, (f) => String(f?.values[widget]) === edited, Number(process.env.SEMIO_PROBE_EDIT_WAIT_MS ?? 150000), 250);
  log("document value after edit", { widget, expected: edited, actual: editValue.value?.values[widget], committed: editValue.reached });
  const focusHistory = () => page.locator(`[data-surface-id="${MAIN}"] [tabindex="0"]`).first().focus({ timeout: 800 }).catch(focusMain);
  if (!base.reached) {
    const detail = { artifact, inspection, baseValue, edited, valueAfterEdit: editValue.value?.values[widget], editCommitted: editValue.reached, reason: "preview never rendered baseline geometry", baseline };
    if (!editValue.reached) return { ok: false, detail };
    await focusHistory();
    await chord("mod+z");
    const undoDocument = await until(fixture, (f) => String(f?.values[widget]) === baseValue, Number(process.env.SEMIO_PROBE_UNDO_WAIT_MS ?? 45000), 250);
    log("document value after undo", { widget, expected: baseValue, actual: undoDocument.value?.values[widget], committed: undoDocument.reached });
    if (!undoDocument.reached) return { ok: false, detail: { ...detail, valueAfterUndo: undoDocument.value?.values[widget], documentUndo: false, documentRedo: null } };
    await chord("mod+shift+z");
    const redoDocument = await until(fixture, (f) => String(f?.values[widget]) === edited, 45000, 250);
    log("document value after redo", { widget, expected: edited, actual: redoDocument.value?.values[widget], committed: redoDocument.reached });
    return { ok: false, detail: { ...detail, valueAfterUndo: undoDocument.value?.values[widget], valueAfterRedo: redoDocument.value?.values[widget], documentUndo: undoDocument.reached, documentRedo: redoDocument.reached } };
  }
  const geometryWaitMs = Number(process.env.SEMIO_PROBE_GEOMETRY_WAIT_MS ?? 70000);
  const editedOracle = oracleAtHeight(oracle, Number(edited));
  const after = await until(snap, (s) => host(s, PREVIEW)?.digest !== baseDigest && geometryDrawn(host(s, PREVIEW)), geometryWaitMs, 250);
  const editGrade = after.reached ? await gradeCurrentGeometry(editedOracle) : null;
  const editPixels = after.reached ? await pixels() : null;
  const editedDigest = host(after.value, PREVIEW)?.digest; const editedFx = (await fixture())?.values;
  await focusHistory();
  await chord("mod+z");
  const undo = await until(snap, (s) => host(s, PREVIEW)?.digest === baseDigest && geometryDrawn(host(s, PREVIEW)), geometryWaitMs, 250);
  const undoGrade = undo.reached ? await gradeCurrentGeometry(oracle) : null;
  const undoPixels = undo.reached ? await pixels() : null;
  const undoFx = (await fixture())?.values;
  let redo = { reached: false };
  if (after.reached && undo.reached) {
    await chord("mod+shift+z");
    redo = await until(snap, (s) => host(s, PREVIEW)?.digest === editedDigest && geometryDrawn(host(s, PREVIEW)), geometryWaitMs, 250);
  }
  const redoGrade = redo.reached ? await gradeCurrentGeometry(editedOracle) : null;
  const redoPixels = redo.reached ? await pixels() : null;
  const redoFx = (await fixture())?.values;
  const frames = [basePixels, editPixels, undoPixels, redoPixels];
  const pixelDifferences = frames.slice(1).map((pixels, index) => pixels && frames[index] ? gridDiff(frames[index].grid, pixels.grid) : null);
  const detail = { example, widget, baseValue, edited, geometryGrade, editGrade, undoGrade, redoGrade, recomputed: after.reached, undid: undo.reached, redid: redo.reached, pixelDifferences, valueAfterEdit: editedFx?.[widget], valueAfterUndo: undoFx?.[widget], valueAfterRedo: redoFx?.[widget], canUndo: undo.value?.history?.canUndo, canRedo: undo.value?.history?.canRedo };
  detail.valuesOk = String(detail.valueAfterUndo) === String(baseFx?.[widget]) && String(detail.valueAfterRedo) === String(editedFx?.[widget]) && String(editedFx?.[widget]) !== String(baseFx?.[widget]);
  return { ok: Boolean([geometryGrade, editGrade, undoGrade, redoGrade].every((grade) => grade?.ok) && pixelDifferences.every((value) => (value ?? 0) > 0.05) && detail.valuesOk), detail };
});
//#endregion

//#region 5 add node, connect, disconnect, delete, undo all
await row("5-graph-edit", async () => {
  await pickExample("Box Shell Preview"); await waitConverged("box", 30000, "Box Shell Preview");
  const base = await fixture(); const detail = { baseWidgets: base?.ids, baseWires: base?.wires };
  if (!base || !base.wires.length) return { ok: false, detail: { ...detail, reason: "no published graph document" } };
  const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);
  // ➕️ Catalogue add
  await openPanel("framework.panel.catalogue", "catalogue");
  const candidates = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"] [role="treeitem"]')].map((el) => ({ id: el.id, text: (el.textContent ?? "").trim().slice(0, 40) })));
  const pick = candidates.find((r) => /catalogue\.(inputSlider|slider)/i.test(r.id)) ?? candidates.find((r) => /catalogue\.neuron\./.test(r.id));
  detail.catalogueRows = candidates.length; detail.picked = pick;
  let added = [];
  if (pick) {
    await page.locator(`[data-slot="panel"] [id="${pick.id}"]`).first().click({ timeout: 8000 }).catch(() => {});
    const a = await until(fixture, (f) => f && f.ids.length > base.ids.length, 20000, 250);
    added = (a.value?.ids ?? []).filter((id) => !base.ids.includes(id)); detail.added = added;
  }
  // ✂️ Disconnect: drag the sink handle onto empty canvas; 🔌 reconnect: drag source -> sink.
  await focusMain();
  const wire = base.wires[0]; const [src, sink] = wire.split("->");
  const host_ = await rectOf(`[data-surface-id="${MAIN}"]`); const empty = { x: host_.x + host_.width * 0.5, y: host_.y + host_.height - 60 };
  const sp = (await until(() => handlePoint(sink), Boolean, 15000)).value;
  const drag = async (from, to) => { await page.mouse.move(from.x, from.y); await page.mouse.down(); await page.mouse.move((from.x + to.x) / 2, (from.y + to.y) / 2, { steps: 8 }); await page.mouse.move(to.x, to.y, { steps: 8 }); await page.mouse.up(); };
  let cut = null; let rewired = null;
  if (sp) {
    await drag(sp, empty);
    cut = await until(fixture, (f) => f && f.wires.length < base.wires.length, 20000, 250);
    const a = (await until(() => handlePoint(src), Boolean, 15000)).value; const b = (await until(() => handlePoint(sink), Boolean, 15000)).value;
    if (a && b) { await drag(a, b); rewired = await until(fixture, (f) => f && f.wires.includes(wire), 20000, 250); }
  }
  detail.disconnect = cut?.reached ?? false; detail.connect = rewired?.reached ?? false;
  // 🗑️ Delete the added node (Delete chord on its selected row).
  let deleted = false;
  if (added[0]) {
    await openPanel("framework.panel.artifact", "procedural-play-graph");
    await page.locator(`[data-slot="panel"] [id="panel:procedural-play-graph/${added[0]}"]`).first().click({ timeout: 8000 }).catch(() => {});
    await focusMain().catch(() => {});
    const np = await entityPoint("node", added[0]); if (np) await page.mouse.click(np.x, np.y);
    await page.keyboard.press("Delete");
    deleted = (await until(fixture, (f) => f && !f.ids.includes(added[0]), 15000, 250)).reached;
  }
  detail.deleted = deleted;
  // ⏪️ Undo everything back to the baseline document.
  let undone = false; let presses = 0;
  for (; presses < 8 && !undone; presses += 1) {
    await focusMain(); await chord("mod+z"); await sleep(400);
    const f = await fixture(); undone = Boolean(f) && same(f.ids, base.ids) && same(f.wires, base.wires);
  }
  detail.undoAll = undone; detail.undoPresses = presses; detail.afterUndo = await fixture();
  const c = await waitConverged("after-undo", 30000);
  detail.previewSettled = c.reached;
  return { ok: Boolean(pick) && added.length > 0 && detail.disconnect && detail.connect && deleted && undone && c.reached, detail };
});
//#endregion

//#region 6 lazy extension
await row("6-lazy-extension", async () => {
  await pickExample("No example");
  await waitConverged("empty", 20000);
  await page.evaluate(() => { window.__journeyNotices.length = 0; });
  await pickExample("Sphere Box Fuse");
  const c = await waitConverged("lazy", 60000, "Sphere Box Fuse");
  const notices = await page.evaluate(() => [...window.__journeyNotices]);
  const door = await page.evaluate(() => typeof window.semioReactActivateOnDemand);
  const s = c.value;
  return { ok: notices.length > 0 && c.reached && (host(s, PREVIEW)?.meshCount ?? 0) > 0, detail: { notices, converged: c.reached, meshes: host(s, PREVIEW)?.meshCount, activateOnDemandDoor: door, note: "notice captured by a MutationObserver installed before app code; regex from react-lazy-activation-probe.mjs" } };
});
//#endregion

//#region 7 cancel mid-run, progress, responsiveness
await row("7-cancel-progress-latency", async () => {
  const slow = "Sphere Cut With Torus";
  let armed = null; const phases = new Set(); const ratios = []; const texts = new Set();
  for (let attempt = 0; attempt < 3 && !armed; attempt += 1) {
    await pickExample("No example"); await waitConverged("empty", 15000);
    await page.evaluate(() => { window.__journeyInput.length = 0; });
    await pickExample(slow);
    const end = Date.now() + 40000; let k = 0;
    while (Date.now() < end && !armed) {
      await page.mouse.move(40 + (k % 20) * 3, 400 + (k % 7) * 5); k += 1;
      const s = await snap(); const p = host(s, PREVIEW);
      if (p?.phase) phases.add(p.phase); if (p?.ratio != null) ratios.push(p.ratio); if (s.pane?.text) texts.add(s.pane.text.slice(0, 80));
      if (s.cancel && p?.cancellable !== false) armed = s;
      else if (settled(p) && p.meshCount > 0) break;
      await sleep(50);
    }
  }
  const input = await page.evaluate(() => { const v = [...window.__journeyInput].sort((a, b) => a - b); return { samples: v.length, p95: v[Math.floor(v.length * 0.95)] ?? null, max: v.at(-1) ?? null }; });
  if (!armed) return { ok: false, detail: { reason: "cancel affordance never offered while evaluating", phases: [...phases], ratios: ratios.slice(0, 20), texts: [...texts], input } };
  await page.locator('[data-slot="world-compute-cancel"]').first().click({ timeout: 5000 });
  const cancelled = await until(snap, (s) => host(s, PREVIEW)?.phase === "cancelled" || s.pane?.phase === "cancelled", 15000, 100);
  const afterText = cancelled.value?.pane?.text ?? null;
  const respond = await page.evaluate(() => new Promise((r) => { const t = performance.now(); requestAnimationFrame(() => r(performance.now() - t)); }));
  return { ok: cancelled.reached && (input.p95 ?? 1e9) < 200 && respond < 200 && (new Set(ratios).size > 1 || phases.size > 1 || texts.size > 0), detail: { cancelled: cancelled.reached, status: afterText, phases: [...phases], distinctRatios: new Set(ratios).size, progressTexts: [...texts], input, rafAfterCancelMs: Math.round(respond), phaseAfter: host(cancelled.value, PREVIEW)?.phase } };
});
//#endregion


//#region 10-16 open gaps (📓️open-gaps-audit-2026-11-25.md)
const inspectorRow = () => page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"] [id*="procedural-play-inspector"]')].map((el) => ({ id: el.id, text: (el.textContent ?? "").trim().slice(0, 80) })));
const sweepHit = async (stateKey) => {
  const box = await rectOf(`[data-surface-id="${PREVIEW}"] canvas`) ?? await rectOf(`[data-surface-id="${PREVIEW}"]`);
  if (!box) return null;
  const pts = []; for (let r = 1; r <= 5; r += 1) for (let c = 1; c <= 7; c += 1) pts.push([box.x + box.width * c / 8, box.y + box.height * r / 6]);
  pts.sort((a, b) => Math.hypot(a[0] - box.x - box.width / 2, a[1] - box.y - box.height / 2) - Math.hypot(b[0] - box.x - box.width / 2, b[1] - box.y - box.height / 2));
  for (const [x, y] of pts) { await page.mouse.move(x, y); const h = await until(snap, (s) => Boolean(host(s, PREVIEW)?.selection?.hoverTarget?.id), 300, 60); if (h.reached) return { x, y, target: host(h.value, PREVIEW).selection.hoverTarget }; }
  return null;
};
const selectAndInspect = async (example) => {
  await pickExample(example); await waitConverged(example, 30000, example);
  await openPanel("framework.panel.inspection", "procedural-play-inspector");
  const hit = await sweepHit();
  if (!hit) return { ok: false, detail: { example, reason: "no geometry under the pointer" } };
  await page.mouse.click(hit.x, hit.y);
  const sel = await until(snap, (s) => { const ids = host(s, PREVIEW)?.selection?.selectedIds ?? []; return ids.length === 1 && ids[0] === hit.target.id; }, 8000, 150);
  const widget = (hit.target.id ?? "").split("@")[0].replace(/#\d+$/, "").replace(/^eval-/, "");
  const idRow = await until(async () => { if (!(await page.locator('[data-slot="panel"] [id*="procedural-play-inspector"]').count())) await page.locator('[id="framework.panel.inspection"]').first().click({ timeout: 3000 }).catch(() => {}); return (await inspectorRow()).find((r) => r.id.endsWith("procedural-play-inspector.id")) ?? null; }, (r) => Boolean(r) && r.text.includes(widget), 15000, 500);
  return { ok: sel.reached && idRow.reached, detail: { example, hovered: hit.target, selected: host(sel.value, PREVIEW)?.selection?.selectedIds, widget, inspector: idRow.value } };
};
await row("10-add-generation-chord", async () => {
  await chord("mod+alt+ArrowRight"); await until(snap, (s) => s.hosts.some((h) => h.surfaceId === GEN_PREVIEW), 20000);
  const rowsNow = () => page.evaluate(() => [...document.querySelectorAll('[id*="procedural3d-play-generate.generation."]')].filter((e) => !e.id.endsWith(".rename")).map((e) => e.id));
  const before = await rowsNow();
  const anchor = page.locator('[role="treeitem"][id*=".generation."], [role="treeitem"][id$=".add-generation"]').first();
  await anchor.focus().catch(() => {});
  await chord("mod+shift+g");
  const grown = await until(rowsNow, (r) => r.length === before.length + 1, 30000, 300);
  const c = await until(snap, (s) => s.hosts.some((h) => h.surfaceId === GEN_PREVIEW && settled(h)), 40000, 300);
  await chord("mod+alt+ArrowLeft"); await until(snap, (s) => s.hosts.some((h) => h.surfaceId === PREVIEW), 20000);
  return { ok: grown.reached && c.reached, detail: { before: before.length, after: grown.value?.length, generatePreviewSettled: c.reached } };
});
await row("11-select-rectangle-wire-inspector", () => selectAndInspect("Rectangle Wire Preview"));
await row("12-select-hexagonal-inspector", () => selectAndInspect("Hexagonal Mushroom Column"));
await row("13-gumball-drag", async () => {
  await pickExample("Box Shell Preview"); await waitConverged("box", 30000, "Box Shell Preview");
  const hit = await sweepHit(); if (!hit) return { ok: false, detail: { reason: "no geometry hit" } };
  await page.mouse.click(hit.x, hit.y);
  const g = await until(snap, (s) => host(s, PREVIEW)?.selection?.gumballActive === true, 8000, 150);
  const sel = host(g.value, PREVIEW)?.selection;
  if (!g.reached) return { status: "missing", ok: false, detail: { reason: "selection never published gumballActive=true", selection: sel, evidence: "data-selection-json.gumballActive/gumballTarget (World3dHost worldSurfaceSelectionDomV1); action translateSelection (generation3d/🦀️.rs:501)" } };
  const inst = () => page.evaluate(() => { const el = [...document.querySelectorAll("[data-instances-json]")].find((e) => e.getAttribute("data-surface-id")?.endsWith("-preview")); try { return JSON.parse(el.getAttribute("data-instances-json")).map((i) => i.position); } catch { return null; } });
  const before = await inst(); const digest0 = host(await snap(), PREVIEW)?.digest;
  const box = await rectOf(`[data-surface-id="${PREVIEW}"] canvas`) ?? await rectOf(`[data-surface-id="${PREVIEW}"]`);
  let moved = null;
  for (const [dx, dy] of [[0, -60], [60, 0], [0, 60], [-60, 0], [40, -40], [-40, 40]]) for (const off of [40, 70, 100]) {
    const sx = hit.x + (dx ? Math.sign(dx) * off : 0); const sy = hit.y + (dy ? Math.sign(dy) * off : 0);
    await page.mouse.move(sx, sy); await page.mouse.down(); await page.mouse.move(sx + dx, sy + dy, { steps: 10 }); await page.mouse.up();
    const m = await until(async () => JSON.stringify(await inst()) !== JSON.stringify(before), Boolean, 1200, 150);
    if (m.reached) { moved = { from: [sx, sy], delta: [dx, dy] }; break; }
  }
  const after = await until(snap, (s) => host(s, PREVIEW)?.digest !== digest0 || moved, 8000, 200);
  return { ok: Boolean(moved), detail: { gumballTarget: sel?.gumballTarget, transformMode: sel?.transformMode, moved, instancesBefore: before, instancesAfter: await inst(), previewChanged: after.reached } };
});
await row("14-undo-rearm-after-edit", async () => {
  await pickExample("Hexagonal Mushroom Column"); await waitConverged("hex", 30000, "Hexagonal Mushroom Column");
  await openPanel("framework.panel.artifact", "procedural-play-graph");
  await page.locator('[data-slot="panel"] [role="treeitem"]').filter({ hasText: /^Column Height\b/ }).first().click({ timeout: 8000 });
  await openPanel("framework.panel.inspection", "procedural-play-inspector");
  if (!(await until(async () => (await sliderReadout().count()) > 0, Boolean, 120000)).reached) return { ok: false, detail: { reason: "no inspector slider" } };
  const d0 = host(await snap(), PREVIEW)?.digest; const v0 = Number(await sliderValue()); const cycles = [];
  for (const delta of [2, 3]) {
    await editSliderValue(v0 + delta);
    const e = await until(snap, (s) => host(s, PREVIEW)?.digest !== d0 && settled(host(s, PREVIEW)), 40000, 250);
    await focusMain(); await chord("mod+z");
    const u = await until(snap, (s) => host(s, PREVIEW)?.digest === d0 && settled(host(s, PREVIEW)), 40000, 250);
    cycles.push({ delta, edited: e.reached, undone: u.reached, canRedo: u.value?.history?.canRedo });
  }
  return { ok: cycles.every((c) => c.edited && c.undone), detail: { cycles } };
});
await row("15-status-row-a11y-en-de", async () => {
  const read = () => page.evaluate(() => { const p = document.querySelector('[data-slot="world-compute-status"]'); const hosts = [...document.querySelectorAll("[data-status-json]")].map((e) => ({ id: e.getAttribute("data-surface-id"), role: e.getAttribute("role"), label: e.getAttribute("aria-label"), live: e.getAttribute("aria-live") })); return { pane: p ? { role: p.getAttribute("role"), label: p.getAttribute("aria-label"), live: p.getAttribute("aria-live"), text: p.innerText.replace(/\s+/g, " ").trim() } : null, hosts, lang: document.documentElement.lang }; });
  const setLocale = async (re) => { if (!(await page.locator("button#framework\\.settings\\.language").count())) { await page.locator("#framework\\.settings").first().click({ timeout: 8000 }).catch(() => {}); await sleep(900); } await page.locator("button#framework\\.settings\\.language").first().click({ timeout: 8000 }); await sleep(400); await page.locator("[role='option']").filter({ hasText: re }).first().click({ timeout: 6000 }); await sleep(1000); await page.keyboard.press("Escape"); await sleep(500); };
  const en = await read(); await setLocale(/Deutsch/);
  const de = await until(read, (r) => r.lang === "de" || r.hosts.some((h, i) => h.label && h.label !== en.hosts[i]?.label), 15000, 300);
  await setLocale(/English/);
  const okFor = (r) => r.hosts.length > 0 && r.hosts.every((h) => h.label) && (!r.pane || (r.pane.live && r.pane.live !== "off" || r.pane.role === "status"));
  const differs = JSON.stringify(en.hosts.map((h) => h.label)) !== JSON.stringify(de.value?.hosts.map((h) => h.label)) || en.pane?.text !== de.value?.pane?.text;
  return { ok: okFor(en) && okFor(de.value) && differs, detail: { en, de: de.value, differs } };
});
await row("16-fit-camera", async () => {
  await pickExample("Box Shell Preview"); await waitConverged("box", 30000, "Box Shell Preview");
  const b = host(await snap(), PREVIEW)?.camera;
  const pbox = await rectOf(`[data-surface-id="${PREVIEW}"] canvas`);
  const ox = pbox.x + pbox.width * 0.6; const oy = pbox.y + pbox.height * 0.45;
  await page.keyboard.down("Alt"); await page.mouse.move(ox, oy); await page.mouse.down({ button: "right" }); await page.mouse.move(ox + 90, oy + 30, { steps: 10 }); await page.mouse.up({ button: "right" }); await page.keyboard.up("Alt");
  await page.mouse.move(pbox.x + pbox.width / 2, pbox.y + pbox.height / 2); await page.mouse.wheel(0, 600); await sleep(600);
  const off = host(await snap(), PREVIEW)?.camera;
  const fit = page.locator('[data-slot="world-frame-instances"]').last();
  if (!(await fit.count())) return { status: "missing", ok: false, detail: { reason: "no [data-slot=world-frame-instances] button", evidence: "interaction-matrix-probe.mjs fit step / World3dHost" } };
  await fit.click({ timeout: 6000 });
  const moved = await until(snap, (s) => JSON.stringify(host(s, PREVIEW)?.camera) !== JSON.stringify(off), 8000, 150);
  const oracle = oracleForLabel("Box Shell Preview"); const idx = (await snap()).hosts.findIndex((h) => h.surfaceId === PREVIEW);
  const stats = await page.evaluate(meshStatsScript, oracle?.previewMeshId ?? null);
  const g = gradeCameraFrames(host(moved.value, PREVIEW)?.camera ?? null, stats[idx]?.allBounds ?? null, pbox.width / pbox.height);
  return { ok: moved.reached && g.ok, detail: { moved: moved.reached, cameraBefore: b, cameraOff: off, cameraFit: host(moved.value, PREVIEW)?.camera, grade: { ok: g.ok, reasons: g.reasons, worstX: g.worstX, worstY: g.worstY } } };
});
//#endregion

await row("17-example-picker-document", async () => {
  const populated = await until(fixture, (document) => (document?.ids.length ?? 0) > 0, Number(process.env.SEMIO_PROBE_PICKER_MOUNT_MS ?? 60000), 500);
  if (!populated.reached) return { ok: false, detail: { reason: "example document did not mount" } };
  const label = (await snap()).example;
  const options = await comboOptions();
  const target = options.find((option) => option === "Rectangle Wire Preview" && option !== label);
  if (!target) return { ok: false, detail: { label, options, reason: "rectangle example absent from picker" } };
  await pickExample(target);
  const switched = await until(async () => ({
    label: (await snap()).example,
    ids: (await fixture())?.ids,
    domIds: await page.evaluate((id) => {
      const source = document.querySelector(`[data-surface-id="${id}"]`)?.getAttribute("data-host-snapshot-json");
      try { return JSON.parse(source ?? "null")?.widgets?.map((widget) => widget.id ?? Object.values(widget)[0]?.id).sort(); } catch { return null; }
    }, MAIN),
  }), (state) => state.label === target && state.ids?.length > 0 && JSON.stringify(state.ids) !== JSON.stringify(populated.value.ids), Number(process.env.SEMIO_PROBE_PICKER_MS ?? 65000), 300);
  return { ok: switched.reached, detail: { options, label, target, idsBefore: populated.value.ids, selected: switched.value?.label, idsAfter: switched.value?.ids, domIds: switched.value?.domIds } };
});

await row("18-rectangle-history-viewer", async () => {
  const rectangle = await until(fixture, (document) => document?.ids.includes("rect") && typeof document.values.height === "number", 30000, 300);
  if (!rectangle.reached) return { ok: false, detail: { reason: "rectangle document did not mount" } };
  const value = () => fixture().then((document) => document?.values.height);
  const original = rectangle.value.values.height;
  const edited = original + 1;
  await openPanel("framework.panel.artifact", "procedural-play-graph");
  await page.getByRole("treeitem", { name: /^Height\b/ }).first().click({ timeout: 10000 });
  await openPanel("framework.panel.inspection", "procedural-play-inspector");
  await sliderReadout().waitFor({ state: "visible", timeout: 40000 });
  await editSliderValue(edited);
  const change = await until(value, (current) => current === edited, 100000, 350);
  log("rectangle edit", { original, expected: edited, actual: change.value, committed: change.reached });
  let undo;
  let redo;
  if (change.reached) {
    await page.locator(`[data-surface-id="${MAIN}"][role="application"]`).first().focus({ timeout: 5000 });
    await chord("mod+z");
    undo = await until(value, (current) => current === original, 60000, 350);
    log("rectangle undo", { expected: original, actual: undo.value, committed: undo.reached });
    if (undo.reached) {
      await chord("mod+shift+z");
      redo = await until(value, (current) => current === edited, 60000, 350);
      log("rectangle redo", { expected: edited, actual: redo.value, committed: redo.reached });
    }
  }
  await page.locator('[id="playground.navbar.roles.viewer"]').click({ timeout: 8000 });
  const viewer = await until(snap, (state) => state.roles.viewer === "true" && state.windows.includes(VIEW_PREVIEW.slice("window:".length)), 35000, 350);
  log("rectangle viewer", { mounted: viewer.reached, roles: viewer.value?.roles, windows: viewer.value?.windows });
  await page.locator('[id="playground.navbar.roles.editor"]').click({ timeout: 8000 });
  const editor = await until(snap, (state) => state.roles.editor === "true" && state.windows.includes(MAIN.slice("window:".length)), 35000, 350);
  const restoredValue = await value();
  return { ok: Boolean(change.reached && undo?.reached && redo?.reached && viewer.reached && editor.reached && restoredValue === edited), detail: { original, edited, change, undo, redo, viewer: { mounted: viewer.reached, roles: viewer.value?.roles, windows: viewer.value?.windows }, editor: { mounted: editor.reached, roles: editor.value?.roles, windows: editor.value?.windows, value: restoredValue } } };
});

await row("19-camera-inspector", async () => {
  const ready = await until(snap, (state) => Boolean(host(state, MAIN)?.nodeIds?.includes("height") && host(state, PREVIEW)?.camera), 160000, 650);
  if (!ready.reached) return { ok: false, detail: { reason: "Edit graph and camera did not mount" } };
  const camera0 = host(ready.value, PREVIEW).camera;
  const graphViewport = () => page.evaluate((id) => window.__semioFlowGraphProbe?.[id]?.viewport?.() ?? null, MAIN);
  const graph0 = await graphViewport();
  const target = await rectOf(`[data-surface-id="${PREVIEW}"] canvas`);
  if (!target) return { ok: false, detail: { reason: "Edit preview canvas did not mount" } };
  const x = target.x + target.width / 2;
  const y = target.y + target.height / 2;
  await page.keyboard.down("Alt");
  try {
    await page.mouse.move(x, y);
    await page.mouse.down({ button: "right" });
    await page.mouse.move(x + target.width * 0.2, y + target.height * 0.075, { steps: 12 });
    await page.mouse.up({ button: "right" });
  } finally {
    await page.keyboard.up("Alt");
  }
  const orbit = await until(snap, (state) => Boolean(host(state, PREVIEW)?.camera && JSON.stringify(host(state, PREVIEW).camera.position) !== JSON.stringify(camera0.position)), 12000, 200);
  const camera1 = host(orbit.value, PREVIEW)?.camera;
  const distance = (camera) => camera ? Math.hypot(...camera.position.map((axis, index) => axis - camera.target[index])) : null;
  await page.mouse.move(x, y);
  await page.mouse.wheel(0, -420);
  const zoom = await until(snap, (state) => {
    const current = distance(host(state, PREVIEW)?.camera);
    return current !== null && camera1 && Math.abs(current - distance(camera1)) > 0.02;
  }, 12000, 200);
  const camera2 = host(zoom.value, PREVIEW)?.camera;
  const graph1 = await graphViewport();
  const camera = { orbit: orbit.reached, zoom: zoom.reached, before: camera0, afterOrbit: camera1, afterZoom: camera2, graphViewportUnchanged: graph0 !== null && JSON.stringify(graph0) === JSON.stringify(graph1) };
  log("camera witness", camera);
  await openPanel("framework.panel.artifact", "procedural-play-graph");
  await page.locator('[data-slot="panel"] [role="treeitem"]').filter({ hasText: /^Column Height\b/ }).first().click({ timeout: 10000 });
  await openPanel("framework.panel.inspection", "procedural-play-inspector");
  const selection = await until(snap, (state) => host(state, MAIN)?.selection?.selectedIds?.includes("height"), 30000, 300);
  const control = await until(async () => sliderReadout().isVisible(), Boolean, 30000, 300);
  const ariaValue = control.reached ? await sliderValue() : null;
  const selected = { graph: host(selection.value, MAIN)?.selection?.selectedIds, inspectorValue: ariaValue, documentValue: (await fixture())?.values.height };
  log("selection witness", selected);
  return { ok: camera.orbit && camera.zoom && camera.graphViewportUnchanged && selection.reached && control.reached && ariaValue === String(selected.documentValue), detail: { camera, selected } };
});

//#region 9 export / import, reload persistence (before locale row, which changes labels)
await row("9-export-import-reload", async () => {
  await pickExample("Box Shell Preview"); await waitConverged("box", 30000, "Box Shell Preview");
  const detail = {};
  await focusMain();
  const folded = await page.evaluate(() => document.getElementById("framework.window.proceduralMain.engagement")?.getAttribute("data-folded") === "true");
  if (folded) await page.locator("[id='framework.window.proceduralMain.engagement.toggle']").first().click({ timeout: 8000 }).catch(() => {});
  await sleep(600);
  let saved = null;
  try {
    for (let i = 0; i < 3 && !(await page.locator("#format").count()); i += 1) { await page.locator("#action\\.exportDocument").click({ timeout: 8000 }).catch(() => {}); await sleep(900); }
    const [download] = await Promise.all([page.waitForEvent("download", { timeout: 60000 }), page.locator("[id='framework.window.proceduralMain.action.exportDocument.execute']").click({ timeout: 10000 })]);
    saved = join(OUT, download.suggestedFilename() || "export.bin"); await download.saveAs(saved);
    detail.export = { file: download.suggestedFilename(), bytes: readFileSync(saved).length };
  } catch (e) { detail.export = { error: String(e).slice(0, 300) }; }
  if (saved && detail.export.bytes > 0) {
    try {
      const [chooser] = await Promise.all([page.waitForEvent("filechooser", { timeout: 30000 }), page.locator("#action\\.importDocumentRequest").click({ timeout: 8000 })]);
      await chooser.setFiles(saved);
      const imp = await until(fixture, (f) => f && f.ids.includes("imported-geometry"), 60000, 500);
      detail.reopen = { ok: imp.reached, ids: imp.value?.ids };
    } catch (e) { detail.reopen = { ok: false, error: String(e).slice(0, 300) }; }
  } else detail.reopen = { ok: false, reason: "nothing exported" };
  const before = await fixture();
  await page.reload({ waitUntil: "domcontentloaded" });
  const back = await until(fixture, (f) => f && f.ids.length > 0, 90000, 500);
  const after = back.value;
  detail.reload = { before: before?.ids, after: after?.ids, persisted: Boolean(before && after) && JSON.stringify(before.ids) === JSON.stringify(after.ids) && JSON.stringify(before.values) === JSON.stringify(after.values) };
  return { ok: Boolean(detail.export?.bytes > 0) && Boolean(detail.reopen?.ok) && detail.reload.persisted, detail };
});
//#endregion

//#region 8 locale + keyboard + accessible names
await row("8-locale-keyboard-a11y", async () => {
  await until(snap, (s) => s.windows.length > 0, 60000, 500);
  const detail = {};
  const setLocale = async (re) => {
    if (!(await page.locator("button#framework\\.settings\\.language").count())) { await page.locator("#framework\\.settings").first().click({ timeout: 8000 }).catch(() => {}); await sleep(900); }
    await page.locator("button#framework\\.settings\\.language").first().click({ timeout: 8000 }); await sleep(500);
    const opt = page.locator("[role='option']").filter({ hasText: re }).first();
    if (!(await opt.count())) { await page.keyboard.press("Escape"); return false; }
    await opt.click(); await sleep(1200); await page.keyboard.press("Escape"); await sleep(600); return true;
  };
  const labels = async () => page.evaluate(() => ({ lang: document.documentElement.lang, text: document.body.innerText.replace(/\s+/g, " "), tabs: [...document.querySelectorAll('[data-slot="mode-dock-tab"],[data-slot="panel-tab-button"]')].map((e) => (e.getAttribute("aria-label") ?? e.textContent ?? "").trim()) }));
  const en0 = await labels();
  const toDe = await setLocale(/Deutsch/); const de = await until(labels, (l) => l.lang === "de" || /Vorschau|Katalog|Inspektion|Generationen/.test(l.text), 15000, 300);
  const enTabsReturned = (state) => ["Artifact", "Catalogue", "Inspection"].every((label) => state?.tabs.includes(label));
  const toEn = await setLocale(/English|Englisch/); const en1 = await until(labels, (l) => l.lang === "en" && enTabsReturned(l), 15000, 300);
  detail.locale = { toDe, toEn, deChangedTabs: JSON.stringify(en0.tabs) !== JSON.stringify(de.value?.tabs), deLang: de.value?.lang, enLang: en1.value?.lang, enBackTabs: en1.reached, enTabs: en0.tabs.slice(0, 8), deTabs: de.value?.tabs.slice(0, 8), returnedTabs: en1.value?.tabs.slice(0, 8) };
  // ⌨️ Tab sweep: every window must receive focus; a mode chord must still work with focus inside the app.
  await page.evaluate(() => document.activeElement?.blur?.());
  const reached = new Set(); const windows = await page.evaluate(() => [...document.querySelectorAll('[data-slot="window"]')].map((e) => e.id));
  for (let i = 0; i < 160 && reached.size < windows.length; i += 1) {
    await page.keyboard.press("Tab");
    const w = await page.evaluate(() => { const a = document.activeElement; return (a?.closest?.('[data-slot="window"]') ?? a?.closest?.("[data-surface-id]"))?.id || a?.closest?.("[data-surface-id]")?.getAttribute("data-surface-id") || null; });
    if (w) reached.add(w);
  }
  detail.tab = { windows, reached: [...reached], missed: windows.filter((w) => !reached.has(w) && ![...reached].some((r) => r.includes(w.replace(/^window:/, "")))) };
  const g0 = (await snap()).modes.generate;
  await chord("mod+alt+ArrowRight");
  const sw = await until(snap, (s) => s.hosts.some((h) => h.surfaceId === GEN_PREVIEW), 15000, 250);
  await chord("mod+alt+ArrowLeft");
  const back = await until(snap, (s) => s.hosts.some((h) => h.surfaceId === PREVIEW), 15000, 250);
  detail.shortcut = { modeChordMountsGenerate: sw.reached, generatePressedBefore: g0, generatePressedAfter: sw.value?.modes.generate, generateWindows: sw.value?.windows, editPressedAfter: back.value?.modes.edit, editWindows: back.value?.windows };
  // 🏷️ Accessible-name sweep over the browser's own accessibility tree (CDP), plus raw interactive DOM.
  const cdp = await page.context().newCDPSession(page); await cdp.send("Accessibility.enable");
  const { nodes } = await cdp.send("Accessibility.getFullAXTree");
  const INTERACTIVE = new Set(["button", "textbox", "combobox", "slider", "spinbutton", "checkbox", "switch", "tab", "menuitem", "treeitem", "link", "option", "radio", "searchbox", "listbox"]);
  const ax = nodes.filter((n) => !n.ignored && INTERACTIVE.has(n.role?.value));
  const unnamed = ax.filter((n) => !(n.name?.value ?? "").trim());
  const domUnnamed = await page.evaluate(() => [...document.querySelectorAll('button,a[href],input:not([type=hidden]),select,textarea,[role=button],[role=slider],[role=combobox],[role=treeitem]')].filter((e) => { const r = e.getBoundingClientRect(); return r.width > 0 && r.height > 0; }).filter((e) => !(e.getAttribute("aria-label") || e.getAttribute("aria-labelledby") || e.getAttribute("title") || (e.labels && e.labels.length) || (e.textContent ?? "").trim() || e.getAttribute("placeholder"))).map((e) => `${e.tagName.toLowerCase()}#${e.id}.${(e.getAttribute("data-slot") ?? "")}`).slice(0, 25));
  const unnamedAxSample = await Promise.all(unnamed.slice(0, 10).map(async (entry) => {
    const node = entry.backendDOMNodeId ? (await cdp.send("DOM.describeNode", { backendNodeId: entry.backendDOMNodeId })).node : undefined;
    const at = (name) => { const i = node?.attributes?.indexOf(name) ?? -1; return i < 0 ? undefined : node.attributes[i + 1]; };
    return { role: entry.role.value, tag: node?.nodeName, id: at("id"), labelledBy: at("aria-labelledby") };
  }));
  await cdp.detach();
  detail.a11y = { interactive: ax.length, unnamedAx: unnamed.length, unnamedAxSample, unnamedDom: domUnnamed };
  const ok = detail.locale.toDe && detail.locale.deChangedTabs && detail.locale.enBackTabs && detail.tab.missed.length === 0 && detail.shortcut.modeChordMountsGenerate && unnamed.length === 0 && domUnnamed.length === 0;
  return { ok, detail };
});
//#endregion

clearTimeout(watchdog);
if (componentFiles.size) {
  const served = new Set([...artifacts.keys()].map((url) => decodeURIComponent(new URL(url).pathname)));
  const missing = [...componentFiles.keys()].filter((path) => (path.endsWith(".js") || path.endsWith(".core.wasm")) && !served.has(path));
  rows.push({ id: "component-inputs", ok: missing.length === 0, status: missing.length === 0 ? "pass" : "fail", ms: Date.now() - T0, detail: { missing }, screenshot: null });
}
const errors = pageErrors();
rows.push({ id: "page-errors", ok: errors.length === 0, status: errors.length === 0 ? "pass" : "fail", ms: Date.now() - T0, detail: errors.slice(0, 10), screenshot: null });
flush();
console.log(JSON.stringify(rows.map(({ id, ok, status, ms }) => ({ id, ok, status, ms }))));
await finishBrowser(rows.every((r) => r.ok) ? 0 : 1);
