/** 🧪️ Probe: the silhouette guard measures of an asset with some bones' shapes and solids left out (e.g. a prop), to see who causes a difference. bun probe-guard.ts <file> <scale> [bone,bone] */
import { measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
const [f, sc, drop] = process.argv.slice(2);
const a = (await Bun.file(f!).json()).asset;
const bones = new Set((drop ?? "").split(",").filter(Boolean));
if (bones.size) {
  for (const rep of Object.values(a.representations) as any[]) if (rep.status === "authored") for (const v of ["elevation", "front", "plan"]) rep.geometry[v] = rep.geometry[v].filter((s: any) => !bones.has(s.bone));
  for (const vol of Object.values(a.volumes) as any[]) if (vol.status === "authored") { const ids = new Set(vol.body.filter((s: any) => bones.has(s.bone)).map((s: any) => s.id)); vol.body = vol.body.filter((s: any) => !ids.has(s.id)); vol.thermal = vol.thermal.filter((s: any) => !ids.has(s.of) && !bones.has(s.bone)); vol.structure = vol.structure.filter((s: any) => !ids.has(s.of) && !bones.has(s.bone)); }
}
const m = measureAsset(a, Number(sc), undefined, undefined, featureOf(Number(sc)));
for (const v of ["elevation", "front", "plan"] as const) { const x: any = m.views[v]; console.log(v, "iou", x.iou.toFixed(3), "h", (x.hausdorffM * 1000).toFixed(0), "p95", (x.p95M * 1000).toFixed(0), "recall", x.recall.toFixed(3), "precision", x.precision.toFixed(3)); }
