/** ✏️ Ruling E1 for the japanese maple: every 2D shape of every scale and view is redrawn as the true orthographic view of that scale's body, keeping id, role, bone, layer, tone and hatch; the tier limbs get one filled wood outline per limb bone (replacing the old centre strokes, open limb outlines and limb lines), the leader gets its own elevation outline, twigs are texture strokes at 1:50, and the slivers and patch fills of the old drawings are removed.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-japanese-maple.md */
import type { Frame, Json, P2 } from "./volumes-lib.ts";
import { lobeOutline, nearness, outline, pathOf, pointsOf, project, r4, toDrawing, tubePolygons, type Lobe, type Tube, type View } from "./r3-japanese-maple-kit.ts";
import type { Spray } from "./r3-japanese-maple-tree.ts";

export type ScaleParts = { scale: number; tubes: Tube[]; twigs: Tube[]; lobes: Spray[]; stemV: (bone: string, u: number) => number };
const VIEWS: View[] = ["elevation", "front", "plan"];
const LIMB_BONES = ["t1w", "t1e", "t2w", "t2e", "t3w", "t3e"];
const REMOVED = /^wood-t\d[ew]$|^twig-\d+$|^tier-t\d[ew]$|^limb-t\d[ew]-line$|^t3[ew]-mass$|^f[l]-|^pl-|^crown-fill-|-3-line$/;

const inside = (p: P2, ring: P2[]): boolean => {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) hit = !hit;
  }
  return hit;
};
/** 📏️ Distance from `c` along the direction (dx, dy) to the boundary of a convex ring around it. */
function reach(ring: P2[], c: P2, dx: number, dy: number): number {
  let best = Infinity;
  for (let i = 0; i < ring.length; i++) {
    const [a, b] = [ring[i]!, ring[(i + 1) % ring.length]!];
    const ex = b.x - a.x, ey = b.y - a.y, den = dx * ey - dy * ex;
    if (Math.abs(den) < 1e-12) continue;
    const t = ((a.x - c.x) * ey - (a.y - c.y) * ex) / den, u = ((a.x - c.x) * dy - (a.y - c.y) * dx) / den;
    if (t > 0 && u >= 0 && u <= 1) best = Math.min(best, t);
  }
  return Number.isFinite(best) ? best : 0;
}
const angDist = (a: number, b: number): number => Math.abs(((a - b + 540) % 360) - 180);

/** 🔢️ The absolute points of a path made of M/L/C commands, as lists per subpath. */
function pathPoints(d: string): P2[][] {
  const tokens = d.match(/[MLCZ]|-?\d*\.?\d+(?:e-?\d+)?/gi) ?? [];
  const out: P2[][] = [];
  let cmd = "";
  const nums: number[] = [];
  const flush = (): void => {
    for (let i = 0; i + 1 < nums.length; i += 2) {
      if (cmd === "M" && i === 0) out.push([]);
      out[out.length - 1]?.push({ x: nums[i]!, y: nums[i + 1]! });
    }
    nums.length = 0;
  };
  for (const t of tokens) {
    if (/^[MLCZ]$/i.test(t)) {
      flush();
      cmd = t.toUpperCase();
    } else nums.push(Number(t));
  }
  flush();
  return out;
}
const shifted = (d: string, dx: number, dy: number): string =>
  pathPoints(d).map((ring) => ring.map((p, i) => `${i === 0 ? "M" : "L"} ${r4(p.x + dx)} ${r4(p.y + dy)}`).join(" ")).join(" ");
const centroid = (rings: P2[][]): P2 => {
  const pts = rings.flat();
  return { x: pts.reduce((s, p) => s + p.x, 0) / pts.length, y: pts.reduce((s, p) => s + p.y, 0) / pts.length };
};

