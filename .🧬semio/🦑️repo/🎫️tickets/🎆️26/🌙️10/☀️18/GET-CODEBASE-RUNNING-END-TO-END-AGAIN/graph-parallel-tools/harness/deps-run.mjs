// 🧪️ Runs the REAL collectImportEdges (dependency-edges) over the source files of a stride sample of real projects in a scratch record store and dumps a canonical digest of the edges and of the published module facts.
// env: N (projects), MIX, GP_WORKSPACE, GP_SCRATCH, SEMIO_GRAPH_WORKERS, OUT, MAX_FILES
import { installVariant, WORKSPACE, LIBRARY, SCRATCH } from "./variant.mjs";
installVariant();
import { existsSync, mkdirSync, rmSync, writeFileSync, readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join } from "node:path";

const env = process.env, N = Number(env.N ?? "30"), MAX = Number(env.MAX_FILES ?? "100000"), OFFSET_N = Number(env.OFFSET_N ?? "0");
const strip = s => s.replace(/️/gu, "");
if (!env.KEEP) rmSync(SCRATCH, { recursive: true, force: true });
mkdirSync(SCRATCH, { recursive: true });

const inference = await import(join(LIBRARY, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
await inference.libraryBootstrap;
const edgesModule = await import(join(LIBRARY, "🏗️build/🕸️graph/📥️inference/🕸️dependency-edges/🟨️.mjs"));
const observation = await import(join(LIBRARY, "🏗️build/👁️observation/🟨️.mjs"));
const files = JSON.parse(readFileSync(env.GP_FILES ?? "/private/tmp/graph-parallel/files.json", "utf8"));
const PROJECT = strip("📋️project.json");
const projectFiles = files.filter(f => strip(f.split("/").at(-1)) === PROJECT && !f.startsWith(".🧬semio/")).sort();
const TS = /\.[cm]?[jt]sx?$/u, tsFiles = files.filter(f => TS.test(f) && !f.includes("/node_modules/") && !f.startsWith(".🧬semio/"));
const rootsOf = projectFiles.map(f => dirname(f)), counts = rootsOf.map(root => tsFiles.reduce((n, f) => n + (f.startsWith(root + "/") ? 1 : 0), 0));
const sample = projectFiles.map((f, i) => [f, counts[i]]).sort((x, y) => y[1] - x[1] || (x[0] < y[0] ? -1 : 1)).slice(OFFSET_N, OFFSET_N + N).map(row => row[0]);
const roots = sample.map(f => dirname(f)), all = new Set(projectFiles.map(f => dirname(f)));
const projects = {}, map = {};
let total = 0;
for (const config of sample) {
  const root = dirname(config), name = JSON.parse(readFileSync(join(WORKSPACE, config), "utf8")).name ?? root;
  projects[name] = { name, root };
  map[name] = files.filter(f => f.startsWith(root + "/") && /\.[cm]?[jt]sx?$/u.test(f) && !f.includes("/node_modules/")).filter(() => total++ < MAX).map(file => ({ file }));
}
const edges = new Set(), publication = {};
const add = (source, target, file) => edges.add(source + "\0" + target + "\0" + file);
const scheduled = undefined;
const t0 = performance.now(), cpu0 = process.cpuUsage();
let peak = 0; const poll = setInterval(() => { peak = Math.max(peak, process.memoryUsage().rss); }, 250);
await observation.observeDependencies({ receiptPath: join(SCRATCH, "inference", "deps.receipt.json"), recordDirectory: join(SCRATCH, "mr") }, () => edgesModule.collectImportEdges(WORKSPACE, map, projects, new Map(), undefined, add, publication, scheduled, {}));
clearInterval(poll);
const wall = (performance.now() - t0) / 1000, cpu = process.cpuUsage(cpu0);
const canon = v => JSON.stringify(v, (_k, x) => (x && typeof x === "object" && !Array.isArray(x) ? Object.fromEntries(Object.keys(x).sort().map(k => [k, x[k]])) : x)).split(WORKSPACE).join("<WS>");
const sortedEdges = [...edges].sort();
console.log("SUMMARY " + JSON.stringify({ projects: sample.length, files: total, workers: env.SEMIO_GRAPH_WORKERS ?? "default", wallS: +wall.toFixed(1), cpuS: +((cpu.user + cpu.system) / 1e6).toFixed(1), peakRssMB: Math.round(peak / 1e6), edges: sortedEdges.length, edgesSha: createHash("sha256").update(sortedEdges.join("\n").split(WORKSPACE).join("<WS>")).digest("hex").slice(0, 16), publicationSha: createHash("sha256").update(canon(publication)).digest("hex").slice(0, 16), stats: observation.sourceObservationStatsV1?.() }));
process.exit(0);
