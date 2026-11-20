/** ✏️ Ruling E1 for the apple tree: every 2D shape of every scale and view is redrawn as the true orthographic view of that scale's body, shape by shape, keeping id, role, bone, layer and tone; patch shapes that only filled holes of the old drawings are removed and the lower leader gets its own shape on its own bone.
 * @see ./round3-brief.md */
import type { Frame, Json, P2, V3 } from "./volumes-lib.ts";
import { circlePath, lobeOutline, nearness, outline, pathOf, pointsOf, project, r4, toDrawing, tubePolygons, type Fruit, type Lobe, type Tube, type View } from "./r3-apple-tree-kit.ts";

export type ScaleParts = { scale: number; tubes: Tube[]; lobes: Lobe[]; fruit: Fruit[] };
const VIEWS: View[] = ["elevation", "front", "plan"];
const VIEW_DIR: Record<View, V3> = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] };
const hits = (l: Lobe, p: V3, d: V3): boolean => {
  const o = l.axes.map((a, i) => ((p[0] - l.c[0]) * a[0] + (p[1] - l.c[1]) * a[1] + (p[2] - l.c[2]) * a[2]) / l.radii[i]!);
  const v = l.axes.map((a, i) => (d[0] * a[0] + d[1] * a[1] + d[2] * a[2]) / l.radii[i]!);
  const [A, B, C] = [v[0]! * v[0]! + v[1]! * v[1]! + v[2]! * v[2]!, o[0]! * v[0]! + o[1]! * v[1]! + o[2]! * v[2]!, o[0]! * o[0]! + o[1]! * o[1]! + o[2]! * o[2]! - 1];
  if (C < 0) return true;
  const disc = B * B - A * C;
  return disc > 0 && -B + Math.sqrt(disc) > 0;
};
const inside = (p: P2, ring: P2[]): boolean => {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) hit = !hit;
  }
  return hit;
};

/** 🍃️ An ovate leaf outline of 8.5 cm at `c`, turned by `deg`. */
function leaf(c: P2, deg: number): P2[] {
  const half: [number, number][] = [[0, 0], [0.012, 0.014], [0.03, 0.024], [0.05, 0.022], [0.07, 0.012], [0.085, 0]];
  const pts = [...half, ...half.slice(1, -1).reverse().map(([x, y]) => [x, -y] as [number, number])];
  const [cs, sn] = [Math.cos((deg * Math.PI) / 180), Math.sin((deg * Math.PI) / 180)];
  return pts.map(([x, y]) => ({ x: c.x + (x - 0.042) * cs - y * sn, y: c.y + (x - 0.042) * sn + y * cs }));
}

