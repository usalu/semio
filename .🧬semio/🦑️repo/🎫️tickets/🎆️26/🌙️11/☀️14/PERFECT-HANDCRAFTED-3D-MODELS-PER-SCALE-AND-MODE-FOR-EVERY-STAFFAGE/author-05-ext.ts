import { entryOf, poseState } from "./volumes-lib.ts";
const [spec] = process.argv.slice(2);
const { asset } = entryOf(spec!);
for (const sc of [50, 100, 200, 500]) {
  if (asset.representations[String(sc)].status !== "authored") continue;
  const st = poseState(asset, sc);
  const r: string[] = [];
  for (const v of ["elevation", "front", "plan"] as const) {
    let x0 = 1e9, x1 = -1e9, y0 = 1e9, y1 = -1e9;
    for (const e of (st as any)[v]) if (e.closed && e.shape.filled !== false && ["body", "detail", "glazing"].includes(e.shape.role)) for (const p of e.rings.flat()) { x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); }
    r.push(`${v} x ${x0.toFixed(3)}..${x1.toFixed(3)} y ${y0.toFixed(3)}..${y1.toFixed(3)}`);
  }
  console.log(sc, r.join(" | "));
}
console.log(JSON.stringify(asset.dimensions));
