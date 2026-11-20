/** ✒️ Ruling E1 for the Lombardy poplar: every 2D shape of every scale and view is redrawn as the true orthographic view of that scale's body (one outline per stem part, branch and flame group of a foliage bone, layered by nearness to the viewer), the strokes inside them are restated on the new forms, and the plan symbol follows the new crown.
 * @see ./round3-brief.md
 * @see ./r3-lombardy-poplar-model.ts */
import type { Frame, Json, P2 } from "./volumes-lib.ts";
import { corePolygons, lobeOutline, nearness, outline, pathOf, pointsOf, project, r4, toDrawing, tubePolygons, type Core, type Lobe, type Tube, type Tuft, type View } from "./r3-lombardy-poplar-kit.ts";

export type ScaleParts = { scale: number; wood: Tube[]; tufts: Tuft[]; cores: Core[]; burls: Lobe[]; lobes: Lobe[] };
const VIEWS: View[] = ["elevation", "front", "plan"];
const GEOMETRY = ["kind", "d", "points", "closed", "followTip", "cx", "cy", "rx", "ry", "rotationDeg", "x0", "y0", "x1", "y1", "r0", "r1", "solid", "x", "y", "w", "h", "r"];
const strip = (shape: Json): Json => Object.fromEntries(Object.entries(shape).filter(([k]) => !GEOMETRY.includes(k)));
const mean = (values: number[]): number => values.reduce((s, v) => s + v, 0) / Math.max(1, values.length);
const FOL = ["fol-l1", "fol-r1", "fol-l2", "fol-r2", "fol-l3", "fol-r3", "fol-l4", "fol-r4", "fol-top"];
const RIG = ["br-l1", "br-r1", "br-l2", "br-r2", "br-l3", "br-r3", "br-l4", "br-r4"];
const STEMS = ["trunk-1", "trunk-2", "trunk-3", "trunk-4"];

