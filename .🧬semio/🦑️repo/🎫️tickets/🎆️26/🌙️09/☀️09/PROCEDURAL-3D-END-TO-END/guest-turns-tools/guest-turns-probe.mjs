import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🧭️ Capped probe (one chromium): per-crossing log (start, duration, class, status) from boot, optional wasm CPU profile windows through raw CDP on every worker target (PROBE_PROFILE="15:20,60:20" = start s : length s), optional guest diagnostics (PROBE_DIAG=1). Rewrites only served modules. Outputs 🗑️generated/guest-turns/<tag>-*. */
const out = join(import.meta.dirname, "🗑️generated/guest-turns"); mkdirSync(out, { recursive: true });
const url = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const tag = process.env.PROBE_TAG ?? "p1";
const total = Number(process.env.PROBE_TOTAL ?? 200000);
const windows = (process.env.PROBE_PROFILE ?? "").split(",").filter(Boolean).map((w) => w.split(":").map(Number));
const port = 9339;
const browser = await chromium.launch({ headless: true, args: [`--remote-debugging-port=${port}`, "--js-flags=--max-old-space-size=1024", "--renderer-process-limit=1", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const cap = setTimeout(() => { browser.close().finally(() => process.exit(3)); }, total + 120000);
const ctx = await browser.newContext({ viewport: { width: 1440, height: 900 } });
if (process.env.PROBE_DIAG === "1") await ctx.addInitScript(() => { try { localStorage.SEMIO_RUNTIME_DIAGNOSTICS = "1"; } catch {} });
const L = `(globalThis.__el ??= [])`;
const patched = {};
const rewrite = (name, fn) => ctx.route((u) => decodeURIComponent(u.toString()).split("?")[0].includes(name), async (route) => {
  const res = await route.fetch(); let body = await res.text(); const before = body; body = fn(body); patched[name] = body !== before; await route.fulfill({ response: res, body });
});
await rewrite("PluginRuntime/🧵️turn-dispatch/🟦️.ts", (b) => b
  .replace("const raw = await (payload.activation", "const __t0 = performance.now(); const raw = await (payload.activation")
  .replace("noteCrossingCensusV1(payload.commandPage", `${L}.push(Math.round(__t0) + " " + Math.round(performance.now() - __t0) + " " + crossingCensusClassV1(payload.events) + " ev=" + payload.events.length + " " + (result.status?.tag ?? String(result.status)) + " ef=" + result.effects.length + " up=" + result.uiPatches.length + " fx=" + result.effects.map((e) => e.tag).slice(0, 3).join(",") + " " + actorId.slice(-14)); $&`)
  .replace(/^/, ""));
await rewrite("ShellHost/🔁️contributions-refresh/🟦️.ts", (b) => b
  .replace("const generation = ++refreshGenerationRef.current;", `$& ${L}.push(Math.round(performance.now()) + " REFRESH-START gen=" + generation);`)
  .replace(/program\.refreshUi\(nextSession\.instanceId, bodyRequest\)/, `(async () => { const __t = performance.now(); try { return await program.refreshUi(nextSession.instanceId, bodyRequest); } finally { ${L}.push(Math.round(performance.now()) + " GUEST-REFRESH dur=" + Math.round(performance.now() - __t) + " gen=" + generation); } })()`));
await rewrite("ShellHost/⚡️host-effects/🟦️.ts", (b) => { const n = "for (const effect of effects) {"; const i = b.indexOf(n); return i < 0 ? b : b.slice(0, i + n.length) + ` { if (effect.tag !== "send-message") ${L}.push(Math.round(performance.now()) + " APPLY " + effect.tag + ":" + JSON.stringify(effect.val?.params?.action ?? "").slice(0, 60)); }` + b.slice(i + n.length); });
const page = await ctx.newPage();
const consoleLines = []; const t0 = Date.now();
page.on("console", (m) => { const t = m.text(); if (/\[DEBUG\]|error|warn/i.test(m.type() + t)) consoleLines.push(`${Date.now() - t0} ${t.slice(0, 600)}`); });
const ver = await (await fetch(`http://127.0.0.1:${port}/json/version`)).json();
const ws = new WebSocket(ver.webSocketDebuggerUrl);
await new Promise((r) => (ws.onopen = r));
let id = 0; const pending = new Map(); const targets = new Map();
ws.onmessage = (m) => { const d = JSON.parse(m.data); if (d.id && pending.has(d.id)) { pending.get(d.id)(d); pending.delete(d.id); } else if (d.method === "Target.targetCreated") targets.set(d.params.targetInfo.targetId, d.params.targetInfo); };
const send = (method, params = {}, sessionId) => new Promise((r) => { const i = ++id; pending.set(i, r); ws.send(JSON.stringify({ id: i, method, params, sessionId })); });
await send("Target.setDiscoverTargets", { discover: true });
await page.goto(url, { waitUntil: "load" });
const wait = (ms) => new Promise((r) => setTimeout(r, ms));
const bootT = Date.now();
const profiles = [];
for (const [start, len] of windows) {
  await wait(Math.max(0, start * 1000 - (Date.now() - bootT)));
  const workers = [...targets.values()].filter((t) => t.type === "worker" || t.type === "shared_worker" || t.type === "page");
  const sess = [];
  for (const w of workers) { const r = await send("Target.attachToTarget", { targetId: w.targetId, flatten: true }); const sessionId = r.result?.sessionId; if (!sessionId) continue; await send("Profiler.enable", {}, sessionId); await send("Profiler.setSamplingInterval", { interval: 1000 }, sessionId); await send("Profiler.start", {}, sessionId); sess.push({ sessionId, w }); }
  const s0 = Math.round((Date.now() - bootT)); await wait(len * 1000);
  for (const { sessionId, w } of sess) { const r = await send("Profiler.stop", {}, sessionId); if (r.result?.profile) { const f = join(out, `${tag}-profile-${start}-${w.targetId.slice(0, 6)}.json`); writeFileSync(f, JSON.stringify(r.result.profile)); profiles.push(`${f} start=${s0}ms len=${len}s url=${w.url.slice(-60)}`); } }
}
const rest = total - (Date.now() - bootT); if (rest > 0) await wait(rest);
const status = await page.evaluate(() => { const g = (id) => document.querySelector(`[data-surface-id="${id}"]`); const p = JSON.parse(g("window:procedural-preview")?.getAttribute("data-status-json") ?? "{}"); return `${p.phase} ${p.fault?.code ?? ""} units=${p.progress?.unitsDone}/${p.progress?.unitsTotal} cancellable=${p.cancellable ?? ""}`; });
const log = await page.evaluate(() => (globalThis.__el ?? []).join("\n"));
writeFileSync(join(out, `${tag}-console.txt`), consoleLines.join("\n"));
writeFileSync(join(out, `${tag}-crossings.txt`), `patched=${JSON.stringify(patched)}\nstatus=${status}\n${profiles.join("\n")}\n---\n${log}`);
console.log(`patched=${JSON.stringify(patched)} status=${status}\n${profiles.join("\n")}\nlines=${log.split("\n").length}`);
clearTimeout(cap); ws.close(); await browser.close();
