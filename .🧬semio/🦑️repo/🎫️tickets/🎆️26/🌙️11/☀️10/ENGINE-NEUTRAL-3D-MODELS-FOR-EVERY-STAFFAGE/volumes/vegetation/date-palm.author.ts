import { Author, type V3, type Section } from "../../veg-lib.ts";
const a = new Author("date-palm");
const AX = -0.047;
a.loftFromViews("root-bulb", "root-plate", { elev: "root-bulb", front: "root-bulb", range: [0, 0.5], n: 4, exp: 2.2, pinch: 1 });
const trunk = (id: string, bone: string, eIds: string, fId: string, r: [number, number]) => a.loftFromViews(id, bone, { elev: eIds, front: fId, range: r, n: 5, exp: 2.1, pinch: 1, zScale: 0.58 });
trunk("stem-1", "stem-1", "stem-1", "bole-stem-1", [0.3, 3.3]);
trunk("stem-2", "stem-2", "stem-2", "bole-stem-2", [3.3, 6.0]);
trunk("stem-3", "stem-3", "stem-3", "bole-stem-3", [6.0, 8.4]);
a.loftFromViews("crown-shaft", "crown", { elev: "crown-shaft", front: "bole-crown", range: [8.4, 9.17], n: 4, exp: 2.1, pinch: 0.9, zScale: 0.9 });
const frond = a.meta("fr-l1a-blade"), rachis = a.meta("rachis-l1-a", { material: "palm-frond" });
const fronds = (side: "l" | "r") => {
  for (const i of [1, 2, 3, 4]) for (const part of ["a", "b"]) {
    const bone = `fr-${side}${i}${part}`;
    const o = a.origin(bone), t = a.tip(bone);
    const thick = a.area("elevation", `${bone}-blade`) / Math.hypot(t[0] - o[0], t[1] - o[1]);
    const prof: [number, number, number][] = [[0, 0.3, 0.7], [0.15, 0.85, 1], [0.5, 1, 1], [0.85, 0.75, 0.9], [1, 0.25, 0.45]];
    const secs: Section[] = prof.map(([q, w, h]) => ({ c: [o[0] + (t[0] - o[0]) * q, o[1] + (t[1] - o[1]) * q, 0] as V3, rx: 0.22 * w, ry: Math.max(0.02, (thick / 2) * h), exp: 2.2 }));
    const repeat = { count: 6, rotationAxis: a.D(bone, [0, 1, 0]), rotationRad: -Math.PI / 6, origin: a.L(bone, [AX, 0, 0]) };
    a.loft(`${bone}-blade`, bone, secs, a.meta(`${bone}-blade`), { axis: [(t[0] - o[0]), (t[1] - o[1]), 0], extra: { repeat } });
    a.cap(`${bone}-rachis`, bone, [o[0], o[1], 0], [t[0], t[1], 0], 0.035, 0.015, rachis, { repeat });
  }
};
fronds("l");
fronds("r");
for (const [side, sign] of [["l", -1], ["r", 1]] as const) {
  const bone = `bunch-${side}`, id = `bunch-${side}`;
  const e = a.box2("elevation", id), f = a.box2("front", id);
  const c: V3 = [(e.minX + e.maxX) / 2, (e.minY + e.maxY) / 2, (f.minX + f.maxX) / 2];
  a.ell(id, bone, c, [(e.maxX - e.minX) / 2, (e.maxY - e.minY) / 2, (f.maxX - f.minX) / 2], a.meta(id));
  const o = a.origin(bone);
  a.cap(`${id}-stalk`, bone, [o[0], o[1], 0], [c[0] * 0.8 + o[0] * 0.2, c[1] + 0.25, c[2]], 0.04, 0.03, a.meta("stem-1", { region: "stem-bark" }));
}
for (const [id, ring] of [["skirt-w1", 8], ["skirt-w2", 8]] as const) {
  const b = a.box2("elevation", id);
  a.cap(id, "stem-3", [b.maxX + 0.05, b.maxY, 0], [b.minX, b.minY + 0.05, 0], 0.07, 0.045, a.meta(id), { repeat: { count: ring, rotationAxis: a.D("stem-3", [0, 1, 0]), rotationRad: Math.PI / 4, origin: a.L("stem-3", [AX, 0, 0]) } });
}
a.note("The crown is a rosette: the eight authored frond chains (four per side, each in two bones, inner and outer part) are drawn in the sagittal plane only, but the asset's depth of 7 m and the biology need fronds all round. Each frond part is therefore one flat blade loft plus a rachis capsule, repeated 6 times at 30 degree steps about the crown axis, so left and right chains together give 48 fronds in 12 directions and the repeats follow the sway of their bone.", "Die Krone ist eine Rosette: die acht gezeichneten Wedelketten (vier je Seite, je in zwei Knochen, Innen- und Außenteil) sind nur in der Längsebene gezeichnet, die Tiefe von 7 m und die Biologie verlangen aber Wedel ringsum. Jeder Wedelteil ist deshalb ein flacher Fiederloft plus eine Rhachis-Kapsel, sechsmal in 30-Grad-Schritten um die Kronenachse wiederholt, sodass linke und rechte Ketten zusammen 48 Wedel in 12 Richtungen ergeben und die Wiederholungen dem Schwanken ihres Knochens folgen.");
a.note("The trunk is round: its lofts use the elevation half width in x and 0.58 times the front half width in z (the front outline of the bole is about twice as wide as the elevation), the crown shaft the elevation width; dead frond bases (skirt) are two rings of eight inclined capsules; the date bunches are ellipsoids on their own bones with a stalk capsule.", "Der Stamm ist rund: seine Lofts nehmen die Halbbreite der Ansicht in x und das 0,58-fache der Halbbreite der Front in z (der Front-Umriss des Schafts ist etwa doppelt so breit wie in der Ansicht), der Kronenschaft die Breite der Ansicht; tote Wedelbasen (Rock) sind zwei Ringe aus acht geneigten Kapseln; die Dattelbüschel sind Ellipsoide an eigenen Knochen mit Stielkapsel.");
a.save();
