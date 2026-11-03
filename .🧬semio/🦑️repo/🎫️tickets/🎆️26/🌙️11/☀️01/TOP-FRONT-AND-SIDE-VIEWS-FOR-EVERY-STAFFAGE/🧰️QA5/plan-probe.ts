import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const asset = JSON.parse(readFileSync("C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/tour-guide/🔣️.json", "utf8")).asset;
const s: any = evaluate(asset, { kind: "pose", pose: "waving" }, { kind: "scale", scale: { denominator: 50 } } as any);
for (const v of ["plan", "front"] as const) for (const e of s.shapes[v]) if (/arm|hand|pointer/.test(e.shape.id)) {
  const pts = e.rings.flat();
  console.log("[DEBUG]", v, e.shape.id, e.shape.bone, "x", Math.min(...pts.map((p: any) => p.x)).toFixed(3), Math.max(...pts.map((p: any) => p.x)).toFixed(3), "y", Math.min(...pts.map((p: any) => p.y)).toFixed(3), Math.max(...pts.map((p: any) => p.y)).toFixed(3));
}
for (const b of ["upper-arm-r", "forearm-r", "hand-r"]) console.log("[DEBUG] bone", b, JSON.stringify(s.bones[b]));
