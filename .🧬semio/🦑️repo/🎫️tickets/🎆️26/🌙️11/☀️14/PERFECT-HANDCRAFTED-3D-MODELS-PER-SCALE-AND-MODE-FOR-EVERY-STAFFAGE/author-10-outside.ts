import { entryOf, loadSource, merged, scaleMeshes, extentOf } from "./volumes-lib.ts";
const { entry, asset } = entryOf(process.argv[2]!); const scale = Number(process.argv[3] ?? 50); const list = (process.argv[4] ?? "thermal") as "thermal" | "structure";
const source = loadSource(entry)!; const a = merged(asset, source);
const sm = scaleMeshes(a, scale, source.volumes[String(scale)]);
const b = extentOf(sm.body)!; console.log("body", b.min.map((x) => +x.toFixed(3)), b.max.map((x) => +x.toFixed(3)));
for (const m of sm[list]) { const e = extentOf([m])!; const o = [0, 1, 2].map((k) => Math.max(b.min[k]! - e.min[k]!, e.max[k]! - b.max[k]!, 0)); if (Math.max(...o) > 0.004) console.log(m.id, o.map((x) => +x.toFixed(3)), e.min.map((x) => +x.toFixed(3)), e.max.map((x) => +x.toFixed(3))); }
