/** 🧪️ What each drawing solid can carry, measured on the body itself and without the engine: `bun capacity.ts <id|file.json> <scale> <bone> [--solids a,b] [--cell 0.001]` takes the body solids of the band on the bone (kernel triangles in the bone frame, `copyMesh`), projects them along seven directions (the three bone axes and four oblique ones, which is what a pose that twists or leans the bone shows) and compares the true outline with the outline of every candidate body on a raster: the tightest prism (`extruded`, best sweep), one prism per volume solid (what `MH-6/bodies.ts` draws), a circular and an elliptic body of revolution about the bone (`revolved`, `lateralScale`), prisms stacked along the bone (the workaround of `used-concert-harp`) and a loft between cross-sections cut from the body (`lofted`). Per candidate: shapes and vertices to author, the largest boundary distance in millimetres and the smallest IoU over the three axis views and over all seven.
 * @see ../MH-6/bodies.ts — the asset-side candidates */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { copyMesh } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));

type P2 = [number, number];
type P3 = [number, number, number];
type Body = { readonly name: string; readonly shapes: number; readonly vertices: number; readonly hulls: readonly (readonly P3[])[] };

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => {
  const i = args.indexOf(`--${name}`);
  return i < 0 ? undefined : args[i + 1];
};
const asset = readDoc(args[0]!).doc.asset;
const scale = args[1]!;
const bone = args[2]!;
const only = flag("solids")?.split(",");
const cell = Number(flag("cell") ?? "0.001");
const QUALITY = { chordM: 0.0005, minSegments: 16, maxSegments: 64 };
const solids: any[] = (asset.volumes[scale].body as any[]).filter((solid) => solid.bone === bone && (only === undefined || only.includes(solid.id)));
if (solids.length === 0) throw new Error(`no body solid of 1:${scale} on bone "${bone}"`);

const meshes = solids.map((solid) => copyMesh(solid, QUALITY) as { positions: Float64Array; indices: Uint32Array });
const pointsOf = (mesh: { positions: Float64Array }): P3[] => Array.from({ length: mesh.positions.length / 3 }, (_, i) => [mesh.positions[3 * i]!, mesh.positions[3 * i + 1]!, mesh.positions[3 * i + 2]!]);
const clouds = meshes.map(pointsOf);
const cloud = clouds.flat();
const triangles: P3[][] = meshes.flatMap((mesh, m) => Array.from({ length: mesh.indices.length / 3 }, (_, i) => [0, 1, 2].map((k) => clouds[m]![mesh.indices[3 * i + k]!]!)));

function hull(points: readonly P2[]): P2[] {
  const sorted = [...points].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  if (sorted.length < 3) return sorted;
  const turn = (o: P2, a: P2, b: P2): number => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const half = (list: readonly P2[]): P2[] => {
    const out: P2[] = [];
    for (const p of list) {
      while (out.length >= 2 && turn(out[out.length - 2]!, out[out.length - 1]!, p) <= 0) out.pop();
      out.push(p);
    }
    out.pop();
    return out;
  };
  return [...half(sorted), ...half([...sorted].reverse())];
}

function thin(ring: readonly P2[], most: number): P2[] {
  let tolerance = 0.0003;
  for (;;) {
    const out = [...ring];
    for (let changed = true; changed && out.length > 4; ) {
      changed = false;
      for (let i = 0; i < out.length && out.length > 4; i++) {
        const [a, b, c] = [out[(i + out.length - 1) % out.length]!, out[i]!, out[(i + 1) % out.length]!];
        const length = Math.hypot(c[0] - a[0], c[1] - a[1]) || 1;
        if (Math.abs((c[0] - a[0]) * (b[1] - a[1]) - (c[1] - a[1]) * (b[0] - a[0])) / length < tolerance) {
          out.splice(i, 1);
          changed = true;
        }
      }
    }
    if (out.length <= most) return out;
    tolerance *= 1.5;
  }
}

