import { Author } from "../authoring-animals-lib.ts";

const a = new Author("horse");

a.fit("hindbody", "hindbody", { zc: 0, zh: 0.3, p: 0.3, ex: 2.4 });
a.fit("chest", "chest", { zc: 0, zh: 0.3, p: 0.3, ex: 2.3 });
for (const k of ["1", "2", "3"]) a.fit(`neck-${k}`, `neck-${k}`, { p: 0.5, ex: 2.2 });
for (const k of ["1", "2", "3"]) a.fit(`mane-${k}`, `mane-${k}`, { from: `mane-${k}`, zc: 0, zh: 0.026, p: 0, ex: 2 });
a.fit("head", "head", { zc: 0, zh: 0.095, p: 0.35, ex: 2.3 });
a.fit("muzzle", "muzzle", { zc: 0, zh: 0.052, p: 0.3, ex: 2.5 });
a.fit("jaw", "jaw", { zc: 0, zh: 0.075, p: 0.4, ex: 2.3 });
a.fit("forelock", "forelock", { zc: 0, zh: 0.03, p: 0.3, ex: 2 });
for (const [n, z, bone] of [["near", -1, "ear-n"], ["far", 1, "ear-f"]] as const) {
  a.fit(`${n}-ear`, `${n}-ear`, { p: 0.7, ex: 2 });
  a.ell(`${n}-eye`, "head", [2.0, 0.5, z * 0.088], [0.016, 0.016, 0.012], 0, [`${n}-eye`]);
}
for (const [n, z] of [["n", -1], ["f", 1]] as const) a.ell(`nostril-${n}`, "head", [2.18, 0.32, z * 0.026], [0.012, 0.01, 0.01], 0, [`nostril-${n}`], { mat: "bare-skin", tone: 0.1 });
for (const side of ["near", "far"]) {
  for (const p of ["thigh", "gaskin", "hind-cannon", "hind-pastern", "hind-hoof", "scapula", "humerus", "radius", "fore-cannon", "fore-pastern", "fore-hoof"]) a.fit(`${side}-${p}`, `${side}-${p}`, { p: p.includes("hoof") ? 0.5 : 0.5, ex: p.includes("hoof") ? 2.6 : 2.2 });
}
for (const k of ["1", "2", "3", "4"]) a.fit(`tail-${k}`, `tail-${k}`, { p: 0.3, ex: 2 });
a.decide("Trunk, three neck segments, head, limbs and tail are lofts: sagittal half-heights from the 1:50 elevation outline of the bone, lateral half-widths and offsets from the same-id shapes of the plan and front views (barrel 0.30 m, thigh 0.115 m, cannon 0.05 m, legs 0.1 to 0.2 m off the midline), so the four legs hang on their own bones under the body.", "Rumpf, drei Halssegmente, Kopf, Beine und Schweif sind Lofts: die senkrechten Halbhöhen stammen aus der 1:50-Ansicht der Bone, die seitlichen Halbbreiten und Versätze aus den gleichnamigen Formen in Grundriss und Vorderansicht (Rumpf 0,30 m, Oberschenkel 0,115 m, Röhrbein 0,05 m, Beine 0,1 bis 0,2 m neben der Mittellinie), sodass die vier Beine auf eigenen Bones unter dem Körper hängen.");
a.decide("The mane is three thin ribbons (52 mm wide) on the neck crest, each bound to its neck bone; ears are upright thin lofts; eyes and nostrils are small ellipsoids that only the fine scales show (maxDenominator). Engraved lines (hip, shoulder, stifle, knee, hock, hoof, mouth) and the brow are not modelled.", "Die Mähne besteht aus drei dünnen Bändern (52 mm breit) auf dem Halskamm, je an ihrem Halsknochen; die Ohren sind aufrechte dünne Lofts; Augen und Nüstern sind kleine Ellipsoide, die nur die feinen Massstäbe zeigen (maxDenominator). Gravurlinien (Hüfte, Schulter, Knie, Sprunggelenk, Huf, Maul) und die Braue sind nicht modelliert.");
a.decide("The authored 2D elevation measures 2.76 m by 2.24 m while dimensions say 2.91 m by 2.37 m (5.3 % more on both axes); the body follows the drawn 1:50 shapes and stays inside the 8 % gate.", "Die gezeichnete 2D-Ansicht misst 2,76 m mal 2,24 m, die Abmessungen nennen 2,91 m mal 2,37 m (5,3 % mehr in beiden Achsen); der Körper folgt den gezeichneten 1:50-Formen und liegt innerhalb der 8-%-Schranke.");
a.write();
