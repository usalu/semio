/** 🔬️ Measures the product silhouette of the city bus at one scale with groups of body solids left out, to find which solids a view disagrees on: `bun r3-city-bus-probe.ts 50 [idPrefix,idPrefix...]`. */
import { measureAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";

const scale = Number(process.argv[2] ?? "50");
const drop = (process.argv[3] ?? "").split(",").filter((s) => s !== "");
const { entry, asset } = entryOf("vehicles/city-bus");
const m = merged(asset, loadSource(entry)!) as any;
const v = m.volumes[String(scale)];
v.body = v.body.filter((s: any) => !drop.some((p) => s.id.startsWith(p)));
const r = measureAsset(m, scale) as any;
for (const view of ["elevation", "front", "plan"]) console.log(view, "iou", r.views[view].iou.toFixed(4), "p95", r.views[view].p95M.toFixed(4), "max", r.views[view].hausdorffM.toFixed(3));
console.log("bbox", r.bbox.map((x: number) => x.toFixed(3)).join(" "));
