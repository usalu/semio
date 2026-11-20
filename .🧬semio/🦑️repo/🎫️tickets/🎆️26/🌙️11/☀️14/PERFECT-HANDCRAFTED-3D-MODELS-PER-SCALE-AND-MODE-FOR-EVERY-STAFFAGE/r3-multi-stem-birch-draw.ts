/** ✏️ Ruling E1 for the multi-stem birch: the 2D drawings of every scale and view are redrawn as the true orthographic view of that scale's body, bone by bone: stool, dark stem bases, white stems, leaders, branches and branch tips as filled outlines; the foliage of every bone as one outline behind the wood and one in front of it; hanging strands, leaf ticks, lenticels, fissures and branch scars as strokes on the faces that look at the viewer; the plan symbol follows the 1:100 body.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-multi-stem-birch.md */
import type { Frame, Json, P2, V3 } from "./volumes-lib.ts";
import { add, mul, nearness, outline, partPolygons, pathOf, pointsOf, project, r4, sub, toDrawing, type Mark, type Part, type View } from "./r3-multi-stem-birch-kit.ts";
import { DARK, LEAF, TONE, WHITE, stemAt, type Tree } from "./r3-multi-stem-birch-tree.ts";

const VIEWS: View[] = ["elevation", "front", "plan"];
const STEM_BONES = ["stem-l-1", "stem-l-2", "fol-l-top", "stem-c-1", "stem-c-2", "fol-c-top", "stem-r-1", "stem-r-2", "fol-r-top"];
const keyOf = (bone: string): string => (/^fol-.-(a|b|top)$/.test(bone) ? bone.slice(4) : bone);
const centroid = (part: Part): V3 => mul(part.sections.reduce((s, q) => add(s, q.p), [0, 0, 0] as V3), 1 / part.sections.length);
const ringArea = (ring: P2[]): number => ring.reduce((s, p, i) => s + p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y, 0) / 2;

