import { entryOf, poseState } from "./volumes-lib.ts";
const { asset } = entryOf(process.argv[2]!);
const d = Number(process.argv[3]);
const views = (process.argv[4] ?? "elevation,front,plan").split(",");
const st = poseState(asset, d);
const rep: any = asset.representations[String(d)];
console.log("pens", JSON.stringify(rep.pens));
for (const view of views) {
  console.log("##", view);
  for (const s of (st as any)[view] as any[]) {
    const pts = s.rings.flat();
    if (!pts.length) continue;
    const ys = pts.map((p: any) => p.y), xs = pts.map((p: any) => p.x);
    const stroke = !s.closed || s.shape.filled === false;
    const f = (n: number) => n.toFixed(3);
    console.log(String(s.shape.id).padEnd(18), String(s.shape.bone).padEnd(9), String(s.shape.role).slice(0, 3), s.closed ? "C" : "o", s.shape.filled ? "F" : "-", s.shape.kind.padEnd(7), "x", f(Math.min(...xs)), f(Math.max(...xs)), "y", f(Math.min(...ys)), f(Math.max(...ys)), "rings", s.rings.length, "pen", s.shape.penMm ?? rep.pens[s.shape.role] ?? "", stroke ? "STROKE" : "", s.shape.region ?? "");
  }
}
