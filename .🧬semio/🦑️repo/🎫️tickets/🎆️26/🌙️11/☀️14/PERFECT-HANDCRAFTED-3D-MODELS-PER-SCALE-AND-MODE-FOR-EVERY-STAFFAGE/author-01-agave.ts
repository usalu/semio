import { Author, type V3, type Json } from "./author-01-kit.ts";
import { treeFinish, type TreeCfg } from "./author-01-tree.ts";
import type { Section } from "./author-01-kit.ts";
import { draftBody } from "./volumes-draft.ts";
import { simplifyRing } from "./volumes-base.ts";

const LEAF = "agave-leaf", CAUDEX = "agave-caudex";

function body(scale: 50 | 100 | 200 | 500): Author {
  const a = new Author("agave", scale);
  a.into("body");
  const { solids } = draftBody(a.asset, scale);
  if (scale !== 500) for (const sol of solids) {
    const c = JSON.parse(JSON.stringify(sol));
    if (c.kind === "extrude" && scale > 50) {
      const ring = simplifyRing(c.profile.map(([x, y]: number[]) => ({ x, y })), a.feature * 0.8);
      if (ring.length >= 3) c.profile = ring.map((p) => [Math.round(p.x * 1e5) / 1e5, Math.round(p.y * 1e5) / 1e5]);
    }
    if (c.kind === "extrude" && c.from !== undefined) { const h = Math.max(Math.abs(c.from), Math.abs(c.to), a.feature * 0.6); c.from = -h; c.to = h; }
    a.lists.body.push(c);
  }
  if (scale !== 500) return a;
  if (scale === 500) {
    a.loftFromViews("caudex", "caudex", { elev: "caudex", front: "caudex", range: [0, 0.42], n: 3, exp: 2.2, pinch: 0.8, m: a.mat(CAUDEX, 0.3) });
    for (const side of ["left", "right"]) {
      const E = a.rings("elevation", `rosette-${side}-0`), eb = a.box2("elevation", `rosette-${side}-0`), fb = a.box2("front", `rosette-${side}`), pb = a.box2("plan", `rosette-${side}`);
      void E;
      a.ell(`rosette-${side}`, "caudex", [(eb.minX + eb.maxX) / 2, (eb.minY + eb.maxY) / 2, (fb.minX + fb.maxX) / 2], [(eb.maxX - eb.minX) / 2, (eb.maxY - eb.minY) / 2, 0.8], a.mat(LEAF, 0.55));
    }
    const hb = a.box2("elevation", "heart-0"), hf = a.box2("front", "heart");
    a.cap("heart", "leaf-s", [(hb.minX + hb.maxX) / 2, hb.minY + 0.12, 0], [(hb.minX + hb.maxX) / 2, hb.maxY - 0.1, 0], 0.12, 0.08, a.mat(LEAF, 0.5));
    void hf;
    return a;
  }
  a.loftFromViews("caudex", "caudex", { elev: "caudex", front: "caudex", range: [0, 0.42], n: scale === 50 ? 5 : 4, exp: 2.2, pinch: 0.8, m: a.mat(CAUDEX, 0.3) });
  const leafBones = a.asset.rig.bones.map((b: Json) => b.id).filter((id: string) => /^leaf-/.test(id));
  const n = { 50: 7, 100: 5, 200: 4 }[scale];
  for (const bone of leafBones) {
    const planIds = a.shapesOf("plan", bone);
    if (planIds.length === 0) continue;
    const id = planIds.find((x) => x === bone) ?? planIds[0]!;
    const rings = a.rings("plan", id), bb = a.box2("plan", id);
    const raw = a.rawOf(bone)?.solid ?? { z1: 0.02, zTip: [-0.01, 0.01] };
    const t0 = raw.z1 ?? 0.02, t1 = raw.zTip ? raw.zTip[1] : t0 * 0.5;
    const secs: Section[] = [];
    for (let q = 0; q < n; q++) {
      const t = q / (n - 1), u = bb.minX + (bb.maxX - bb.minX) * t;
      const r = a.slice(rings, "x", Math.min(bb.maxX - 1e-4, Math.max(bb.minX + 1e-4, u)));
      const w = r ? (r[1] - r[0]) / 2 : 0.01, wc = r ? (r[0] + r[1]) / 2 : 0;
      secs.push({ c: [u, wc, 0] as V3, rx: Math.max(0.004, (t0 + (t1 - t0) * t) * (q === n - 1 ? 0.5 : 1), a.feature * 0.55), ry: Math.max(w * (q === n - 1 ? 0.15 : 1), a.feature * 0.55), exp: 2.4 });
    }
    a.loft(bone, bone, secs, a.mat(LEAF, 0.5), { local: true, axis: [1, 0, 0] });
  }
  return a;
}
const SPREAD = /^leaf-(r1|r2|l1|l2)-/;
const cfg: TreeCfg = {
  slug: "agave",
  woody: () => false, foliage: () => false,
  pick: {
    "leaves-spreading": (s) => s.material === LEAF && (SPREAD.test(s.id) || s.id === "rosette-left" || s.id === "rosette-right"),
    "leaves-erect": (s) => s.material === LEAF && !SPREAD.test(s.id) && !/^rosette/.test(s.id),
    caudex: (s) => s.material === CAUDEX,
  },
  memberR: (b, sc) => [Math.max(0.012, 0.006 * Math.sqrt(sc / 50)), 0.008],
  emitterD: 0.4,
  notes: {
    50: [["Body: the caudex is a loft read from its elevation and front outlines; each of the 19 leaf bones is its own loft in the bone frame whose width follows the leaf's plan outline slice by slice, whose thickness tapers from the authored solid hint to the spine and whose spine is a rounded point; the leaves lie in their bone planes at rest and fan out with the abduction and twist of the default pose.", "Körper: der Caudex ist ein Loft aus Aufriss- und Frontkontur; jeder der 19 Blattknochen ist ein eigener Loft im Knochenrahmen, dessen Breite der Grundrisskontur des Blattes Schnitt für Schnitt folgt, dessen Dicke vom gezeichneten Körperhinweis zur Spitze hin abnimmt und dessen Ende ein abgerundeter Dorn ist; die Blätter liegen in Ruhe in ihren Knochenebenen und fächern sich mit Abspreizung und Verdrehung der Standardpose auf."],
         ["Thermal: references to the leaf solids (spreading lower leaves versus erect inner leaves and spear) and the caudex; transpiration emitters are flat discs inside the rosette. Exposed leaf area is smaller than the authored leaf area; areaM2 corrected to the model's exposed surface per ruling C1.", "Thermisch: Verweise auf die Blattkörper (ausgebreitete untere gegenüber aufrechten inneren Blättern und Herzblatt) und den Caudex; die Transpirationsemitter sind flache Scheiben in der Rosette. Die freie Blattfläche ist kleiner als die angegebene Blattfläche; areaM2 gemäß Regel C1 auf die freie Modellfläche korrigiert."],
         ["Structure: one member capsule along every leaf bone and the caudex inside the leaf solids, joint balls at the leaf bases, a fixed support block under the root plate.", "Tragwerk: eine Stabkapsel entlang jedem Blattknochen und dem Caudex im Blattkörper, Gelenkkugeln an den Blattbasen, ein fester Auflagerblock unter der Wurzelplatte."]],
    100: [["Body modelled anew against the 1:100 drawings: five sections per leaf instead of seven and four for the caudex; the spear and leaf tips keep their rounded points above the 15 mm feature.", "Körper neu gegen die 1:100-Zeichnungen modelliert: fünf Schnitte je Blatt statt sieben und vier für den Caudex; Herzblatt und Blattspitzen behalten ihre abgerundeten Spitzen über dem Merkmal von 15 mm."],
          ["Thermal: references as at 1:50 to the coarser leaf lofts.", "Thermisch: Verweise wie bei 1:50 auf die gröberen Blattlofts."],
          ["Structure: member capsules grown to the 15 mm feature, joint balls omitted below the feature of a leaf base.", "Tragwerk: Stabkapseln auf das Merkmal von 15 mm vergrößert, Gelenkkugeln entfallen."]],
    200: [["Body modelled anew against the 1:200 drawings: four sections per leaf; leaf thickness is raised to the 30 mm feature.", "Körper neu gegen die 1:200-Zeichnungen modelliert: vier Schnitte je Blatt; die Blattdicke wird auf das Merkmal von 30 mm angehoben."],
          ["Thermal: references to the leaf lofts.", "Thermisch: Verweise auf die Blattlofts."],
          ["Structure: member capsules at 30 mm, support block.", "Tragwerk: Stabkapseln mit 30 mm, Auflagerblock."]],
    500: [["Body modelled anew against the 1:500 drawings: the 19 leaves merge into two rosette ellipsoids (left and right, as wide as the drawn rosette and as deep as its plan), the heart is one capsule and the caudex one loft; at this scale the rosette is rigid with the caudex.", "Körper neu gegen die 1:500-Zeichnungen modelliert: die 19 Blätter verschmelzen zu zwei Rosettenellipsoiden (links und rechts, so breit wie die gezeichnete Rosette und so tief wie ihr Grundriss), das Herz ist eine Kapsel und der Caudex ein Loft; bei diesem Maßstab ist die Rosette starr mit dem Caudex verbunden."],
          ["Thermal: references to the rosette ellipsoids, the heart capsule and the caudex.", "Thermisch: Verweise auf die Rosettenellipsoide, die Herzkapsel und den Caudex."],
          ["Structure: square-free member capsules at 75 mm along the merged bones, support block.", "Tragwerk: Stabkapseln mit 75 mm entlang der verschmolzenen Knochen, Auflagerblock."]],
  },
};
function build(scale: 50 | 100 | 200 | 500): Author { const a = body(scale); treeFinish(a, cfg); return a; }
export { body, build };
