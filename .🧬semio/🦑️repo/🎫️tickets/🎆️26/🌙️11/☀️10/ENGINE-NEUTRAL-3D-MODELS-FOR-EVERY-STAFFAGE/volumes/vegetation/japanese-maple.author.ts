import { Author } from "../../veg-lib.ts";
const a = new Author("japanese-maple");
const stem = a.meta("wood-stem-c"), young = a.meta("wood-stem-c", { material: "maple-bark-young", region: "limb-bark" });
a.loftFromViews("root-collar", "root-plate", { elev: "root-collar", front: "root-collar", range: [0, 0.255], n: 4, exp: 2.1, pinch: 1 });
a.limbs([
  { bone: "stem-c", zEnd: 0, r0: 0.095, r1: 0.06, meta: stem },
  { bone: "stem-c-top", zEnd: 0, r0: 0.06, r1: 0.04, meta: stem },
  { bone: "stem-w", front: "wood-stem-w", r0: 0.09, r1: 0.055, meta: stem },
  { bone: "stem-e", front: "wood-stem-e", r0: 0.09, r1: 0.055, meta: stem },
  { bone: "t1w", zEnd: 0, r0: 0.06, r1: 0.035, meta: young }, { bone: "t1e", zEnd: 0, r0: 0.06, r1: 0.035, meta: young },
  { bone: "t2w", zEnd: 0, r0: 0.055, r1: 0.03, meta: young }, { bone: "t2e", zEnd: 0, r0: 0.055, r1: 0.03, meta: young },
  { bone: "t3w", zEnd: 0, r0: 0.05, r1: 0.03, meta: young }, { bone: "t3e", zEnd: 0, r0: 0.05, r1: 0.03, meta: young },
]);
const from = (bone: string): [number, number, number] => { const o = a.origin(bone); const parent = a.ends.get(a.bone(bone).parent); return [o[0], o[1], 0]; };
const tierBone: Record<string, string> = {};
const lobes: [string, string][] = [];
for (const sh of a.st.front) {
  if (!sh.closed || sh.shape.role !== "body" || !/^f-/.test(sh.shape.id)) continue;
  lobes.push([sh.shape.id, sh.shape.id.replace(/^f-/, "p-")]);
  tierBone[sh.shape.id] = sh.shape.bone;
}
for (const [f, p] of lobes) {
  const bone = tierBone[f]!;
  const e = a.box2("elevation", a.shapesOf("elevation", bone)), fb = a.box2("front", f);
  if (/core|top/.test(f)) {
    a.loftFromViews(`crown-${f.slice(2)}`, bone, { elev: a.shapesOf("elevation", bone), front: f, n: 8, exp: 2.4, pinch: 0.3, m: a.meta(f) });
    continue;
  }
  a.lobe(`lobe-${f.slice(2)}`, bone, p, f, { m: a.meta(f), yRange: [(fb.minY + e.minY) / 2, (fb.maxY + e.maxY) / 2], from: from(bone), twig: 0.03, twigMeta: young });
}
a.note("Each tier is a ring of flat foliage lobes (Japanese maple layers): one oriented ellipsoid per authored plan outline at the height of its front outline, bound to the west or east tier bone that carries it, with a thin twig from the tier's origin on the stem to the lobe centre so lobes at every azimuth hang on a branch.", "Jede Etage ist ein Ring flacher Laublappen (Etagenwuchs des Fächerahorns): ein ausgerichtetes Ellipsoid je Umriss der Draufsicht in der Höhe seines Front-Umrisses, an den west- oder ostseitigen Etagenknochen gebunden, mit einem dünnen Zweig vom Ursprung der Etage am Stamm zur Lappenmitte, sodass Lappen in jeder Richtung an einem Ast hängen.");
a.note("The cores and the crown top are lofts read from the elevation and front outlines because their plan outlines are about 30 % narrower than elevation and front agree on (inconsistent 2D views); elevation and front are trusted.", "Kerne und Kronenspitze sind Lofts aus den Umrissen von Ansicht und Front, weil ihre Umrisse in der Draufsicht etwa 30 % schmaler sind als Ansicht und Front übereinstimmend zeigen (widersprüchliche 2D-Ansichten); Ansicht und Front gelten.");
a.note("Three stems (central, west, east) are tapered capsules; the west and east stems lean sideways as the front view draws them. The tiers' main branches are capsules along their bones.", "Drei Stämme (mittig, west, ost) sind verjüngte Kapseln; West- und Oststamm neigen sich seitlich wie in der Front gezeichnet. Die Hauptäste der Etagen sind Kapseln entlang ihrer Knochen.");
a.save();
