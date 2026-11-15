/** 🐦️ Author 41: 2D root repair for the herring gull: the elevation leg strokes of 1:100 and 1:200 end at different heights (near 0.01, far -0.004, below the ground line), so both are lifted to half a pen above the ground. Usage: bun author-41-gull2d.ts > 🗑️generated/author-41/gull2d-patch.json */
import { openCtx } from "./kit-lib.ts";
const out: any[] = [];
const r5 = (x: number) => Math.round(x * 1e5) / 1e5;
for (const [sc, pen] of [[100, 0.01], [200, 0.016]] as const) {
  const c = openCtx("animals/herring-gull", sc);
  for (const id of ["near-leg", "far-leg"]) {
    const sh = (c.state.elevation as any[]).find((q) => q.shape.id === id)!;
    const ring = (sh.rings[0] as { x: number; y: number }[]).map((p) => [p.x, p.y] as [number, number]);
    const f: any = c.state.frames[sh.shape.bone]!;
    const pts = ring.map(([x, y], i) => { const yy = i >= 2 ? pen : y; const dx = x - f.o[0], dy = yy - f.o[1]; return [r5(dx * f.u[0] + dy * f.u[1]), r5(dx * f.v[0] + dy * f.v[1])]; });
    out.push({ scale: String(sc), view: "elevation", id, set: { points: pts } });
  }
}
console.log(JSON.stringify(out));
