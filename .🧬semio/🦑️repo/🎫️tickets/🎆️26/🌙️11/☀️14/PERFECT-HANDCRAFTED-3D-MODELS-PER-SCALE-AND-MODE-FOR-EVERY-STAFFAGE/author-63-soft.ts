/** bun author-63-soft.ts <asset> <scale> : the soft-mass boundary cells farther than 40 mm (world coords per view) and the soft solids */
import { entryOf, loadSource, merged, featureM } from "./volumes-lib.ts";
import { measureAsset } from "./🗑️generated/author-63/sil-copy.ts";
const [spec, sc] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m: any = merged(asset, loadSource(entry)!);
const f = featureM(Number(sc), m);
const softR = new Set(m.thermal.regions.filter((r: any) => ["foliage", "fur", "feather", "hair"].includes(r.kind)).map((r: any) => r.id));
const soft = new Set<string>(m.volumes[sc!].thermal.flatMap((s: any) => (s.of !== undefined && softR.has(s.region) ? [s.of] : [])));
console.log("soft solids", [...soft].join(","));
const dbgs: any[] = [];
Object.defineProperty(globalThis, "__dbg", { set(v) { dbgs.push(v); }, get() { return dbgs[dbgs.length - 1]; }, configurable: true });
const far: any[] = [];
(globalThis as any).__softfar = far;
const r: any = measureAsset(m, Number(sc), m.presentation.defaultPose, undefined, f, soft);
console.log("p95Soft", ["elevation", "front", "plan"].map((v) => `${v} ${r.views[v].p95SoftM?.toFixed(4)} p95 ${r.views[v].p95M?.toFixed(4)}`).join(" | "), "far soft cells", far.length);
const byView = new Map<string, any[]>();
for (const f of far) { const a = byView.get(f[0]) ?? []; a.push(f); byView.set(f[0], a); }
for (const [v, a] of byView) { const xs = a.map((q) => q[1]), ys = a.map((q) => q[2]), ds = a.map((q) => q[3]); console.log(v, a.length, "x", Math.min(...xs), Math.max(...xs), "y", Math.min(...ys), Math.max(...ys), "d", Math.min(...ds), Math.max(...ds)); }
