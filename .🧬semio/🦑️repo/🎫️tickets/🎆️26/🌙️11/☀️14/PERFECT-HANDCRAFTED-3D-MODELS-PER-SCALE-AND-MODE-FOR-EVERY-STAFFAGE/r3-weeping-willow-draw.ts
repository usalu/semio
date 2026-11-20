/** ✏️ Ruling E1 for the weeping willow: every 2D view of every scale is redrawn as the true orthographic view of that scale's body, part by part in depth order (far first), with the parts that the nearer ones hide completely left out; strands on the tresses, arcs under the lobes and bark fissures on the stem are drawn on the visible faces.
 * @see ./round3-brief.md */
import type { Frame, Json, P2, V3 } from "./volumes-lib.ts";
import { add, dot3, inRing, lobeOutline, mul, nearness, outline, partPolygons, pathOf, pointsOf, project, r4, secFrame, toDrawing, VIEW_AXES, type Lobe, type Loft, type Part, type View } from "./r3-weeping-willow-kit.ts";

type Model = { scale: number; parts: Part[] };
const VIEWS: View[] = ["elevation", "front", "plan"];
const regionOf = (p: Part): [string, string] => (p.group === "root" || p.group === "stem" ? ["trunk-bark", "willow-bark"] : p.group === "limb" || p.group === "branch" ? ["limb-bark", "willow-bark"] : ["leaf-curtain", "willow-leaf"]);
const isWood = (p: Part): boolean => ["root", "stem", "limb", "branch"].includes(p.group);

/** 🔭️ The largest nearness of a part to the viewer of a view. */
function frontMost(view: View, p: Part): number {
  const d = VIEW_AXES[view][2];
  if (p.kind === "lobe") return nearness(view, p.c) + Math.hypot(...([0, 1, 2].map((i) => p.radii[i]! * dot3(p.axes[i]!, d)) as [number, number, number]));
  return Math.max(...p.secs.flatMap((s, i) => {
    const [x, y] = secFrame(p, i);
    return [nearness(view, s.p) + Math.hypot(s.rx * dot3(x, d), s.ry * dot3(y, d))];
  }));
}

/** 🧵️ Strand lines on the visible face of a tress at the given fractions of its width. */
function strands(view: View, l: Loft, fractions: number[]): P2[][] {
  const d = VIEW_AXES[view][2];
  const keep = l.secs.map((_, i) => i > 0 && i < l.secs.length - 1 && (view !== "plan" || Math.abs(secFrame(l, i)[2][1]) < 0.85));
  return fractions.map((k, j) => l.secs.flatMap((s, i) => {
    if (!keep[i]) return [];
    const [x, y] = secFrame(l, i);
    const kk = Math.max(-0.95, Math.min(0.95, k + 0.08 * Math.sin(i * 1.7 + j)));
    const c = Math.sqrt(1 - kk * kk) * (dot3(x, d) * s.rx >= 0 ? 1 : -1);
    return [project(view, add(add(s.p, mul(x, s.rx * c * 0.98)), mul(y, s.ry * kk * 0.98)))];
  }));
}

/** 🪵️ Fissure lines on the visible face of a stem part: wavy lines along it at fractions of its width. */
function fissures(view: View, l: Loft, fractions: number[]): P2[][] {
  const d = VIEW_AXES[view][2];
  return fractions.map((k, j) => l.secs.map((s, i) => {
    const [x, y] = secFrame(l, i);
    const kk = Math.max(-0.9, Math.min(0.9, k + 0.07 * Math.sin(i * 2.3 + j * 1.3)));
    const ax = Math.abs(dot3(x, d)) < Math.abs(dot3(y, d)) ? x : y;
    const other = ax === x ? y : x;
    const r = ax === x ? s.rx : s.ry, ro = ax === x ? s.ry : s.rx;
    const c = Math.sqrt(1 - kk * kk) * (dot3(other, d) >= 0 ? 1 : -1);
    return project(view, add(add(s.p, mul(ax, r * kk)), mul(other, ro * c)));
  }));
}

