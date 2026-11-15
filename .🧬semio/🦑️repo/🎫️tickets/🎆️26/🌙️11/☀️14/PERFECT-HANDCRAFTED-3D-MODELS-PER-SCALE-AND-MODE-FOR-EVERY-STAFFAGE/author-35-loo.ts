import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { measureView, worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const [spec, scaleS, view] = process.argv.slice(2) as [string, string, string];
const prefixes = process.argv.slice(5);
const { entry, asset } = entryOf(spec);
const sc = Number(scaleS);
const src = loadSource(entry)! as any;
const run = (drop: string | null) => {
  const s = JSON.parse(JSON.stringify(src));
  if (drop) s.volumes[String(sc)].body = s.volumes[String(sc)].body.filter((x: any) => !x.id.startsWith(drop));
  const m = merged(asset, s) as any;
  const st = evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: sc } });
  const solids = worldSolids(m, st.bones as never, sc).filter((w: any) => w.texture !== true);
  return measureView(m, st, solids, view as any, sc, undefined, 0.15e-3 * sc) as any;
};
const b = run(null);
console.log("base iou", b.iou.toFixed(4), "p95", b.p95M.toFixed(4), JSON.stringify(b.counts));
for (const p of prefixes) { const r = run(p); console.log(p.padEnd(22), "iou", r.iou.toFixed(4), "p95", r.p95M.toFixed(4), "vol", r.counts.volume - b.counts.volume, "both", r.counts.both - b.counts.both, "drawn", r.counts.drawn - b.counts.drawn); }
