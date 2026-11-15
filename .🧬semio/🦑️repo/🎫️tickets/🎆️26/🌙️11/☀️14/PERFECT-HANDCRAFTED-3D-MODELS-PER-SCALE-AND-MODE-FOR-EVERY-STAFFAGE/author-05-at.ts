import { Mask, entryOf, poseState } from "./volumes-lib.ts";
const [spec, scaleS, view, xs, ys] = process.argv.slice(2);
const { asset } = entryOf(spec!);
const st = poseState(asset, Number(scaleS));
const [x, y] = [Number(xs), Number(ys)];
for (const e of (st as any)[view!]) {
  const m = new Mask({ minX: x - 0.01, minY: y - 0.01, maxX: x + 0.01, maxY: y + 0.01 }, 0.005);
  m.fill(e.rings);
  if (m.cells.some((c: number) => c)) console.log(e.shape.id, e.shape.bone, e.shape.kind, e.shape.role);
}
