import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { bandVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const [spec, coarseS, fineS] = process.argv.slice(2) as string[];
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!) as any;
const W = (s: number) => { const st = evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: s } }); return worldSolids(m, st.bones as never, s, "body").filter((w: any) => w.slender !== true && w.texture !== true); };
const [c, f] = [W(Number(coarseS)), W(Number(fineS))];
const feat = 0.15e-3 * Number(coarseS);
const hull = new Set(c.filter((w: any) => w.envelope === true).map((w: any) => w.bone ?? ""));
const fi = f.filter((w: any) => !hull.has(w.bone ?? "")), co = c.filter((w: any) => w.envelope !== true);
const bones = new Set([...fi, ...co].map((w: any) => w.bone ?? ""));
let tf = 0, tc = 0, te = 0, td = 0, bad = 0;
for (const b of bones) {
  const F = fi.filter((w: any) => (w.bone ?? "") === b), C = co.filter((w: any) => (w.bone ?? "") === b);
  const r = bandVolumes(F, C, feat, 120_000_000);
  const ok = r.coarse >= r.eroded - 1e-9 && r.coarse <= r.dilated + 1e-9;
  if (!ok) bad++;
  tf += r.fine; tc += r.coarse; te += r.eroded; td += r.dilated;
  console.log(b.padEnd(14), "fine", r.fine.toFixed(3), "eroded", r.eroded.toFixed(3), "dilated", r.dilated.toFixed(3), "coarse", r.coarse.toFixed(3), "cell", r.cellM.toFixed(4), ok ? "inside" : "OUTSIDE");
}
console.log(`1:${coarseS} vs 1:${fineS}: fine ${tf.toFixed(3)} band [${te.toFixed(3)}, ${td.toFixed(3)}] coarse ${tc.toFixed(3)} (feature ${feat}) bones outside: ${bad}`, tc >= te && tc <= td ? "-> total inside the band" : "-> total OUTSIDE");
