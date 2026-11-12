import { Author } from "../authoring-animals-lib.ts";

const a = new Author("pig");

a.fit("hindbody", "hindbody", { zc: 0, zh: 0.213, p: 0.3, ex: 2.4 });
a.fit("chest", "chest", { zc: 0, zh: 0.21, p: 0.3, ex: 2.4 });
a.fit("neck", "neck", { zc: 0, zh: 0.14, p: 0.5, ex: 2.3 });
a.fit("head", "head", { zc: 0, zh: 0.1, p: 0.35, ex: 2.3 });
a.fit("snout", "snout", { zc: 0, zh: 0.048, p: 0.3, ex: 2.2 });
a.fit("jaw", "jaw", { zc: 0, zh: 0.06, p: 0.4, ex: 2.3 });
a.fit("tail-root", "tail-root", { zc: 0, zh: 0.016, p: 0.3, ex: 2 });
a.fit("tail-curl", "tail-curl", { zc: 0, zh: 0.02, p: 0.3, ex: 2 });
for (const [n, z] of [["n", -1], ["f", 1]] as const) {
  a.fit(`ear-${n}`, `ear-${n}`, { zc: z * 0.105, zh: 0.03, p: 0.4, ex: 2 });
  a.ell(`eye-${n}`, "head", [0.909, 0.082, z * 0.075], [0.01, 0.01, 0.008], 0, ["eye"]);
  a.fit(`ham-${n}`, "ham-n", { bone: `thigh-h${n}`, zc: z * 0.11, zh: 0.1, p: 0.4, ex: 2.2 }, { tone: n === "n" ? 0.72 : 0.5 });
  a.fit(`shoulder-${n}`, "shoulder-n", { bone: `scapula-f${n}`, zc: z * 0.105, zh: 0.095, p: 0.4, ex: 2.2 }, { tone: n === "n" ? 0.72 : 0.5 });
  a.tube(`humerus-${n}`, `humerus-f${n}`, [
    { p: [0.694, -0.062, z * 0.1], rz: 0.058, ry: 0.065 },
    { p: [0.66, -0.135, z * 0.1], rz: 0.054, ry: 0.058 },
    { p: [0.626, -0.207, z * 0.1], rz: 0.046, ry: 0.048 },
  ], ["shoulder-n"], { region: "skin-trunk", tone: 0.72 });
  a.fit(`gaskin-${n}`, `gaskin-${n}`, { p: 0.5, ex: 2.2 });
  a.fit(`cannon-h-${n}`, `cannon-h-${n}`, { p: 0.2 });
  a.fit(`hoof-h-${n}`, `hoof-h-${n}`, { p: 0.5, ex: 2.6 });
  a.fit(`forearm-${n}`, `forearm-${n}`, { p: 0.4, ex: 2.2 });
  a.fit(`cannon-f-${n}`, `cannon-f-${n}`, { p: 0.2 });
  a.fit(`hoof-f-${n}`, `hoof-f-${n}`, { p: 0.5, ex: 2.6 });
}
a.decide("Trunk, neck, head, snout and limbs are lofts with sagittal sizes from the 1:50 elevation outline and lateral sizes from plan and front (barrel 0.21 m, head 0.10 m, snout 0.048 m, legs 0.1 m off the midline); the ham and the shoulder have no lateral twin and are given 0.10 m half-width and 0.11 m offset, the far ham and shoulder reuse the near outline on their own bones.", "Rumpf, Hals, Kopf, Rüssel und Beine sind Lofts mit senkrechten Massen aus der 1:50-Ansicht und seitlichen Massen aus Grundriss und Vorderansicht (Rumpf 0,21 m, Kopf 0,10 m, Rüssel 0,048 m, Beine 0,1 m neben der Mittellinie); Schinken und Schulter haben keine seitliche Entsprechung und erhalten 0,10 m Halbbreite und 0,11 m Versatz, ferner Schinken und ferne Schulter übernehmen die nahe Kontur auf ihren Bones.");
a.decide("The humerus has no drawn outline and is a tapered tube; ears are 60 mm thick forward flaps; eyes are ellipsoids; the mouth line is not modelled.", "Der Oberarm hat keine gezeichnete Kontur und ist eine sich verjüngende Röhre; die Ohren sind 60 mm dicke, nach vorn hängende Lappen; die Augen sind Ellipsoide; die Maullinie ist nicht modelliert.");
a.write();
