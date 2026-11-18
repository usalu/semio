/** 🔍 Evaluated drawn shapes (world rings of the view) whose bounding box contains a point. bun humansa-shapes-at.ts <asset> <scale> <view> <x> <y> */
import { applyOps, loadAsset, stateOf, type Viewpoint } from "./humansa-lib.ts";
const [name, sc, view, xs, ys] = process.argv.slice(2) as [string, string, Viewpoint, string, string];
const asset = await loadAsset(name);
applyOps(asset, process.env.OPS);
const [x, y] = [Number(xs), Number(ys)];
for (const s of (stateOf(asset, Number(sc)).shapes as any)[view]) {
  const pts = s.rings.flat();
  if (pts.length === 0) continue;
  const b = [Math.min(...pts.map((p: any) => p.x)), Math.max(...pts.map((p: any) => p.x)), Math.min(...pts.map((p: any) => p.y)), Math.max(...pts.map((p: any) => p.y))];
  if (x >= b[0]! && x <= b[1]! && y >= b[2]! && y <= b[3]!) console.log(`${s.shape.id} role=${s.shape.role} bone=${s.shape.bone} closed=${s.closed} filled=${s.shape.filled} z=${s.shape.z} bbox x ${b[0]!.toFixed(3)}..${b[1]!.toFixed(3)} y ${b[2]!.toFixed(3)}..${b[3]!.toFixed(3)}`);
}