const DECISIONS: Record<number, Json> = {
  50: {
    silhouette: { en: "At 1:50 (1 m is 20 mm) every view is the true orthographic view of the 1:50 model: the fountain crown of 48 tresses arching from the apex over the dome and hanging to a ragged hem, the arches drawn over the hanging parts they lead into, the inner tresses where they show, the short stout stem with flare and surface roots; in plan the tresses radiate from the apex. Parts the nearer ones hide completely are not drawn.", de: "Bei 1:50 (1 m sind 20 mm) ist jede Ansicht die wahre Parallelprojektion des 1:50-Modells: die Springbrunnenkrone aus 48 Strähnen, die vom Scheitel über die Kuppel bogen und zu einem ausgefransten Saum hängen, die Bögen über den hängenden Teilen, in die sie übergehen, die inneren Strähnen, wo sie sichtbar sind, der kurze kräftige Stamm mit Anlauf und Oberflächenwurzeln; in der Draufsicht strahlen die Strähnen vom Scheitel aus. Teile, die nähere ganz verdecken, werden nicht gezeichnet." },
    retained: { en: "Retained: every tress and arch with its own outline and three strand lines along it, the fissures of the stem and the fork collars, the limbs and secondary branches under the crown.", de: "Beibehalten: jede Strähne und jeder Bogen mit eigener Kontur und drei Strähnenlinien entlang, die Rindenrisse von Stamm und Gabelwülsten, die Äste und Seitenäste unter der Krone." },
    merged: { en: "Each tress is one outline of its hanging branchlets; each limb one outline per bone.", de: "Jede Strähne ist eine Kontur ihrer Hängezweige; jeder Ast eine Kontur je Knochen." },
    omitted: { en: "Omitted: single leaves and single branchlets, which are below 0.5 mm on paper.", de: "Weggelassen: einzelne Blätter und einzelne Hängezweige, die auf dem Papier unter 0,5 mm liegen." },
    thinElements: { en: "Strands, the apex arc and bark fissures are the finest strokes (0.13 mm).", de: "Strähnen, der Scheitelbogen und Rindenrisse sind die feinsten Striche (0,13 mm)." },
    hatching: { en: "No hatch: the willow has one foliage region; tones separate bark, the sunlit arches, the hanging veil and the inner tresses.", de: "Keine Schraffur: die Weide hat eine Laubregion; Töne trennen Rinde, die besonnten Bögen, den hängenden Schleier und die inneren Strähnen." },
    styles: { en: "Light line: 0.25 mm outlines, 0.13 mm texture. Dark solid: tonal fills for bark, arches, veil and inner tresses.", de: "Helle Linie: Umrisse 0,25 mm, Textur 0,13 mm. Dunkle Fläche: getönte Füllungen für Rinde, Bögen, Schleier und innere Strähnen." },
    overlays: { en: "Each thermal region has its own shapes; strokes carry no region and follow their bone.", de: "Jede thermische Region hat eigene Formen; Striche tragen keine Region und folgen ihrem Knochen." },
  },
  100: {
    silhouette: { en: "At 1:100 (1 m is 10 mm) every view is the true view of the 1:100 model: the fountain crown of 32 tresses with the same dome and ragged hem, the inner tresses where they show, the stem with flare and three surface roots.", de: "Bei 1:100 (1 m sind 10 mm) ist jede Ansicht die wahre Ansicht des 1:100-Modells: die Springbrunnenkrone aus 32 Strähnen mit derselben Kuppel und demselben ausgefransten Saum, die inneren Strähnen, wo sie sichtbar sind, der Stamm mit Anlauf und drei Oberflächenwurzeln." },
    retained: { en: "Retained: every visible tress and arch, two strand lines along each, stem fissures.", de: "Beibehalten: jede sichtbare Strähne und jeder sichtbare Bogen, je zwei Strähnenlinien, Rindenrisse des Stamms." },
    merged: { en: "The inner fringes of 1:50 are merged into the shade of the room; each limb is one outline per bone.", de: "Die inneren Fransen von 1:50 gehen im Schatten des Raums auf; jeder Ast ist eine Kontur je Knochen." },
    omitted: { en: "Omitted: leaves, single branchlets and the third strand line.", de: "Weggelassen: Blätter, einzelne Hängezweige und die dritte Strähnenlinie." },
    thinElements: { en: "Strands and fissures are 0.1 mm texture strokes.", de: "Strähnen und Risse sind Texturstriche von 0,1 mm." },
    hatching: { en: "No hatch.", de: "Keine Schraffur." },
    styles: { en: "Light line: 0.2 mm outlines, 0.1 mm texture. Dark solid: tonal fills for bark, arches, veil and inner tresses.", de: "Helle Linie: Umrisse 0,2 mm, Textur 0,1 mm. Dunkle Fläche: getönte Füllungen für Rinde, Bögen, Schleier und innere Strähnen." },
    overlays: { en: "Each thermal region has its own shapes; strokes carry no region.", de: "Jede thermische Region hat eigene Formen; Striche tragen keine Region." },
  },
  200: {
    silhouette: { en: "At 1:200 (1 m is 5 mm) every view is the true view of the 1:200 model: 24 broad tresses arching over the dome and hanging to the hem, the stem with its flare below.", de: "Bei 1:200 (1 m sind 5 mm) ist jede Ansicht die wahre Ansicht des 1:200-Modells: 24 breite Strähnen, die über die Kuppel bogen und bis zum Saum hängen, darunter der Stamm mit seinem Anlauf." },
    retained: { en: "Retained: every visible tress and arch, one strand line along each.", de: "Beibehalten: jede sichtbare Strähne und jeder sichtbare Bogen, je eine Strähnenlinie." },
    merged: { en: "The inner tresses are merged into the veil; the surface roots into the flare.", de: "Die inneren Strähnen gehen im Schleier auf; die Oberflächenwurzeln im Anlauf." },
    omitted: { en: "Omitted: bark fissures and the further strand lines.", de: "Weggelassen: Rindenrisse und die weiteren Strähnenlinien." },
    thinElements: { en: "The strand lines are 0.13 mm detail strokes.", de: "Die Strähnenlinien sind Detailstriche von 0,13 mm." },
    hatching: { en: "No hatch.", de: "Keine Schraffur." },
    styles: { en: "Light line: 0.18 mm outlines, 0.13 mm detail. Dark solid: tonal fills for bark, arches and veil.", de: "Helle Linie: Umrisse 0,18 mm, Detail 0,13 mm. Dunkle Fläche: getönte Füllungen für Rinde, Bögen und Schleier." },
    overlays: { en: "Each thermal region has its own shapes.", de: "Jede thermische Region hat eigene Formen." },
  },
  500: {
    silhouette: { en: "At 1:500 (1 m is 2 mm) every view is the true view of the 1:500 model: sixteen broad tresses from the apex to a toothed hem, the stem below; no inner lines.", de: "Bei 1:500 (1 m sind 2 mm) ist jede Ansicht die wahre Ansicht des 1:500-Modells: sechzehn breite Strähnen vom Scheitel bis zu einem gezähnten Saum, darunter der Stamm; keine Innenlinien." },
    retained: { en: "Retained: the outlines of the visible tresses and of the stem.", de: "Beibehalten: die Konturen der sichtbaren Strähnen und des Stamms." },
    merged: { en: "The crown is merged into sixteen tresses of one tier; the limbs stay inside the crown.", de: "Die Krone geht in sechzehn Strähnen eines Rangs auf; die Äste bleiben in der Krone." },
    omitted: { en: "Omitted: every stroke inside the outlines.", de: "Weggelassen: jeder Strich innerhalb der Konturen." },
    thinElements: { en: "No strokes thinner than the 0.15 mm outline.", de: "Keine Striche dünner als die Kontur von 0,15 mm." },
    hatching: { en: "No hatch.", de: "Keine Schraffur." },
    styles: { en: "Light line: 0.15 mm outlines. Dark solid: tonal fills for bark and foliage.", de: "Helle Linie: Umrisse 0,15 mm. Dunkle Fläche: getönte Füllungen für Rinde und Laub." },
    overlays: { en: "Each thermal region has its own shapes.", de: "Jede thermische Region hat eigene Formen." },
  },
};

