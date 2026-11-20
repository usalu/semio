/** 🔁️ Round 3 English oak: the drawn side against the model: every vertex of the filled 2D shapes of the woody bones that lies outside the projected body meshes, with its distance to the nearest mesh vertex of the same bone. Usage: bun r3-english-oak-reverse.ts <scale> [front|elevation] */
import { entryOf, poseState, scaleMeshes } from "./volumes-lib.ts";
const scale = Number(process.argv[2] ?? 50);
const view = (process.argv[3] ?? "front") as "front" | "elevation";
const { asset } = entryOf("vegetation/english-oak");
const sm = scaleMeshes(asset, scale, asset.volumes[String(scale)]);
const st = poseState(asset, scale);
type Tri = [number, number, number, number, number, number];
const tris: Tri[] = [];
const verts = new Map<string, [number, number][]>();
sm.body.forEach((mesh: any, i: number) => {
  const s = sm.solids.body[i]!;
  if (s.texture === true || s.slender === true) return;
  const p = (k: number): [number, number] => [view === "front" ? mesh.world[3 * k + 2]! : mesh.world[3 * k]!, mesh.world[3 * k + 1]!];
  const list = verts.get(s.bone) ?? verts.set(s.bone, []).get(s.bone)!;
  for (let k = 0; k < mesh.world.length / 3; k++) list.push(p(k));
  const idx = mesh.indices ?? mesh.triangles;
  for (let k = 0; k < idx.length; k += 3) tris.push([...p(idx[k]), ...p(idx[k + 1]), ...p(idx[k + 2])] as Tri);
});
const inside = (x: number, y: number): boolean => tris.some(([ax, ay, bx, by, cx, cy]) => {
  const d1 = (x - bx) * (ay - by) - (ax - bx) * (y - by);
  const d2 = (x - cx) * (by - cy) - (bx - cx) * (y - cy);
  const d3 = (x - ax) * (cy - ay) - (cx - ax) * (y - ay);
  return !((d1 < 0 || d2 < 0 || d3 < 0) && (d1 > 0 || d2 > 0 || d3 > 0));
});
const off = Number(process.argv[4] ?? 0.02);
const all = (st[view] as any[]).filter((e) => e.shape.role === "body" && e.shape.filled && e.closed);
const inRing = (r: any[], x: number, y: number): boolean => {
  let c = false;
  for (let i = 0, j = r.length - 1; i < r.length; j = i++) if (r[i].y > y !== r[j].y > y && x < ((r[j].x - r[i].x) * (y - r[i].y)) / (r[j].y - r[i].y) + r[i].x) c = !c;
  return c;
};
const drawn = (x: number, y: number): boolean => all.some((e) => e.rings.some((r: any[]) => inRing(r, x, y)));
for (const e of all) {
  if (String(e.shape.bone).startsWith("fol-") || e.shape.id === "crown") continue;
  const bad: string[] = [];
  let n = 0;
  for (const r of e.rings) for (let i = 0; i < r.length; i++) {
    const [a, b] = [r[i], r[(i + 1) % r.length]];
    const len = Math.hypot(b.x - a.x, b.y - a.y);
    if (len < 1e-6) continue;
    const [nx, ny] = [(b.y - a.y) / len, -(b.x - a.x) / len];
    for (let t = 0.05; t < len; t += 0.05) {
      const [x, y] = [a.x + ((b.x - a.x) * t) / len, a.y + ((b.y - a.y) * t) / len];
      for (const sgn of [1, -1]) {
        const [qx, qy] = [x + sgn * off * nx, y + sgn * off * ny];
        const [d, m] = [drawn(qx, qy), inside(qx, qy)];
        n++;
        if (d !== m) bad.push(`${d ? "drawn-only" : "model-only"} ${qx.toFixed(2)},${qy.toFixed(2)}`);
      }
    }
  }
  console.log(String(e.shape.id).padEnd(18), `${bad.length}/${n}`, bad.filter((_, i) => i % Math.ceil(bad.length / 6) === 0).join(" | "));
}
