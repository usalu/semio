/** ✂️ The rings of the evaluated default pose of assets at every authored scale, simplified to the paper tolerance of the depiction (0.01 mm) by the pass before (`old-simplify.ts`) and after the change of the extremes: simplify-compare.ts <asset id>… prints per asset the rings compared and the rings that differ. */
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { simplify as after } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/📐️geometry/🟦️.ts";
import { simplify as before } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/📐️geometry/old-simplify.ts";
const root = process.env.S + "/🗂️catalog";
let total = 0, changed = 0;
for (const id of process.argv.slice(2)) {
  const file = readdirSync(root).map((c) => join(root, c, id, "🔣️.json")).find((f) => { try { readFileSync(f); return true; } catch { return false; } })!;
  const asset = JSON.parse(readFileSync(file, "utf8")).asset;
  let compared = 0, differ = 0;
  for (const scale of [50, 100, 200, 500]) {
    if ((asset.representations[String(scale)] as any)?.status !== "authored") continue;
    const k = 1000 / scale;
    let state;
    try { state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }); } catch { continue; }
    for (const view of ["elevation", "front", "plan"] as const) for (const e of state.shapes[view]) for (const ring of e.rings) {
      const paper = ring.map((p: any) => ({ x: p.x * k, y: -p.y * k }));
      compared++;
      const a = before(paper, e.closed, 0.01), b = after(paper, e.closed, 0.01);
      if (a.length !== b.length || a.some((p, i) => p !== b[i])) differ++;
    }
  }
  total += compared; changed += differ;
  console.log(`${id}: rings ${compared} differ ${differ}`);
}
console.log(`total rings ${total} differ ${changed}`);
