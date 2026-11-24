import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🔬️ Profiles the WGPU frame Worker's CPU for a few seconds via raw CDP and prints the hottest self-time and inclusive functions. */
const out = join(import.meta.dir, "🗑️generated/wgpu-paint"); mkdirSync(out, { recursive: true });
const url = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const port = 9333;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal", `--remote-debugging-port=${port}`] });
const page = await (await browser.newContext({ viewport: { width: 1280, height: 800 } })).newPage();
await page.goto(url, { waitUntil: "load" });
await page.waitForTimeout(Number(process.env.SMOKE_MS ?? 28000));
const version = await (await fetch(`http://127.0.0.1:${port}/json/version`)).json();
const ws = new WebSocket(version.webSocketDebuggerUrl);
await new Promise(r => ws.addEventListener("open", r));
let id = 0; const pending = new Map();
ws.addEventListener("message", e => { const m = JSON.parse(e.data); if (m.id && pending.has(m.id)) { pending.get(m.id)(m); pending.delete(m.id); } });
const send = (method, params = {}, sessionId) => new Promise(r => { const i = ++id; pending.set(i, r); ws.send(JSON.stringify({ id: i, method, params, ...(sessionId ? { sessionId } : {}) })); });
const { result: { targetInfos } } = await send("Target.getTargets");
const workers = targetInfos.filter(t => t.type === "worker" && t.url.includes("frame"));
console.log("targets", targetInfos.map(t => `${t.type}:${t.url.slice(0, 90)}`));
for (const w of workers) {
  const { result: { sessionId } } = await send("Target.attachToTarget", { targetId: w.targetId, flatten: true });
  await send("Profiler.enable", {}, sessionId); await send("Profiler.setSamplingInterval", { interval: 200 }, sessionId); await send("Profiler.start", {}, sessionId);
  await new Promise(r => setTimeout(r, Number(process.env.PROFILE_MS ?? 5000)));
  const { result } = await send("Profiler.stop", {}, sessionId);
  const profile = result.profile; writeFileSync(join(out, "worker.cpuprofile"), JSON.stringify(profile));
  const nodes = new Map(profile.nodes.map(n => [n.id, n])); const self = new Map(); const parent = new Map();
  for (const n of profile.nodes) for (const c of n.children ?? []) parent.set(c, n.id);
  const dt = profile.timeDeltas; const incl = new Map(); const selfByName = new Map();
  profile.samples.forEach((s, i) => { const d = dt[i] ?? 0; const n = nodes.get(s); const name = n.callFrame.functionName || "(anon)"; selfByName.set(name, (selfByName.get(name) ?? 0) + d); const seen = new Set(); for (let c = s; c !== undefined; c = parent.get(c)) { const nm = nodes.get(c).callFrame.functionName || "(anon)"; if (!seen.has(nm)) { seen.add(nm); incl.set(nm, (incl.get(nm) ?? 0) + d); } } });
  const top = m => [...m].sort((a, b) => b[1] - a[1]).slice(0, 45).map(([k, v]) => `${(v / 1000).toFixed(0)}ms ${k}`).join("\n");
  console.log("SELF\n" + top(selfByName) + "\n\nINCLUSIVE\n" + top(incl));
}
await browser.close();
