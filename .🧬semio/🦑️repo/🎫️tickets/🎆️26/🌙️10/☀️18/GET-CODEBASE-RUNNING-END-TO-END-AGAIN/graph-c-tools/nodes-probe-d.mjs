// 🧪️ Runs the REAL emojiProjectJsonNodes (project-nodes) over a stride sample of real projects in a scratch cache and dumps a canonical, order-independent digest of everything it returns.
// env: N, MIX (cargo share), GP_SCRATCH, GP_VARIANT (ops.json), SEMIO_GRAPH_WORKERS, KEEP=1 (warm run on existing records), OUT=<prefix>, RSSMAX_MB, PASS=cold|warm
import { installVariant, installLedger, WORKSPACE, LIBRARY, SCRATCH } from "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/graph-parallel-tools/harness/variant.mjs";
installVariant();
import { existsSync, mkdirSync, rmSync, writeFileSync, readdirSync, statSync, readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join } from "node:path";

const env = process.env, N = Number(env.N ?? "30"), MIX = Number(env.MIX ?? "0.5"), RSSMAX = Number(env.RSSMAX_MB ?? "3800") * 1e6;
const strip = s => s.replace(/️/gu, "");
if (!env.KEEP) rmSync(SCRATCH, { recursive: true, force: true });
mkdirSync(SCRATCH, { recursive: true });

