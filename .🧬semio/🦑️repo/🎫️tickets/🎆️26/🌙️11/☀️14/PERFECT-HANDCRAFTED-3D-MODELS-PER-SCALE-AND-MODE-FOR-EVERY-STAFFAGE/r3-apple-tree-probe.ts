/** 🔎️ Prints the paper extents of every drawn shape of the candidate apple tree at one scale and view (rest pose). */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, restState } from "./volumes-lib.ts";
const asset = JSON.parse(readFileSync(join(GENERATED, "r3-apple-tree", "🔣️.json"), "utf8")).asset;
const [scale, view] = [Number(process.argv[2] ?? 500), process.argv[3] ?? "elevation"];
for (const e of (restState(asset, scale) as any)[view]) {
  const pts = e.rings.flat();
  const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
  console.log(e.shape.id.padEnd(22), e.shape.bone.padEnd(12), "x", Math.min(...xs).toFixed(3), Math.max(...xs).toFixed(3), "y", Math.min(...ys).toFixed(3), Math.max(...ys).toFixed(3), "rings", e.rings.length, "pts", pts.length);
}
