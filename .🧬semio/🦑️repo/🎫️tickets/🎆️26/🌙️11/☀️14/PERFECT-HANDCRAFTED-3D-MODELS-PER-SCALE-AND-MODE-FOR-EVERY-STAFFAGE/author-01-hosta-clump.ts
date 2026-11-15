import { Author, type V3 } from "./author-01-kit.ts";
import type { Section } from "./author-01-kit.ts";
import { treeFinish, draftBase, type TreeCfg } from "./author-01-tree.ts";

const BLADE = "hosta-blade", PETIOLE = "hosta-petiole";

function body(scale: 50 | 100 | 200): Author {
  const a = new Author("hosta-clump", scale);
  a.into("body");
  if (scale !== 200) { draftBase(a); return a; }
  if (scale === 200) {
    for (const id of ["outer-left", "outer-right", "inner"]) {
      const eb = a.box2("front", `mass-${id}`), pb = a.box2("plan", `mass-${id}`);
      const E = a.shapesOf("elevation", "root-plate");
      void E;
      const ee = id === "inner" ? a.box2("elevation", "mass-inner-0") : a.box2("elevation", `mass-${id === "outer-left" ? "outer-0" : "outer-1"}`);
      a.ell(`mass-${id}`, "root-plate", [(ee.minX + ee.maxX) / 2, (ee.minY + ee.maxY) / 2, (pb.minY + pb.maxY) / 2], [(ee.maxX - ee.minX) / 2, (ee.maxY - ee.minY) / 2, (pb.maxY - pb.minY) / 2], a.mat(BLADE, 0.5));
      void eb;
    }
    return a;
  }
  const bones: string[] = [];
  for (const side of ["r", "l"]) for (const n of [1, 2, 3, 4]) bones.push(`${side}${n}`);
  bones.push("c1");
  const secsN = scale === 50 ? 6 : 4;
  for (const n of bones) {
    const pb = `petiole-${n}`, bb = `blade-${n}`;
    const po = a.origin(pb), pt = a.tip(pb);
    if (a.shapesOf("elevation", pb).length || a.has(pb)) a.cap(pb, pb, po, pt, Math.max(0.016, a.feature * 0.55), Math.max(0.012, a.feature * 0.55), a.mat(PETIOLE, 0.35));
    const bo = a.origin(bb), bt = a.tip(bb);
    const pl = a.box2("plan", bb);
    const half = (pl.maxY - pl.minY) / 2, cz = (pl.minY + pl.maxY) / 2;
    const prof: [number, number][] = scale === 50 ? [[0, 0.25], [0.15, 0.8], [0.4, 1], [0.7, 0.8], [0.92, 0.4], [1, 0.08]] : [[0, 0.3], [0.35, 1], [0.75, 0.75], [1, 0.1]];
    const secs: Section[] = prof.slice(0, secsN).map(([q, w]) => ({ c: [bo[0] + (bt[0] - bo[0]) * q, bo[1] + (bt[1] - bo[1]) * q, cz] as V3, rx: Math.max(half * w, a.feature * 0.55), ry: Math.max(0.004, a.feature * 0.55), exp: 2.2 }));
    a.loft(bb, bb, secs, a.mat(BLADE, 0.5), { axis: [bt[0] - bo[0], bt[1] - bo[1], 0] as V3 });
  }
  return a;
}
const OUT = /blade-(r|l)[12]$|mass-outer/;
const cfg: TreeCfg = {
  slug: "hosta-clump",
  woody: () => false, foliage: () => false,
  pick: {
    "blades-outer": (s) => s.material === BLADE && OUT.test(s.id),
    "blades-inner": (s) => s.material === BLADE && !OUT.test(s.id),
  },
  memberR: (b, sc) => [Math.max(0.012, 0.01 * Math.sqrt(sc / 50)), 0.008],
  emitterD: 0.12,
  hidden: (a) => {
    a.into("thermal", { region: "petioles" });
    const ids = ["r1", "r2", "r3", "r4", "l1", "l2", "l3", "l4", "c1"];
    a.fitArea("petioles", 0.25, (k) => { for (const n of ids) { const o = a.origin(`petiole-${n}`), t = a.tip(`petiole-${n}`); const m: V3 = [o[0] + (t[0] - o[0]) * 0.15, o[1] + (t[1] - o[1]) * 0.15, o[2] + (t[2] - o[2]) * 0.15]; const e: V3 = [o[0] + (t[0] - o[0]) * 0.85, o[1] + (t[1] - o[1]) * 0.85, o[2] + (t[2] - o[2]) * 0.85]; a.cap(`th-petiole-${n}`, `petiole-${n}`, m, e, Math.max(a.feature * 0.55, 0.01 * k), Math.max(a.feature * 0.55, 0.008 * k), a.mat(PETIOLE, 0.3)); } }, 0.3, 3);
  },
  notes: {
    50: [["Body: nine petioles are tapered capsules along their bones and nine blades are lofts whose half widths follow each blade's plan outline and whose axes follow the elevation, so each blade is a slightly cupped leaf, not a plate.", "Körper: neun Blattstiele sind verjüngte Kapseln entlang ihrer Knochen und neun Spreiten sind Lofts, deren Halbbreiten der Grundrisskontur jeder Spreite folgen und deren Achsen dem Aufriss folgen, sodass jede Spreite ein leicht gewölbtes Blatt ist, keine Platte."],
         ["Thermal: outer and inner blades reference the blade lofts; the hidden petiole region is nine thin capsules inside the petiole bodies, fitted to its authored area; emitters are flat discs in the clump.", "Thermisch: äußere und innere Spreiten verweisen auf die Spreitenlofts; die verdeckte Stielregion sind neun dünne Kapseln in den Stielkörpern, auf ihre angegebene Fläche abgestimmt; Emitter sind flache Scheiben im Horst."],
         ["Structure: member capsules along each petiole and blade bone, joint balls at the petiole bases, support block under the root plate.", "Tragwerk: Stabkapseln entlang jedem Stiel- und Spreitenknochen, Gelenkkugeln an den Stielbasen, Auflagerblock unter der Wurzelplatte."]],
    100: [["Body modelled anew against the 1:100 drawings: four-section blade lofts, petioles as capsules at the 15 mm feature.", "Körper neu gegen die 1:100-Zeichnungen modelliert: Spreitenlofts mit vier Schnitten, Blattstiele als Kapseln im Merkmal von 15 mm."],
          ["Thermal: references and hidden petiole capsules as at 1:50.", "Thermisch: Verweise und verdeckte Stielkapseln wie bei 1:50."],
          ["Structure: member capsules at the 1:100 feature, no joint balls.", "Tragwerk: Stabkapseln im Merkmal von 1:100, keine Gelenkkugeln."]],
    200: [["Body modelled anew against the 1:200 drawings: three masses (left and right outer leaves, inner leaves) as ellipsoids spanning their elevation and plan outlines; petioles and blades are below the 30 mm feature individually.", "Körper neu gegen die 1:200-Zeichnungen modelliert: drei Massen (äußere Blätter links und rechts, innere Blätter) als Ellipsoide über ihre Aufriss- und Grundrisskonturen; Stiele und Spreiten liegen einzeln unter dem Merkmal von 30 mm."],
          ["Thermal: references to the masses, hidden petiole capsules at the 30 mm feature.", "Thermisch: Verweise auf die Massen, verdeckte Stielkapseln im Merkmal von 30 mm."],
          ["Structure: member capsules at 30 mm, support block.", "Tragwerk: Stabkapseln mit 30 mm, Auflagerblock."]],
  },
};
function build(scale: 50 | 100 | 200): Author { const a = body(scale); treeFinish(a, cfg); return a; }
export { body, build };
