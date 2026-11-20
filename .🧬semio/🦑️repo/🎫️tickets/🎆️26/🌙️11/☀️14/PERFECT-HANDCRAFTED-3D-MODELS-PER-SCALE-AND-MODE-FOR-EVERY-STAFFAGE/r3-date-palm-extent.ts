/** 📦️ Prints the calm-pose bounding box of the date palm body per scale, with and without texture solids, and the solid that reaches each extreme. */
import { listsIn, load, rigAt } from "./r3-lib.ts";
const { asset } = load("vegetation/date-palm", "r3-date-palm-extent.ts");
for (const scale of [50, 100, 200, 500]) {
  const rig = rigAt(asset, scale, { kind: "pose", pose: "calm" });
  const sm = listsIn(asset, scale, rig.frames);
  for (const tex of [true, false]) {
    const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
    const who: string[] = ["", "", "", "", "", ""];
    sm.body.forEach((m, i) => {
      if (!tex && sm.solids.body[i]?.texture) return;
      for (let k = 0; k < m.world.length; k += 3) for (let a = 0; a < 3; a++) {
        const v = m.world[k + a]!;
        if (v < min[a]!) { min[a] = v; who[a] = m.id; }
        if (v > max[a]!) { max[a] = v; who[a + 3] = m.id; }
      }
    });
    console.log(`1:${scale} ${tex ? "all " : "noTx"} x ${min[0]!.toFixed(3)}..${max[0]!.toFixed(3)} (${(max[0]! - min[0]!).toFixed(3)}) y ${min[1]!.toFixed(3)}..${max[1]!.toFixed(3)} z ${min[2]!.toFixed(3)}..${max[2]!.toFixed(3)} (${(max[2]! - min[2]!).toFixed(3)})  ${who.join(" ")}`);
  }
}
