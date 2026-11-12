import { Author } from "../authoring-animals-lib.ts";

const a = new Author("labrador-dog");

a.fit("hindbody", "hindbody", { zc: 0, zh: 0.13, p: 0.3, ex: 2.3 });
a.fit("chest", "chest", { zc: 0, zh: 0.14, p: 0.3, ex: 2.3 });
a.fit("neck", "neck", { zc: 0, zh: 0.075, p: 0.5, ex: 2.2 });
a.fit("head", "head", { zc: 0, zh: 0.06, p: 0.35, ex: 2.3 });
a.fit("jaw", "jaw", { zc: 0, zh: 0.04, p: 0.4, ex: 2.3 });
a.fit("tongue", "tongue", { zc: 0, zh: 0.02, p: 0.3, ex: 2 });
a.ell("nose-leather", "head", [0.976, 0.134, 0], [0.014, 0.014, 0.022], 0, ["nose-leather"]);
a.fit("tail-root", "tail-root", { from: "tail-1", zc: 0, p: 0.4, ex: 2 });
a.fit("tail-mid", "tail-mid", { from: "tail-2", zc: 0, p: 0.4, ex: 2 });
a.fit("tail-tip", "tail-tip", { zc: 0, p: 0.4, ex: 2 });
for (const side of ["near", "far"]) {
  const z = side === "near" ? -1 : 1;
  const s = side === "near" ? "n" : "f";
  a.fit(`${side}-ear`, `${side}-ear`, { zc: z * 0.07, zh: 0.025, p: 0.5, ex: 2 });
  a.ell(`${side}-eye`, "head", [0.8645, 0.1605, z * 0.035], [0.0075, 0.0065, 0.006], 0, ["eye"], { mat: "bare-skin", tone: 0.05 });
  for (const p of ["thigh", "shank", "hock", "hind-paw", "scapula", "humerus", "forearm", "wrist", "forepaw"]) a.fit(`${side}-${p}`, `${side}-${p}`, { p: 0.5, ex: p.includes("paw") ? 2.4 : 2.2 });
  for (const p of ["hind", "fore"]) a.ell(`${side}-${p}-pad`, `paw-${p === "hind" ? "h" : "f"}${s}`, p === "hind" ? [0.05, -0.498, z * 0.08] : [0.499, -0.5145, z * 0.075], [0.047, 0.013, 0.026], -16, [`${side}-${p}-pad`]);
}
a.decide("Trunk, neck, head, jaw, tail and the nine segments of each leg are lofts: sagittal sizes from the 1:50 elevation outline of the bone, lateral sizes and offsets from plan and front (barrel 0.13 to 0.14 m, legs 0.03 to 0.14 m off the midline), so the four legs sit under the body on their own bones.", "Rumpf, Hals, Kopf, Kiefer, Schwanz und die neun Segmente jedes Beins sind Lofts: senkrechte Masse aus der 1:50-Ansicht der Bone, seitliche Masse und Versätze aus Grundriss und Vorderansicht (Rumpf 0,13 bis 0,14 m, Beine 0,03 bis 0,14 m neben der Mittellinie), sodass die vier Beine auf eigenen Bones unter dem Körper stehen.");
a.decide("Ears are thin hanging lofts 0.07 m off the midline; the tongue is a thin loft inside the jaw bone so it appears when the jaw opens; the four paw pads are flat ellipsoids under the paws; eyes and the nose leather are ellipsoids; brow, mouth line, toe, shoulder, hip and elbow lines are drawn detail and not modelled.", "Die Ohren sind dünne hängende Lofts 0,07 m neben der Mittellinie; die Zunge ist ein dünner Loft in der Kieferbone, damit sie beim Öffnen sichtbar wird; die vier Ballen sind flache Ellipsoide unter den Pfoten; Augen und Nasenleder sind Ellipsoide; Braue, Maullinie, Zehen-, Schulter-, Hüft- und Ellbogenlinien sind Zeichnungsdetail und nicht modelliert.");
a.write();
