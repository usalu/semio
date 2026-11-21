/** 🖋️ Ruling E1 for the small-leaved lime: every 2D shape of every scale and view is redrawn as the true orthographic view of that scale's body, shape by shape, keeping id, role, bone, layer and tone; patch shapes that only filled holes of the old drawings are removed, root flare and lower leader get a shape on their own bone, and the plan symbol takes the true crown outline.
 * @see ./round3-brief.md */
import type { Frame, Json, P2, V3 } from "./volumes-lib.ts";
import { lobeOutline, nearness, outline, pathOf, pointsOf, project, r4, toDrawing, tubePolygons, type Lobe, type Tube, type View } from "./r3-small-leaved-lime-kit.ts";
import type { CrownLobe, Model } from "./r3-small-leaved-lime-model.ts";

const VIEWS: View[] = ["elevation", "front", "plan"];
const inside = (p: P2, ring: P2[]): boolean => {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) hit = !hit;
  }
  return hit;
};

/** 💚️ A heart-shaped lime leaf outline of 7 cm at `c`, turned by `deg` (tip first). */
function leaf(c: P2, deg: number): P2[] {
  const half: [number, number][] = [[0.07, 0], [0.056, 0.012], [0.04, 0.024], [0.024, 0.031], [0.01, 0.03], [0.001, 0.021], [-0.002, 0.01], [0.004, 0]];
  const pts = [...half, ...half.slice(1, -1).reverse().map(([x, y]) => [x, -y] as [number, number])];
  const [cs, sn] = [Math.cos((deg * Math.PI) / 180), Math.sin((deg * Math.PI) / 180)];
  return pts.map(([x, y]) => ({ x: c.x + (x - 0.034) * cs - y * sn, y: c.y + (x - 0.034) * sn + y * cs }));
}

/** 🧵️ Which wood carries the limb line of a foliage mass in the front and plan drawings. */
const LINE_OF: Record<string, string[]> = {
  "r1-0": ["le1a", "le1b"], "r1-180": ["lw1a", "lw1b"], "r1l-90": ["l1n"], "r1l-270": ["l1s"],
  "r2-0": ["le2a"], "r2-180": ["lw2a"], "r2l-90": ["l2n"], "r2l-270": ["l2s"],
  "r3-0": ["le3a"], "r3-180": ["lw3a"], "r3l-90": ["l3n"], "r3l-270": ["l3s"],
  "r4-0": ["le4a"], "r4-180": ["lw4a"],
  "core-low": ["ld1"], "core-high": ["ld2"], top: ["ld3", "ld4"], "epi-e": ["epi-e"], "epi-w": ["epi-w"],
};

const STRIP = ["d", "points", "closed", "cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1"];
const bare = (shape: Json): Json => Object.fromEntries(Object.entries(shape).filter(([k]) => !STRIP.includes(k)));

