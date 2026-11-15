import { entryOf, loadSource, merged } from "./volumes-lib.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { exposedArea } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts";
const [spec, sc, region] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m: any = merged(asset, loadSource(entry)!);
const st = evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } } as any);
const W: Record<string, any> = {};
for (const l of ["body", "thermal", "structure"] as const) W[l] = worldSolids(m, st.bones as any, Number(sc), l);
const against = [...W.body, ...W.thermal, ...W.structure];
const vol = m.volumes[sc!];
for (const t of vol.thermal) {
  if (t.region !== region) continue;
  const w = W.thermal.find((x: any) => x.id === t.id);
  console.log(t.id.padEnd(24), "total", exposedArea([w], [], [], () => false).toFixed(4), "exposed", exposedArea([w], against, [], (o) => o === w).toFixed(4));
}
if (process.argv[5] === "who") {
  for (const t of vol.thermal) {
    if (t.region !== region) continue;
    const w = W.thermal.find((x: any) => x.id === t.id);
    for (const o of against) { if (o === w) continue; const a = exposedArea([w], [o], [], () => false); console.log(t.id, "vs", o.id, "exposed", a.toFixed(5)); }
  }
}