const AXES = { u: [0, 1, 2], v: [1, 0, 2], w: [2, 0, 1] } as const;
const lift = (sweep: "u" | "v" | "w", along: number, p: P2): P3 => (sweep === "u" ? [along, p[0], p[1]] : sweep === "v" ? [p[0], along, p[1]] : [p[0], p[1], along]);
const range = (values: readonly number[]): [number, number] => [values.reduce((a, b) => Math.min(a, b), Infinity), values.reduce((a, b) => Math.max(a, b), -Infinity)];
const area = (ring: readonly P2[]): number => Math.abs(ring.reduce((sum, p, i) => sum + p[0] * ring[(i + 1) % ring.length]![1] - ring[(i + 1) % ring.length]![0] * p[1], 0)) / 2;

function prism(points: readonly P3[]): { hull: P3[]; vertices: number } {
  const choices = (["u", "v", "w"] as const).map((sweep) => {
    const [k, a, b] = AXES[sweep];
    const ring = thin(hull(points.map((p) => [p[a]!, p[b]!] as P2)), 20);
    const [low, high] = range(points.map((p) => p[k]!));
    return { sweep, ring, low, high, volume: area(ring) * (high - low) };
  });
  const best = choices.reduce((a, b) => (b.volume < a.volume ? b : a));
  return { hull: [...best.ring.map((p) => lift(best.sweep, best.low, p)), ...best.ring.map((p) => lift(best.sweep, best.high, p))], vertices: best.ring.length };
}

const [uLow, uHigh] = range(cloud.map((p) => p[0]));

function stacked(n: number): Body {
  const step = (uHigh - uLow) / n;
  const hulls: P3[][] = [];
  let vertices = 0;
  for (let k = 0; k < n; k++) {
    const [from, to] = [uLow + k * step, uLow + (k + 1) * step];
    const ring = thin(hull(cloud.filter((p) => p[0] >= from - step / 4 && p[0] <= to + step / 4).map((p) => [p[1], p[2]] as P2)), 20);
    if (ring.length < 3) continue;
    vertices += ring.length;
    hulls.push([...ring.map((p) => lift("u", from, p)), ...ring.map((p) => lift("u", to, p))]);
  }
  return { name: `prisms stacked along u ×${n}`, shapes: hulls.length, vertices, hulls };
}

function sectionAt(u: number, from: readonly P3[][] = triangles): P2[] {
  const points: P2[] = [];
  for (const [a, b, c] of from)
    for (const [p, q] of [[a, b], [b, c], [c, a]] as const) {
      if ((p![0] - u) * (q![0] - u) > 0 || p![0] === q![0]) continue;
      const t = (u - p![0]) / (q![0] - p![0]);
      points.push([p![1] + t * (q![1] - p![1]), p![2] + t * (q![2] - p![2])]);
    }
  return points;
}

function loftOf(from: readonly P3[][], n: number, most: number): { hulls: P3[][]; vertices: number } {
  const [low, high] = range(from.flat().map((p) => p[0]));
  const inset = Math.min(0.001, (high - low) / (4 * n));
  const stations = Array.from({ length: n + 1 }, (_, k) => low + ((high - low) * k) / n);
  const rings = stations.map((u, k) => thin(hull(sectionAt(k === 0 ? u + inset : k === n ? u - inset : u, from)), most));
  const hulls: P3[][] = [];
  for (let k = 0; k < n; k++) if (rings[k]!.length >= 3 && rings[k + 1]!.length >= 3) hulls.push([...rings[k]!.map((p) => lift("u", stations[k]!, p)), ...rings[k + 1]!.map((p) => lift("u", stations[k + 1]!, p))]);
  return { hulls, vertices: rings.reduce((sum, ring) => sum + ring.length, 0) };
}

function lofted(n: number, most: number): Body {
  return { name: `one loft of ${n + 1} convex sections (≤${most} points)`, shapes: 1, ...loftOf(triangles, n, most) };
}