const T = (en: string, de: string): { en: string; de: string } => ({ en, de });
const DRAWN: Record<number, Record<string, { en: string; de: string }>> = {
  50: {
    silhouette: T("At 1:50 (1 m is 20 mm) the three drawings are the true views of the 1:50 body: a narrow, spindle-shaped column of pointed leaf flames, widest (4.2 m) in its lower third and drawn to a point at 25.96 m, above a short bare stem with a fluted, buttressed foot, longitudinal bark ridges and burls. Side and front elevation show the flames of each foliage bone as one outline (front: up to three groups by direction, overlapping by 0.6 m so that no slivers open between them), layered by their distance from the viewer; the plan shows the flames from above, the higher over the lower, with the eight rig branches as hints.", "Bei 1:50 (1 m sind 20 mm) sind die drei Zeichnungen die wahren Ansichten des 1:50-Körpers: eine schmale, spindelförmige Säule aus spitzen Laubflammen, am breitesten (4,2 m) im unteren Drittel und spitz auslaufend bei 25,96 m, über einem kurzen kahlen Stamm mit gerilltem Brettwurzelfuß, Längsrippen der Borke und Maserknollen. Seiten- und Vorderansicht zeigen die Flammen jedes Laubknochens als einen Umriss (vorne bis zu drei Gruppen nach Richtung, um 0,6 m überlappend, damit sich zwischen ihnen keine Splitter öffnen), nach Abstand zum Betrachter geschichtet; der Grundriss zeigt die Flammen von oben, die höheren über den tieferen, mit den acht Rig-Ästen als Hinweis."),
    retained: T("Retained: every flame group with its own outline, flank lines of the flames that face the viewer, leaf marks and ticks; root flare with its buttresses, the four stem parts with burls, and the eight rig branches as filled outlines under the flames; bark ridges and cracks on the stem.", "Beibehalten: jede Flammengruppe mit eigenem Umriss, Flankenlinien der zum Betrachter gewandten Flammen, Blattzeichen und Striche; Wurzelanlauf mit seinen Brettwurzeln, die vier Stammteile mit Maserknollen und die acht Rig-Äste als gefüllte Umrisse unter den Flammen; Borkenrippen und Risse am Stamm."),
    merged: T("The flames and the inner leaf mass of one foliage bone are merged into one outline (in front and plan into up to three groups); the 45 branches without their own bone merge with their stem part, where they show at all.", "Die Flammen und die innere Laubmasse eines Laubknochens sind zu einem Umriss vereinigt (vorne und im Grundriss zu bis zu drei Gruppen); die 45 Äste ohne eigenen Knochen gehen in ihrem Stammteil auf, wo sie überhaupt sichtbar sind."),
    omitted: T("Omitted: single leaves, petioles and buds, which are below 0.5 mm on paper; the leader tip line, now inside the spire.", "Weggelassen: einzelne Blätter, Blattstiele und Knospen, die auf dem Papier unter 0,5 mm liegen; die Linie der Gipfelspitze, die nun im Wipfel liegt."),
    thinElements: T("Leaf marks and ticks are the finest strokes (0.13 mm); flank lines use the detail pen.", "Blattzeichen und Striche sind die feinsten Linien (0,13 mm); Flankenlinien verwenden den Detailstift."),
  },
  100: {
    silhouette: T("At 1:100 (1 m is 10 mm) the drawings are the true views of the 1:100 body: the narrow flame-shaped column of broader leaf flames on a short stem with a buttressed foot; each foliage bone is one outline in the side elevation and two groups in front and plan, layered by distance; the four stem parts are drawn separately so they bend with the rig.", "Bei 1:100 (1 m sind 10 mm) sind die Zeichnungen die wahren Ansichten des 1:100-Körpers: die schmale, flammenförmige Säule aus breiteren Laubflammen auf einem kurzen Stamm mit Brettwurzelfuß; jeder Laubknochen ist in der Seitenansicht ein Umriss und vorne und im Grundriss zwei Gruppen, nach Abstand geschichtet; die vier Stammteile sind getrennt gezeichnet, damit sie sich mit dem Rig biegen."),
    retained: T("Retained: the flame outlines of the nine foliage bones, two flank lines per bone, the stem in four parts with flare and buttresses, two bark ridges, the eight rig branches under the flames (in the plan they lie under the crown, as seen from above).", "Beibehalten: die Flammenumrisse der neun Laubknochen, zwei Flankenlinien je Knochen, der Stamm in vier Teilen mit Anlauf und Brettwurzeln, zwei Borkenrippen, die acht Rig-Äste unter den Flammen (im Grundriss liegen sie, von oben gesehen, unter der Krone)."),
    merged: T("Flames are broader and fewer; bark relief is two ridge strokes; burls merge with the stem.", "Die Flammen sind breiter und weniger; das Borkenrelief sind zwei Rippenstriche; Maserknollen gehen im Stamm auf."),
    omitted: T("Omitted: leaf marks and ticks, inner flames above 19 m, the bark cracks.", "Weggelassen: Blattzeichen und Striche, innere Flammen über 19 m, die Borkenrisse."),
  },
  200: {
    silhouette: T("At 1:200 (1 m is 5 mm) the drawings are the true views of the 1:200 body: the flame-shaped column of broad leaf masses, one outline per foliage bone in the side elevation and two groups in front and plan, over the short stem with its flare; the stem parts are drawn separately.", "Bei 1:200 (1 m sind 5 mm) sind die Zeichnungen die wahren Ansichten des 1:200-Körpers: die flammenförmige Säule aus breiten Laubmassen, ein Umriss je Laubknochen in der Seitenansicht und zwei Gruppen vorne und im Grundriss, über dem kurzen Stamm mit Anlauf; die Stammteile sind getrennt gezeichnet."),
    retained: T("Retained: the nine foliage outlines, the stem in four parts with its flare and the eight rig branches under the masses.", "Beibehalten: die neun Laubumrisse, der Stamm in vier Teilen mit Anlauf und die acht Rig-Äste unter den Massen."),
    merged: T("Buttresses merge into the flare, flames into broad masses.", "Brettwurzeln gehen im Anlauf auf, Flammen in breiten Massen."),
    omitted: T("Omitted: flank lines, leaf marks, bark ridges and every texture stroke.", "Weggelassen: Flankenlinien, Blattzeichen, Borkenrippen und alle Texturstriche."),
  },
  500: {
    silhouette: T("At 1:500 (1 m is 2 mm) the Lombardy poplar is a narrow flame-shaped column, widest in its lower third and pointed at the top, on a short bare stem: the drawings are the true views of the 1:500 body, the crown four flame-shaped tiers with their few flame swells, each tier drawn on its stem part so that the column bends with the stem; the eight rig branches are hint lines over it in the side elevation and the plan (seen from ahead they lie in line with the stem and are not drawn). Plan: the crown as a slightly lobed oval round the stem.", "Bei 1:500 (1 m sind 2 mm) ist die Pyramidenpappel eine schmale, flammenförmige Säule, am breitesten im unteren Drittel und oben spitz, auf einem kurzen kahlen Stamm: die Zeichnungen sind die wahren Ansichten des 1:500-Körpers, die Krone vier flammenförmige Stufen mit ihren wenigen Flammenwülsten, jede Stufe an ihrem Stammteil gezeichnet, sodass sich die Säule mit dem Stamm biegt; die acht Rig-Äste sind Hinweislinien darüber in Seitenansicht und Grundriss (von vorn liegen sie in einer Flucht mit dem Stamm und werden nicht gezeichnet). Grundriss: die Krone als leicht gelapptes Oval um den Stamm."),
    retained: T("Retained: the short bare stem with its flare below the crown base at about 1.8 m, the column outline with its swells.", "Beibehalten: der kurze kahle Stamm mit Anlauf unter dem Kronenansatz bei etwa 1,8 m, der Säulenumriss mit seinen Wülsten."),
  },
};

