/** 🔬️ W14 probe of the per-owner assembly authority of a selected read (no farm: every owner runs it in the read's main thread): N Cargo owners of the registry (evenly spaced), shared native caches as the wave keeps them, once with a validation round per owner (today) and once inside one pass round; prints synchronous fs calls, distinct paths and the top callers per mode. Env: WAVE (session.bin), N, MODE=owner|pass. */
import { readFileSync } from "node:fs"; import { dirname, join } from "node:path"; import { deserialize } from "node:v8";
const R = "/Users/ueli/Documents/semio", B = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/", INF = B + "🕸️graph/📥️inference/";
const inference = await import(INF + "🟨️.mjs"); await inference.libraryBootstrap;
const od = await import(INF + "🔎️owner-discovery/🟨️.mjs"), obs = await import(B + "👁️observation/🟨️.mjs");
const session = deserialize(readFileSync(process.env.WAVE + "/session.bin")), all = session.registry.owners.filter(o => String(o.manifest ?? "").endsWith("Cargo.toml"));
const N = Number(process.env.N ?? 40), step = Math.max(1, Math.floor(all.length / N)), owners = all.filter((_, i) => i % step === 0).slice(0, N);
const C = globalThis.__w14, snap = () => Object.fromEntries(Object.entries(C.calls)), reset = () => { C.calls = {}; C.callers = {}; C.paths = {}; };
const cache = new Map(), closures = new Map(), byRoot = new Map(all.map(o => [dirname(o.manifest), o.name]));
const run = () => { const per = []; for (const o of owners) { const before = C.calls.statSync ?? 0, t = performance.now(); try { od.projectAssemblyAuthorityV1(R, dirname(o.manifest), byRoot, undefined, cache, closures); } catch (e) { per.push({ name: o.name, error: String(e.message).slice(0, 120) }); continue; } per.push({ name: o.name, stat: (C.calls.statSync ?? 0) - before, ms: Math.round(performance.now() - t) }); } return per; };
reset(); const cold = obs.withObservationPassRoundV1(new Map(), run); const coldCalls = snap();
for (const mode of ["owner", "pass"]) {
  reset(); const t = performance.now(); const per = mode === "pass" ? obs.withObservationPassRoundV1(new Map(), run) : run();
  const top = Object.entries(C.callers).sort((a, b) => b[1] - a[1]).slice(0, 12);
  console.log(JSON.stringify({ mode, owners: owners.length, wallMs: Math.round(performance.now() - t), calls: C.calls, distinct: Object.fromEntries(Object.entries(C.paths).map(([k, v]) => [k, v.size])), statPerOwner: per.slice(0, 6).concat(per.slice(-3)).map(r => r.stat), errors: per.filter(r => r.error).length, top }, null, 1));
}
console.log(JSON.stringify({ cold: coldCalls }));
