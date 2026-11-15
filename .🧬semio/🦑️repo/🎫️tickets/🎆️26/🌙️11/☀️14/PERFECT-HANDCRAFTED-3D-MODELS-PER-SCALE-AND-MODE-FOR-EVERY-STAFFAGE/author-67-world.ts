import { Kit } from "./author-02-lib.ts";
import { toWorld } from "./volumes-lib.ts";
const k = new Kit(process.argv[2]!, Number(process.argv[3]));
const ids = process.argv[4]!.split(",");
for (const s of k.body.length ? k.body : (JSON.parse(require("fs").readFileSync(`volumes/${process.argv[2]}.json`, "utf8")).volumes[process.argv[3]].body)) {
  if (!ids.includes(s.id) || s.kind !== "loft") continue;
  console.log(s.id, s.bone, s.sections.map((x: any) => { const w = toWorld(k.fr[s.bone]!, x.at); return `(${w.map((v: number) => v.toFixed(2)).join(",")} r${x.rx.toFixed(2)}/${x.ry.toFixed(2)})`; }).join(" "));
}
