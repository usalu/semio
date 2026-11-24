/** 📏️ The local extents of shapes and the rest frame of their bone: `bun ext.ts <id|file.json> <scale> <bone-regex>` prints per shape of the matching bones in the three views the flattened extent of its geometry (paths and primitives included), its solid, and once per bone the rest origin and the axes u, v, w. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const { flattenGeometry } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [arg, scale, pattern] = process.argv.slice(2);
const asset = readDoc(arg!).doc.asset;
const frames = restFrames(asset.rig) as Record<string, { frame: { o: number[]; u: number[]; v: number[]; w: number[] } }>;
const f3 = (v: number[]): string => `(${v.map((x) => x.toFixed(3)).join(", ")})`;
for (const bone of asset.rig.bones) {
  if (!new RegExp(pattern!).test(bone.id)) continue;
  const f = frames[bone.id]!.frame;
  console.log(`${bone.id} len ${bone.lengthM} o ${f3(f.o)} u ${f3(f.u)} v ${f3(f.v)} w ${f3(f.w)}`);
  for (const view of ["elevation", "front", "plan"])
    for (const shape of asset.representations[scale!].geometry[view]) {
      if (shape.bone !== bone.id) continue;
      const points = flattenGeometry(shape).rings.flat();
      const xs = points.map((p: any) => p.x);
      const ys = points.map((p: any) => p.y);
      console.log(`  ${view.padEnd(9)} ${shape.id.padEnd(24)} ${shape.kind} x [${Math.min(...xs).toFixed(4)}, ${Math.max(...xs).toFixed(4)}] y [${Math.min(...ys).toFixed(4)}, ${Math.max(...ys).toFixed(4)}] ${JSON.stringify(shape.solid ?? "flat")}`);
    }
}
