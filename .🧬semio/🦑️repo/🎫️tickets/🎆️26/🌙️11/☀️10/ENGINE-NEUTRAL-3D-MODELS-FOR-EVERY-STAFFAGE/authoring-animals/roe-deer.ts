import { Author } from "../authoring-animals-lib.ts";

const a = new Author("roe-deer");

a.fit("hindbody", "hindbody", { zc: 0, zh: 0.1, p: 0.3, ex: 2.3 });
a.fit("chest", "chest", { zc: 0, zh: 0.09, p: 0.3, ex: 2.2 });
a.fit("neck", "neck", { zc: 0, zh: 0.045, p: 0.5, ex: 2.2 });
a.fit("head", "head", { zc: 0, zh: 0.046, p: 0.35, ex: 2.2 });
a.fit("jaw", "jaw", { zc: 0, zh: 0.022, p: 0.4, ex: 2.2 });
a.ell("nose-leather", "head", [1.29, 0.218, 0], [0.014, 0.012, 0.016], 0, ["nose-leather"]);
a.patch("rump-patch-n", "rump-patch", "hindbody", -1, 0.01);
a.patch("rump-patch-f", "rump-patch", "hindbody", 1, 0.01);
for (const [n, z] of [["near", -1], ["far", 1]] as const) {
  const s = n === "near" ? "n" : "f";
  a.fit(`${n}-ear`, `${n}-ear`, { p: 0.7, ex: 2 });
  a.ell(`${n}-eye`, "head", [1.182, 0.286, z * 0.04], [0.009, 0.009, 0.006], 0, ["eye"]);
  a.fit(`${n}-thigh`, `${n}-thigh`, { zc: z * 0.05, zh: 0.045, p: 0.4, ex: 2.2 });
  a.fit(`${n}-scapula`, `${n}-scapula`, { zc: z * 0.048, zh: 0.042, p: 0.4, ex: 2.2 });
  a.fit(`${n}-humerus`, `${n}-humerus`, { zc: z * 0.05, zh: 0.04, p: 0.4, ex: 2.2 });
  void s;
  for (const p of ["shank", "hind-cannon", "hind-hoof", "forearm", "fore-cannon", "fore-hoof"]) a.fit(`${n}-${p}`, `${n}-${p}`, { p: p.includes("cannon") ? 0.2 : 0.5, ex: p.includes("hoof") ? 2.6 : 2.2 });
}
a.decide("The roe deer is only 0.20 m wide: barrel 0.09 to 0.10 m, neck 0.045 m, head 0.046 m, legs 0.03 to 0.09 m off the midline as drawn in the plan; thigh, scapula and humerus have no lateral twin and are 0.04 to 0.045 m half-width at 0.05 m offset so both legs of a pair stay apart.", "Das Reh ist nur 0,20 m breit: Rumpf 0,09 bis 0,10 m, Hals 0,045 m, Kopf 0,046 m, Beine 0,03 bis 0,09 m neben der Mittellinie wie im Grundriss; Oberschenkel, Schulterblatt und Oberarm haben keine seitliche Entsprechung und erhalten 0,04 bis 0,045 m Halbbreite bei 0,05 m Versatz, damit die beiden Beine eines Paares getrennt bleiben.");
a.decide("The white rump patch is two thin lofts lying 10 mm above the rump surface on both flanks; ears are thin upright lofts leaning outwards; eyes and the nose leather are ellipsoids; brow, mouth line, inner ear and hoof clefts are drawn detail and not modelled; no antlers (none drawn).", "Der weisse Spiegel besteht aus zwei dünnen Lofts, 10 mm über der Hinterteiloberfläche auf beiden Flanken; die Ohren sind dünne, aufrechte, nach aussen geneigte Lofts; Augen und Nasenleder sind Ellipsoide; Braue, Maullinie, Innenohr und Klauenspalt sind Zeichnungsdetail und nicht modelliert; kein Geweih (nicht gezeichnet).");
a.decide("The authored 1:50 elevation is 1.19 m high (hooves at -0.70 m, ear tips at +0.49 m) while dimensions say 1.08 m (+10.3 %); the body follows the drawn shapes and misses the 8 % height gate by this 2D inconsistency.", "Die gezeichnete 1:50-Ansicht ist 1,19 m hoch (Klauen bei -0,70 m, Ohrspitzen bei +0,49 m), die Abmessungen nennen 1,08 m (+10,3 %); der Körper folgt den gezeichneten Formen und verfehlt die 8-%-Höhenschranke wegen dieser Unstimmigkeit der 2D-Ansicht.");
a.write();
