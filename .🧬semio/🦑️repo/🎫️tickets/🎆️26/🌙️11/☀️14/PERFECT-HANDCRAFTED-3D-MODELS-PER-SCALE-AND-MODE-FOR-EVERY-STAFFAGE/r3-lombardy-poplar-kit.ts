/** 🎋️ Small world-space kit of the round-3 Lombardy poplar on top of the Scots pine kit: flame tufts (pointed lofts), vertical core lofts with elliptic sections and their bone-local solids and orthographic outlines.
 * Every form is stated in `r3-lombardy-poplar-model.ts`; this file only converts and projects.
 * @see ./round3-brief.md
 * @see ./r3-scots-pine-kit.ts */
import { dirToLocal, toLocal, type Frame, type Json, type P2, type V3 } from "./volumes-lib.ts";
import { add, mul, project, quaternion, r5, unit, type Tube, type View } from "./r3-scots-pine-kit.ts";

export * from "./r3-scots-pine-kit.ts";

/** 🔥️ Radius profile of a flame tuft along its axis: blunt base, fullest just below the middle, drawn to a point. */
export const FLAME: [number, number][] = [[0, 0.42], [0.1, 0.78], [0.28, 0.97], [0.45, 1], [0.62, 0.9], [0.78, 0.68], [0.9, 0.42], [1, 0.1]];

export type Tuft = Tube & { band: string; cluster: number; c: V3; dir: V3; length: number; radius: number; role: "tip" | "side" | "inner" | "spire" };

/** 🌿️ A flame tuft from the point `base` along the unit direction `dir`. */
export function tuft(id: string, bone: string, material: string, tone: number, base: V3, dir: V3, length: number, radius: number, role: Tuft["role"]): Tuft {
  const points = FLAME.map(([s, k]) => ({ p: add(base, mul(dir, s * length)), r: radius * k }));
  return { kind: "tube", id, bone, material, tone, group: "tuft", shape: bone, points, band: bone, cluster: 0, c: add(base, mul(dir, 0.45 * length)), dir, length, radius, role };
}

export type Core = { kind: "core"; id: string; bone: string; material: string; tone: number; rows: { p: V3; rx: number; rz: number }[] };

/** 🧊️ The bone-local loft of a vertical core: section x along world x, section y along world -z, travel up. */
export function coreSolid(core: Core, frames: Record<string, Frame>): Json {
  const f = frames[core.bone]!;
  const rotation = quaternion(unit(dirToLocal(f, [1, 0, 0])), unit(dirToLocal(f, [0, 0, -1])), unit(dirToLocal(f, [0, 1, 0])));
  const sections = core.rows.map((s) => ({ at: toLocal(f, s.p).map(r5), rx: r5(s.rx), ry: r5(s.rz), rotation }));
  return { id: core.id, kind: "loft", bone: core.bone, material: core.material, role: "body", tone: core.tone, sections, capStart: true, capEnd: true };
}

const ellipse = (c: P2, a: number, b: number, n = 40): P2[] => Array.from({ length: n }, (_, i) => ({ x: c.x + a * Math.cos((2 * Math.PI * i) / n + 0.07), y: c.y + b * Math.sin((2 * Math.PI * i) / n + 0.07) }));

/** 🫙️ The polygons whose union is a core's outline in a view: trapezoids between sections in the elevations, section ellipses in the plan. */
export function corePolygons(view: View, core: Core): P2[][] {
  const out: P2[][] = [];
  if (view === "plan") {
    for (const s of core.rows) out.push(ellipse(project(view, s.p), s.rx, s.rz));
    return out;
  }
  const half = (s: Core["rows"][number]): number => (view === "elevation" ? s.rx : s.rz);
  for (let i = 0; i + 1 < core.rows.length; i++) {
    const [a, b] = [core.rows[i]!, core.rows[i + 1]!];
    const [pa, pb] = [project(view, a.p), project(view, b.p)];
    out.push([{ x: pa.x - half(a), y: pa.y }, { x: pa.x + half(a), y: pa.y }, { x: pb.x + half(b), y: pb.y }, { x: pb.x - half(b), y: pb.y }]);
  }
  const [first, last] = [core.rows[0]!, core.rows[core.rows.length - 1]!];
  for (const s of [first, last]) {
    const c = project(view, s.p);
    out.push(ellipse(c, half(s), Math.min(half(s), 0.05) * 0.5, 24));
  }
  return out;
}
