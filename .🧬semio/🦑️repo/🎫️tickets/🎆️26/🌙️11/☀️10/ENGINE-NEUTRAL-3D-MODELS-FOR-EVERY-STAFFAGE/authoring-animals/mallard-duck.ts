import { Author } from "../authoring-animals-lib.ts";

const a = new Author("mallard-duck");

a.fit("body", "body", { zc: 0, zh: 0.102, p: 0.3, ex: 2 });
a.slice("breast", "body", 0.247, 0.346, 0.004, ["breast"]);
a.fit("tail", "tail", { p: 0.3, ex: 2.2 });
a.fit("neck-1", "neck-1", { p: 0.4, ex: 2 });
a.fit("neck-2", "neck-2", { p: 0.4, ex: 2 });
a.slice("neck-ring", "neck-2", 0.022, 0.042, 0.003, ["neck-ring"]);
a.fit("neck-3", "neck-3", { p: 0.4, ex: 2 });
a.fit("head", "head", { p: 0.4, ex: 2 });
a.fit("bill", "bill", { p: 0.4, ex: 2.6 });
for (const side of ["near", "far"]) {
  const n = side === "near" ? "n" : "f";
  a.fit(`${side}-wing`, `${side}-wing`, { p: 0.2, ex: 2.2 });
  a.fit(`${side}-speculum`, "speculum", { bone: `wing-${n}`, from: `${side}-wing`, zhK: 1.08, p: 0.2, ex: 2.2 }, { tone: n === "n" ? undefined : 0.5 });
  for (const p of ["thigh", "shank", "tarsus", "foot"]) a.fit(`${side}-${p}`, `${side}-${p}`, { p: p === "foot" ? 0.3 : 0.5, ex: p === "foot" ? 2.4 : 2 });
}
a.decide("The body is a round loft 0.20 m wide and high with the chestnut breast as a 4 mm thicker slice of its front end; neck in three lofts with the white ring as a 3 mm collar slice; head and flat bill are lofts. Each wing is a single 0.3 m plate loft (0.038 m thick) on the wing bone 0.058 m off the midline, lying in the u-v plane so that abduction about v spreads it sideways; the speculum is a thin overlay on the wing.", "Der Körper ist ein runder Loft von 0,20 m Breite und Höhe, die kastanienbraune Brust eine 4 mm dickere Scheibe am Vorderende; der Hals besteht aus drei Lofts mit dem weissen Ring als 3 mm dicker Kragenscheibe; Kopf und flacher Schnabel sind Lofts. Jeder Flügel ist ein einzelner 0,3 m langer Plattenloft (0,038 m dick) auf der Flügelbone 0,058 m neben der Mittellinie, in der u-v-Ebene, sodass die Abduktion um v ihn seitlich ausbreitet; der Spiegel ist eine dünne Auflage auf dem Flügel.");
a.decide("The webbed feet are flat lofts 0.07 to 0.09 m wide; primary lines, toe line and bill line are drawn detail and not modelled; no eyes are drawn so none are modelled.", "Die Schwimmfüsse sind flache Lofts von 0,07 bis 0,09 m Breite; Handschwingenlinien, Zehenlinie und Schnabellinie sind Zeichnungsdetail und nicht modelliert; Augen sind nicht gezeichnet und daher nicht modelliert.");
a.decide("The authored 1:50 elevation is 0.491 m high (feet at -0.241 m, head top at +0.250 m) while dimensions say 0.46 m (+6.7 %); the body follows the drawn shapes.", "Die gezeichnete 1:50-Ansicht ist 0,491 m hoch (Füsse bei -0,241 m, Kopfoberkante bei +0,250 m), die Abmessungen nennen 0,46 m (+6,7 %); der Körper folgt den gezeichneten Formen.");
a.write();