export function draw(asset: Json, F: Record<string, Frame>, scales: ScaleParts[], axisX: (y: number) => number): string[] {
  const report: string[] = [];
  for (const { scale, wood, tufts, cores, burls, lobes } of scales) {
    const feature = (0.15 * scale) / 1000;
    const tolerance = feature / 4;
    const rep = asset.representations[String(scale)];
    Object.assign(rep.decisions, DRAWN[scale]);
    const everyOld: Json[] = VIEWS.flatMap((v) => rep.geometry[v]);
    for (const view of VIEWS) {
      const old: Json[] = rep.geometry[view];
      const template = (...ids: string[]): Json | null => {
        for (const id of ids) {
          const hit = old.find((s) => s.id === id) ?? everyOld.find((s) => s.id === id);
          if (hit) return strip(hit);
        }
        return null;
      };
      const next: Json[] = [];
      const closed = (base: Json, bone: string, rings: P2[][]): Json | null => (rings.length === 0 || rings[0]!.length < 3 ? null : { ...base, kind: "path", d: pathOf(toDrawing(view, F[bone]!, rings), true) });
      const open = (base: Json, bone: string, lines: P2[][]): Json | null => {
        const kept = lines.filter((l) => l.length >= 2 && l.slice(1).reduce((sum, q, i) => sum + Math.hypot(q.x - l[i]!.x, q.y - l[i]!.y), 0) > 2 * feature);
        return kept.length === 0 ? null : { ...base, kind: "path", d: pathOf(toDrawing(view, F[bone]!, kept), false) };
      };
      const push = (shape: Json | null): void => {
        if (shape) next.push(shape);
      };
      const out2 = (p: P2, y: number): P2 => (view === "elevation" ? { x: p.x - axisX(y), y: 0 } : view === "front" ? { x: p.x, y: 0 } : { x: p.x - axisX(y), y: p.y });
      const flank = (t: Tuft, k = 0.86): P2[] => {
        const pts = t.points.map((q) => ({ c: project(view, q.p), r: q.r, y: q.p[1] }));
        const mid = pts[3]!;
        const o = out2(mid.c, mid.y);
        const res: P2[] = [];
        for (let i = 1; i < pts.length - 1; i++) {
          const [a, b] = [pts[i - 1]!.c, pts[i + 1]!.c];
          let [dx, dy] = [b.x - a.x, b.y - a.y];
          const d = Math.hypot(dx, dy) || 1;
          [dx, dy] = [dx / d, dy / d];
          let [px, py] = [-dy, dx];
          if (px * o.x + py * o.y < 0) [px, py] = [-px, -py];
          res.push({ x: pts[i]!.c.x + px * pts[i]!.r * k, y: pts[i]!.c.y + py * pts[i]!.r * k });
        }
        return res;
      };
      const crownArc = (t: Tuft): P2[] => {
        const c = project(view, t.points[4]!.p);
        const mid = Math.atan2(c.y, c.x - axisX(t.c[1]));
        return Array.from({ length: 9 }, (_, i) => {
          const a = mid - 1.05 + (2.1 * i) / 8;
          return { x: c.x + 0.75 * t.radius * Math.cos(a), y: c.y + 0.75 * t.radius * Math.sin(a) };
        });
      };
      const visible = (t: Tuft): boolean => (view === "plan" ? t.c[1] > 0 : nearness(view, t.c) > -0.15 * Math.hypot(t.c[0] - axisX(t.c[1]), t.c[2]));

      const stemZ: Record<string, number> = { "root-plate": 3, "trunk-1": 3, "trunk-2": 3.1, "trunk-3": 3.2, "trunk-4": 3.3 };
      if (scale === 500) {
        const stemParts = wood.filter((t) => t.group === "stem" && t.bone === "trunk-1");
        const b = template("bole")!;
        if (view === "plan") push(closed({ ...b, z: 1 }, "trunk-1", outline(stemParts.flatMap((t) => tubePolygons(view, t)), tolerance, (3 * feature) ** 2)));
        else push(closed({ ...b, z: 1 }, "trunk-1", outline(stemParts.flatMap((t) => tubePolygons(view, t)), tolerance, (3 * feature) ** 2)));
        const c = template("crown", "crown-2")!;
        cores.forEach((k, i) => {
          const swells = lobes.filter((l) => l.c[1] >= k.rows[0]!.p[1] + 0.45 && (i + 1 === cores.length || l.c[1] < cores[i + 1]!.rows[0]!.p[1] + 0.45));
          push(closed({ ...c, id: k.id, bone: k.bone, z: r4(2 + (cores.length - i) * 0.01) }, k.bone, outline([...corePolygons(view, k), ...swells.map((l) => lobeOutline(view, l, 48))], tolerance, (4 * feature) ** 2)));
        });
        for (const rig of view === "front" ? [] : RIG) {
          const t = wood.find((x) => x.id === rig)!;
          const l = template(`limb-${rig}-line`)!;
          push(open({ ...l, id: `limb-${rig}-line`, bone: rig }, rig, [t.points.map((q) => project(view, q.p))]));
        }
      } else {
        const solidStem = (bone: string): Tube[] => wood.filter((t) => t.group === "stem" && t.bone === bone);
        const unboned = (bone: string): Tube[] => wood.filter((t) => t.group === "branch" && t.bone === bone);
        if (view === "plan") {
          const b = template("bole")!;
          const foot = wood.filter((t) => t.group === "stem" && (t.bone === "root-plate" || t.id === "trunk-1"));
          push(closed({ ...b, z: 1 }, "trunk-1", outline(foot.flatMap((t) => tubePolygons(view, t)), tolerance, (3 * feature) ** 2)));
          const dot = old.find((s) => s.id === "trunk-dot");
          if (dot) next.push({ ...dot, cx: 0, cy: 0, rx: 0.24, ry: 0.24 });
        } else {
          const ids = scale === 50 ? ["root-plate", ...STEMS] : STEMS;
          for (const bone of ids) {
            const id = scale === 50 ? (bone === "root-plate" ? "root-flare" : bone) : bone === "trunk-1" ? "bole" : bone;
            const base = template(id, scale === 50 ? "trunk-1" : "bole")!;
            delete base.followTip;
            const parts = [...solidStem(bone), ...(scale !== 50 && bone === "trunk-1" ? solidStem("root-plate") : []), ...unboned(bone)];
            const polys = [...parts.flatMap((t) => tubePolygons(view, t)), ...burls.filter((l) => l.bone === bone).map((l) => lobeOutline(view, l, 24))];
            push(closed({ ...base, id, bone: bone === "root-plate" && scale !== 50 ? "trunk-1" : bone, z: view === "front" ? r4(stemZ[bone]! - 1.5) : stemZ[bone], region: "stem-bark", ...(scale === 50 ? { material: "poplar-bark" } : {}) }, bone === "root-plate" && scale !== 50 ? "trunk-1" : bone, outline(polys, tolerance, (3 * feature) ** 2)));
          }
        }
        for (const rig of RIG) {
          const t = wood.find((x) => x.id === rig)!;
          if (view === "plan") {
            if (scale === 50) {
              const b = template(`pl-${rig}`)!;
              push(open({ ...b, id: `pl-${rig}`, bone: rig }, rig, [t.points.slice(1).map((q) => project(view, q.p))]));
            } else if (scale === 100) {
              const b = template(`plan-${rig}`, "plan-br-r1")!;
              delete b.followTip;
              push(closed({ ...b, id: `plan-${rig}`, bone: rig, z: 0.95 }, rig, outline(tubePolygons(view, t), tolerance, (3 * feature) ** 2)));
            }
            continue;
          }
          const b = template(rig)!;
          delete b.followTip;
          push(closed({ ...b, id: rig, bone: rig, region: "branch-bark" }, rig, outline(tubePolygons(view, t), tolerance, (3 * feature) ** 2)));
        }
        const near = (t: Tuft | Core): number => (view === "plan" ? (t.kind === "core" ? mean(t.rows.map((r) => r.p[1])) : t.c[1]) : t.kind === "core" ? mean(t.rows.map((r) => nearness(view, r.p))) : nearness(view, t.c));
        const groups: { id: string; bone: string; parts: (Tuft | Core)[] }[] = [];
        for (const bone of FOL) {
          const mine = tufts.filter((t) => t.bone === bone);
          const core = cores.filter((c) => c.bone === bone);
          if (view === "elevation") {
            groups.push({ id: bone, bone, parts: [...core, ...mine] });
            continue;
          }
          const prefix = view === "front" ? "f" : "p";
          const ids = old.filter((s) => s.bone === bone && s.filled && s.role === "body" && s.id.startsWith(`${prefix}-`)).map((s) => s.id as string);
          if (ids.length === 1) groups.push({ id: ids[0]!, bone, parts: [...core, ...mine] });
          else if (ids.length === 2) {
            const axis = view === "front" ? 2 : 2;
            groups.push({ id: ids[0]!, bone, parts: [...core, ...mine.filter((t) => t.c[axis] < 0)] }, { id: ids[1]!, bone, parts: mine.filter((t) => t.c[axis] >= 0) });
          } else ids.forEach((id, k) => groups.push({ id, bone, parts: [...(k === 0 ? core : []), ...mine.filter((t) => t.cluster === k)] }));
        }
        if (view === "front") for (const bone of FOL) {
          const mine = groups.filter((g) => g.bone === bone);
          if (mine.length < 2) continue;
          const own = mine.map((g) => g.parts.filter((p): p is Tuft => p.kind === "tube"));
          const span = own.map((ts) => [Math.min(...ts.map((t) => t.c[2])), Math.max(...ts.map((t) => t.c[2]))]);
          mine.forEach((g, i) => own.forEach((ts, k) => {
            if (k !== i && own[i]!.length > 0) g.parts.push(...ts.filter((t) => t.c[2] > span[i]![0]! - 0.6 && t.c[2] < span[i]![1]! + 0.6));
          }));
        }
        const ranked = [...groups].sort((a, b) => mean(a.parts.map(near)) - mean(b.parts.map(near)));
        for (const g of groups) {
          if (g.parts.length === 0) continue;
          const base = template(g.id)!;
          delete base.penMm;
          const depth = mean(g.parts.map(near));
          const z = view === "elevation" ? base.z : view === "front" ? (depth >= 0 || g.bone === "fol-top" ? 4 : 2) + ranked.indexOf(g) * 0.01 : 3 + (depth - 3) / 10 + ranked.indexOf(g) * 0.0005;
          const coreHidden = (view === "elevation" && g.bone.startsWith("fol-r")) || (view === "front" && depth >= 0);
          const polys = g.parts.filter((p) => !(coreHidden && p.kind === "core")).flatMap((p) => (p.kind === "core" ? corePolygons(view, p) : tubePolygons(view, p)));
          const area = (ring: P2[]): number => Math.abs(ring.reduce((sum, q, i) => sum + q.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * q.y, 0)) / 2;
          const rings = outline(polys, tolerance, (4 * feature) ** 2).filter((ring, i) => i === 0 || area(ring) >= Math.max((20 * feature) ** 2, 0.15));
          push(closed({ ...base, id: g.id, bone: g.bone, z: r4(z), region: "leaf-crown", ...(scale === 50 ? { material: "poplar-leaf" } : {}) }, g.bone, rings));
          const seen = g.parts.filter((p): p is Tuft => p.kind === "tube" && visible(p)).sort((a, b) => near(b) - near(a));
          const stroke = (id: string, lines: P2[][], dz: number): void => {
            const b = template(id);
            if (b) push(open({ ...b, id, bone: g.bone, filled: false, z: r4(z + dz) }, g.bone, lines));
          };
          if (view === "elevation" && scale === 50) {
            stroke(`${g.id}-arcs`, seen.filter((t) => t.role !== "inner").slice(0, 14).map((t) => flank(t)), 0.0002);
            const leaves: P2[][] = [], ticks: P2[][] = [];
            seen.slice(0, 12).forEach((t, i) => {
              for (const [f, side] of [[0.3, -0.35], [0.55, 0.2], [0.75, -0.1]] as [number, number][]) {
                const c = project(view, t.points[Math.round(f * (t.points.length - 1))]!.p);
                const x = c.x + side * t.radius, y = c.y;
                const w = 0.045 + ((i * 3) % 4) * 0.006;
                leaves.push([{ x: x - w, y: y - w * 0.7 }, { x, y: y + w * 0.9 }, { x: x + w, y: y - w * 0.7 }]);
                ticks.push([{ x: x - side * t.radius * 1.4, y: y - 0.12 }, { x: x - side * t.radius * 1.4 + 0.02, y: y - 0.04 }]);
              }
            });
            stroke(`${g.id}-leaves`, leaves, 0.0004);
            stroke(`${g.id}-fine`, ticks, 0.0006);
          } else if (view === "elevation" && scale === 100) {
            const pick = seen.filter((t) => t.role === "tip");
            stroke(`${g.id}-arc1-100`, pick.filter((_, i) => i % 2 === 0).slice(0, 4).map((t) => flank(t)), 0.0002);
            stroke(`${g.id}-arc2-100`, pick.filter((_, i) => i % 2 === 1).slice(0, 4).map((t) => flank(t)), 0.0004);
          } else if (view !== "elevation" && scale <= 100) {
            stroke(`${g.id}-arc`, seen.filter((t) => t.role !== "inner").slice(0, scale === 50 ? 5 : 3).map((t) => (view === "plan" ? crownArc(t) : flank(t))), 0.0005);
          }
        }
        if (view === "elevation" && (scale === 50 || scale === 100)) {
          const ridges = wood.filter((t) => t.group === "ridge" && nearness(view, t.points[3]!.p) > 0.02);
          const line = (t: Tube): P2[] => t.points.map((q) => project(view, q.p));
          if (scale === 50) {
            for (const [id, bone] of [["bark-trunk", "trunk-1"], ["bark-upper", "trunk-2"]] as const) {
              const b = template(id);
              if (b) push(open({ ...b, id, bone }, bone, ridges.filter((t) => t.bone === bone).map(line)));
            }
            const b = template("bark-trunk-fine");
            if (b) {
              const cracks: P2[][] = [];
              for (let y = 1.2; y < 6.1; y += 0.42) {
                const r = wood.find((t) => t.id === "trunk-1")!.points.find((q) => q.p[1] >= y)!.r;
                const x0 = axisX(y) + (((y * 7) % 1) - 0.5) * r * 1.2;
                cracks.push([{ x: x0 - 0.05, y }, { x: x0, y: y + 0.03 }, { x: x0 + 0.06, y: y - 0.01 }]);
              }
              push(open({ ...b, id: "bark-trunk-fine", bone: "trunk-1" }, "trunk-1", cracks));
            }
          } else {
            const t1 = wood.find((t) => t.id === "trunk-1")!;
            [[-0.38, "bole-furrow-1"], [0.32, "bole-furrow-2"]].forEach(([k, id]) => {
              const b = template(id as string);
              if (b) push(open({ ...b, id, bone: "trunk-1" }, "trunk-1", [t1.points.slice(1).map((q) => ({ x: q.p[0] + (k as number) * q.r, y: q.p[1] }))]));
            });
          }
        }
      }
      const gone = old.map((s) => s.id as string).filter((id) => !next.some((s) => s.id === id));
      const added = next.map((s) => s.id as string).filter((id) => !old.some((s) => s.id === id));
      rep.geometry[view] = next;
      report.push(`1:${scale} ${view}: ${next.length} shapes (was ${old.length}); removed ${gone.length ? gone.join(" ") : "none"}; new ${added.length ? added.join(" ") : "none"}`);
    }
    if (scale === 200) {
      const symbol = asset.symbols[0];
      for (const view of VIEWS) {
        const crown = symbol.geometry[view].find((s: Json) => s.id === "crown");
        const pts = [...tufts.flatMap((t) => outline(tubePolygons(view, t), 0.05, 1)[0] ?? []), ...cores.flatMap((c) => corePolygons(view, c).flat())];
        crown.points = pointsOf(hull(pts));
        const stemShape = symbol.geometry[view].find((s: Json) => s.id === "stem-1");
        if (view === "plan") stemShape.points = Array.from({ length: 24 }, (_, i) => [r4(0.28 * Math.cos((i * Math.PI) / 12)), r4(0.28 * Math.sin((i * Math.PI) / 12))]);
        else {
          const rows: [number, number][] = [[0, 0.56], [0.4, 0.315], [6.3, 0.233], [12.3, 0.169], [18.3, 0.114], [24.4, 0.03]];
          const x = (y: number): number => (view === "elevation" ? axisX(y) : 0);
          stemShape.points = [...rows.map(([y, r]) => [r4(x(y) - r), y]), ...[...rows].reverse().map(([y, r]) => [r4(x(y) + r), y])];
        }
        if (view === "plan") ["br-l1", "br-r1", "br-l2", "br-r2"].forEach((id, i) => {
          const t = wood.find((w) => w.id === id)!;
          const tip = t.points[t.points.length - 1]!.p;
          symbol.geometry.plan.find((s: Json) => s.id === `scaffold-${i + 1}`).points = [[0, 0], [r4(tip[0]), r4(tip[2])]];
        });
      }
      const d = asset.dimensions;
      const num = (n: number, comma: boolean): string => (comma ? n.toFixed(2).replace(".", ",") : n.toFixed(2));
      symbol.dimensionalNote.en = symbol.dimensionalNote.en.replace(/\([^)]*\)/, `(${num(d.widthM, false)} m wide, ${num(d.depthM, false)} m deep and ${num(d.heightM, false)} m high)`);
      symbol.dimensionalNote.de = symbol.dimensionalNote.de.replace(/\([^)]*\)/, `(${num(d.widthM, true)} m breit, ${num(d.depthM, true)} m tief und ${num(d.heightM, true)} m hoch)`);
    }
  }
  return report;
}

/** 🎈️ The convex hull of points (counter-clockwise). */
function hull(points: P2[]): P2[] {
  const sorted = [...points].sort((a, b) => a.x - b.x || a.y - b.y);
  const turn = (o: P2, a: P2, b: P2): number => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const half = (list: P2[]): P2[] => {
    const out: P2[] = [];
    for (const p of list) {
      while (out.length >= 2 && turn(out[out.length - 2]!, out[out.length - 1]!, p) <= 0) out.pop();
      out.push(p);
    }
    return out.slice(0, -1);
  };
  return [...half(sorted), ...half([...sorted].reverse())];
}