function loftedEach(n: number, most: number): Body {
  const parts = meshes.map((mesh, m) => loftOf(Array.from({ length: mesh.indices.length / 3 }, (_, i) => [0, 1, 2].map((k) => clouds[m]![mesh.indices[3 * i + k]!]!)), n, most));
  return { name: `one loft per volume solid, ${n + 1} sections (≤${most} points)`, shapes: parts.length, vertices: parts.reduce((sum, part) => sum + part.vertices, 0), hulls: parts.flatMap((part) => part.hulls) };
}

function revolved(elliptic: boolean): Body {
  const [vLow, vHigh] = range(cloud.map((p) => p[1]));
  const [wLow, wHigh] = range(cloud.map((p) => p[2]));
  const [v0, w0] = [(vLow + vHigh) / 2, (wLow + wHigh) / 2];
  const ratio = elliptic ? (wHigh - wLow) / (vHigh - vLow) : 1;
  const stations = 16;
  const step = (uHigh - uLow) / stations;
  const radius = Array.from({ length: stations + 1 }, (_, k) => {
    const u = uLow + k * step;
    const near = cloud.filter((p) => Math.abs(p[0] - u) <= step / 2);
    return near.reduce((r, p) => Math.max(r, elliptic ? Math.hypot(p[1] - v0, (p[2] - w0) / ratio) : Math.hypot(p[1] - v0, p[2] - w0)), 0);
  });
  const ring = (k: number): P3[] => Array.from({ length: 32 }, (_, i) => [uLow + k * step, v0 + radius[k]! * Math.cos((Math.PI * i) / 16), w0 + ratio * radius[k]! * Math.sin((Math.PI * i) / 16)] as P3);
  return { name: elliptic ? `revolved about u, lateralScale ${ratio.toFixed(2)}` : "revolved about u, circular", shapes: 1, vertices: stations + 1, hulls: Array.from({ length: stations }, (_, k) => [...ring(k), ...ring(k + 1)]) };
}

const single = prism(cloud);
const each = clouds.map(prism);
const bodies: Body[] = [
  { name: "one prism (extruded, best sweep)", shapes: 1, vertices: single.vertices, hulls: [single.hull] },
  { name: "one prism per volume solid (bodies.ts)", shapes: each.length, vertices: each.reduce((sum, p) => sum + p.vertices, 0), hulls: each.map((p) => p.hull) },
  revolved(false),
  revolved(true),
  stacked(4),
  stacked(8),
  lofted(4, 12),
  lofted(8, 12),
  lofted(16, 16),
  loftedEach(6, 12),
];

const r2 = Math.SQRT1_2;
const VIEWS: readonly { name: string; a: P3; b: P3 }[] = [
  { name: "along w", a: [1, 0, 0], b: [0, 1, 0] },
  { name: "along v", a: [1, 0, 0], b: [0, 0, 1] },
  { name: "along u", a: [0, 1, 0], b: [0, 0, 1] },
  { name: "twisted 45°", a: [1, 0, 0], b: [0, r2, -r2] },
  { name: "leaning 45° in w", a: [r2, 0, -r2], b: [0, 1, 0] },
  { name: "leaning 45° in v", a: [r2, -r2, 0], b: [0, 0, 1] },
  { name: "diagonal", a: [r2, -r2, 0], b: [1 / Math.sqrt(6), 1 / Math.sqrt(6), -2 / Math.sqrt(6)] },
];
const dot = (a: P3, b: P3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];

