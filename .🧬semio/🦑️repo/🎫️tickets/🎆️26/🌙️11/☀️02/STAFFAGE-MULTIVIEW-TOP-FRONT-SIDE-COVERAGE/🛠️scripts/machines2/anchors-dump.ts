import { load } from "../machines/lib.ts";
import { ALL } from "./ids.ts";
for (const id of ALL) {
  const d = load(id);
  const a = d.asset;
  const an = a.anchors ?? a.rig?.anchors ?? [];
  console.log(`## ${id} anchors=${an.length} keys=${Object.keys(a).join(",")}`);
  for (const x of an) console.log(`  ${x.id} bone=${x.bone} x=${x.x} y=${x.y} z=${x.z ?? 0} ${x.kind??""}`);
}
