import { L } from "./lib.ts";
import { bandOf } from "./bands.ts";
import { readFileSync, writeFileSync } from "node:fs";
const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(`${L}/🔨️modules/🖨️representation/🟦️.ts`);
const { validateAsset } = await import(`${L}/🔨️modules/🗂️catalog/🟦️.ts`);
const write = process.argv[2] === "write";
const text = { en: { silhouette: "Merged site-plan silhouette of a standing figure: head, torso, one form per arm and per leg; side, front and top view are drawn from the same bones, so every pose and clip of the test figure has all three views at 1:500.", retained: "Head, torso, arms and legs at the real height of 1.73 m, with the arms and legs of each side apart.", merged: "Hair, face, hands and shoes are part of the neighbouring forms.", omitted: "Clothing detail, face features and fingers fall below the 0.2 mm feature limit at 1:500.", thinElements: "Limbs are widened to the 0.1 m minimum feature (0.2 mm on paper).", hatching: "None.", styles: "Line and solid styles as at 1:200.", overlays: "Thermal and structural overlays use the merged regions." },
  de: { silhouette: "Zusammengefasste Lageplan-Silhouette einer stehenden Figur: Kopf, Rumpf, je eine Form pro Arm und Bein; Seiten-, Vorder- und Draufsicht stammen aus denselben Knochen, sodass jede Pose und jeder Clip der Testfigur bei 1:500 alle drei Ansichten hat.", retained: "Kopf, Rumpf, Arme und Beine in der wirklichen Größe von 1,73 m, Arme und Beine jeder Seite getrennt.", merged: "Haar, Gesicht, Hände und Schuhe gehören zu den Nachbarformen.", omitted: "Bekleidungsdetails, Gesichtszüge und Finger unterschreiten bei 1:500 die Mindestbreite von 0,2 mm.", thinElements: "Gliedmaßen sind auf die Mindestbreite von 0,1 m (0,2 mm auf dem Papier) verbreitert.", hatching: "Keine.", styles: "Linien- und Flächenstile wie bei 1:200.", overlays: "Thermische und strukturelle Überlagerungen nutzen die zusammengefassten Regionen." } };
for (const f of process.argv.slice(3)) {
  const path = `${L}/🧫️fixtures/${f}/🔣️.json`; const doc = JSON.parse(readFileSync(path, "utf8")); const a = doc.asset ?? doc;
  const spec: any = { slug: a.id, den: 500, drop: /^(ear.*|eye.*|brow.*|nose|mouth|hair.*|lace.*|sole.*|cuff.*|neck.*|collar.*|thumb.*|finger.*|button.*|hood.*|seam.*|crease.*|fist.*)$/, minFeatureM: 0.105, epsM: 0.01, pens: { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.18 }, range: [350, 900], en: text.en, de: text.de };
  const rep = bandOf(a, spec); a.representations["500"] = rep;
  const issues = validateAsset(a);
  const s = measureSuitability(a, rep, 500, DEFAULT_PRINT_PROFILE);
  console.log(f, "shapes", ["elevation", "front", "plan"].map((v) => rep.geometry[v].length).join("/"), "issues", issues.length, issues.slice(0, 6).map((i: any) => i.code + ":" + (i.message?.en ?? "").slice(0, 120)).join(" | "), "| suitable", s.suitable, s.violations.length, s.violations.slice(0, 3).map((v: any) => v.check + ":" + v.shapes.join("+")).join(" "));
  if (write) writeFileSync(path, JSON.stringify(doc, null, 2) + "\n");
}