await installLedger();
const inference = await import(join(LIBRARY, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
await inference.libraryBootstrap;
const projectNodes = await import(join(LIBRARY, "🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"));
const observation = await import(join(LIBRARY, "🏗️build/👁️observation/🟨️.mjs"));

const git = JSON.parse(readFileSync(env.GP_FILES ?? "/private/tmp/graph-parallel/files.json", "utf8"));
const ignored = new Set(["🧫️fixtures", "🧫️examples", "🧪️fixtures", "🧪️examples"].map(strip));
const eligible = git.filter(f => strip(f.split("/").at(-1)) === strip("📋️project.json") && existsSync(join(WORKSPACE, f)) && (parts => parts[0] !== strip(".🧬semio") && !parts.slice(0, -1).some(x => ignored.has(x)))(f.split("/").map(strip))).sort();
const roots = new Set(eligible.map(f => dirname(f)));
const cargoOnly = git.filter(f => f.endsWith("/Cargo.toml") && !roots.has(dirname(f)) && !f.split("/").map(strip).some(x => x === strip(".🧬semio") || ignored.has(x) || x === "compose" || x === "temp") && existsSync(join(WORKSPACE, f))).sort();
const OFFSET = Number(env.OFFSET ?? "0");
const pick = (list, k) => { const step = Math.max(1, Math.floor(list.length / Math.max(k, 1))); return list.filter((_, i) => i % step === OFFSET % step).slice(0, k); };
const nCargo = Math.round(N * MIX), sample = [...pick(env.FILTER ? eligible.filter(f => new RegExp(env.FILTER, "u").test(f)) : eligible, N - nCargo), ...pick(cargoOnly, nCargo)];
const PROJECT_FILE = "📋️project.json";
const everything = git;
const allProjectFiles = everything.filter(f => f.endsWith(PROJECT_FILE) && existsSync(join(WORKSPACE, f)));
// dependency-closed universe: every project.json (cheap), plus the sampled crates, their native dependency closure and the owners of those roots; computed once per sample and kept
const universeFile = join(dirname(env.GP_FILES ?? "/private/tmp/graph-parallel/files.json"), `universe-${N}-${MIX}-${OFFSET}${env.FILTER ? "-f" : ""}.json`);
let configFiles;
if (existsSync(universeFile)) configFiles = JSON.parse(readFileSync(universeFile, "utf8"));
else {
  const { nativeDependencyRoots } = inference.cacheInternals;
  const universe = new Set(allProjectFiles), cargoFor = root => (root === "." ? "Cargo.toml" : root + "/Cargo.toml");
  const nativeOwner = new Map();
  for (const f of allProjectFiles) { try { const j = JSON.parse(readFileSync(join(WORKSPACE, f), "utf8")); if (j.metadata?.nativeRoot) nativeOwner.set(j.metadata.nativeRoot, f); } catch {} }
  const seeds = new Set();
  for (const f of sample) {
    const root = dirname(f) || ".";
    if (f.endsWith("Cargo.toml")) seeds.add(root);
    else { if (existsSync(join(WORKSPACE, cargoFor(root)))) seeds.add(root); try { const nr = JSON.parse(readFileSync(join(WORKSPACE, f), "utf8")).metadata?.nativeRoot; if (nr) seeds.add(nr); } catch {} }
  }
  const cache = new Map(), closures = new Map(), crates = new Set();
  for (const root of seeds) {
    crates.add(root);
    try { for (const d of nativeDependencyRoots(root, WORKSPACE, true, cache, closures)) crates.add(d); } catch (error) { if (error.code !== "NATIVE_INPUT_ADMISSION") throw error; }
  }
  for (const root of crates) { const c = cargoFor(root); if (existsSync(join(WORKSPACE, c))) universe.add(c); if (nativeOwner.has(root)) universe.add(nativeOwner.get(root)); }

  configFiles = [...universe].sort();
  writeFileSync(universeFile, JSON.stringify(configFiles));
}

const ownerRoots = new Set(sample.map(f => dirname(f) || "."));

const norm = text => text.split(WORKSPACE).join("<WS>"), loose = text => text.replace(/SEMIO_SOURCE_CLOSURE_[0-9a-f]{64}/gu, "SEMIO_SOURCE_CLOSURE_*").replace(/SEMIO_CARGO_LOCK_[0-9a-f]{64}/gu, "SEMIO_CARGO_LOCK_*");
const canon = value => norm(JSON.stringify(value, (_k, v) => (v && typeof v === "object" && !Array.isArray(v) ? Object.fromEntries(Object.keys(v).sort().map(k => [k, v[k]])) : v)));
const rowKey = r => norm(r.kind + "\0" + r.path + "\0" + JSON.stringify(r.fact));
const observed = {};
const registryText = readFileSync(join(WORKSPACE, "📋️build-owners.json"), "utf8"); const context = { workspaceRoot: WORKSPACE, registry: JSON.parse(registryText) };
const marks = [], markNames = [], cpuMarks = [], options = { ownerRoots, snapshot: true, onOwnerObservations: (name, rows) => { marks.push(performance.now()); markNames.push(name); cpuMarks.push(process.cpuUsage()); observed[name] = rows; } };
if (env.RAW) delete options.onOwnerObservations;

const peakRss = 0;
const stage = () => {};
{const {heapStats}=await import("bun:jsc");Bun.gc(true);const m=process.memoryUsage(),h=heapStats();console.log("MEM before run rss",Math.round(m.rss/1048576),"jscHeap",Math.round(h.heapSize/1048576),"strings",h.objectTypeCounts.string);}
const cpu0 = process.cpuUsage(), t0 = performance.now();
const { withImplementationPinV1 } = await import(join(LIBRARY, "🏗️build/📥️inputs/🔏️revision/🟨️.mjs"));
const results = env.UNPINNED ? await observation.observeInferenceSnapshotV1(() => projectNodes.emojiProjectJsonNodes(configFiles, options, context)) : await withImplementationPinV1(() => observation.observeInferenceSnapshotV1(() => projectNodes.emojiProjectJsonNodes(configFiles, options, context)));
const wall = (performance.now() - t0) / 1000, cpu = process.cpuUsage(cpu0);

const projects = {};
for (const [file, entry] of results) for (const [name, project] of Object.entries(entry.projects ?? {})) if (ownerRoots.has(project.root)) projects[name] = { file, project };
const names = Object.keys(projects).sort();
const projectDigest = createHash("sha256"), looseDigest = createHash("sha256");
for (const name of names) { const line = name + "\0" + canon(projects[name]) + "\n"; projectDigest.update(line); looseDigest.update(loose(line)); }
const rowDigest = createHash("sha256"), rowCounts = {};
for (const name of Object.keys(observed).sort()) {
  const keys = [...new Set(observed[name].map(rowKey))].sort();
  rowCounts[name] = keys.length;
  rowDigest.update(name + "\0" + createHash("sha256").update(keys.join("\n")).digest("hex") + "\n");
}
const unavailableRows = results.flatMap(([, e]) => e.unavailable ?? []), unavailable = unavailableRows.map(u => u.root).sort();
const unavailableDigest = createHash("sha256").update(canon(unavailableRows.map(u => [u.root, u.project ?? null, u.reason]).sort())).digest("hex").slice(0, 16);
const np = join(SCRATCH, "inference", "np");
let records = 0, recordBytes = 0, blocks = 0;
try { for (const f of readdirSync(np)) { if (f.endsWith(".json") && statSync(join(np, f)).isFile()) { records++; recordBytes += statSync(join(np, f)).size; } } } catch {}
try { for (const f of readdirSync(join(np, "blocks") ?? np)) blocks++; } catch {}
const firstS = (Math.min(...marks) - t0) / 1000, lastS = (Math.max(...marks) - t0) / 1000;
const cpuAt = i => { const c = cpuMarks[i]; return (c.user + c.system) / 1e6; }, inferenceCpu = cpuMarks.length > 1 ? cpuAt(cpuMarks.length - 1) - cpuAt(0) : 0;
const summary = { pass: env.PASS ?? "cold", inferenceCpuS: +inferenceCpu.toFixed(1), firstOwnerS: +firstS.toFixed(1), lastOwnerS: +lastS.toFixed(1), ownersPerMin: +(((marks.length - 1) / Math.max(lastS - firstS, 0.001)) * 60).toFixed(1), sample: sample.length, owners: names.length, workers: env.SEMIO_GRAPH_WORKERS ?? "default", wallS: +wall.toFixed(1), cpuS: +((cpu.user + cpu.system) / 1e6).toFixed(1), peakRssMB: Math.round(peakRss / 1e6), projectsSha: projectDigest.digest("hex").slice(0, 16), looseSha: looseDigest.digest("hex").slice(0, 16), rowsSha: rowDigest.digest("hex").slice(0, 16), rows: Object.values(rowCounts).reduce((a, b) => a + b, 0), unavailable: unavailable.length, unavailableSha: unavailableDigest, records, recordMB: Math.round(recordBytes / 1e5) / 10, stats: observation.sourceObservationStatsV1?.() };
if (globalThis.__gpAccounting) { globalThis.__gpAccounting.flush(true); summary.accounting = globalThis.__gpAccounting.facts(); }
if (globalThis.__gpLedgerStats) summary.ledger = { ...globalThis.__gpLedgerStats, wallMs: Math.round(globalThis.__gpLedgerStats.wallMs) };
const counts = observation.projectInferenceCountsV1?.() ?? {};
summary.computed = Object.values(counts).reduce((a, b) => a + b, 0);
summary.computedOwners = Object.keys(counts).length;
{const kinds={};for(const rows of Object.values(observed))for(const r of rows)kinds[r.kind]=(kinds[r.kind]??0)+1;console.log("KINDS "+JSON.stringify(kinds));}

{
 const {heapStats}=await import("bun:jsc");const mu=l=>{Bun.gc(true);const m=process.memoryUsage();const h=heapStats();console.log("MEM",l,"rss",Math.round(m.rss/1048576),"heapUsed",Math.round(m.heapUsed/1048576),"jscHeap",Math.round(h.heapSize/1048576),"extra",Math.round(h.extraMemorySize/1048576));return h;};
 mu("after run");
 results.length=0;for(const k of Object.keys(observed))delete observed[k];for(const k of Object.keys(projects))delete projects[k];marks.length=0;cpuMarks.length=0;mu("after clearing harness results");
 const script=await import(join(LIBRARY,"🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"));
 observation.releaseInferenceMemoryV1();mu("after releaseInferenceMemory");(await import(join(LIBRARY,"🏗️build/🧠️memory/🟨️.mjs"))).graphStringInternerV1.clear();mu("after interner clear");if(env.SNAP){const jsc=await import("bun:jsc");const snap=jsc.generateHeapSnapshotForDebugging();const text=typeof snap==="string"?snap:JSON.stringify(snap);writeFileSync(env.SNAP,text);console.log("SNAP written",text.length);}
 script.resetClosureEntriesV1();const h1=mu("after resetClosureEntries");
 const top=Object.entries(h1.objectTypeCounts).sort((a,b)=>b[1]-a[1]).slice(0,8);console.log("TYPES",JSON.stringify(top));
}

{const {heapStats}=await import("bun:jsc");for(const wait of [5000,10000,20000]){await new Promise(r=>setTimeout(r,wait));Bun.gc(true);const m=process.memoryUsage(),h=heapStats();console.log("MEM idle +"+wait+"ms rss",Math.round(m.rss/1048576),"jscHeap",Math.round(h.heapSize/1048576),"strings",h.objectTypeCounts.string);}}

{const {spawnSync}=await import("node:child_process");const jsc=await import("bun:jsc");
 const foot=l=>{const r=spawnSync("top",["-l","1","-pid",String(process.pid),"-stats","pid,mem,cmprs,rsize"]).stdout.toString().split("\n").filter(x=>x.startsWith(String(process.pid)))[0];console.log("FOOT",l,r?.replace(/\s+/g," "));};
 foot("after release+idle");
 Bun.gc(true);await new Promise(r=>setTimeout(r,2000));foot("after Bun.gc(true)+2s");
 jsc.gcAndSweep();await new Promise(r=>setTimeout(r,2000));foot("after gcAndSweep+2s");
 Bun.shrink();await new Promise(r=>setTimeout(r,2000));foot("after Bun.shrink+2s");
}

console.log("SUMMARY " + JSON.stringify(summary));
if (env.TIMES) writeFileSync(env.TIMES, JSON.stringify(markNames.map((n, i) => [n, +(((marks[i] - (i ? marks[i - 1] : t0)) / 1000).toFixed(3))])));
if (env.AUTH) { const cap = await observation.captureObservedSourceInputsV1(async () => 0, undefined, true); const byKind = {}; for (const r of cap.authorityObservations) byKind[r.kind] = (byKind[r.kind] ?? 0) + 1; console.log("AUTHORITY " + JSON.stringify(byKind)); writeFileSync(env.AUTH, JSON.stringify(cap.authorityObservations.map(r => [r.kind, r.path.replace(WORKSPACE, "<WS>")]), null, 0).replaceAll("],[", "],\n[")); }
if (env.OUT) {
  writeFileSync(env.OUT + ".projects.json", canon(Object.fromEntries(names.map(n => [n, projects[n]]))));
  if (env.ROWS_FULL) writeFileSync(env.OUT + ".rowkeys.json", JSON.stringify(Object.fromEntries(Object.keys(observed).sort().map(n => [n, [...new Set(observed[n].map(rowKey))].sort()]))));
  writeFileSync(env.OUT + ".rows.json", canon(Object.fromEntries(Object.keys(observed).sort().map(n => [n, [...new Set(observed[n].map(rowKey))].sort().map(k => createHash("sha256").update(k).digest("hex").slice(0, 12))]))));
}
if(globalThis.__accessSites&&env.SITES){const rows=[...globalThis.__accessSites].sort((a,b)=>b[1]-a[1]);writeFileSync(env.SITES,rows.map(r=>r[1]+" "+r[0]).join("\n"));}
if (!env.NOEXIT) process.exit(0);
