import { Author } from "../authoring-animals-lib.ts";

const a = new Author("dairy-cow");

a.fit("hindbody", "hindbody", { zh: 0.31, p: 0.3, ex: 2.4, zhAt: (f) => 0.3 + 0.03 * Math.sin(Math.PI * Math.min(1, f * 1.4)) });
a.fit("chest", "chest", { zh: 0.36, p: 0.3, ex: 2.3 });
a.patch("patch-rump", "patch-rump", "hindbody", -1);
a.patch("patch-barrel", "patch-barrel", "chest", -1);
a.patch("patch-chest", "patch-chest", "chest", -1);
a.patch("rumen-flank", "rumen-flank", "hindbody", 1);

a.fit("neck", "neck", { zh: 0.19, p: 0.5, ex: 2.2 });
a.fit("head", "head", { zh: 0.11, p: 0.35, ex: 2.3 });
a.fit("muzzle", "muzzle", { zh: 0.062, p: 0.3, ex: 2.5 });
a.fit("jaw", "jaw", { zh: 0.05, p: 0.4, ex: 2.3 });
for (const [n, z, bone] of [["near", -1, "ear-n"], ["far", 1, "ear-f"]] as const) {
  a.ell(`${n}-ear`, bone, [1.86, 0.21, z * 0.19], [0.11, 0.04, 0.13], -8, [`${n}-ear`]);
  a.ell(`${n}-eye`, "head", [1.995, 0.063, z * 0.1], [0.02, 0.02, 0.013], 0, [`${n}-eye`]);
}
for (const [n, z] of [["n", -1], ["f", 1]] as const) a.ell(`nostril-${n}`, "head", [2.171, -0.27, z * 0.035], [0.008, 0.011, 0.012], 0, [`nostril-${n}`], { mat: "bare-skin", tone: 0.1 });

const legs: [string, string, number, number, number, string][] = [];
for (const [side, sfx, zs] of [["near", "n", -1], ["far", "f", 1]] as const) {
  a.fit(`${side}-thigh`, `${side}-thigh`, { zh: 0.13, zc: zs * 0.22, p: 0.5, ex: 2.2 });
  a.fit(`${side}-gaskin`, `${side}-gaskin`, { zh: 0.09, zc: zs * 0.22, p: 0.5, ex: 2.2 });
  a.fit(`${side}-hind-cannon`, `${side}-hind-cannon`, { zh: 0.05, zc: zs * 0.2, p: 0.2 });
  a.fit(`${side}-hind-hoof`, `${side}-hind-hoof`, { zh: 0.07, zc: zs * 0.2, p: 0.5, ex: 2.6 });
  a.fit(`${side}-scapula`, `${side}-scapula`, { zh: 0.11, zc: zs * 0.2, p: 0.5, ex: 2.2 });
  a.fit(`${side}-humerus`, `${side}-humerus`, { zh: 0.1, zc: zs * 0.2, p: 0.5, ex: 2.2 });
  a.fit(`${side}-forearm`, `${side}-forearm`, { zh: 0.08, zc: zs * 0.19, p: 0.4, ex: 2.2 });
  a.fit(`${side}-fore-cannon`, `${side}-fore-cannon`, { zh: 0.05, zc: zs * 0.18, p: 0.2 });
  a.fit(`${side}-fore-hoof`, `${side}-fore-hoof`, { zh: 0.07, zc: zs * 0.18, p: 0.5, ex: 2.6 });
}
a.fit("tail-1", "tail-1", { zh: 0.04, p: 0.3 });
a.fit("tail-2", "tail-2", { zh: 0.034, p: 0.2 });
a.fit("tail-3", "tail-3", { zh: 0.028, p: 0.2 });
a.fit("tail-switch", "tail-switch", { zh: 0.065, p: 0.6 });
a.fit("udder", "udder", { zh: 0.12, p: 0.5, ex: 2.3 });
a.cap("teat-front", "udder", [0.26, -0.57, -0.06], [0.26, -0.7, -0.06], 0.017, 0.014, ["teat-front"], { repeat: { count: 2, translation: [0, 0, 0.12] } });
a.cap("teat-rear", "udder", [0.1, -0.57, -0.06], [0.1, -0.7, -0.06], 0.017, 0.014, ["teat-rear"], { repeat: { count: 2, translation: [0, 0, 0.12] } });
a.stripe("blaze", "head", 1, 0.1, 0.5, 0.03, 0.012, ["blaze"], {});
a.decide("Trunk, neck, head and limb segments are lofts whose sagittal half-heights come from the 1:50 elevation outline of the same bone and whose lateral half-widths come from the front and plan views (barrel 0.30 to 0.36 m, thighs 0.13 m, cannons 0.05 m); the four legs sit laterally at 0.18 to 0.22 m on their own bones.", "Rumpf, Hals, Kopf und Beinsegmente sind Lofts: die senkrechten Halbhöhen stammen aus der 1:50-Ansicht der jeweiligen Bone, die seitlichen Halbbreiten aus Vorderansicht und Grundriss (Rumpf 0,30 bis 0,36 m, Oberschenkel 0,13 m, Röhrbeine 0,05 m); die vier Beine stehen seitlich bei 0,18 bis 0,22 m auf eigenen Bones.");
a.decide("Black coat patches are thin lofts that follow the outline of the elevation patch and lie 10 mm above the barrel surface of the trunk loft on the near side; the rumen flank patch lies on the left (far) side where the rumen is.", "Schwarze Fellflecken sind dünne Lofts entlang der Umrisse der Ansichtsflecken, 10 mm über der Rumpfoberfläche auf der nahen Seite; der Pansenfleck liegt links (ferne Seite), wo der Pansen liegt.");
a.decide("Engraved detail lines (mouth line, hoof clefts, knee crease) are not modelled; the white blaze is a thin stripe on the face, ears are flat ellipsoids sticking out sideways, teats are two repeated capsules each.", "Gravurlinien (Maullinie, Klauenspalt, Karpalfalte) sind nicht modelliert; die weisse Blesse ist ein dünner Streifen am Gesicht, die Ohren sind flache, seitlich abstehende Ellipsoide, die Zitzen je eine wiederholte Kapsel.");
a.write();
