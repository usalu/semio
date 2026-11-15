import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const [spec, scaleS, only] = process.argv.slice(2) as string[];
const { entry, asset } = entryOf(spec!);
const sc = Number(scaleS);
const m = merged(asset, loadSource(entry)!) as any;
const st = evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: sc } });
let solids = worldSolids(m, st.bones as never, sc).filter((w: any) => w.texture !== true);
console.log(solids.filter((w: any) => w.id.startsWith("lugs-r-l") || w.id === "tyre-r").map((w: any) => [w.id, w.positions.length / 3, Math.min(...Array.from({ length: w.positions.length / 3 }, (_, i) => w.positions[i * 3 + 1])).toFixed(3)]));
import { referenceOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
for (const sel of [["lugs-r-l-a"], ["tyre-r"], ["lugs-r-l-a", "tyre-r"]]) {
  const sub = solids.filter((w: any) => sel.includes(w.id));
  const r = referenceOf("elevation", sub, undefined);
  const box = { minX: -1.9, minY: -0.02, maxX: -1.5, maxY: 0.12 };
  const mk = new Mask(box, 0.01);
  for (const g of r.cut) mk.fill(g);
  console.log(sel.join("+"));
  for (let j = mk.height - 1; j >= 0; j--) { let l = ""; for (let i = 0; i < mk.width; i++) l += mk.cells[j * mk.width + i] ? "#" : "."; console.log(l); }
}
