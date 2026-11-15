import { entryOf, poseState } from "./volumes-lib.ts";
const [spec, sc, view] = process.argv.slice(2);
const { asset } = entryOf(spec!);
const st = poseState(asset, Number(sc)) as any;
for (const s of st[view!]) {
  const pts = s.rings.flat();
  const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
  console.log(s.shape.id, s.shape.role, "closed", s.closed, "x", Math.min(...xs).toFixed(3), Math.max(...xs).toFixed(3), "y", Math.min(...ys).toFixed(3), Math.max(...ys).toFixed(3));
}
