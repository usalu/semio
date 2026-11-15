/** 🔬️ Probe (author 42): p95 of the product silhouette of the bollard 1:50 elevation with drawn shapes removed: bun author-42-probe.ts <asset> <scale> <view> [id,id|unfilled|none] */
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const sil = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts");
const [spec, sc, view, drop] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m: any = JSON.parse(JSON.stringify(merged(asset, loadSource(entry)!)));
const rep = m.representations[sc!];
const list = rep.geometry[view!] as any[];
const gone = new Set((drop ?? "").split(","));
rep.geometry[view!] = list.filter((s) => !(gone.has(s.id) || (gone.has("unfilled") && s.filled === false) || (gone.has("detail") && s.role === "detail") || (gone.has("texture") && s.role === "texture")));
const r = sil.measureAsset(m, Number(sc), m.presentation.defaultPose, undefined, 0.00015 * Number(sc));
const v = (r.views as any)[view!];
console.log(spec, sc, view, "drop", drop, "iou", v.iou.toFixed(4), "p95M", v.p95M, "hausdorffM", v.hausdorffM);