export function draw(asset: Json, F: Record<string, Frame>, models: Tree[]): string[] {
  const report: string[] = [];
  const axis: { p: V3 }[] = [];
  for (const b of STEM_BONES) {
    const length = asset.rig.bones.find((x: Json) => x.id === b).lengthM;
    for (let u = 0; u <= length - 0.05; u += 0.25) axis.push({ p: stemAt(F, asset, b, u).p });
  }
  /** 👁️ Whether a point lies in front of the stem that is nearest to it on the paper of a view. */
  const inFront = (view: View, p: V3): boolean => {
    if (view === "plan") return true;
    const q = project(view, p);
    const near = axis.reduce((best, a) => {
      const d = project(view, a.p);
      const dist = Math.hypot(d.x - q.x, d.y - q.y);
      return dist < best.dist ? { dist, p: a.p } : best;
    }, { dist: Infinity, p: [0, 0, 0] as V3 });
    return nearness(view, p) >= nearness(view, near.p);
  };
  for (const m of models) {
    const scale = m.scale;
    const feature = (0.15 * scale) / 1000;
    const tolerance = 0.6 * feature;
    const rep = asset.representations[String(scale)];
    const fine = scale <= 100;
    for (const view of VIEWS) {
      const next: Json[] = [];
      const shapeOf = (base: Json, bone: string, rings: P2[][], closed: boolean): Json | null => {
        if (rings.length === 0 || rings[0]!.length < 2) return null;
        const local = toDrawing(view, F[bone]!, rings);
        if (closed && local.length === 1) return { ...base, bone, kind: "poly", points: pointsOf(local[0]!), closed: true };
        return { ...base, bone, kind: "path", d: pathOf(local, closed) };
      };
      const push = (shape: Json | null): void => {
        if (shape) next.push(shape);
      };
      const filled = (parts: Part[], hole = 3): P2[][] => outline(parts.flatMap((p) => partPolygons(view, p, 14)), tolerance, (hole * feature) ** 2);
      const body = (id: string, bone: string, z: number, tone: number, region: string, material: string, parts: Part[]): void => {
        if (parts.length > 0) push(shapeOf({ id, role: "body", z, filled: true, tone, region, material }, bone, filled(parts), true));
      };
      const zWood = view === "plan" ? 2 : 3;
      if (scale === 500) {
        body("stems", "stem-c-1", 1, TONE.stem, "stem-bark", WHITE, m.wood);
        const rings = filled(m.leaves, 4);
        push(shapeOf({ id: "crown", role: "body", z: 2, filled: true, tone: 0.6, regions: ["leaf-crown", "branch-bark"] }, "stem-c-2", rings, true));
      } else {
        const stroke = (id: string, bone: string, role: string, z: number, tone: number, lines: V3[][]): void => {
          const rings = lines.map((line) => line.map((p) => project(view, p))).filter((line) => line.length >= 2);
          if (rings.length > 0) push(shapeOf({ id, role, z, filled: false, tone }, bone, rings, false));
        };
        const leafBones = [...new Set(m.leaves.map((l) => l.bone))];
        const layer = (near: boolean): void => {
          const groups = leafBones.map((bone) => {
            const parts = m.leaves.filter((l) => l.bone === bone && inFront(view, centroid(l)) === near);
            const depth = parts.length === 0 ? 0 : parts.reduce((s, l) => s + nearness(view, centroid(l)), 0) / parts.length;
            return { bone, parts, depth };
          }).filter((g) => g.parts.length > 0).sort((a, b) => a.depth - b.depth);
          groups.forEach((g, rank) => {
            const key = keyOf(g.bone);
            const prefix = view === "elevation" ? "fol" : view === "front" ? "f" : "p";
            const id = `${prefix}-${key}${near ? "" : "-back"}`;
            const z = (view === "plan" ? 3 : near ? 4.6 : 1.2) + rank / 100;
            const tone = g.parts[0]!.tone;
            body(id, g.bone, r4(z), tone, "leaf-crown", LEAF, g.parts);
            if (!near || !fine) return;
            const tresses = g.parts.filter((l) => l.az !== undefined), sleeves = g.parts.filter((l) => l.az === undefined);
            if (view === "plan") stroke(`${id}-arc`, g.bone, scale === 50 ? "texture" : "detail", r4(z + 0.004), 0.25, sleeves.map((l) => l.sections.slice(1, -1).map((q) => q.p)));
            else {
              stroke(`${id}-arcs`, g.bone, "detail", r4(z + 0.004), 0.3, tresses.map((l) => l.sections.slice(2, -1).map((q) => q.p)));
              if (scale === 50) stroke(`${id}-leaves`, g.bone, "texture", r4(z + 0.006), 0.3, [...tresses, ...sleeves].flatMap((l) => l.sections.slice(1, -1).map((q, i) => {
                const side = i % 2 === 0 ? 1 : -1;
                const from = add(q.p, mul(q.x, side * 0.5 * q.rx));
                return [from, add(from, add(mul(q.x, side * 0.25 * q.rx), [0, -0.07, 0]))];
              })));
            }
          });
        };
        if (view !== "plan") layer(false);
        const wood = (group: string, bone: string): Part[] => m.wood.filter((p) => p.group === group && p.bone === bone);
        for (const p of m.wood.filter((x) => x.id.startsWith("branch-"))) body(p.id, p.bone, view === "plan" ? 1.6 : 2.4, TONE.branch, "branch-bark", WHITE, [p]);
        for (const c of ["l-a", "l-b", "c-a", "c-b", "r-a", "r-b"]) {
          body(`br-${c}`, `br-${c}`, view === "plan" ? 1.7 : 2.45, TONE.branch, "branch-bark", WHITE, wood("branch", `br-${c}`));
          body(`tip-${c}`, `fol-${c}`, view === "plan" ? 1.7 : 2.45, TONE.branch, "branch-bark", WHITE, wood("tip", `fol-${c}`));
        }
        body("stool", "root-plate", zWood, TONE.stool, "stem-bark", DARK, m.wood.filter((p) => p.bone === "root-plate"));
        for (const x of ["l", "c", "r"]) {
          const lower = scale === 50 ? `stem-${x}-1` : `stem-${x}`;
          body(lower, `stem-${x}-1`, zWood + 0.05, TONE.stem, "stem-bark", WHITE, wood("stem", `stem-${x}-1`));
          body(`base-${x}`, `stem-${x}-1`, zWood + 0.07, TONE.base, "stem-bark", DARK, wood("base", `stem-${x}-1`));
          body(`stem-${x}-2`, `stem-${x}-2`, zWood + 0.1, TONE.stem, "stem-bark", WHITE, wood("stem", `stem-${x}-2`));
          body(`leader-${x}`, `fol-${x}-top`, zWood + 0.12, TONE.stem, "stem-bark", WHITE, wood("leader", `fol-${x}-top`));
        }
        if (fine && view !== "plan") {
          const seen = (k: Mark): boolean => nearness(view, k.axes[0]) > 0.2;
          const across = (k: Mark): V3[] => [add(k.c, mul(k.axes[2], k.radii[2])), sub(k.c, mul(k.axes[2], k.radii[2]))];
          for (const x of ["l", "c", "r"]) {
            for (const [suffix, bone] of [["", `stem-${x}-1`], ["-fine", `stem-${x}-2`]] as [string, string][]) {
              stroke(`lenticels${suffix}-${x}`, bone, "texture", zWood + 0.2, 0.1, m.marks.filter((k) => k.bone === bone && k.id.startsWith("lenticel-") && seen(k)).map(across));
              stroke(`scars-${bone}`, bone, "texture", zWood + 0.22, 0.1, m.marks.filter((k) => k.bone === bone && k.id.startsWith("scar-") && seen(k)).map((k) => {
                const [a, b] = across(k);
                return [sub(a!, mul(k.axes[1], k.radii[1])), add(k.c, mul(k.axes[1], 0.4 * k.radii[1])), sub(b!, mul(k.axes[1], k.radii[1]))];
              }));
            }
            stroke(x === "c" ? "base-dark" : `base-dark-${x}`, `stem-${x}-1`, "detail", zWood + 0.2, 0.1, m.marks.filter((k) => k.bone === `stem-${x}-1` && k.id.startsWith("fissure-") && seen(k)).map((k) => [add(k.c, mul(k.axes[1], k.radii[1])), add(k.c, mul(k.axes[2], 0.6 * k.radii[2])), sub(k.c, mul(k.axes[1], k.radii[1]))]));
          }
        }
        if (view !== "plan") for (const bone of [...new Set(m.strands.map((s) => s.bone))]) {
          const prefix = view === "elevation" ? "fol" : "f";
          stroke(`${prefix}-${keyOf(bone)}-fine`, bone, "texture", 4.95, TONE.strand, m.strands.filter((s) => s.bone === bone).map((s) => s.sections.map((q) => q.p)));
        }
        layer(true);
        if (view === "plan") for (const x of ["l", "c", "r"]) {
          const at = [0.6, 0.8, 1.0, 1.2].map((u) => stemAt(F, asset, `stem-${x}-1`, u)).reduce((best, s) => (Math.abs(s.p[1] - 1.3) < Math.abs(best.p[1] - 1.3) ? s : best));
          const c = project("plan", at.p);
          next.push({ id: `stem-${x}-1-dot`, bone: `stem-${x}-1`, role: "body", z: 8, filled: true, tone: 0.25, region: "stem-bark", kind: "poly", points: Array.from({ length: 16 }, (_, i) => [r4(c.x + at.r * Math.cos((i * Math.PI) / 8)), r4(c.y + at.r * Math.sin((i * Math.PI) / 8))]), closed: true });
        }
      }
      const before = rep.geometry[view].length;
      rep.geometry[view] = next;
      const points = next.reduce((s, x) => s + (x.points ? x.points.length : (x.d.match(/[ML]/g) ?? []).length), 0);
      report.push(`1:${scale} ${view}: ${before} shapes replaced by ${next.length} (${points} points)`);
    }
  }
  const symbol = asset.symbols.find((x: Json) => x.id === "multi-stem-birch-symbol");
  const m100 = models[1]!;
  const ext: Record<string, number[]> = {};
  const chainOf: Record<string, string> = { "stem-1": "l", "stem-2": "c", "stem-3": "r" };
  const scaffold = ["br-l-a", "br-r-a", "br-l-b", "br-r-b"];
  for (const view of VIEWS) for (const shape of symbol.geometry[view] as Json[]) {
    if (shape.id === "crown") {
      const ring = outline(m100.leaves.flatMap((p) => partPolygons(view, p, 12)), 0.05, 100).filter((r) => ringArea(r) > 0)[0]!;
      shape.points = pointsOf(ring);
      ext[view] = [Math.min(...ring.map((p) => p.x)), Math.max(...ring.map((p) => p.x)), Math.min(...ring.map((p) => p.y)), Math.max(...ring.map((p) => p.y))];
    }
    const x = chainOf[shape.id as string];
    if (x && view !== "plan") shape.points = pointsOf(outline(m100.wood.filter((p) => ["stem", "base", "leader"].includes(p.group) && p.bone[5] === x[0] || (p.group === "leader" && p.id === `leader-${x}`)).flatMap((p) => partPolygons(view, p, 12)), 0.012, 0.01)[0]!);
    if (x && view === "plan") {
      const at = stemAt(F, asset, `stem-${x}-1`, 1.03);
      const c = project("plan", at.p);
      shape.points = Array.from({ length: 24 }, (_, i) => [r4(c.x + at.r * Math.cos((i * Math.PI) / 12)), r4(c.y + at.r * Math.sin((i * Math.PI) / 12))]);
    }
    const limb = /^scaffold-(\d)$/.exec(shape.id);
    if (limb) {
      const part = m100.wood.find((p) => p.id === scaffold[Number(limb[1]) - 1])!;
      shape.points = [part.sections[0]!, part.sections[part.sections.length - 1]!].map((q) => {
        const p = project("plan", q.p);
        return [r4(p.x), r4(p.y)];
      });
    }
  }
  const [w, d, h] = [ext.elevation![1]! - ext.elevation![0]!, ext.plan![3]! - ext.plan![2]!, ext.elevation![3]!];
  const de = (v: number): string => v.toFixed(2).replace(".", ",");
  symbol.dimensionalNote = {
    en: `A conventional simplification of the plant drawn at its true extent in model metres in all three views (${w.toFixed(2)} m wide, ${d.toFixed(2)} m deep and ${h.toFixed(2)} m high): the outline is the crown, the three stems are drawn at their real position and thickness. It is a symbol, never used silently for a pictorial scale.`,
    de: `Eine konventionelle Vereinfachung der Pflanze in ihrer wahren Ausdehnung in Modellmetern in allen drei Ansichten (${de(w)} m breit, ${de(d)} m tief und ${de(h)} m hoch): Der Umriss ist die Krone, die drei Stämme sind in wirklicher Lage und Stärke gezeichnet. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.`,
  };
  symbol.description = {
    en: "A conventional plan sign of the plant for site and landscape plans: the true outline of the crown, the three stem sections at breast height and the directions of four main branches. It is a separate symbol and is never used silently for a pictorial scale.",
    de: "Ein konventionelles Grundrisszeichen der Pflanze für Lage- und Freiraumpläne: der wahre Umriss der Krone, die drei Stammquerschnitte in Brusthöhe und die Richtungen von vier Hauptästen. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.",
  };
  return report;
}
