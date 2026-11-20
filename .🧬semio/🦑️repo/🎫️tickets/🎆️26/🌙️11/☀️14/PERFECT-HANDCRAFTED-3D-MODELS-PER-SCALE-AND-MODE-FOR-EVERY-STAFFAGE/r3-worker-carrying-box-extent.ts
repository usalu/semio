/** 📏️ Prints, per view of one scale of the round-3 worker candidate, the evaluated shapes that reach highest and furthest (debug aid for the extent rules). Usage: `bun r3-worker-carrying-box-extent.ts <scale>` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GENERATED } from "./volumes-lib.ts";

const asset = JSON.parse(readFileSync(join(GENERATED, "r3-worker-carrying-box", "🔣️.json"), "utf8")).asset;
const scale = process.argv[2] ?? "50";
const state = evaluate(asset as never, { kind: "pose", pose: asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: Number(scale) } } as never) as any;
for (const v of ["elevation", "front", "plan"]) {
  const rows = state.shapes[v].map((e: any) => {
    const pts = e.rings.flat();
    return { id: e.shape.id, maxY: Math.max(...pts.map((p: any) => p.y)), minY: Math.min(...pts.map((p: any) => p.y)), minX: Math.min(...pts.map((p: any) => p.x)), maxX: Math.max(...pts.map((p: any) => p.x)) };
  });
  for (const k of ["maxY", "maxX"] as const) console.log(v, k, rows.sort((a: any, b: any) => b[k] - a[k]).slice(0, 3).map((r: any) => `${r.id} ${r[k].toFixed(3)}`).join(", "));
  for (const k of ["minY", "minX"] as const) console.log(v, k, rows.sort((a: any, b: any) => a[k] - b[k]).slice(0, 3).map((r: any) => `${r.id} ${r[k].toFixed(3)}`).join(", "));
}
