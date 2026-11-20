/** ✏️ Ruling E1 for the Scots pine: every 2D shape of every scale and view is redrawn as the true orthographic view of that scale's body (one outline per bone and bark zone, one per needle cloud, layered by nearness to the viewer), the strokes inside them are restated on the new forms, and the plan symbol follows the new crown.
 * @see ./round3-brief.md */
import type { Frame, Json, P2, V3 } from "./volumes-lib.ts";
import { toLocal } from "./volumes-lib.ts";
import { lobeOutline, nearness, outline, pathOf, pointsOf, project, r4, toDrawing, tubePolygons, type Lobe, type Tube, type View } from "./r3-scots-pine-kit.ts";

export type ScaleParts = { scale: number; tubes: Tube[]; knots: Lobe[]; lobes: Lobe[] };
const VIEWS: View[] = ["elevation", "front", "plan"];
const BOLE: Record<string, string> = { "trunk-1": "bole-lower", "trunk-2": "bole-upper", "trunk-3": "bole-top", leader: "bole-leader" };
const GEOMETRY = ["kind", "d", "points", "closed", "followTip", "cx", "cy", "rx", "ry", "rotationDeg", "x0", "y0", "x1", "y1", "r0", "r1", "solid", "x", "y", "w", "h", "r"];
const strip = (shape: Json): Json => Object.fromEntries(Object.entries(shape).filter(([k]) => !GEOMETRY.includes(k)));
const inside = (p: P2, ring: P2[]): boolean => {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) hit = !hit;
  }
  return hit;
};
/** 🎈️ The convex hull of points (counter-clockwise), with vertices nearer than 0.25 m to the previous one dropped. */
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
  const ring = [...half(sorted), ...half(sorted.reverse())];
  return ring.filter((p, i) => i === 0 || Math.hypot(p.x - ring[i - 1]!.x, p.y - ring[i - 1]!.y) > 0.25 || [Math.min(...ring.map((q) => q.x)), Math.max(...ring.map((q) => q.x))].includes(p.x) || [Math.min(...ring.map((q) => q.y)), Math.max(...ring.map((q) => q.y))].includes(p.y));
}
const shrunk = (l: Lobe, k: number): Lobe => ({ ...l, radii: l.radii.map((r) => r * k) as V3 });
const mean = (values: number[]): number => values.reduce((s, v) => s + v, 0) / Math.max(1, values.length);
const regionOfTube = (t: Tube): string => (t.group === "root" || t.group === "lower" || t.group === "stub" ? "bark-lower" : t.group === "upper" ? "bark-upper" : "limb-bark");

