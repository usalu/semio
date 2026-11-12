import { Author } from "../authoring-animals-lib.ts";

const a = new Author("hedgehog");

a.fit("dome", "dome", { zc: 0, zh: 0.075, p: 0.5, ex: 2.2 });
a.fit("belly", "belly", { zc: 0, zh: 0.055, p: 0.4, ex: 2.4 });
a.fit("neck", "neck", { zc: 0, zh: 0.03, p: 0.4, ex: 2.2 });
a.fit("head", "head", { zc: 0, zh: 0.03, p: 0.4, ex: 2.2 });
a.ell("nose", "head", [0.224, -0.033, 0], [0.007, 0.007, 0.008], 0, ["head"], { regions: ["nose"], mat: "bare-skin", tone: 0.1 });
for (const [n, z] of [["n", -1], ["f", 1]] as const) {
  for (const k of ["front", "rear"]) a.fit(`skirt-${k}-${n === "n" ? "near" : "far"}`, `skirt-${k}-near`, { bone: `coat-${k === "front" ? "f" : "r"}${n}`, zc: z * 0.04, zh: 0.03, p: 0.4, ex: 2.2 }, { tone: n === "n" ? undefined : 0.5 });
  a.cap(`${n === "n" ? "near" : "far"}-fore`, `fore-${n}`, [0.11, -0.03, z * 0.035], [0.11, -0.058, z * 0.035], 0.014, 0.012, [`${n === "n" ? "near" : "far"}-fore`]);
  a.ell(`${n === "n" ? "near" : "far"}-forepaw`, `forepaw-${n}`, [0.118, -0.0665, z * 0.035], [0.02, 0.0065, 0.013], 0, [`${n === "n" ? "near" : "far"}-forepaw`]);
  a.cap(`${n === "n" ? "near" : "far"}-hind`, `hind-${n}`, [0.02, -0.03, z * 0.042], [0.02, -0.063, z * 0.042], 0.014, 0.012, [`${n === "n" ? "near" : "far"}-hind`]);
  a.ell(`${n === "n" ? "near" : "far"}-hindpaw`, `hindpaw-${n}`, [0.03, -0.0665, z * 0.042], [0.024, 0.0065, 0.013], 0, [`${n === "n" ? "near" : "far"}-hindpaw`]);
}
a.decide("The spine coat is a dome loft 0.15 m wide and 0.096 m high over a 0.11 m wide belly loft; the four skirts hang as lofts on the four coat bones, 0.04 m off the midline; head and neck are 0.06 m wide lofts with a small nose ellipsoid.", "Das Stachelkleid ist ein Kuppel-Loft von 0,15 m Breite und 0,096 m Höhe über einem 0,11 m breiten Bauch-Loft; die vier Säume hängen als Lofts an den vier Kleidbones, 0,04 m neben der Mittellinie; Kopf und Hals sind 0,06 m breite Lofts mit einer kleinen Nasen-Ellipsoide.");
a.decide("The limbs are drawn only in the front view; the elevation hides them under the spine skirt, whose lower edge is at -0.073 m. Legs are short capsules on their bones and flat paw ellipsoids ending at -0.072 m so that the volume height (0.136 m) follows the elevation and dimensions (0.128 m, +6 %); the front view draws the paws down to -0.105 m, which the volume deliberately does not follow.", "Die Gliedmassen sind nur in der Vorderansicht gezeichnet; die Ansicht verbirgt sie unter dem Stachelsaum, dessen Unterkante bei -0,073 m liegt. Die Beine sind kurze Kapseln auf ihren Bones, die Pfoten flache Ellipsoide bis -0,072 m, sodass die Höhe des Volumens (0,136 m) der Ansicht und den Abmessungen (0,128 m, +6 %) folgt; die Vorderansicht zeichnet die Pfoten bis -0,105 m, dem das Volumen bewusst nicht folgt.");
a.write();
