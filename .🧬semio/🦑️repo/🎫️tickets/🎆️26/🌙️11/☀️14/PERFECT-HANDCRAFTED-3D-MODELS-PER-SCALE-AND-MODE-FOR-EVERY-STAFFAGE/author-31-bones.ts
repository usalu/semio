import { Kit } from "./author-02-lib.ts";
const k = new Kit(process.argv[2]!, 50);
const f = (v: number[]) => v.map((x) => x.toFixed(2)).join(",");
for (const b of k.asset.rig.bones) console.log(b.id, "o", f(k.origin(b.id)), "tip", f(k.tip(b.id)), "L", b.lengthM);