const DRAWN: Record<number, Record<string, { en: string; de: string }>> = {
  50: {
    silhouette: { en: "At 1:50 (1 m is 20 mm) the three drawings are the true views of the 1:50 body: the root flare with its surface roots, the straight bole in dark plated bark below and orange flaking bark above, two dead stubs and three snags, six crooked limbs that leave the bole in different directions, and nine flat needle plates (seven on the limbs and the leader, two small ones on the upper bole) whose outlines follow their lobes, with the branches showing under them. Side and front elevation layer the plates by their distance from the viewer, so plates behind the bole are drawn behind it; the plan shows the plates from above, higher ones over lower ones, with the limbs and branches as hints over them.", de: "Bei 1:50 (1 m sind 20 mm) sind die drei Zeichnungen die wahren Ansichten des 1:50-Körpers: der Wurzelanlauf mit seinen Wurzelanläufen, der gerade Schaft unten in dunkler Plattenborke und oben in oranger Spiegelrinde, zwei tote Aststummel und drei Totäste, sechs krumme Äste, die den Schaft in verschiedene Richtungen verlassen, und neun flache Nadelplatten (sieben an den Ästen und am Gipfeltrieb, zwei kleine am oberen Schaft), deren Umrisse ihren Lappen folgen und unter denen die Zweige sichtbar sind. Seiten- und Vorderansicht schichten die Platten nach ihrem Abstand zum Betrachter, Platten hinter dem Schaft liegen also dahinter; der Grundriss zeigt die Platten von oben, höhere über tieferen, mit den Ästen und Zweigen als Hinweis darüber." },
    retained: { en: "Retained: every needle cloud with its own scalloped outline, inner lobe arcs, needle fans and needle ticks; root flare, bole parts, stubs, limbs and the branch systems under the plates as filled outlines; bark plates and furrows on the lower bole, flake marks on the upper bole, a ridge line on the two middle limbs.", de: "Beibehalten: jede Nadelwolke mit eigenem gewelltem Umriss, inneren Lappenbögen, Nadelfächern und Nadelstrichen; Wurzelanlauf, Schaftteile, Stummel, Äste und die Zweigsysteme unter den Platten als gefüllte Umrisse; Borkenplatten und Furchen am unteren Schaft, Schuppenmarken am oberen Schaft, eine Gratlinie an den zwei mittleren Ästen." },
    merged: { en: "The lobes of one cloud are merged into one outline; a plate has up to three clouds in the front elevation and the plan. Roots merge with the root flare, snags and knots with their bole part.", de: "Die Lappen einer Wolke sind zu einem Umriss vereinigt; eine Platte hat in der Vorderansicht und im Grundriss bis zu drei Wolken. Wurzeln gehen im Wurzelanlauf auf, Totäste und Astnarben in ihrem Schaftteil." },
    omitted: { en: "Omitted: single needles, cones and bud scales, which are below 0.5 mm on paper; bark texture is not repeated in the front elevation and the plan.", de: "Weggelassen: einzelne Nadeln, Zapfen und Knospenschuppen, die auf dem Papier unter 0,5 mm liegen; die Borkentextur wird in Vorderansicht und Grundriss nicht wiederholt." },
  },
  100: {
    silhouette: { en: "At 1:100 (1 m is 10 mm) the drawings are the true views of the 1:100 body: flared root, two-tone bole, two stubs and one snag, six limbs in different directions and nine needle plates with broader lobes; the branch under each plate with two side branches shows below the needles. Plates behind the bole are drawn behind it. Plan: the plates from above, the higher over the lower, with limb and branch hints.", de: "Bei 1:100 (1 m sind 10 mm) sind die Zeichnungen die wahren Ansichten des 1:100-Körpers: ausgestellter Wurzelanlauf, zweifarbiger Schaft, zwei Stummel und ein Totast, sechs Äste in verschiedene Richtungen und neun Nadelplatten mit breiteren Lappen; der Ast unter jeder Platte mit zwei Seitenästen ist unter den Nadeln sichtbar. Platten hinter dem Schaft liegen dahinter. Grundriss: die Platten von oben, die höheren über den tieferen, mit Ast- und Zweighinweisen." },
    retained: { en: "Retained: nine needle plates as scalloped outlines (the two lowest and the two small inner ones marked as shade needles), three needle arcs per plate, root flare, the four bole parts, six limbs and two small limbs as tapered outlines, the two dead stubs, five bark plate marks on the lower bole.", de: "Beibehalten: neun Nadelplatten als gewellte Umrisse (die zwei untersten und die zwei kleinen inneren als Schattennadeln markiert), drei Nadelbögen je Platte, Wurzelanlauf, die vier Schaftteile, sechs Äste und zwei kleine Äste als verjüngte Umrisse, die zwei toten Stummel, fünf Borkenplattenmarken am unteren Schaft." },
    merged: { en: "Shoot tufts are merged into broader lobes, the surface roots into one flared root, bark relief into a few plate marks.", de: "Triebbüschel gehen in breiteren Lappen auf, die Wurzelanläufe in einem ausgestellten Wurzelanlauf, das Borkenrelief in wenigen Plattenmarken." },
    omitted: { en: "Omitted: needle fans and ticks, knots, two of the three snags, the fork of the west stub and the finer side branches.", de: "Weggelassen: Nadelfächer und Nadelstriche, Astnarben, zwei der drei Totäste, die Gabel des westlichen Stummels und die feineren Seitenäste." },
  },
  200: {
    silhouette: { en: "At 1:200 (1 m is 5 mm) the drawings are the true views of the 1:200 body: the two-tone bole with its root flare and two stubs, six limbs and two small limbs as tapered outlines, and nine needle plates of two to five masses each; plates behind the bole are drawn behind it. Plan: the plates from above with the bole as a dot.", de: "Bei 1:200 (1 m sind 5 mm) sind die Zeichnungen die wahren Ansichten des 1:200-Körpers: der zweifarbige Schaft mit Wurzelanlauf und zwei Stummeln, sechs Äste und zwei kleine Äste als verjüngte Umrisse und neun Nadelplatten aus je zwei bis fünf Massen; Platten hinter dem Schaft liegen dahinter. Grundriss: die Platten von oben mit dem Schaft als Punkt." },
    retained: { en: "Retained: root flare, the four bole parts, the two stubs (0.3 mm on paper), the limbs and the needle plates as separate outlines, the two lowest plates and the small inner ones in the shade tone.", de: "Beibehalten: Wurzelanlauf, die vier Schaftteile, die zwei Stummel (0,3 mm auf dem Papier), die Äste und die Nadelplatten als getrennte Umrisse, die zwei untersten Platten und die kleinen inneren im Schattenton." },
    merged: { en: "The lobes of a plate are merged into two to five masses; the branches under the plates vanish inside them.", de: "Die Lappen einer Platte sind zu zwei bis fünf Massen vereinigt; die Zweige unter den Platten verschwinden darin." },
    omitted: { en: "Omitted: snags, knots, bark plates, needle texture and every texture stroke.", de: "Weggelassen: Totäste, Astnarben, Borkenplatten, Nadeltextur und alle Texturstriche." },
  },
  500: {
    silhouette: { en: "At 1:500 (1 m is 2 mm) the Scots pine is a long bare two-tone bole with its root flare and, above 12 m, six bare limbs carrying an open, flat-topped crown of nine needle plates in tiers, drawn as one crown shape through which the sky shows between the tiers; the drawings are the true views of the 1:500 body. Plan: the crown as one lobed outline around the bole dot.", de: "Bei 1:500 (1 m sind 2 mm) ist die Waldkiefer ein langer, kahler, zweifarbiger Schaft mit Wurzelanlauf und oberhalb von 12 m sechs kahlen Ästen, die eine lichte, schirmförmige Krone aus neun Nadelplatten in Etagen tragen, gezeichnet als eine Kronenform, durch die zwischen den Etagen der Himmel scheint; die Zeichnungen sind die wahren Ansichten des 1:500-Körpers. Grundriss: die Krone als ein gelappter Umriss um den Schaftpunkt." },
    retained: { en: "Retained: the bole in two tones (dark plated lower bark, light flaking upper bark), the six limbs as tapered outlines, the crown outline with its gaps.", de: "Beibehalten: der Schaft in zwei Tönen (dunkle Plattenborke unten, helle Spiegelrinde oben), die sechs Äste als verjüngte Umrisse, der Kronenumriss mit seinen Lücken." },
    merged: { en: "Merged into the crown path: the nine needle plates, twenty masses in all, and the two small inner limbs.", de: "Im Kronenpfad vereinigt: die neun Nadelplatten, insgesamt zwanzig Massen, und die zwei kleinen inneren Äste." },
    omitted: { en: "Omitted: snags, bark plates and needle texture; the two dead stubs are single hairlines.", de: "Weggelassen: Totäste, Borkenplatten und Nadeltextur; die zwei toten Stummel sind einzelne Haarlinien." },
    overlays: { en: "Thermal and structural overlays map onto bark-lower and bark-upper (the bole paths), limb-bark (the six limb paths) and, merged in the crown path, needles-sun and needles-shade. Merged paths show the area-weighted value of their regions, labelled as merged.", de: "Thermische und statische Überlagerungen liegen auf bark-lower und bark-upper (den Schaftpfaden), limb-bark (den sechs Astpfaden) und, im Kronenpfad vereinigt, needles-sun und needles-shade. Vereinigte Pfade zeigen den flächengewichteten Wert ihrer Regionen und sind als vereinigt beschriftet." },
  },
};

