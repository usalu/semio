/** 🐐️ Author 41: 2D root repair for the goat: the elevation ears are drawn 93 mm tall while the front draws them as 25-40 mm blades (a horizontal leaf seen from the side), so the elevation ear outline is flattened to the front's thickness (centre kept).
 * Usage: bun author-41-goat2d.ts > 🗑️generated/author-41/goat2d-patch.json */
import { openCtx, drawn } from "./kit-lib.ts";
const out: any[] = [];
const r5 = (x: number) => Math.round(x * 1e5) / 1e5;
for (const [sc, ids] of [[50, ["ear-n", "ear-f"]], [100, ["ear-n", "ear-f"]], [200, ["ear"]]] as const) {
  const c = openCtx("animals/goat", sc);
  for (const id of ids) {
    const sh = drawn(c, "elevation").find((s) => s.id === id)!;
    const ring = sh.rings[0]!;
    const ys = ring.map((p) => p.y);
    const cy = (Math.min(...ys) + Math.max(...ys)) / 2;
    const f: any = c.state.frames[sh.bone]!;
    const pts = ring.map((p) => { const y = 0.908 + (p.y - cy) * 0.85; const dx = p.x - f.o[0], dy = y - f.o[1]; return [r5(dx * f.u[0] + dy * f.u[1]), r5(dx * f.v[0] + dy * f.v[1])]; });
    out.push({ scale: String(sc), view: "elevation", id, set: { points: pts } });
  }
}
import { hull, simplifyRing, type P } from "./author-41-geo.ts";
{
  const c200 = openCtx("animals/goat", 200);
  const c500 = openCtx("animals/goat", 500);
  const w = (id: string): P[] => drawn(c200, "elevation").find((s) => s.id === id)!.rings[0]!.map((p) => [p.x, p.y] as P);
  const loc = (bone: string, p: P): P => { const f: any = c500.state.frames[bone]; const dx = p[0] - f.o[0], dy = p[1] - f.o[1]; return [r5(dx * f.u[0] + dy * f.u[1]), r5(dx * f.v[0] + dy * f.v[1])]; };
  const set = (id: string, bone: string, pts: P[]) => out.push({ scale: "500", view: "elevation", id, set: { closed: true, filled: true, points: simplifyRing(hull(pts), 0.008).map((p) => loc(bone, p)) } });
  set("leg-hind-upper-n", "shank-hn", w("gaskin-n"));
  set("leg-hind-lower-n", "metatarsus-hn", [...w("cannon-h-n"), ...w("hoof-h-n")]);
  set("leg-fore-upper-n", "forearm-fn", w("forearm-n"));
  set("leg-fore-lower-n", "metacarpus-fn", [...w("cannon-f-n"), ...w("hoof-f-n")]);
}
console.log(JSON.stringify(out));
