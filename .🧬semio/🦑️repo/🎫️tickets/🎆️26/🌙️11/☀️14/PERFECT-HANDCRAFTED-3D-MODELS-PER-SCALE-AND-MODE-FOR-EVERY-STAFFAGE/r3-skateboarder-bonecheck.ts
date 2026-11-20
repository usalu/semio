/** 🩻 Round 3 skateboarder: per bone and view the agreement of the drawn 2D shapes with the projected body solids of the candidate in the default pose (IoU, area drawn only, area 3D only), to find the parts that pull the silhouette down.
 *
 * Usage: `bun r3-skateboarder-bonecheck.ts <candidate json> --scale N --view elevation|front|plan` */
import { loadAsset, meshesAt, poseState, projectWorld } from "./volumes-lib.ts";
import { cli } from "./r3-lib.ts";

const { spec, flag } = cli(["--scale", "--view"]);
const asset = loadAsset(spec);
const scale = Number(flag("--scale"));
const view = (flag("--view") ?? "plan") as "elevation" | "front" | "plan";
const st = poseState(asset, scale);
if (st.degraded !== undefined) process.stdout.write(`[DEBUG] degraded: ${st.degraded}\n`);
const body = asset.volumes[String(scale)].body.filter((s: { slender?: boolean; texture?: boolean }) => !s.slender && !s.texture);
const cell = 0.004;
const key = (x: number, y: number): string => `${Math.round(x / cell)},${Math.round(y / cell)}`;
const bones = [...new Set(body.map((s: { bone: string }) => s.bone))] as string[];
const inside = (p: [number, number], ring: { x: number; y: number }[]): number => {
  let wn = 0;
  for (let i = 0; i < ring.length; i++) {
    const a = ring[i]!, b = ring[(i + 1) % ring.length]!;
    if (a.y <= p[1]) { if (b.y > p[1] && (b.x - a.x) * (p[1] - a.y) - (p[0] - a.x) * (b.y - a.y) > 0) wn++; }
    else if (b.y <= p[1] && (b.x - a.x) * (p[1] - a.y) - (p[0] - a.x) * (b.y - a.y) < 0) wn--;
  }
  return wn;
};
const allD = new Set<string>(), allT = new Set<string>();
for (const bone of [...bones, "ALL"]) {
  if (bone === "ALL") {
    let b = 0;
    for (const k of allD) if (allT.has(k)) b++;
    process.stdout.write(`ALL          iou ${(b / (allD.size + allT.size - b)).toFixed(3)}  drawn-only ${((allD.size - b) * cell * cell * 1e4).toFixed(0)} cm2  3d-only ${((allT.size - b) * cell * cell * 1e4).toFixed(0)} cm2\n`);
    break;
  }
  const meshes = meshesAt(asset, scale, body.filter((s: { bone: string }) => s.bone === bone));
  const three = new Set<string>();
  for (const m of meshes) for (let i = 0; i < m.indices.length; i += 3) {
    const t = [0, 1, 2].map((k) => { const j = m.indices[i + k]! * 3; const q = projectWorld(view as never, [m.world[j]!, m.world[j + 1]!, m.world[j + 2]!]); return [q.x, q.y] as [number, number]; });
    const xs = t.map((p) => p[0]), ys = t.map((p) => p[1]);
    for (let x = Math.min(...xs); x <= Math.max(...xs); x += cell) for (let y = Math.min(...ys); y <= Math.max(...ys); y += cell) {
      const d = (t[1]![0] - t[0]![0]) * (t[2]![1] - t[0]![1]) - (t[2]![0] - t[0]![0]) * (t[1]![1] - t[0]![1]);
      if (Math.abs(d) < 1e-14) continue;
      const a = ((t[1]![0] - x) * (t[2]![1] - y) - (t[2]![0] - x) * (t[1]![1] - y)) / d, b = ((t[2]![0] - x) * (t[0]![1] - y) - (t[0]![0] - x) * (t[2]![1] - y)) / d;
      if (a >= -1e-9 && b >= -1e-9 && a + b <= 1 + 1e-9) three.add(key(x, y));
    }
  }
  const shapes = (st[view] as { shape: { bone: string; filled?: boolean; role: string }; rings: { x: number; y: number }[][] }[]).filter((e) => e.shape.bone === bone && e.shape.filled !== false && e.shape.role !== "ground");
  const drawn = new Set<string>();
  for (const e of shapes) for (const ring of e.rings) {
    const xs = ring.map((p) => p.x), ys = ring.map((p) => p.y);
    for (let x = Math.min(...xs); x <= Math.max(...xs); x += cell) for (let y = Math.min(...ys); y <= Math.max(...ys); y += cell) if (inside([x, y], ring) !== 0) drawn.add(key(x, y));
  }
  for (const k of drawn) allD.add(k);
  for (const k of three) allT.add(k);
  let both = 0;
  for (const k of drawn) if (three.has(k)) both++;
  const union = drawn.size + three.size - both;
  process.stdout.write(`${bone.padEnd(12)} iou ${(both / Math.max(union, 1)).toFixed(3)}  drawn-only ${((drawn.size - both) * cell * cell * 1e4).toFixed(0).padStart(4)} cm2  3d-only ${((three.size - both) * cell * cell * 1e4).toFixed(0).padStart(4)} cm2  (3d ${(three.size * cell * cell * 1e4).toFixed(0)} cm2)\n`);
}
