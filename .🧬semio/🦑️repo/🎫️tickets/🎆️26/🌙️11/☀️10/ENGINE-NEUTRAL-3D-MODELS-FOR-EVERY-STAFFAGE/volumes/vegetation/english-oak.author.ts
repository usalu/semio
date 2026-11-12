import { Author } from "../../veg-lib.ts";
const a = new Author("english-oak");
a.loftFromViews("root-flare", "root-plate", { elev: "root-flare", front: "trunk", range: [0, 0.4], n: 4, exp: 2.2, pinch: 1 });
a.loftFromViews("trunk", "trunk", { elev: "trunk", front: "trunk", range: [0.4, 3.4], n: 6, exp: 2.2, pinch: 1 });
a.loftFromViews("trunk-upper", "trunk-upper", { elev: "trunk-upper", front: "trunk", range: [3.4, 6.0], n: 5, exp: 2.2, pinch: 1 });
a.loftFromViews("leader", "leader", { elev: "leader", front: "leader", range: [6.0, 10.4], n: 6, exp: 2.2, pinch: 1 });
a.loftFromViews("leader-top", "leader-top", { elev: "leader-top", front: "leader", range: [10.4, 13.75], n: 5, exp: 2.2, pinch: 0.3 });
a.limbs([
  { bone: "limb-w-low" }, { bone: "limb-e-low" }, { bone: "limb-w-mid" }, { bone: "limb-e-mid" },
  { bone: "br-w-low" }, { bone: "br-e-low" }, { bone: "br-w-mid" }, { bone: "br-e-mid" },
  { bone: "br-leader-w" }, { bone: "br-leader-e" }, { bone: "br-low-w-up", zEnd: 3.0 }, { bone: "br-low-e-up", zEnd: -3.0 },
]);
a.masses(["fol-core", "fol-w-low", "fol-e-low", "fol-w-mid", "fol-e-mid", "fol-leader-w", "fol-leader-e", "fol-top"], { twig: 0.07, twigMeta: a.meta("leader", { region: "limb-bark" }) });
const E = ["core-fol", "fol-w-low", "fol-e-low", "fol-w-mid", "fol-e-mid", "fol-leader-w", "fol-leader-e", "fol-top"];
const F = ["core", "left-low", "right-low", "left-mid", "right-mid", "leader-left", "leader-right", "top"];
const W: Record<number, [string, string]> = { 1: ["fol-e-mid", "fol-e-low"], 2: ["fol-w-mid", "fol-w-low"], 3: ["fol-leader-e", "fol-e-mid"], 4: ["fol-leader-w", "fol-w-mid"] };
for (const i of [1, 2, 3, 4]) a.fill(`crown-fill-${i}`, W[i]![0], `crown-fill-${i}`, { elevIds: E, frontIds: F, shrink: 0.78, extra: { weights: [{ bone: W[i]![0], weight: 0.6 }, { bone: W[i]![1], weight: 0.4 }] } });
a.note("Trunk, root flare, leader and leader top are lofts whose x and z half widths are read slice by slice from the authored elevation and front outlines; the two long lower limbs and mid limbs are tapered capsules with their lateral drift to the extreme of the front outline, each bound to its own bone.", "Stamm, Wurzelanlauf, Stammverlängerung und Wipfel sind Lofts, deren Halbbreiten in x und z Schnitt für Schnitt aus den Umrissen von Ansicht und Front stammen; die langen unteren und mittleren Leitäste sind verjüngte Kapseln, deren Seitendrift dem Extrem des Front-Umrisses folgt, je an den eigenen Knochen gebunden.");
a.note("Eight foliage masses are vertical lofts of superelliptic sections bound to their foliage bones so that sway clips move the foliage; four plan crown fills close the quadrants the diagonal arrangement of the masses leaves open and are shrunk to stay inside the elevation and front envelopes.", "Acht Laubmassen sind senkrechte Lofts aus superelliptischen Querschnitten an ihren Laubknochen, damit Schwankungsclips das Laub mitnehmen; vier Kronenfüllungen der Draufsicht schließen die Quadranten, die die diagonale Anordnung offen lässt, und sind so eingezogen, dass sie in den Hüllen von Ansicht und Front bleiben.");
a.note("Leaves, acorns, bark ridges and twig strokes of the 2D drawing are not modelled: a crown is a believable mass, not thousands of leaves.", "Blätter, Eicheln, Rindenrippen und Zweigstriche der 2D-Zeichnung sind nicht modelliert: eine Krone ist eine glaubhafte Masse, keine tausend Blätter.");
a.save();