type Furrow = [y0: number, y1: number, f0: number, f1: number, bow: number];
const PLATES: Furrow[] = [[0.5, 1.5, -0.55, -0.5, 0.06], [0.45, 1.2, 0.1, 0.18, -0.05], [0.7, 2.0, 0.6, 0.55, 0.05], [1.4, 2.6, -0.2, -0.12, -0.06], [1.9, 3.3, -0.62, -0.55, 0.05], [2.3, 3.4, 0.25, 0.32, 0.05], [2.9, 4.3, 0.66, 0.6, -0.05], [3.2, 4.4, -0.15, -0.22, 0.06], [3.9, 5.2, -0.6, -0.52, -0.05], [4.5, 5.5, 0.3, 0.22, -0.05], [5.0, 6.3, 0.62, 0.56, 0.05], [5.6, 6.6, -0.18, -0.1, 0.05], [6.0, 7.1, -0.58, -0.5, 0.04], [6.4, 7.2, 0.26, 0.3, -0.04]];
const CRACKS: Furrow[] = [[0.85, 0.9, -0.5, -0.05, 0.02], [1.25, 1.2, 0.15, 0.58, -0.02], [1.75, 1.8, -0.18, 0.5, 0.02], [2.35, 2.3, -0.6, -0.18, -0.02], [2.85, 2.9, 0.3, 0.62, 0.02], [3.6, 3.55, -0.2, 0.25, -0.02], [4.15, 4.2, -0.56, -0.2, 0.02], [4.75, 4.7, 0.26, 0.6, -0.02], [5.35, 5.4, -0.15, 0.28, 0.02], [5.9, 5.85, -0.55, -0.16, -0.02], [6.5, 6.55, 0.28, 0.58, 0.02], [6.95, 6.9, -0.5, 0.2, -0.02]];
const FLAKES: Furrow[] = [[7.9, 7.96, -0.5, 0.1, 0.03], [8.5, 8.44, 0.05, 0.6, -0.03], [9.1, 9.16, -0.6, -0.1, 0.03], [9.75, 9.7, -0.2, 0.45, -0.03], [10.4, 10.46, 0.1, 0.62, 0.03], [10.95, 10.9, -0.55, 0.0, -0.03], [11.6, 11.66, -0.1, 0.5, 0.03], [12.2, 12.15, -0.6, -0.05, -0.03], [12.8, 12.86, 0.0, 0.55, 0.03]];
const FLAKES_FINE: Furrow[] = [[13.8, 13.84, -0.5, 0.1, 0.02], [14.5, 14.46, 0.0, 0.55, -0.02], [15.1, 15.14, -0.55, 0.0, 0.02], [15.7, 15.66, -0.1, 0.5, -0.02], [16.3, 16.34, -0.5, 0.05, 0.02], [17.0, 16.96, 0.0, 0.5, -0.02]];

