import { Author } from "../authoring-animals-lib.ts";

const a = new Author("domestic-cat");

a.fit("hindbody", "hindbody", { zc: 0, zh: 0.06, p: 0.3, ex: 2.3 });
a.fit("midbody", "midbody", { zc: 0, zh: 0.064, p: 0.3, ex: 2.3 });
a.fit("chest", "chest", { zc: 0, zh: 0.064, p: 0.3, ex: 2.3 });
a.fit("neck", "neck", { zc: 0, zh: 0.042, p: 0.5, ex: 2.2 });
a.fit("head", "head", { zc: 0, zh: 0.046, p: 0.35, ex: 2.1 });
a.fit("jaw", "jaw", { zc: 0, zh: 0.026, p: 0.4, ex: 2.2 });
a.fit("tongue", "tongue", { zc: 0, zh: 0.012, p: 0.3, ex: 2 });
a.ell("nose-leather", "head", [0.462, 0.049, 0], [0.006, 0.005, 0.009], 0, ["nose-leather", "nose"]);
for (const k of ["1", "2", "3", "4", "5"]) a.fit(`tail-${k}`, `tail-${k}`, { p: 0.3, ex: 2 });
for (const side of ["near", "far"]) {
  const z = side === "near" ? -1 : 1;
  const s = side === "near" ? "n" : "f";
  a.fit(`${side}-ear`, `${side}-ear`, { p: 0.6, ex: 2 });
  a.ell(`${side}-eye`, "head", [0.4315, 0.064, z * 0.022], [0.0058, 0.0062, 0.0055], 0, ["eye", `${side}-eye`]);
  for (const p of ["thigh", "shank", "hock", "hind-paw", "scapula", "humerus", "forearm", "wrist", "fore-paw"]) a.fit(`${side}-${p}`, `${side}-${p}`, { p: 0.5, ex: p.includes("paw") ? 2.4 : 2.2 });
  a.ell(`${side}-hind-pad`, `paw-h${s}`, [0.0285, -0.2445, z * 0.035], [0.031, 0.011, 0.014], -10, [`${side}-hind-pad`]);
  a.ell(`${side}-fore-pad`, `paw-f${s}`, [0.2665, -0.232, z * 0.032], [0.031, 0.011, 0.014], -10, [`${side}-fore-pad`]);
}
a.decide("The cat is 0.13 m wide: three trunk lofts (hind, mid, chest) of 0.06 to 0.064 m half-width, neck 0.042 m, head 0.046 m, five tail lofts of 0.016 to 0.02 m and nine limb segments per leg with lateral offsets from plan and front (0.015 to 0.06 m off the midline); sagittal sizes come from the 1:50 elevation outline of each bone.", "Die Katze ist 0,13 m breit: drei Rumpflofts (Hinterleib, Mitte, Brust) mit 0,06 bis 0,064 m Halbbreite, Hals 0,042 m, Kopf 0,046 m, fünf Schwanzlofts mit 0,016 bis 0,02 m und neun Beinsegmente je Bein mit seitlichen Versätzen aus Grundriss und Vorderansicht (0,015 bis 0,06 m neben der Mittellinie); die senkrechten Masse stammen aus der 1:50-Ansicht jeder Bone.");
a.decide("Ears are thin upright lofts; the tongue is a thin loft in the jaw bone; pads are flat ellipsoids under the paws; eyes and nose leather are ellipsoids; brow, mouth, inner ear, toe, shoulder and hip lines are drawn detail and not modelled.", "Die Ohren sind dünne aufrechte Lofts; die Zunge ist ein dünner Loft in der Kieferbone; die Ballen sind flache Ellipsoide unter den Pfoten; Augen und Nasenleder sind Ellipsoide; Braue, Maul-, Innenohr-, Zehen-, Schulter- und Hüftlinien sind Zeichnungsdetail und nicht modelliert.");
a.decide("The authored 1:50 elevation is 0.386 m high (paws at -0.257 m, ear tips at +0.129 m) while dimensions say 0.36 m (+7.2 %); the body follows the drawn shapes.", "Die gezeichnete 1:50-Ansicht ist 0,386 m hoch (Pfoten bei -0,257 m, Ohrspitzen bei +0,129 m), die Abmessungen nennen 0,36 m (+7,2 %); der Körper folgt den gezeichneten Formen.");
a.write();