export function draw(asset: Json, F: Record<string, Frame>, models: Model[]): string[] {
  const report: string[] = [];
  for (const m of models) {
    const scale = m.scale;
    const feature = (0.15 * scale) / 1000;
    const tolerance = feature / 4;
    const rep = asset.representations[String(scale)];
    const tubes: Tube[] = [...m.wood, ...m.shoots];
    const crownLobes = m.lobes;
    for (const view of VIEWS) {
      const old: Json[] = rep.geometry[view];
      const next: Json[] = [];
      const note = { redrawn: 0, kept: [] as string[], removed: [] as string[], added: [] as string[] };
      const arcPoints = (l: Lobe): P2[] => {
        const c = project(view, l.c);
        const mid = view === "plan" ? (Math.atan2(c.y, c.x) * 180) / Math.PI : 90;
        const half = view === "plan" ? 40 : 58;
        return lobeOutline(view, l, 10, mid - half, mid + half).map((p) => ({ x: c.x + (p.x - c.x) * 0.72, y: c.y + (p.y - c.y) * 0.72 }));
      };
      const rims = crownLobes.filter((l) => l.main).map((l) => ({ l, ring: lobeOutline(view, l, 36), near: nearness(view, l.c) }));
      const visible = crownLobes
        .filter((l) => l.main && (view === "elevation" ? [2, 5, 8] : [5]).filter((i) => !rims.some((o) => o.l !== l && o.near > nearness(view, l.c) && inside(arcPoints(l)[i]!, o.ring))).length >= (view === "elevation" ? 2 : 1))
        .sort((a, b) => nearness(view, b.c) - nearness(view, a.c));
      const used: Record<string, number> = {};
      const nth = (key: string): number => (used[key] = (used[key] ?? 0) + 1) - 1;
      const set = (shape: Json, rings: P2[][], closed: boolean): Json | null => {
        if (rings.length === 0 || rings[0]!.length < 2) return null;
        const local = toDrawing(view, F[shape.bone]!, rings);
        const rest = bare(shape);
        if (shape.kind === "path" || rings.length > 1) return { ...rest, kind: "path", d: pathOf(local, closed) };
        return { ...rest, kind: "poly", points: pointsOf(local[0]!), closed };
      };
      const woodOf = (b: string): P2[][] => outline([...tubes.filter((t) => t.bone === b).flatMap((t) => tubePolygons(view, t)), ...m.burrs.filter((l) => l.bone === b).map((l) => lobeOutline(view, l, 40))], tolerance, (3 * feature) ** 2);
      const massOf = (list: Lobe[]): P2[][] => outline(list.map((l) => lobeOutline(view, l)), tolerance, (4 * feature) ** 2);
      const push = (shape: Json | null, id: string): void => {
        if (shape === null) note.removed.push(id);
        else {
          next.push(shape);
          note.redrawn++;
        }
      };
      const arcOf = (shape: Json, l: CrownLobe | undefined, id: string): void => {
        if (!l) return void note.removed.push(id);
        push(set(shape, [arcPoints(l)], false), id);
      };
      const groupOf = (base: string): CrownLobe[] => {
        const ring = /^r(\d)[ld]?-(\d+)$/.exec(base);
        if (ring) return crownLobes.filter((l) => l.group === "sun" && l.tier === Number(ring[1]) && l.sector === Number(ring[2]));
        return crownLobes.filter((l) => l.bone === `fol-${base}`);
      };
      const ridgesSeen = m.ridges.filter((t) => nearness(view, t.points[Math.floor(t.points.length / 2)]!.p) > 0.06).sort((a, b) => project(view, a.points[0]!.p).x - project(view, b.points[0]!.p).x);
      const arcTemplates: Record<string, Json> = {};
      for (const shape of old) {
        const id: string = shape.id;
        const b: string = shape.bone;
        const sector = /^[fp]-(r\d[ld]?-\d+|core-low|core-high|top|epi-e|epi-w)$/.exec(id);
        const sectorArc = /^[fp]-(r\d[ld]?-\d+|core-low|core-high|top|epi-e|epi-w)-arc$/.exec(id);
        const line = /^[fp]l-(.+)$/.exec(id);
        if (/^crown-fill-|^[fp]-epi-s$/.test(id)) note.removed.push(id);
        else if (id === "trunk-dot") {
          next.push(shape);
          note.kept.push(id);
        } else if (id === "stem" || id === "root-plate-mass" || /^wood-/.test(id) || /^epi-[ew](-mass)?$/.test(id)) {
          const made = set(shape, woodOf(b), true);
          if (made && b.startsWith("epi-") && made.region === undefined) made.region = "epicormic-stem";
          push(made, id);
        } else if (/^twig-epi-/.test(id)) {
          const t = tubes.find((x) => x.id === b);
          push(t ? set(shape, [t.points.slice(1).map((q) => project(view, q.p))], false) : null, id);
        } else if (/^bark-ridge-\d+$/.test(id)) {
          const t = ridgesSeen[nth("ridge")];
          push(t ? set(shape, [t.points.map((q) => project(view, q.p))], false) : null, id);
        } else if (/^bark-knot-\d+$/.test(id)) {
          const burr = m.burrs.filter((l) => nearness(view, l.c) > 0.1)[nth("knot")];
          push(burr ? set(shape, [lobeOutline(view, burr, 16)], true) : null, id);
        } else if (id === "crown") push(set(shape, massOf(crownLobes.filter((l) => l.group !== "epi")), true), id);
        else if (id === "epicormic") push(set(shape, massOf(crownLobes.filter((l) => l.group === "epi")), true), id);
        else if (/^fol-.+-mass$/.test(id)) push(set(shape, massOf(crownLobes.filter((l) => l.bone === b)), true), id);
        else if (sector) push(set(shape, massOf(groupOf(sector[1]!)), true), id);
        else if (sectorArc) arcTemplates[sectorArc[1]!] ??= shape;
        else if (/-arc\d+$/.test(id)) arcTemplates[b] ??= shape;
        else if (/-leaf\d+$/.test(id)) {
          const mine = visible.filter((l) => l.bone === b);
          const i = nth(`leaf ${b}`);
          const l = mine[(i * 3 + 1) % Math.max(1, mine.length)];
          if (!l) note.removed.push(id);
          else {
            const c = project(view, l.c);
            push(set({ ...shape, z: 9.5 }, [leaf({ x: c.x + 0.35 * l.radii[2] * Math.cos(i * 2.4 + 0.6), y: c.y - 0.1 - 0.3 * l.radii[1] * Math.abs(Math.sin(i * 1.7 + 0.4)) }, 215 + i * 67)], true), id);
          }
        } else if (line) {
          const parts = (LINE_OF[line[1]!] ?? []).map((t) => tubes.find((x) => x.id === t)).filter((t): t is Tube => t !== undefined);
          const pts = parts.flatMap((t, k) => t.points.slice(k === 0 ? 1 : 2).map((q) => project(view, q.p)));
          push(pts.length > 1 ? set(shape, [pts], false) : null, id);
        } else if (/^plan-/.test(id)) note.removed.push(id);
        else {
          next.push(shape);
          note.kept.push(id);
        }
      }
      const most = scale === 50 ? (view === "elevation" ? 6 : 2) : view === "elevation" ? 3 : 1;
      for (const [key, template] of Object.entries(arcTemplates)) {
        const sectorKey = view !== "elevation";
        const mine = visible.filter((l) => (sectorKey ? groupOf(key).includes(l) : l.bone === key));
        const spread = [...mine.filter((_, i) => i % 2 === 0), ...mine.filter((_, i) => i % 2 === 1)].slice(0, most);
        if (spread.length === 0) note.removed.push(template.id);
        spread.forEach((l, i) => arcOf({ ...template, z: (view === "elevation" ? 9 : 6) + i / 100, id: sectorKey ? `${template.id}${i === 0 ? "" : i + 1}` : `${key}-arc${i + 1}` }, l, template.id));
      }
      if (view === "plan") for (const s of next) if (/^p-core-/.test(s.id)) s.z = s.id.startsWith("p-core-low") ? 3.0 : 3.05;
      const stemShape = next.find((s) => s.id === "stem");
      if (stemShape && view !== "plan") {
        const template = bare(stemShape);
        delete template.followTip;
        if (!next.some((s) => s.id === "root-plate-mass")) {
          const made = set({ ...template, id: "root-plate-mass", bone: "root-plate", z: 0.9, kind: "poly" }, woodOf("root-plate"), true);
          if (made) {
            next.unshift(made);
            note.added.push("root-plate-mass");
          }
        }
        if (scale !== 500 && !next.some((s) => s.id === "wood-ld1")) {
          const made = set({ ...template, id: "wood-ld1", bone: "ld1", z: 1.5, kind: "poly" }, woodOf("ld1"), true);
          if (made) {
            next.splice(next.indexOf(stemShape) + 1, 0, made);
            note.added.push("wood-ld1");
          }
        }
      }
      rep.geometry[view] = next;
      report.push(`1:${scale} ${view}: redrawn ${note.redrawn}, kept ${note.kept.length} (${note.kept.join(" ")}), removed ${note.removed.length} (${note.removed.join(" ")}), added ${note.added.join(" ")}`);
    }
  }
  return report;
}

