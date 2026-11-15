import { Kit } from "./author-02-lib.ts";
const k = new Kit(process.argv[2]!, Number(process.argv[3]));
for (const b of k.asset.rig.bones) console.log(b.id, b.parent, "o", k.origin(b.id).map((x: number) => x.toFixed(2)).join(","), "t", k.tip(b.id).map((x: number) => x.toFixed(2)).join(","), "len", b.lengthM);
