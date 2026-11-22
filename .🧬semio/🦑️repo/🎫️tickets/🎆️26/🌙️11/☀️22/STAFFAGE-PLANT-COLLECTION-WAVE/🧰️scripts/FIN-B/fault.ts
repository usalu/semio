/** 🩻️ Finisher diagnosis of the polyhedra of a candidate that the kernel refuses (`polyhedronFault`): the mesh integrity numbers on the normalised vertices as the kernel reads them, the closest pair of distinct vertices, and with `--pairs` the first triangles that cut each other after a face is taken out one by one.
 *
 * `repo tools exec bun -- fault.ts <asset document json> [--scale N] [--id <solid id>] [--pairs]`
 *
 * @see ../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/✅️validation/🪨️polyhedron/🟦️.ts */
import { readFileSync } from "node:fs";
import { polyhedronFault } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { meshIntegrity } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🩻️integrity/🟦️.ts";

type V3 = [number, number, number];
const args = process.argv.slice(2);
const flag = (name: string) => (args.includes(name) ? args[args.indexOf(name) + 1] : undefined);
const asset = JSON.parse(readFileSync(args[0]!, "utf8")).asset;
let bad = 0;
for (const [scale, v] of Object.entries(asset.volumes as Record<string, { body: { id: string; kind: string; vertices: V3[]; triangles: V3[] }[] }>)) {
  if (flag("--scale") && flag("--scale") !== scale) continue;
  for (const s of v.body) {
    if (s.kind !== "polyhedron" || (flag("--id") && flag("--id") !== s.id)) continue;
    const fault = polyhedronFault(s.vertices, s.triangles);
    if (fault === null) continue;
    bad++;
    const low = [0, 1, 2].map((k) => Math.min(...s.vertices.map((p) => p[k]!)));
    const span = Math.max(...[0, 1, 2].map((k) => Math.max(...s.vertices.map((p) => p[k]!)) - low[k]!));
    const points = s.vertices.map((p) => p.map((x, k) => (x - low[k]!) / span) as V3);
    const m = meshIntegrity(points.flat(), s.triangles.flat(), { toleranceM: 1e-11 });
    let near = Infinity, pair = [0, 0];
    for (let i = 0; i < s.vertices.length; i++) for (let j = i + 1; j < s.vertices.length; j++) {
      const d = Math.hypot(s.vertices[i]![0] - s.vertices[j]![0], s.vertices[i]![1] - s.vertices[j]![1], s.vertices[i]![2] - s.vertices[j]![2]);
      if (d < near) [near, pair] = [d, [i, j]];
    }
    console.log(`[DEBUG] ${asset.id} 1:${scale} ${s.id}: ${fault}; vertices ${s.vertices.length} welded ${m.vertices} triangles ${s.triangles.length} degenerate ${m.degenerate} closed ${m.closed} manifold ${m.manifold} oriented ${m.oriented} components ${m.components} volume ${m.signedVolume.toExponential(2)}; nearest vertices ${pair} ${(near * 1000).toFixed(4)} mm`);
    if (args.includes("--pairs")) for (const line of touching(s.vertices, s.triangles).slice(0, 8)) console.log(`[DEBUG]   ${line}`);
  }
}
console.log(`[DEBUG] ${asset.id}: ${bad} faulty polyhedra`);

function touching(points: V3[], faces: V3[]): string[] {
  const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
  const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
  const dot = (a: V3, b: V3) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
  const normal = (f: V3) => cross(sub(points[f[1]]!, points[f[0]]!), sub(points[f[2]]!, points[f[0]]!));
  const show = (f: V3) => JSON.stringify(f.map((i) => [i, ...points[i]!.map((x) => +x.toFixed(4))]));
  const out: string[] = [];
  const pierces = (a: V3, b: V3, shared: number): boolean => {
    const [p, q] = a.filter((i) => i !== shared).map((i) => points[i]!) as [V3, V3];
    const n = normal(b), o = points[shared]!, tri = b.map((i) => points[i]!);
    const [h, k] = [dot(sub(p, o), n), dot(sub(q, o), n)];
    if (h * k > 0 || Math.abs(h - k) < 1e-20) return false;
    const t = h / (h - k);
    const x: V3 = [p[0] + (q[0] - p[0]) * t, p[1] + (q[1] - p[1]) * t, p[2] + (q[2] - p[2]) * t];
    return tri.every((y, i) => dot(cross(sub(tri[(i + 1) % 3]!, y), sub(x, y)), n) >= -1e-10 * dot(n, n));
  };
  for (let i = 0; i < faces.length; i++) for (let j = i + 1; j < faces.length; j++) {
    const [a, b] = [faces[i]!, faces[j]!];
    const shared = a.filter((x) => b.includes(x));
    const [na, nb] = [normal(a), normal(b)];
    const coplanar = Math.hypot(...cross(na, nb)) <= 1e-10 * Math.hypot(...na) * Math.hypot(...nb) && Math.abs(dot(na, sub(points[b[0]]!, points[a[0]]!))) <= 1e-10 * Math.hypot(...na);
    if (shared.length === 2 && coplanar && dot(na, nb) < 0) out.push(`folded pair ${i},${j} ${show(a)} ${show(b)}`);
    if (shared.length === 1 && (coplanar || pierces(a, b, shared[0]!) || pierces(b, a, shared[0]!))) out.push(`${coplanar ? "coplanar" : "piercing"} pair at shared vertex ${i},${j} ${show(a)} ${show(b)}`);
    if (shared.length === 0 && cuts(points, [a, b])) out.push(`cutting pair ${i},${j} ${show(a)} ${show(b)}`);
  }
  return out;
}

function cuts(points: V3[], faces: V3[]): boolean {
  const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
  const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
  const dot = (a: V3, b: V3) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
  const hit = (a: V3, b: V3, tri: V3[]): boolean => {
    const n = cross(sub(tri[1]!, tri[0]!), sub(tri[2]!, tri[0]!));
    const [h, k] = [dot(sub(a, tri[0]!), n), dot(sub(b, tri[0]!), n)];
    if (h * k >= 0) return false;
    const t = h / (h - k);
    const p: V3 = [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];
    return tri.every((x, i) => dot(cross(sub(tri[(i + 1) % 3]!, x), sub(p, x)), n) > 1e-14 * dot(n, n));
  };
  for (let i = 0; i < faces.length; i++) for (let j = i + 1; j < faces.length; j++) {
    const [a, b] = [faces[i]!, faces[j]!];
    if (a.some((x) => b.includes(x))) continue;
    const [pa, pb] = [a.map((x) => points[x]!), b.map((x) => points[x]!)];
    for (let e = 0; e < 3; e++) if (hit(pa[e]!, pa[(e + 1) % 3]!, pb) || hit(pb[e]!, pb[(e + 1) % 3]!, pa)) return true;
  }
  return false;
}
