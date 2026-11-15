import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const P = "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const { measureAsset } = (await import(P)) as any;
const [spec, sc, view = "elevation", drop = ""] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const src = loadSource(entry)!;
const m = merged(asset, src);
const vol = m.volumes[sc!];
const softRegions = new Set<string>(m.thermal.regions.filter((r: any) => ["foliage", "fur", "feather", "hair"].includes(r.kind)).map((r: any) => r.id));
const soft = new Set<string>(vol.thermal.filter((s: any) => s.of && softRegions.has(s.region)).map((s: any) => s.of));
if (drop) {
  const rep = JSON.parse(JSON.stringify(m.representations[sc!]));
  for (const v of ["elevation", "front", "plan"]) rep.geometry[v] = rep.geometry[v].filter((s: any) => !(drop.split(",").includes(s.role) && (s.filled === false || s.kind === "polyline" || s.closed === false || (s.kind === "path" && !/Z\s*$/i.test(s.d ?? "")))));
  m.representations = { ...m.representations, [sc!]: rep };
}
const r = measureAsset(m, Number(sc), m.presentation.defaultPose, undefined, 0.15e-3 * Number(sc), process.env.NOSOFT ? new Set() : soft);
const v = r.views[view];
const h: number[] = v.spread.hard;
console.log("p95", v.p95M, "soft p95", v.p95SoftM, "iou", v.iou.toFixed(3), "hard total", h.reduce((a: number, b: number) => a + b, 0), "far bin", h[h.length - 1], "cum<=8:", (h.slice(0, 9).reduce((a: number, b: number) => a + b, 0) / h.reduce((a: number, b: number) => a + b, 0)).toFixed(3));