export function draw(asset: Json, F: Record<string, Frame>, scales: ScaleParts[]): string[] {
  const report: string[] = [];
  for (const { scale, tubes, twigs, lobes, stemV } of scales) {
    const feature = (0.15 * scale) / 1000;
    const tolerance = feature / 4;
    const rep = asset.representations[String(scale)];
    for (const view of VIEWS) {
      const old: Json[] = rep.geometry[view];
      const next: Json[] = [];
      const note = { redrawn: 0, kept: [] as string[], removed: [] as string[], added: [] as string[] };
      const visible = (set: Lobe[]): Lobe[] =>
        set.filter((l) => !lobes.some((o) => o !== l && nearness(view, o.c) > nearness(view, l.c) + 0.05 && inside(project(view, l.c), lobeOutline(view, o, 36)))).sort((a, b) => nearness(view, b.c) - nearness(view, a.c));
      const used: Record<string, number> = {};
      const nth = (key: string): number => (used[key] = (used[key] ?? 0) + 1) - 1;
      const set = (shape: Json, rings: P2[][], closed: boolean): Json | null => {
        if (rings.length === 0 || rings[0]!.length < 2) return null;
        const local = toDrawing(view, F[shape.bone]!, rings);
        const { d: _d, points: _p, closed: _c, cx: _cx, cy: _cy, rx: _rx, ry: _ry, followTip: _f, kind, ...rest } = shape;
        if (kind === "poly" && local.length === 1) return { ...rest, kind, points: pointsOf(local[0]!), closed };
        if (kind === "ellipse" && local.length === 1) return { ...rest, kind: "poly", points: pointsOf(local[0]!), closed: true };
        return { ...rest, kind: "path", d: pathOf(local, closed) };
      };
      const woodOf = (pick: (t: Tube) => boolean): P2[][] => outline(tubes.filter(pick).flatMap((t) => tubePolygons(view, t)), tolerance, (3 * feature) ** 2);
      const lobesOutline = (set: Lobe[]): P2[][] => outline(set.map((l) => lobeOutline(view, l)), tolerance, (4 * feature) ** 2);
      const push = (shape: Json | null, id: string): void => {
        if (shape === null) note.removed.push(id);
        else {
          next.push(shape);
          note.redrawn++;
        }
      };
      const sectorShapes = (tier: string, bone: string): { id: string; sector: number }[] =>
        old.flatMap((s) => {
          const m = /^[fp]-(t\d)d?-(\d+)$/.exec(s.id);
          return m && m[1] === tier && s.bone === bone ? [{ id: s.id, sector: Number(m[2]) }] : [];
        });
      const groupOf = (shape: Json): Lobe[] => {
        const m = /^[fp]-(t\d)d?-(\d+)$/.exec(shape.id);
        if (m) {
          const peers = sectorShapes(m[1]!, shape.bone);
          return lobes.filter((l) => l.tier === m[1] && l.bone === shape.bone && peers.reduce((best, p) => (angDist(l.az, p.sector) < angDist(l.az, best.sector) ? p : best)).id === shape.id);
        }
        const core = /^[fp]-(core-low|core-up|top)$/.exec(shape.id);
        if (core) return lobes.filter((l) => l.tier === core[1]);
        if (/^fol-.*-mass$/.test(shape.id)) return lobes.filter((l) => l.bone === shape.bone);
        if (shape.id === "crown") return lobes;
        return [];
      };
      const arcOn = (shape: Json, group: Lobe[], key: string): Json | null => {
        const l = visible(group.filter((x) => Math.min(x.radii[0], x.radii[2]) >= 0.4))[nth(key)];
        if (!l) return null;
        const c = project(view, l.c);
        const mid = view === "plan" ? (Math.atan2(c.y, c.x) * 180) / Math.PI : 90;
        const ring = lobeOutline(view, l, 48);
        const centre = centroid([ring]);
        const arc = Array.from({ length: 11 }, (_, i) => {
          const phi = ((mid - 58 + (116 * i) / 10) * Math.PI) / 180;
          const rho = reach(ring, centre, Math.cos(phi), Math.sin(phi));
          return { x: centre.x + 0.86 * rho * Math.cos(phi), y: centre.y + 0.86 * rho * Math.sin(phi) };
        });
        return set(shape, [arc], false);
      };
      const template = (id: string): Json => old.find((s) => s.id === id) ?? next.find((s) => s.id === id);
      for (const shape of old) {
        const id: string = shape.id;
        const b: string = shape.bone;
        if (REMOVED.test(id) || (scale === 500 && /^limb-/.test(id))) note.removed.push(id);
        else if (/-arc\d*$/.test(id)) {
          const base = old.find((s) => s.id === id.replace(/-arc\d*$/, "") || s.id === `${id.replace(/-arc\d*$/, "")}-mass`);
          push(arcOn(shape, base ? groupOf(base) : lobes.filter((l) => l.bone === b), `arc ${b} ${base?.id ?? ""}`), id);
        } else if (/-leaf\d+$/.test(id)) {
          const l = visible(lobes.filter((x) => x.bone === b))[(nth(`leaf ${b}`) * 2 + 1) % Math.max(1, lobes.filter((x) => x.bone === b).length)];
          if (!l) note.removed.push(id);
          else {
            const i = used[`leaf ${b}`]!;
            const c = project(view, l.c);
            const target = toDrawing(view, F[b]!, [[{ x: c.x + 0.35 * l.radii[2] * Math.cos(i * 2.3 + 0.5), y: c.y - 0.45 * l.radii[1] }]])[0]![0]!;
            const here = centroid(pathPoints(shape.d));
            push({ ...shape, d: shifted(shape.d, target.x - here.x, target.y - here.y) }, id);
          }
        } else if (/^lenticel-|^bark-line-/.test(id)) {
          const pts = pathPoints(shape.d).flat();
          const u = pts.reduce((s, p) => s + p.x, 0) / pts.length;
          push({ ...shape, d: shifted(shape.d, 0, stemV(b, u)) }, id);
        } else if (shape.role === "body" && shape.filled === true && groupOf(shape).length > 0) {
          const group = groupOf(shape);
          const region = id === "crown" || shape.regions ? shape.region : (shape.region ?? (group.every((l) => l.group === "shade") ? "leaf-shade" : "leaf-sun"));
          push(set({ ...shape, ...(region ? { region } : {}) }, lobesOutline(group), true), id);
        }
        else if (id === "root-collar") push(set(shape, woodOf((t) => t.bone === "root-plate"), true), id);
        else if (/^wood-stem-/.test(id)) push(set(shape, woodOf((t) => t.bone === b && t.group === "stem"), true), id);
        else if (id === "stems") push(set(shape, woodOf((t) => t.group !== "limb"), true), id);
        else {
          next.push(shape);
          note.kept.push(id);
        }
      }
      const stemShape = template("wood-stem-c");
      if (stemShape && view === "elevation" && !next.some((s) => s.id === "wood-stem-c-top") && tubes.some((t) => t.bone === "stem-c-top")) {
        const made = set({ ...stemShape, id: "wood-stem-c-top", bone: "stem-c-top", z: 1.25 }, woodOf((t) => t.bone === "stem-c-top"), true);
        if (made) {
          next.splice(next.findIndex((s) => s.id === "wood-stem-c") + 1, 0, made);
          note.added.push("wood-stem-c-top");
        }
      }
      if (stemShape) {
        const at = next.reduce((last, s, i) => (/^wood-stem-|^root-collar$|^stems$/.test(s.id) ? i + 1 : last), 0);
        const made: Json[] = [];
        LIMB_BONES.forEach((bone, i) => {
          if (!tubes.some((t) => t.bone === bone && t.group === "limb")) return;
          const shape = set({ ...stemShape, id: `wood-${bone}`, bone, z: (view === "plan" ? 1.15 : 1.3) + i / 100, region: "limb-bark", material: "maple-bark-young", tone: 0.42 }, woodOf((t) => t.bone === bone && t.group === "limb"), true);
          if (shape) made.push(shape);
        });
        if (view === "elevation" && scale === 50) twigs.forEach((t, i) => {
          const pts = t.points.map((q) => project(view, q.p));
          const shape = set({ id: `twig-${i + 1}`, bone: t.bone, role: "texture", z: 1.36 + i / 1000, filled: false, tone: 0.2, kind: "path" }, [pts], false);
          if (shape) made.push(shape);
        });
        next.splice(at, 0, ...made);
        note.added.push(...made.map((s) => s.id));
      }
      rep.geometry[view] = next;
      report.push(`1:${scale} ${view}: redrawn ${note.redrawn}, kept ${note.kept.length} (${note.kept.join(" ")}), removed ${note.removed.length} (${note.removed.join(" ")}), added ${note.added.length}`);
    }
  }
  return report;
}