function compare(view: { a: P3; b: P3 }, body: Body): { mm: number; iou: number } {
  const flat = (p: P3): P2 => [dot(p, view.a), dot(p, view.b)];
  const truth = triangles.map((t) => t.map(flat));
  const drawn = body.hulls.map((points) => hull(points.map(flat)));
  const all = [...truth.flat(), ...drawn.flat()];
  const [x0, x1] = range(all.map((p) => p[0]));
  const [y0, y1] = range(all.map((p) => p[1]));
  const width = Math.ceil((x1 - x0) / cell) + 4;
  const height = Math.ceil((y1 - y0) / cell) + 4;
  const fill = (polygons: readonly (readonly P2[])[]): Uint8Array => {
    const mask = new Uint8Array(width * height);
    for (const polygon of polygons) {
      const [low, high] = range(polygon.map((p) => p[1]));
      for (let j = Math.max(0, Math.floor((low - y0) / cell) + 1); j <= Math.min(height - 1, Math.ceil((high - y0) / cell) + 2); j++) {
        const y = y0 + (j - 1.5) * cell;
        let [from, to] = [Infinity, -Infinity];
        for (let i = 0; i < polygon.length; i++) {
          const [p, q] = [polygon[i]!, polygon[(i + 1) % polygon.length]!];
          if ((p[1] - y) * (q[1] - y) > 0 || p[1] === q[1]) continue;
          const x = p[0] + ((y - p[1]) / (q[1] - p[1])) * (q[0] - p[0]);
          from = Math.min(from, x);
          to = Math.max(to, x);
        }
        if (!(from <= to)) continue;
        for (let i = Math.max(0, Math.round((from - x0) / cell + 1.5)); i <= Math.min(width - 1, Math.round((to - x0) / cell + 1.5) - 1); i++) mask[j * width + i] = 1;
      }
    }
    return mask;
  };
  const [a, b] = [fill(truth), fill(drawn)];
  let [both, either] = [0, 0];
  for (let k = 0; k < a.length; k++) {
    both += a[k]! & b[k]!;
    either += a[k]! | b[k]!;
  }
  const edge = (mask: Uint8Array): number[] => {
    const out: number[] = [];
    for (let j = 1; j < height - 1; j++) for (let i = 1; i < width - 1; i++) if (mask[j * width + i] === 1 && (mask[j * width + i - 1] === 0 || mask[j * width + i + 1] === 0 || mask[(j - 1) * width + i] === 0 || mask[(j + 1) * width + i] === 0)) out.push(i, j);
    return out;
  };
  const [ea, eb] = [edge(a), edge(b)];
  const far = (from: number[], to: number[]): number => {
    let worst = 0;
    for (let k = 0; k < from.length; k += 2) {
      let best = Infinity;
      for (let l = 0; l < to.length && best > worst; l += 2) best = Math.min(best, (from[k]! - to[l]!) ** 2 + (from[k + 1]! - to[l + 1]!) ** 2);
      worst = Math.max(worst, best);
    }
    return Math.sqrt(worst) * cell * 1000;
  };
  return { mm: eb.length === 0 ? Infinity : Math.max(far(ea, eb), far(eb, ea)), iou: either === 0 ? 0 : both / either };
}

const [vr, wr] = [range(cloud.map((p) => p[1])), range(cloud.map((p) => p[2]))];
console.log(`${asset.id} 1:${scale} bone ${bone}: ${solids.length} body solids (${solids.map((s) => s.kind).join(", ")}), ${triangles.length} triangles, u ${(1000 * (uHigh - uLow)).toFixed(0)} mm, v ${(1000 * (vr[1] - vr[0])).toFixed(0)} mm, w ${(1000 * (wr[1] - wr[0])).toFixed(0)} mm, cell ${cell * 1000} mm`);
console.log("| candidate | shapes per view | vertices | axis views: worst mm | axis views: least IoU | all seven: worst mm (view) | all seven: least IoU |");
console.log("|---|---|---|---|---|---|---|");
for (const body of bodies) {
  const results = VIEWS.map((view) => ({ name: view.name, ...compare(view, body) }));
  const axis = results.slice(0, 3);
  const worst = results.reduce((a, b) => (b.mm > a.mm ? b : a));
  console.log(`| ${body.name} | ${body.shapes} | ${body.vertices} | ${Math.max(...axis.map((r) => r.mm)).toFixed(1)} | ${Math.min(...axis.map((r) => r.iou)).toFixed(3)} | ${worst.mm.toFixed(1)} (${worst.name}) | ${Math.min(...results.map((r) => r.iou)).toFixed(3)} |`);
}