export function draw(asset: Json, F: Record<string, Frame>, scales: ScaleParts[]): string[] {
  const report: string[] = [];
  for (const { scale, tubes, lobes, fruit } of scales) {
    const feature = (0.15 * scale) / 1000;
    const tolerance = feature / 4;
    const rep = asset.representations[String(scale)];
    for (const view of VIEWS) {
      const old: Json[] = rep.geometry[view];
      const next: Json[] = [];
      const note = { redrawn: 0, kept: [] as string[], removed: [] as string[], added: [] as string[] };
      const visible = lobes
        .filter((l) => !lobes.some((o) => o !== l && nearness(view, o.c) > nearness(view, l.c) && inside(project(view, l.c), lobeOutline(view, o, 36))))
        .sort((a, b) => nearness(view, b.c) - nearness(view, a.c));
      const used: Record<string, number> = {};
      const nth = (key: string): number => (used[key] = (used[key] ?? 0) + 1) - 1;
      const set = (shape: Json, rings: P2[][], closed: boolean): Json | null => {
        if (rings.length === 0 || rings[0]!.length < 2) return null;
        const local = toDrawing(view, F[shape.bone]!, rings);
        const { d: _d, points: _p, closed: _c, ...rest } = shape;
        if (shape.kind === "path") return { ...rest, d: pathOf(local, closed) };
        if (shape.kind === "poly") return { ...rest, points: pointsOf(local[0]!), closed };
        throw new Error(`shape ${shape.id} of kind ${shape.kind} cannot take an outline`);
      };
      const woodOf = (b: string): P2[][] => outline(tubes.filter((t) => t.bone === b && t.group !== "twig").flatMap((t) => tubePolygons(view, t)), tolerance, (3 * feature) ** 2);
      const push = (shape: Json | null, id: string): void => {
        if (shape === null) note.removed.push(id);
        else {
          next.push(shape);
          note.redrawn++;
        }
      };
      let fruitSlot: Json | null = null;
      const FRUIT_MARK: Json = { id: "fruit-mark" };
      for (const shape of old) {
        const id: string = shape.id;
        const b: string = shape.bone;
        if (/^crown-fill-|^sc-.*-mass$/.test(id)) note.removed.push(id);
        else if (/-fruit\d+$/.test(id)) {
          if (!fruitSlot) {
            fruitSlot = shape;
            next.push(FRUIT_MARK);
          }
        } else if (/-arc-?\d+$/.test(id)) {
          const l = visible.filter((x) => x.bone === b)[nth(`arc ${b}`)];
          if (!l) note.removed.push(id);
          else {
            const c = project(view, l.c);
            const mid = view === "plan" ? (Math.atan2(c.y, c.x) * 180) / Math.PI : 90;
            push(set({ ...shape, kind: shape.kind }, [lobeOutline(view, l, 10, mid - 62, mid + 62)], false), id);
          }
        } else if (/-leaf\d+$/.test(id)) {
          const mine = visible.filter((x) => x.bone === b);
          const i = nth(`leaf ${b}`);
          const l = mine[(i * 2 + 1) % Math.max(1, mine.length)];
          if (!l) note.removed.push(id);
          else {
            const c = project(view, l.c);
            push(set(shape, [leaf({ x: c.x + 0.3 * l.radii[2] * Math.cos(i * 2.4 + 0.6), y: c.y - 0.12 - 0.22 * l.radii[1] * Math.abs(Math.sin(i * 1.7 + 0.4)) }, 35 + i * 67)], true), id);
          }
        } else if (shape.role === "body" && shape.filled === true && (b.startsWith("fol-") || id === "crown")) {
          const mine = lobes.filter((l) => l.bone === b);
          push(set(shape, outline(mine.map((l) => lobeOutline(view, l)), tolerance, (4 * feature) ** 2), true), id);
        } else if (id === "stem" || id === "stem-fork" || id === "root-plate-mass" || /^wood-/.test(id)) push(set(shape, woodOf(b), true), id);
        else if (/^twig-spur-|^spur-.*-line$/.test(id)) {
          const t = tubes.find((x) => x.id === b);
          push(t ? set(shape, [t.points.slice(1).map((q) => project(view, q.p))], false) : null, id);
        } else if (/^hint-sc-/.test(id)) {
          const t = tubes.find((x) => x.id === b);
          const pts = t ? [t.points[1]!, t.points[Math.floor(t.points.length / 2)]!, t.points[t.points.length - 2]!] : [];
          push(t ? set(shape, [pts.map((q) => project(view, q.p))], false) : null, id);
        } else if (/^plan-/.test(id) && shape.kind === "capsule") {
          const t = tubes.find((x) => x.id === b);
          if (!t) note.removed.push(id);
          else {
            const [a, z] = [t.points[1]!, t.points[t.points.length - 2]!];
            const [pa, pz] = [project(view, a.p), project(view, z.p)];
            next.push({ ...shape, x0: r4(pa.x), y0: r4(pa.y), x1: r4(pz.x), y1: r4(pz.y), r0: r4(a.r), r1: r4(z.r) });
            note.redrawn++;
          }
        } else {
          next.push(shape);
          note.kept.push(id);
        }
      }
      const toViewer = VIEW_DIR[view];
      const seen = fruit.filter((f) => !lobes.some((l) => hits(l, f.c, toViewer)));
      const template = fruitSlot ?? rep.geometry.elevation.find((s: Json) => /-fruit\d+$/.test(s.id)) ?? asset.representations["200"].geometry.elevation.find((s: Json) => /-fruit\d+$/.test(s.id));
      const marks: Json[] = [];
      if (template && scale < 500) {
        const count: Record<string, number> = {};
        for (const f of seen) {
          const n = (count[f.bone] = (count[f.bone] ?? 0) + 1);
          const { d: _d, cx: _cx, cy: _cy, rx: _rx, ry: _ry, kind: _k, ...rest } = template;
          const base = { ...rest, id: `${f.bone}-fruit${n}`, bone: f.bone, z: 9 + (n - 1) / 100 };
          if (view === "elevation") marks.push({ ...base, kind: "path", d: circlePath(toDrawing(view, F[f.bone]!, [[project(view, f.c)]])[0]![0]!, f.r) });
          else {
            const c = project(view, f.c);
            marks.push({ ...base, kind: "ellipse", cx: r4(c.x), cy: r4(c.y), rx: f.r, ry: f.r });
          }
        }
      }
      const slot = next.indexOf(FRUIT_MARK);
      if (slot >= 0) next.splice(slot, 1, ...marks);
      else next.push(...marks);
      note.redrawn += marks.length;
      note.added.push(`${marks.length} fruit`);
      if (!next.some((s) => s.id === "stem")) {
        rep.geometry[view] = next;
        report.push(`1:${scale} ${view}: redrawn ${note.redrawn}, kept ${note.kept.length} (${note.kept.join(" ")}), removed ${note.removed.length} (${note.removed.join(" ")}), added ${note.added.join(" ")}`);
        continue;
      }
      if (!next.some((s) => s.id === "root-plate-mass")) {
        const stemShape = next.find((s) => s.id === "stem")!;
        const { d: _d, points: _p, closed: _c, followTip: _f, ...rest } = stemShape;
        const made = set({ ...rest, id: "root-plate-mass", bone: "root-plate", z: 0.9, kind: "poly" }, woodOf("root-plate"), true);
        if (made) {
          next.push(made);
          note.added.push("root-plate-mass");
        }
      }
      if (!next.some((s) => s.id === "stem-fork")) {
        const stemShape = next.find((s) => s.id === "stem")!;
        const { d: _d, points: _p, closed: _c, followTip: _f, ...rest } = stemShape;
        const base = { ...rest, id: "stem-fork", bone: "ld", z: 1.5, kind: view === "elevation" ? "path" : "poly" };
        const made = set(base, woodOf("ld"), true);
        if (made) {
          next.splice(next.indexOf(stemShape) + 1, 0, made);
          note.added.push("stem-fork");
        }
      } else {
        const fork = next.find((s) => s.id === "stem-fork")!;
        const stemShape = next.find((s) => s.id === "stem")!;
        fork.region = stemShape.region;
        if (stemShape.material !== undefined) fork.material = stemShape.material;
        fork.tone = stemShape.tone;
      }
      if (view !== "plan") next.filter((x) => /^wood-sc-..a$/.test(x.id)).forEach((x, i) => (x.z = 0.93 + i / 1000));
      rep.geometry[view] = next;
      report.push(`1:${scale} ${view}: redrawn ${note.redrawn}, kept ${note.kept.length} (${note.kept.join(" ")}), removed ${note.removed.length} (${note.removed.join(" ")}), added ${note.added.join(" ")}`);
    }
  }
  return report;
}