export function draw(asset: Json, F: Record<string, Frame>, models: Model[]): string[] {
  const report: string[] = [];
  for (const { scale, parts } of models) {
    const feature = (0.15 * scale) / 1000;
    const rep = asset.representations[String(scale)];
    const strandRole = scale === 50 || scale === 100 ? "texture" : "detail";
    const strandFractions = scale === 50 ? [-0.55, 0, 0.55] : scale === 100 ? [-0.4, 0.4] : scale === 200 ? [0.1] : [];
    for (const view of VIEWS) {
      const items = parts.map((p) => ({ p, key: frontMost(view, p) + (p.kind === "lobe" ? 0.06 : 0), rings: outline(partPolygons(view, p), feature / 4, (3 * feature) ** 2) })).filter((i) => i.rings.length > 0);
      for (const it of items) if (it.p.group === "arch") {
        const own = items.find((o) => o.p.id === it.p.id.replace(/-arch$/, ""));
        if (own) it.key = Math.max(it.key, own.key + 1e-4);
      }
      items.sort((a, b) => a.key - b.key);
      const hidden = new Set<number>();
      items.forEach((it, i) => {
        if (isWood(it.p)) return;
        const ring = it.rings[0]!;
        const xs = ring.map((q) => q.x), ys = ring.map((q) => q.y);
        const [x0, x1, y0, y1] = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
        let seen = 0, total = 0;
        for (let a = 0; a < 24; a++)
          for (let b = 0; b < 24; b++) {
            const q = { x: x0 + ((a + 0.5) / 24) * (x1 - x0), y: y0 + ((b + 0.5) / 24) * (y1 - y0) };
            if (!inRing(q, ring)) continue;
            total++;
            if (!items.slice(i + 1).some((o) => o.rings.some((r) => inRing(q, r)))) seen++;
          }
        if (total > 0 && seen === 0) hidden.add(i);
      });
      const next: Json[] = [];
      let n = 0;
      items.forEach((it, i) => {
        if (hidden.has(i)) return;
        const p = it.p;
        const z = r4(1 + n++ * 0.01);
        const [region, material] = regionOf(p);
        const local = toDrawing(view, F[p.bone]!, it.rings);
        const base = { id: p.id, bone: p.bone, role: "body", z, filled: true, tone: p.tone, region, material };
        next.push(local.length === 1 ? { ...base, kind: "poly", points: pointsOf(local[0]!), closed: true } : { ...base, kind: "path", d: pathOf(local) });
        const stroke = (id: string, role: string, lines: P2[][], tone: number): void => {
          const ok = lines.filter((l) => l.length >= 2);
          if (ok.length === 0) return;
          next.push({ id, bone: p.bone, role, z: r4(z + 0.005), filled: false, tone, kind: "path", d: pathOf(toDrawing(view, F[p.bone]!, ok), false) });
        };
        if (p.kind === "loft" && (p.group === "tress" || p.group === "arch" || (p.group === "fringe" && view !== "plan")) && strandFractions.length > 0) stroke(`${p.id}-strands`, strandRole, strands(view, p, strandFractions), 0.3);
        if ((scale === 50 || scale === 100) && p.kind === "lobe") {
          const shrunk: Lobe = { ...p, radii: [p.radii[0] * 0.72, p.radii[1] * 0.72, p.radii[2] * 0.72], c: add(p.c, view === "plan" ? [0.08 * p.radii[0], 0, -0.08 * p.radii[2]] : [0, 0.12 * p.radii[1], 0]) };
          stroke(`${p.id}-arc`, "texture", [view === "plan" ? lobeOutline(view, shrunk, 10, 200, 300) : lobeOutline(view, shrunk, 10, 205, 335)], 0.2);
        }
        if ((scale === 50 || scale === 100) && view !== "plan" && p.kind === "loft" && p.group === "stem" && p.id !== "root-flare") stroke(`${p.id}-bark`, "texture", fissures(view, p, scale === 50 ? [-0.62, -0.3, 0.02, 0.33, 0.64] : [-0.45, 0.05, 0.5]), 0.1);
      });
      rep.geometry[view] = next;
      report.push(`1:${scale} ${view}: ${items.length - hidden.size} of ${items.length} parts drawn (${hidden.size} hidden), ${next.length} shapes`);
    }
    rep.decisions = DECISIONS[scale];
  }
  return report;
}