/** 🗺️ The plan symbol with true crown extent: crown outlines of the 1:500 body, the stem, and the four lowest limb directions. */
export function drawSymbols(asset: Json, models: Model[]): string[] {
  const m = models.find((x) => x.scale === 500)!;
  const low = models.find((x) => x.scale === 200)!;
  const symbol = asset.symbols.find((s: Json) => s.semantics === "true-extent");
  if (!symbol) return ["symbol: none with true extent"];
  for (const view of VIEWS) for (const shape of symbol.geometry[view] as Json[]) {
    if (shape.id === "crown") shape.points = pointsOf(outline(m.lobes.filter((l) => l.group !== "epi").map((l) => lobeOutline(view, l)), 0.04, 1)[0]!);
    else if (shape.id === "stem-1" && view !== "plan") shape.points = [[-0.6, 0], [-0.4, 0.16], [-0.315, 0.52], [-0.287, 2.99], [-0.2, 6.0], [0.2, 6.0], [0.287, 2.99], [0.315, 0.52], [0.4, 0.16], [0.6, 0]];
    else if (shape.id === "stem-1") shape.points = Array.from({ length: 24 }, (_, i) => [r4(0.29 * Math.cos((i * Math.PI) / 12)), r4(0.29 * Math.sin((i * Math.PI) / 12))]);
    else if (/^scaffold-\d$/.test(shape.id)) {
      const t = low.wood.find((x) => x.id === ["le1b", "l1n", "lw1b", "l1s"][Number(shape.id.slice(-1)) - 1]);
      const tip = t!.points[t!.points.length - 1]!.p as V3;
      shape.points = [[0, 0], [r4(tip[0]), r4(tip[2])]];
    }
  }
  return ["symbol: crown, stem and four limb directions redrawn from the 1:500 and 1:200 bodies"];
}
