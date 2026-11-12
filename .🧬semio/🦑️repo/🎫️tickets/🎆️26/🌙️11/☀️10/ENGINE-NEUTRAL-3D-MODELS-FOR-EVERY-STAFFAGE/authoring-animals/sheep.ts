import { Author } from "../authoring-animals-lib.ts";

const a = new Author("sheep");

a.fit("hindbody", "hindbody", { zc: 0, zh: 0.22, p: 0.3, ex: 2.5 });
a.fit("chest", "chest", { zc: 0, zh: 0.215, p: 0.3, ex: 2.4 });
a.fit("neck", "neck", { zc: 0, zh: 0.085, p: 0.5, ex: 2.3 });
a.fit("head", "head", { zc: 0, zh: 0.058, p: 0.35, ex: 2.3 });
a.fit("jaw", "jaw", { zc: 0, zh: 0.034, p: 0.4, ex: 2.3 });
a.fit("tail-root", "tail-root", { p: 0.4, ex: 2 });
a.fit("tail-tip", "tail-tip", { zc: 0, zh: 0.02, p: 0.4, ex: 2 });
for (const [n, z] of [["n", -1], ["f", 1]] as const) {
  a.ell(`ear-${n}`, `ear-${n}`, [0.967, 0.17, z * 0.0875], [0.056, 0.027, 0.0625], 0, [`ear-${n}`]);
  a.ell(`eye-${n}`, "head", [1.023, 0.139, z * 0.04], [0.01, 0.01, 0.008], 0, ["eye"]);
  a.tube(`thigh-${n}`, `thigh-h${n}`, [
    { p: [0.0, 0.0, z * 0.115], rz: 0.06, ry: 0.075 },
    { p: [0.046, -0.095, z * 0.115], rz: 0.062, ry: 0.07 },
    { p: [0.092, -0.189, z * 0.12], rz: 0.05, ry: 0.052 },
  ], ["fold-stifle"], { region: "wool-upper-legs", mat: "fleece", tone: 0.72 });
  a.tube(`humerus-${n}`, `humerus-f${n}`, [
    { p: [0.651, -0.054, z * 0.1], rz: 0.05, ry: 0.058 },
    { p: [0.616, -0.131, z * 0.1], rz: 0.05, ry: 0.052 },
    { p: [0.58, -0.208, z * 0.1], rz: 0.042, ry: 0.044 },
  ], ["fold-elbow"], { region: "wool-upper-legs", mat: "fleece", tone: 0.72 });
  a.fit(`gaskin-${n}`, `gaskin-${n}`, { p: 0.5, ex: 2.2 });
  a.fit(`cannon-h-${n}`, `cannon-h-${n}`, { p: 0.2 });
  a.fit(`hoof-h-${n}`, `hoof-h-${n}`, { p: 0.5, ex: 2.6 });
  a.fit(`forearm-${n}`, `forearm-${n}`, { p: 0.4, ex: 2.2 });
  a.fit(`cannon-f-${n}`, `cannon-f-${n}`, { p: 0.2 });
  a.fit(`hoof-f-${n}`, `hoof-f-${n}`, { p: 0.5, ex: 2.6 });
}
a.decide("Trunk and neck are bulky lofts (superellipse exponent 2.4 to 2.5) for the fleece; half-heights come from the 1:50 elevation outline of each bone, half-widths from plan and front (barrel 0.22 m). The thigh and the humerus are not drawn in elevation because the wool hides them; they are wool tubes inside the fleece, 0.1 to 0.12 m off the midline, so the leg bones carry a solid when they swing.", "Rumpf und Hals sind füllige Lofts (Superellipsen-Exponent 2,4 bis 2,5) für das Vlies; die Halbhöhen stammen aus der 1:50-Ansicht der Bone, die Halbbreiten aus Grundriss und Vorderansicht (Rumpf 0,22 m). Oberschenkel und Oberarm sind in der Ansicht nicht gezeichnet, weil die Wolle sie verdeckt; sie sind Wollröhren im Vlies, 0,1 bis 0,12 m neben der Mittellinie, damit die Beinbones beim Schwingen einen Körper tragen.");
a.decide("Ears are flat ellipsoids that hang sideways; the wool folds at stifle and elbow and the mouth line are drawn detail and are not modelled.", "Die Ohren sind flache, seitlich hängende Ellipsoide; die Wollfalten an Knie und Ellbogen sowie die Maullinie sind Zeichnungsdetail und nicht modelliert.");
a.write();
