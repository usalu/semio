import { Author } from "../authoring-animals-lib.ts";

const a = new Author("herring-gull");

a.fit("body", "body", { zc: 0, zh: 0.085, p: 0.3, ex: 2.2 });
a.shell("mantle", "body", 1, 0.029, 0.282, 0.5, 0.006, ["mantle"]);
a.fit("tail", "tail", { p: 0.3, ex: 2.2 });
a.fit("neck-1", "neck-1", { p: 0.4, ex: 2 });
a.fit("neck-2", "neck-2", { p: 0.4, ex: 2 });
a.fit("head", "head", { p: 0.4, ex: 2 });
a.fit("bill", "bill", { p: 0.4, ex: 2.4 });
for (const side of ["near", "far"]) {
  const n = side === "near" ? "n" : "f";
  a.fit(`${side}-arm`, `${side}-arm`, { p: 0.2, ex: 2.2 });
  a.fit(`${side}-hand`, `${side}-hand`, { p: 0.2, ex: 2.2 });
  a.fit(`${side}-wingtip`, "wingtip", { bone: `hand-${n}`, from: `${side}-hand`, zhK: 1.06, p: 0.2, ex: 2.2 });
  for (const p of ["thigh", "shank", "tarsus", "foot"]) a.fit(`${side}-${p}`, `${side}-${p}`, { p: p === "foot" ? 0.3 : 0.5, ex: p === "foot" ? 2.4 : 2 });
}
a.decide("Wings are two thin plate lofts (0.034 m thick) on the arm and hand bones, with the near wing tip as a darker overlay; the plate lies in the u-v plane of the bone, so the abduction about v in the soaring and landing poses spreads the plate sideways (checked in gliding and landing). The wings sit 0.068 m off the midline on bones that carry that lateral offset.", "Die Flügel sind zwei dünne Plattenlofts (0,034 m dick) auf Arm- und Handbone, die nahe Flügelspitze als dunklere Auflage; die Platte liegt in der u-v-Ebene der Bone, sodass die Abduktion um v in den Segel- und Landeposen die Platte seitlich ausbreitet (in Gleit- und Landepose geprüft). Die Flügel sitzen 0,068 m neben der Mittellinie auf Bones mit diesem seitlichen Versatz.");
a.decide("The body is a 0.17 m wide loft with the grey mantle as a thin shell over its upper half; neck, head and bill are lofts; the webbed feet are flat lofts 0.06 m wide; no eyes are drawn so none are modelled.", "Der Körper ist ein 0,17 m breiter Loft mit dem grauen Mantel als dünner Schale über der oberen Hälfte; Hals, Kopf und Schnabel sind Lofts; die Schwimmfüsse sind flache, 0,06 m breite Lofts; Augen sind nicht gezeichnet und daher nicht modelliert.");
a.write();
