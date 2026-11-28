/** 🦊️ Process 3D e2e via stock headless Firefox over WebDriver BiDi: loads the editor, collects console/page errors, checks DOM and canvas, saves a screenshot. */
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { appendFileSync, cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { inflateSync } from "node:zlib";

const url = process.env.PROCESS_URL ?? "http://127.0.0.1:6022/";
const out = join(import.meta.dir, "🗑️generated");
const port = Number(process.env.BIDI_PORT ?? 9333);
const tag = process.env.TAG ?? "e2e";
const startedAt = new Date().toISOString();
const profile = mkdtempSync(join(tmpdir(), "v11-probe-"));
if (process.env.FF_PREFS) writeFileSync(join(profile, "user.js"), process.env.FF_PREFS);
const crashDir = join(profile, "crash-data");
mkdirSync(crashDir);
const firefox = spawn("/Applications/Firefox.app/Contents/MacOS/firefox", ["--headless", "--no-remote", "--profile", profile, `--remote-debugging-port=${port}`], { stdio: "ignore", env: { ...process.env, MOZ_CRASHREPORTER: "1", MOZ_CRASHREPORTER_NO_REPORT: "1", MOZ_CRASHREPORTER_DATA_DIRECTORY: crashDir } });
/** 🖼️ Counts the pixels of a PNG (8-bit, non-interlaced) that differ from the most frequent colour. */
function nonBackgroundPixels(png: Buffer): { width: number; height: number; differing: number } {
  let width = 0, height = 0, channels = 4; const idat: Buffer[] = [];
  for (let at = 8; at < png.length;) { const length = png.readUInt32BE(at), type = png.toString("ascii", at + 4, at + 8), body = png.subarray(at + 8, at + 8 + length); if (type === "IHDR") { width = body.readUInt32BE(0); height = body.readUInt32BE(4); channels = body[9] === 2 ? 3 : 4; } else if (type === "IDAT") idat.push(body); at += 12 + length; }
  const raw = inflateSync(Buffer.concat(idat)), stride = width * channels, pixels = Buffer.alloc(stride * height);
  for (let y = 0; y < height; y++) { const filter = raw[y * (stride + 1)], row = y * (stride + 1) + 1;
    for (let x = 0; x < stride; x++) { const left = x >= channels ? pixels[y * stride + x - channels] : 0, up = y ? pixels[(y - 1) * stride + x] : 0, upLeft = y && x >= channels ? pixels[(y - 1) * stride + x - channels] : 0; const p = left + up - upLeft, pa = Math.abs(p - left), pb = Math.abs(p - up), pc = Math.abs(p - upLeft);
      pixels[y * stride + x] = (raw[row + x] + (filter === 0 ? 0 : filter === 1 ? left : filter === 2 ? up : filter === 3 ? (left + up) >> 1 : pa <= pb && pa <= pc ? left : pb <= pc ? up : upLeft)) & 255; } }
  const counts = new Map<number, number>(); for (let i = 0; i < pixels.length; i += channels) { const key = (pixels[i] << 16) | (pixels[i + 1] << 8) | pixels[i + 2]; counts.set(key, (counts.get(key) ?? 0) + 1); }
  const background = [...counts.entries()].sort((a, b) => b[1] - a[1])[0]?.[0] ?? 0; let differing = 0;
  for (let i = 0; i < pixels.length; i += channels) { const key = (pixels[i] << 16) | (pixels[i + 1] << 8) | pixels[i + 2]; if (Math.abs((key >> 16) - (background >> 16)) + Math.abs(((key >> 8) & 255) - ((background >> 8) & 255)) + Math.abs((key & 255) - (background & 255)) > 24) differing++; }
  return { width, height, differing };
}
const wait = (ms: number) => new Promise(r => setTimeout(r, ms));
firefox.on("exit", (code, signal) => { console.error(`[FAIL] firefox exited early code=${code} signal=${signal}`); if (existsSync(join(crashDir, "pending"))) cpSync(crashDir, join(out, `crash-data-${Date.now()}`), { recursive: true }); try { rmSync(profile, { recursive: true, force: true }); } catch {} process.exit(2); });

async function connect(): Promise<WebSocket> {
  for (let i = 0; i < 60; i++) {
    try {
      const ws = new WebSocket(`ws://127.0.0.1:${port}/session`);
      await new Promise((ok, ko) => { ws.onopen = ok; ws.onerror = ko; });
      return ws;
    } catch { await wait(500); }
  }
  throw new Error("[DEBUG] firefox bidi unreachable");
}

const ws = await connect();
let id = 0;
const pending = new Map<number, (v: any) => void>();
const logs: { level: string; text: string; at?: number; stack?: string }[] = [];
ws.onmessage = e => {
  const m = JSON.parse(String(e.data));
  if (m.id !== undefined) pending.get(m.id)?.(m);
  else if (m.method === "log.entryAdded") { if ((process.env.LIVE_ALL || m.params.level !== "info") && process.env.LIVE_LOG) appendFileSync(process.env.LIVE_LOG, `${new Date().toISOString().slice(11, 19)} ${m.params.level} ${String(m.params.text ?? "").slice(0, 400).replace(/\n/g, " ")}\n`); logs.push({ level: m.params.level, text: String(m.params.text ?? "").slice(0, 3000), at: Number(m.params.timestamp ?? 0), stack: m.params.level === "error" ? JSON.stringify((m.params.stackTrace?.callFrames ?? []).slice(0, 14).map((f: { functionName?: string; url?: string; lineNumber?: number }) => `${f.functionName ?? ""}@${String(f.url ?? "").split("/").slice(-3).join("/")}:${f.lineNumber ?? 0}`)) : undefined }); }
};
const send = (method: string, params: object = {}, timeoutMs = 120000) => new Promise<any>(ok => { const n = ++id; pending.set(n, ok); ws.send(JSON.stringify({ id: n, method, params })); setTimeout(() => ok({ error: "timeout", message: `${method} timed out` }), timeoutMs); });

await send("session.new", { capabilities: {} });
await send("session.subscribe", { events: ["log.entryAdded"] });
const tree = await send("browsingContext.getTree");
const context = tree.result.contexts[0].context;
await send("browsingContext.setViewport", { context, viewport: { width: 1440, height: 900 } });
if (process.env.PRELOAD) await send("script.addPreloadScript", { functionDeclaration: process.env.PRELOAD });
if (process.env.NO_VITE_RELOAD !== "0") await send("script.addPreloadScript", { functionDeclaration: `() => { const Native = window.WebSocket; const Quiet = function (url, protocols) { const hmr = protocols === "vite-hmr" || (Array.isArray(protocols) && protocols.includes("vite-hmr")); if (!hmr) return new Native(url, protocols); return { addEventListener() {}, removeEventListener() {}, send() {}, close() {}, readyState: 0 }; }; Quiet.prototype = Native.prototype; for (const key of ["CONNECTING", "OPEN", "CLOSING", "CLOSED"]) Quiet[key] = Native[key]; window.WebSocket = Quiet; }` });
await send("browsingContext.navigate", { context, url, wait: "complete" });
const timeline: string[] = [];
const evaluate = async (expression: string, timeoutMs = 90000): Promise<string> => { const r = await Promise.race([send("script.evaluate", { target: { context }, awaitPromise: true, expression }), wait(timeoutMs).then(() => ({ result: { value: "[timeout]" } }))]); if (r.type === "error" || r.error) return `[exception] ${String(r.message ?? r.error)}`.slice(0, 600); if (r.result?.type === "exception") return `[exception] ${String(r.result.exceptionDetails?.text ?? "")} ${String(r.result.exceptionDetails?.exception?.value ?? "")}`.trim(); return String(r.result?.result?.value ?? r.result?.value ?? JSON.stringify(r).slice(0, 300)); };
const probeState = `JSON.stringify([document.title,document.querySelectorAll("canvas").length,(document.body.innerText.match(/has not answered[^\\n]*/i)||[""])[0]])`;
for (let t = 0; t < Number(process.env.SETTLE_MS ?? 45000); t += 5000) { await wait(5000); const state = await evaluate(probeState); timeline.push(`${(t + 5000) / 1000}s ${state}`); const canvases = (() => { try { return Number(JSON.parse(state)[1]); } catch { return 0; } })(); if (canvases >= 1 && process.env.UNTIL_CANVAS !== "0") break; }
const scenario: { name: string; js: string; keys?: string[]; pointer?: string; button?: number; capture?: boolean; minCanvasPixels?: number; until?: string; timeoutMs?: number; settleMs?: number; files?: { selectorJs: string; paths: string[] }; save?: { js: string; path: string } }[] = process.env.STEPS ? JSON.parse(process.env.STEPS) : JSON.parse(readFileSync(join(import.meta.dir, "process-interactions.json"), "utf8"));
const stepResults: { name: string; result: string; newErrors: string[]; canvasPixels?: number }[] = [];
if (process.env.INTERACT !== "0") for (const step of scenario) {
  const before = logs.filter(l => l.level === "error" && !l.text.startsWith("[DEBUG]") && !l.text.startsWith("[vite]")).length;
  let pointerFailure = "";
  const jsResult = await evaluate(step.js, step.timeoutMs ?? 90000);
  if (step.pointer) { const located = await evaluate(step.pointer); const at = (() => { try { return /^\[exception\]|^\[timeout\]/.test(located) ? null : JSON.parse(located); } catch { return null; } })() as [number, number] | null; if (!at) pointerFailure = located; else await send("input.performActions", { context, actions: [{ type: "pointer", id: "mouse", parameters: { pointerType: "mouse" }, actions: [{ type: "pointerMove", x: Math.round(at[0]), y: Math.round(at[1]) }, { type: "pointerDown", button: step.button ?? 0 }, { type: "pointerUp", button: step.button ?? 0 }] }] }); }
  if (step.files) { const found = await send("script.evaluate", { target: { context }, awaitPromise: true, resultOwnership: "root", expression: step.files.selectorJs }); const sharedId = found.result?.result?.sharedId; if (!sharedId) pointerFailure = `file input not found: ${JSON.stringify(found).slice(0, 300)}`; else { const set = await send("input.setFiles", { context, element: { sharedId }, files: step.files.paths }); if (set.type === "error" || set.error) pointerFailure = `setFiles failed: ${JSON.stringify(set).slice(0, 300)}`; } }
  if (step.save) { const b64 = await evaluate(step.save.js); if (/^\[(exception|timeout)\]/.test(b64)) pointerFailure = `save: ${b64}`; else writeFileSync(step.save.path, Buffer.from(b64, "base64")); }
  if (step.keys?.length) { const named: Record<string, string> = { Control: "\uE009", Shift: "\uE008", Alt: "\uE00A", Meta: "\uE03D", Delete: "\uE017", Escape: "\uE00C", Backspace: "\uE003", Enter: "\uE007" }; const chord = (key: string) => { const parts = key.length > 1 && key.includes("+") ? key.split("+") : [key]; const values = parts.map(part => named[part] ?? part); const last = values.pop() as string; return [...values.map(value => ({ type: "keyDown", value })), { type: "keyDown", value: last }, { type: "keyUp", value: last }, ...values.reverse().map(value => ({ type: "keyUp", value }))]; }; await send("input.performActions", { context, actions: [{ type: "key", id: "keyboard", actions: step.keys.flatMap(chord) }] }); }
  let waited = "";
  if (step.until) { const deadline = Date.now() + (step.timeoutMs ?? 300000); while (Date.now() < deadline && (await evaluate(step.until)) !== "true") await wait(5000); waited = (await evaluate(step.until)) === "true" ? "" : "[timeout] until condition not reached"; }
  await wait(step.settleMs ?? 4000);
  if (process.env.LIVE_LOG) appendFileSync(process.env.LIVE_LOG, `${new Date().toISOString().slice(11, 19)} step ${step.name} => ${(pointerFailure || waited || jsResult).slice(0, 300)}\n`);
  let canvasAt: number | undefined, canvasFailure = "";
  if (step.capture) { const rect = JSON.parse(await evaluate(`JSON.stringify((()=>{const c=document.querySelector("canvas");if(!c)return null;const r=c.getBoundingClientRect();return {x:Math.max(0,Math.floor(r.x)),y:Math.max(0,Math.floor(r.y)),width:Math.floor(r.width),height:Math.floor(r.height)}})())`) || "null"); const index = stepResults.length; const full = await send("browsingContext.captureScreenshot", { context }); if (full.result?.data) writeFileSync(join(out, `${tag}-step${String(index).padStart(2, "0")}.png`), Buffer.from(full.result.data, "base64")); if (rect && rect.width > 0 && rect.height > 0) { await evaluate(`(()=>{const s=document.createElement("style");s.id="e2e-canvas-only";s.textContent="body *{visibility:hidden!important}canvas{visibility:visible!important}";document.head.appendChild(s);return "on"})()`); await wait(300); const clip = await send("browsingContext.captureScreenshot", { context, clip: { type: "box", ...rect } }); await evaluate(`(()=>{document.getElementById("e2e-canvas-only")?.remove();return "off"})()`); canvasAt = clip.result?.data ? nonBackgroundPixels(Buffer.from(clip.result.data, "base64")).differing : 0; } else canvasAt = 0; if (step.minCanvasPixels !== undefined && canvasAt < step.minCanvasPixels) canvasFailure = `[exception] canvas shows ${canvasAt} drawn pixels, expected at least ${step.minCanvasPixels}`; }
  stepResults.push({ name: step.name, result: (pointerFailure ? `[exception] pointer target: ${pointerFailure}` : waited || canvasFailure || jsResult).slice(0, 3000), canvasPixels: canvasAt, newErrors: logs.filter(l => l.level === "error" && !l.text.startsWith("[DEBUG]") && !l.text.startsWith("[vite]")).slice(before).map(l => l.text.slice(0, 400)) });
}
if (process.env.EVAL) { const ev = await send("script.evaluate", { target: { context }, awaitPromise: true, expression: process.env.EVAL }); console.log("[DEBUG] EVAL", JSON.stringify(ev).slice(0, 60000)); }
const probe = await send("script.evaluate", { target: { context }, awaitPromise: true, expression: `(async()=>JSON.stringify({title:document.title,canvases:document.querySelectorAll("canvas").length,text:document.body.innerText.slice(0,1500),res:performance.getEntriesByType("resource").filter(r=>/wasm|process/.test(r.name)).map(r=>[decodeURIComponent(r.name).slice(-80),Math.round(r.startTime),Math.round(r.responseEnd),r.transferSize]),pluginModules:performance.getEntriesByType("resource").map(r=>decodeURIComponent(r.name)).filter(n=>n.includes("plugin-modules")).map(n=>n.split("plugin-modules")[1]).slice(0,20)}))()` });
const canvasRects = JSON.parse(await evaluate(`JSON.stringify([...document.querySelectorAll("canvas")].map(c=>{const r=c.getBoundingClientRect();return {x:Math.max(0,Math.floor(r.x)),y:Math.max(0,Math.floor(r.y)),width:Math.floor(r.width),height:Math.floor(r.height)}}))`) || "[]") as { x: number; y: number; width: number; height: number }[];
await evaluate(`(()=>{const s=document.createElement("style");s.id="e2e-canvas-only";s.textContent="body *{visibility:hidden!important}canvas{visibility:visible!important}";document.head.appendChild(s);return "on"})()`);
await wait(500);
const canvasPixels: { width: number; height: number; differing: number }[] = [];
for (const rect of canvasRects.filter(r => r.width > 0 && r.height > 0)) { const clip = await send("browsingContext.captureScreenshot", { context, clip: { type: "box", ...rect } }); canvasPixels.push(clip.result?.data ? nonBackgroundPixels(Buffer.from(clip.result.data, "base64")) : { width: 0, height: 0, differing: 0 }); }
await evaluate(`(()=>{document.getElementById("e2e-canvas-only")?.remove();return "off"})()`);
const minDrawn = Number(process.env.MIN_CANVAS_PIXELS ?? 200);
const canvasDrawn = canvasPixels.some(c => c.differing >= minDrawn);
const shot = await send("browsingContext.captureScreenshot", { context });
if (shot.result?.data) writeFileSync(join(out, `${tag}.png`), Buffer.from(shot.result.data, "base64"));
const relevant = logs.filter(l => /wasm|plugin|process|worker|instantiat|example|fail|refus/i.test(l.text)).slice(-40);
const report = { url, timeline, relevant, probe: JSON.parse(probe.result?.result?.value ?? "{}"), errors: logs.filter(l => l.level === "error"), warnings: logs.filter(l => l.level === "warn").length, logCount: logs.length, all: logs.filter(l => !l.text.includes("staged plugin module")) };
const consoleErrors = logs.filter(l => l.level === "error" && !l.text.startsWith("[DEBUG]") && !l.text.startsWith("[vite]"));
(report as Record<string, unknown>).canvasRects = canvasRects; (report as Record<string, unknown>).canvasPixels = canvasPixels; (report as Record<string, unknown>).stepResults = stepResults; (report as Record<string, unknown>).consoleErrors = consoleErrors;
const failures = [...consoleErrors.map(e => `console: ${e.text.slice(0, 300)}`), ...stepResults.filter(step => step.newErrors.length || /^\[(exception|timeout)\]/.test(step.result)).map(step => `step ${step.name}: ${step.result.slice(0, 300)}`), ...(stepResults.length === (process.env.INTERACT === "0" ? 0 : scenario.length) ? [] : [`ran ${stepResults.length} of ${scenario.length} steps`]), ...((report.probe as { canvases?: number }).canvases === 0 ? ["no canvas"] : []), ...(process.env.CANVAS_CHECK !== "0" && !canvasDrawn ? ["canvas not drawn"] : [])];
(report as Record<string, unknown>).run = { tag, scenario: process.env.SCENARIO ?? "", scenarioSha256: createHash("sha256").update(process.env.STEPS ?? "").digest("hex"), hubReceipt: (() => { try { return JSON.parse(readFileSync(process.env.HUB_RECEIPT ?? "", "utf8")); } catch { return null; } })(), url, bidiPort: port, startedAt, finishedAt: new Date().toISOString(), expectedSteps: scenario.length, ranSteps: stepResults.length, passed: failures.length === 0, failures };
writeFileSync(join(out, `${tag}-report.json`), JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2).slice(0, 6000));
await send("session.end").catch(() => {});
firefox.removeAllListeners("exit");
firefox.once("exit", () => { try { rmSync(profile, { recursive: true, force: true }); } catch {} });
firefox.kill();
await wait(1500);
try { rmSync(profile, { recursive: true, force: true }); } catch {}
if (consoleErrors.length || (report.probe as { canvases?: number }).canvases === 0 || (process.env.CANVAS_CHECK !== "0" && !canvasDrawn) || stepResults.some(step => step.newErrors.length || /^\[(exception|timeout)\]/.test(step.result))) { console.error(`[FAIL] ${consoleErrors.length} console error(s) other than [DEBUG] lines; canvases=${(report.probe as { canvases?: number }).canvases} canvasPixels=${JSON.stringify(canvasPixels)} drawn=${canvasDrawn}`); for (const e of consoleErrors) console.error(`  ${e.text.slice(0, 300)}`); process.exit(1); }
process.exit(0);
