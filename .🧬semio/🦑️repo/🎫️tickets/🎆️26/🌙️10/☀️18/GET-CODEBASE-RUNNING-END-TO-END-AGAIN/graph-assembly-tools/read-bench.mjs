// 📏 Reader benchmark over a generation store (read-only): assemble + expand + validate, with RSS/heap per stage. env: GEN (store root), STAGES=assemble,expand,validate,stringify, SNAPSHOT=1 prints JSON.
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
const W = "/Users/ueli/Documents/semio", LIB = join(W, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const GEN = process.env.GEN ?? join(W, ".cache/b/g/m"), STAGES = new Set((process.env.STAGES ?? "assemble,expand,validate").split(","));
const gen = await import(join(LIB, "🏗️build/🕸️graph/🕰️generations/🟨️.mjs"));
const load = await import(join(LIB, "🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs"));
const { validateJsonSchemaSubset } = await import(join(W, "🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts"));
const { graphStringInternerV1 } = await import(join(LIB, "🏗️build/🧠️memory/🟨️.mjs"));
const mb = n => Math.round(n / 1e6);
const t0 = performance.now(), marks = [];
let peak = 0;
const sampler = setInterval(() => { peak = Math.max(peak, process.memoryUsage.rss()); }, 200);
const mark = (label, extra = {}) => { const m = process.memoryUsage(); peak = Math.max(peak, m.rss); marks.push({ label, s: +((performance.now() - t0) / 1000).toFixed(1), cpuS: +(process.cpuUsage().user / 1e6 + process.cpuUsage().system / 1e6).toFixed(1), rssMB: mb(m.rss), heapMB: mb(m.heapUsed), extMB: mb(m.external), ...extra }); console.error(JSON.stringify(marks.at(-1))); };

const engine = gen.createGenerationStoreV1({ root: join(GEN, "generations"), holds: async () => true, resources: { facts: "bench" } });
const current = await engine.read();
mark("read", { generation: current.pointer.generation, owners: Object.keys(current.manifest.owners).length });

let ticks = 0;
const progress = async () => { ticks++; };
let assembled, streamed;
if (process.env.STREAM === "1") { streamed = engine.assembleStream(current, { compare: (a, b) => a.localeCompare(b), progress }); assembled = { projects: {}, dependencies: [], externalNodes: {}, unavailable: [], membership: current.manifest.membership }; }
else { assembled = await engine.assemble(current, { compare: (a, b) => a.localeCompare(b), progress }); mark("assemble", { projects: Object.keys(assembled.projects).length, edges: assembled.dependencies.length, ticks }); }

let projects = assembled.projects;
if (STAGES.has("expand")) {
  const cache = new Map();
  const closures = (dir, key) => JSON.parse(readFileSync(join(GEN, dir, key + ".json"), "utf8"));
  const token = item => { if (!item || typeof item !== "object" || Array.isArray(item)) return undefined; const o = item; if (Object.keys(o).length !== 1 || typeof o.env !== "string") return undefined; const m = /^SEMIO_(SOURCE_CLOSURE|CARGO_LOCK)_([a-f0-9]{64})$/u.exec(o.env); return m ? { key: o.env, digest: m[2], directory: m[1] === "SOURCE_CLOSURE" ? "build-input-closures" : "build-lock-closures" } : undefined; };
  const expand = value => {
    const pending = [value], seen = new WeakSet(), tokens = new Map();
    while (pending.length) { const item = pending.pop(); if (!item || typeof item !== "object" || seen.has(item)) continue; seen.add(item); const c = token(item); if (c) tokens.set(c.key, c); else for (const child of Object.values(item)) if (child && typeof child === "object") pending.push(child); }
    if (!tokens.size) return value;
    const expanded = new Map();
    for (const c of tokens.values()) { if (process.env.CACHE === "1" && cache.has(c.key)) { expanded.set(c.key, cache.get(c.key)); continue; } const v = closures(c.directory, c.digest); const list = c.directory === "build-input-closures" ? v.files.map(f => graphStringInternerV1.share("{workspaceRoot}/" + f)) : ["{workspaceRoot}/Cargo.lock", "{workspaceRoot}/" + v.manifest]; expanded.set(c.key, list); if (process.env.CACHE === "1") cache.set(c.key, list); }
    const copies = new WeakMap();
    const transform = item => {
      if (!item || typeof item !== "object") return item;
      const c = token(item); if (c) return expanded.get(c.key);
      if (copies.has(item)) return copies.get(item);
      if (Array.isArray(item)) { let result; for (let i = 0; i < item.length; i++) { const child = item[i], next = transform(child), flatten = token(child) !== undefined; if (!result && (flatten || next !== child)) result = item.slice(0, i); if (result) { if (flatten) for (const e of next) result.push(e); else result.push(next); } } copies.set(item, result ?? item); return result ?? item; }
      let result; for (const [k, child] of Object.entries(item)) { const next = transform(child); if (next !== child && !result) result = { ...item }; if (result) result[k] = next; }
      copies.set(item, result ?? item); return result ?? item;
    };
    return transform(value);
  };
  const limits = { maximumProjects: 32, maximumBytes: 16777216 };
  if (streamed) { projects = await load.expandStreamV1(streamed.nodes(), async chunk => expand(chunk), limits); Object.assign(assembled, await streamed.finish()); mark("assemble+expand(stream)", { projects: Object.keys(projects).length, edges: assembled.dependencies.length }); }
  else projects = await load.expandInChunksV1(assembled.projects, async chunk => expand(chunk), limits);
  mark("expand", { projects: Object.keys(projects).length });
}
if (STAGES.has("validate")) {
  const schema = JSON.parse(readFileSync(join(LIB, "🏗️build/🧬️schema/🔣️.json"), "utf8"));
  const value = { version: 1, projects, dependencies: assembled.dependencies, namedInputs: {}, externalNodes: assembled.externalNodes, ...(assembled.unavailable.length ? { unavailable: assembled.unavailable } : {}), admission: { selection: { kind: "all" }, registry: "0".repeat(64), closure: assembled.membership } };
  const errors = validateJsonSchemaSubset(schema, value);
  mark("validate", { errors: errors.length });
}
if (STAGES.has("stringify")) { let total = 0, biggest = "", t1 = performance.now(); const parts = []; for (const [n, v] of Object.entries(projects)) { const x = JSON.stringify(v); total += x.length; if (parts.length < 400) parts.push(x); } mark("stringify(per project)", { jsonMB: mb(total), ms: Math.round(performance.now() - t1) });
  const text = parts.join("\n"); 
  const { gzipSync, gunzipSync, brotliCompressSync, brotliDecompressSync, constants } = await import("node:zlib");
  let t = performance.now(); const gz = gzipSync(text, { level: 1 }); const gzMs = Math.round(performance.now() - t); t = performance.now(); const back = gunzipSync(gz).toString("utf8"); const gunMs = Math.round(performance.now() - t); t = performance.now(); JSON.parse(back); const parseMs = Math.round(performance.now() - t);
  mark("cachedChunkCost(400 projects)", { gzipMB: mb(gz.length), gzipMs: gzMs, gunzipMs: gunMs, parseMs }); }
clearInterval(sampler);
console.log(JSON.stringify({ peakRssMB: mb(peak), marks }));