export function draw(asset: Json, F: Record<string, Frame>, scales: ScaleParts[]): string[] {
  const report: string[] = [];
  const outlines: Record<string, P2[][]> = {};
  for (const { scale, tubes, knots, lobes } of scales) {
    const feature = (0.15 * scale) / 1000;
    const tolerance = feature / 4;
    const rep = asset.representations[String(scale)];
    Object.assign(rep.decisions, DRAWN[scale]);
    const everyOld: Json[] = VIEWS.flatMap((v) => rep.geometry[v]);
    const stemTube = (bone: string): Tube | undefined => tubes.find((t) => t.id === bone);
    const radiusAt = (bone: string, y: number): { x: number; r: number } => {
      const t = stemTube(bone)!;
      for (let i = 0; i + 1 < t.points.length; i++) {
        const [a, b] = [t.points[i]!, t.points[i + 1]!];
        if (y <= b.p[1] || i + 2 === t.points.length) {
          const k = Math.max(0, Math.min(1, (y - a.p[1]) / (b.p[1] - a.p[1])));
          return { x: a.p[0] + (b.p[0] - a.p[0]) * k, r: a.r + (b.r - a.r) * k };
        }
      }
      return { x: 0, r: 0.2 };
    };
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
      const frameOf = (bone: string): Frame => F[bone]!;
      const closed = (base: Json, bone: string, rings: P2[][]): Json | null => (rings.length === 0 || rings[0]!.length < 3 ? null : { ...base, kind: "path", d: pathOf(toDrawing(view, frameOf(bone), rings), true) });
      const open = (base: Json, bone: string, lines: P2[][]): Json | null => {
        const kept = lines.filter((l) => l.length >= 2);
        return kept.length === 0 ? null : { ...base, kind: "path", d: pathOf(toDrawing(view, frameOf(bone), kept), false) };
      };
      const push = (shape: Json | null): void => {
        if (shape) next.push(shape);
      };
      const stemNear = view === "front" ? -0.05 : 0;
      const padKeys = [...new Set(lobes.map((l) => l.pad))];
      const padNear = (key: string): number => mean(lobes.filter((l) => l.pad === key).map((l) => nearness(view, l.c)));
      const ranked = [...padKeys].sort((a, b) => padNear(a) - padNear(b));
      const padZ = (key: string): number => {
        if (view === "plan") return 3 + (mean(lobes.filter((l) => l.pad === key).map((l) => l.c[1])) - 12) / 5;
        return (padNear(key) >= stemNear - 0.05 ? 4.3 : 2.5) + ranked.indexOf(key) * 0.01;
      };
      const woodKeys = [...new Set(tubes.map((t) => t.shape))];
      for (const key of woodKeys) {
        const mine = tubes.filter((t) => t.shape === key);
        const first = mine[0]!;
        const bone = first.bone;
        const id = BOLE[key] !== undefined && !(scale === 50 && view === "elevation") ? BOLE[key]! : key;
        const region = regionOfTube(first);
        const like = region === "limb-bark" ? ["limb-w3"] : region === "bark-upper" ? ["bole-upper", "trunk-2"] : first.group === "stub" ? ["stub-w", "bole-lower"] : ["bole-lower", "trunk-1"];
        const base: Json = template(id, ...like) ?? { role: "body", z: 1, filled: true, tone: 0.4 };
        delete base.penMm;
        delete base.regions;
        const material = scale === 50 ? { material: first.material } : {};
        const polygons = [...mine.flatMap((t) => tubePolygons(view, t)), ...knots.filter((k) => k.bone === bone && mine.some((t) => t.id === bone)).map((k) => lobeOutline(view, k, 24))];
        const near = mean(mine.flatMap((t) => t.points.map((q) => nearness(view, q.p))));
        let z: number;
        if (view === "plan") z = first.group === "root" ? 0.9 : 1 + Math.max(...mine.flatMap((t) => t.points.map((q) => q.p[1]))) / 10;
        else if (first.group === "root") z = 3;
        else if (first.group === "lower" || first.group === "upper") z = 3 + ["trunk-1", "trunk-2", "trunk-3", "leader"].indexOf(key) * 0.1;
        else if (first.group === "stub") z = 2.9;
        else if (first.group === "limb") z = near >= stemNear ? 3.6 : 2.4;
        else z = padZ(key === "pl-top-wood" ? "top" : key.slice(3, 5)) - 0.005;
        if (first.flags?.slender) push(open({ ...base, id, bone, role: "body", filled: false, z: r4(z), region, penMm: 0.1, ...material }, bone, mine.map((t) => t.points.map((q) => project(view, q.p)))));
        else push(closed({ ...base, id, bone, role: "body", filled: true, z: r4(z), region, ...material }, bone, outline(polygons, tolerance, (3 * feature) ** 2)));
      }
      const hasArc = old.some((s) => /-arc$/.test(s.id));
      const visible = lobes.filter((l) => !lobes.some((o) => o !== l && nearness(view, o.c) > nearness(view, l.c) && inside(project(view, l.c), lobeOutline(view, o, 24))));
      const groups: { id: string; bone: string; key: string; cluster: number; mine: Lobe[] }[] = [];
      for (const key of padKeys) {
        const all = lobes.filter((l) => l.pad === key);
        if (scale === 500) continue;
        const name = key === "top" ? "top" : key;
        if (view === "elevation") groups.push({ id: `pl-${name}`, bone: all[0]!.bone, key, cluster: 0, mine: all });
        else {
          const prefix = view === "front" ? "f" : "p";
          const ids = old.map((s) => s.id as string).filter((id) => new RegExp(`^${prefix}-${name}-\\d+$`).test(id)).sort((a, b) => Number(a.split("-")[2]) - Number(b.split("-")[2]));
          if (ids.length === 0) ids.push(`${prefix}-${name}-${key === "n" ? 17 : 18}`);
          ids.forEach((id, c) => {
            const mine = all.filter((l) => Math.min(l.cluster, ids.length - 1) === c);
            if (mine.length > 0) groups.push({ id, bone: all[0]!.bone, key, cluster: c, mine });
          });
        }
      }
      if (scale === 500) groups.push({ id: "crown", bone: "trunk-3", key: "crown", cluster: 0, mine: lobes });
      for (const g of groups) {
        const shade = g.mine[0]!.group === "shade";
        const likePad = shade ? "w3" : "w1";
        const base: Json = template(g.id, view === "elevation" ? `pl-${likePad}` : `${view === "front" ? "f" : "p"}-${likePad}-${shade ? 0 : 2}`)!;
        delete base.penMm;
        const depthOf = (c: number): number => mean(groups.filter((x) => x.key === g.key && x.cluster === c).flatMap((x) => x.mine.map((l) => (view === "plan" ? l.c[1] : nearness(view, l.c)))));
        const order = [...new Set(groups.filter((x) => x.key === g.key).map((x) => x.cluster))].sort((a, b) => depthOf(a) - depthOf(b));
        const z = g.key === "crown" ? base.z : padZ(g.key) + 0.001 * order.indexOf(g.cluster);
        const material = scale === 50 ? { material: g.mine[0]!.material } : {};
        const rings = outline(g.mine.map((l) => lobeOutline(view, l, 48)), tolerance, (4 * feature) ** 2);
        if (g.key === "crown") outlines[`crown-${view}`] = rings;
        push(closed({ ...base, id: g.id, bone: g.bone, z: r4(z), ...(g.key === "crown" ? {} : { region: shade ? "needles-shade" : "needles-sun", ...material }) }, g.bone, rings));
        const seen = g.mine.filter((l) => visible.includes(l)).sort((a, b) => nearness(view, b.c) - nearness(view, a.c));
        const arcOf = (l: Lobe): P2[] => {
          const c = project(view, l.c);
          const mid = view === "plan" ? (Math.atan2(c.y, c.x) * 180) / Math.PI : 90;
          return lobeOutline(view, shrunk(l, 0.82), 10, mid - 62, mid + 62);
        };
        const stroke = (id: string, like: string, lines: P2[][], dz: number): void => {
          const b = template(id, like);
          if (b) push(open({ ...b, id, bone: g.bone, filled: false, z: r4(z + dz) }, g.bone, lines));
        };
        if (view === "elevation" && scale === 50) {
          stroke(`${g.id}-arcs`, "pl-w3-arcs", seen.filter((l) => l.size !== "tuft").map(arcOf), 0.0002);
          const fans: P2[][] = [], ticks: P2[][] = [];
          seen.forEach((l, i) => {
            const c = project(view, l.c);
            const ring = lobeOutline(view, l, 16);
            const [hw, hh] = [(Math.max(...ring.map((p) => p.x)) - Math.min(...ring.map((p) => p.x))) / 2, (Math.max(...ring.map((p) => p.y)) - Math.min(...ring.map((p) => p.y))) / 2];
            const spots: [number, number][] = l.size === "core" ? [[-0.5, -0.1], [0.05, 0.25], [0.5, -0.2]] : l.size === "rim" ? [[-0.35, 0], [0.35, 0.1]] : [[0, 0.1]];
            spots.forEach(([fx, fy], k) => {
              const base: P2 = { x: c.x + fx * hw, y: c.y + fy * hh - 0.05 };
              for (const deg of [58 + ((i + k) % 3) * 6, 90, 122 - ((i + k) % 3) * 6]) fans.push([base, { x: base.x + 0.12 * Math.cos((deg * Math.PI) / 180), y: base.y + 0.12 * Math.sin((deg * Math.PI) / 180) }]);
              const tick: P2 = { x: c.x - fx * 0.7 * hw + 0.08, y: c.y - fy * hh * 0.8 - 0.02 };
              const lean = 78 + ((i * 7 + k * 13) % 5) * 6;
              ticks.push([tick, { x: tick.x + 0.075 * Math.cos((lean * Math.PI) / 180), y: tick.y + 0.075 * Math.sin((lean * Math.PI) / 180) }]);
            });
          });
          stroke(`${g.id}-leaves`, "pl-w3-leaves", fans, 0.0004);
          stroke(`${g.id}-fine`, "pl-w3-fine", ticks, 0.0006);
        } else if (view === "elevation" && scale === 100) {
          [...seen.filter((l) => l.size === "core"), ...seen.filter((l) => l.size === "rim")].slice(0, 3).forEach((l, i) => stroke(`${g.id}-needles${i + 1}`, "pl-w3-needles1", [arcOf(l)], 0.0002 * (i + 1)));
        } else if (view !== "elevation" && hasArc) {
          stroke(`${g.id}-arc`, `${view === "front" ? "f" : "p"}-w3-0-arc`, [...seen.filter((l) => l.size === "core"), ...seen.filter((l) => l.size === "rim")].slice(0, 3).map(arcOf), 0.0005);
        }
        if (view !== "elevation" && scale <= 100 && PADS_WITH_LINES.includes(g.key)) {
          const number = g.id.split("-")[2];
          const id = `${view === "front" ? "fl" : "pl"}-${g.key}-${number}`;
          const boneWood = tubes.filter((t) => t.shape === `${g.bone}-wood`);
          const pick = g.cluster === 0 ? boneWood.filter((t) => t.group === "pad") : boneWood.filter((t) => (g.cluster === 1 ? /-[ac]$/.test(t.id) : /-[bd]$/.test(t.id)));
          const b = template(id, `${view === "front" ? "fl" : "pl"}-w3-0`);
          if (b && pick.length > 0) push(open({ ...b, id, bone: g.bone, filled: false, z: view === "plan" ? r4(7 + Number(number) / 1000) : r4(1 + Number(number) / 1000), region: "limb-bark" }, g.bone, pick.map((t) => t.points.slice(1).map((q) => project(view, q.p)))));
        }
      }
      if (view === "elevation" && scale === 50) {
        const furrows = (id: string, bone: string, rows: Furrow[], vertical: boolean): void => {
          const b = template(id);
          if (!b) return;
          const lines = rows.map(([y0, y1, f0, f1, bow]) => {
            const [a, z] = [radiusAt(bone, y0), radiusAt(bone, y1)];
            const p0: P2 = { x: a.x + f0 * a.r, y: y0 }, p1: P2 = { x: z.x + f1 * z.r, y: y1 };
            const mid: P2 = vertical ? { x: (p0.x + p1.x) / 2 + bow * a.r * 2, y: (y0 + y1) / 2 } : { x: (p0.x + p1.x) / 2, y: (y0 + y1) / 2 + bow * 2 };
            return [p0, mid, p1];
          });
          const f = frameOf(bone);
          const loc = (p: P2): string => {
            const l = toLocal(f, [p.x, p.y, 0]);
            return `${r4(l[0])} ${r4(l[1])}`;
          };
          push({ ...b, id, bone, kind: "path", d: lines.map(([p0, m, p1]) => `M ${loc(p0!)} Q ${loc(m!)} ${loc(p1!)}`).join(" ") });
        };
        furrows("bark-plates", "trunk-1", PLATES, true);
        furrows("bark-furrows", "trunk-1", CRACKS, false);
        furrows("bark-flakes", "trunk-2", FLAKES, false);
        furrows("bark-flakes-fine", "trunk-3", FLAKES_FINE, false);
        for (const [id, bone] of [["limb-lines", "limb-w1"], ["limb-lines-e", "limb-e1"]] as const) {
          const t = stemTube(bone);
          const b = template(id);
          if (t && b) push(open({ ...b, id, bone, z: bone === "limb-w1" ? 3.61 : 2.41 }, bone, [t.points.slice(2, -1).map((q) => ({ x: q.p[0], y: q.p[1] + 0.35 * q.r }))]));
        }
      }
      if (view === "elevation" && scale === 100) for (const s of old) if (/^plate-\d$/.test(s.id)) next.push(s);
      if (view === "plan") {
        const dot = old.find((s) => s.id === "trunk-dot");
        if (dot) next.push({ ...dot, cx: 0, cy: 0, rx: 0.28, ry: 0.28 });
        const likeCapsule = everyOld.find((s) => s.id === "plan-limb-w3");
        if (likeCapsule) for (const t of tubes.filter((x) => x.group === "limb")) {
          const [a, z] = [t.points[0]!, t.points[t.points.length - 1]!];
          const [pa, pz] = [project(view, a.p), project(view, z.p)];
          void pa;
          void pz;
          push(open({ ...strip(likeCapsule), id: `plan-${t.id}`, bone: t.bone, filled: false }, t.bone, [t.points.slice(1).map((q) => project(view, q.p))]));
        }
      }
      const gone = old.map((s) => s.id as string).filter((id) => !next.some((s) => s.id === id));
      const added = next.map((s) => s.id as string).filter((id) => !old.some((s) => s.id === id));
      rep.geometry[view] = next;
      report.push(`1:${scale} ${view}: ${next.length} shapes (was ${old.length}); removed ${gone.length ? gone.join(" ") : "none"}; new ${added.length ? added.join(" ") : "none"}; path characters ${next.reduce((n, s) => n + (s.d?.length ?? 0), 0)} (was ${old.reduce((n: number, s: Json) => n + (s.d?.length ?? JSON.stringify(s.points ?? "").length), 0)})`);
    }
    if (scale === 200) {
      const symbol = asset.symbols[0];
      for (const view of VIEWS) {
        const crown = symbol.geometry[view].find((s: Json) => s.id === "crown");
        crown.points = pointsOf(hull(lobes.flatMap((l) => lobeOutline(view, l, 36))));
        const stemShape = symbol.geometry[view].find((s: Json) => s.id === "stem-1");
        if (view === "plan") stemShape.points = Array.from({ length: 24 }, (_, i) => [r4(0.28 * Math.cos((i * Math.PI) / 12)), r4(0.28 * Math.sin((i * Math.PI) / 12))]);
        else {
          const rows: [number, number][] = [[0, 0.46], [0.44, 0.31], [3.9, 0.252], [7.4, 0.218], [13.4, 0.146], [17.6, 0.098], [20.4, 0.045]];
          const x = (y: number): number => (view === "elevation" ? radiusAt(y < 7.4 ? "trunk-1" : y < 13.4 ? "trunk-2" : y < 17.6 ? "trunk-3" : "leader", y).x : 0);
          stemShape.points = [...rows.map(([y, r]) => [r4(x(y) - r), y]), ...[...rows].reverse().map(([y, r]) => [r4(x(y) + r), y])];
        }
        if (view === "plan") ["limb-w3", "limb-e3", "limb-w1", "limb-e1"].forEach((id, i) => {
          const t = tubes.find((x) => x.id === id)!;
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
const PADS_WITH_LINES = ["w3", "e3", "w1", "e1", "w2", "e2", "top"];
