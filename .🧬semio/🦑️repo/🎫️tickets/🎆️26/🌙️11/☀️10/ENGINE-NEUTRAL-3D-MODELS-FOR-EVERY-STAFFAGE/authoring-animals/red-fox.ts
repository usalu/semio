import { Author } from "../authoring-animals-lib.ts";

const a = new Author("red-fox");

a.fit("hindbody", "hindbody", { zc: 0, zh: 0.08, p: 0.3, ex: 3 });
a.fit("chest", "chest", { zc: 0, zh: 0.07, p: 0.3, ex: 3 });
a.fit("neck", "neck", { zc: 0, zh: 0.058, p: 0.4, ex: 2.6 });
a.fit("head", "head", { zc: 0, zh: 0.044, p: 0.35, ex: 2.3 });
a.fit("jaw", "jaw", { zc: 0, zh: 0.02, p: 0.4, ex: 2.3 });
a.ell("nose", "head", [0.724, 0.026, 0], [0.011, 0.01, 0.012], 0, ["nose"]);
for (const k of ["root", "mid", "tip"]) a.fit(`brush-${k}`, `brush-${k}`, { p: 0.35, ex: 2 });
for (const [n, z] of [["n", -1], ["f", 1]] as const) {
  a.fit(`ear-${n}`, `ear-${n}`, { p: 0.6, ex: 2 });
  a.ell(`eye-${n}`, "head", [0.6525, 0.0965, z * 0.028], [0.0105, 0.0095, 0.008], 0, ["eye"]);
  a.fit(`thigh-${n}`, "thigh-n", { bone: `thigh-h${n}`, zc: z * 0.04, zh: 0.038, p: 0.4, ex: 2.2 }, { tone: n === "n" ? 0.5 : 0.4 });
  a.tube(`scapula-${n}`, `scapula-f${n}`, [
    { p: [0.431, 0.025, z * 0.042], rz: 0.032, ry: 0.036 },
    { p: [0.458, -0.022, z * 0.042], rz: 0.034, ry: 0.04 },
    { p: [0.486, -0.07, z * 0.042], rz: 0.03, ry: 0.034 },
  ], ["thigh-n"], { region: "coat-trunk", tone: 0.5 });
  a.tube(`humerus-${n}`, `humerus-f${n}`, [
    { p: [0.486, -0.07, z * 0.042], rz: 0.03, ry: 0.034 },
    { p: [0.46, -0.119, z * 0.041], rz: 0.029, ry: 0.032 },
    { p: [0.434, -0.167, z * 0.04], rz: 0.025, ry: 0.026 },
  ], ["thigh-n"], { region: "coat-legs", tone: 0.5 });
  for (const p of ["gaskin", "cannon-h", "paw-h", "forearm", "cannon-f", "paw-f"]) a.fit(`${p}-${n}`, `${p}-${n}`, { p: p.includes("cannon") ? 0.2 : 0.5, ex: p.includes("paw") ? 2.4 : 2.2 });
}
a.decide("Trunk, neck, head, jaw, brush and limbs are lofts: sagittal sizes from the 1:50 elevation outline of the bone, lateral sizes from plan and front (barrel 0.07 to 0.08 m, brush up to 0.078 m, lower legs 0.016 to 0.05 m off the midline). The thigh, shoulder blade and humerus are not drawn separately; the far thigh reuses the near outline, shoulder and humerus are short tubes 0.04 m off the midline.", "Rumpf, Hals, Kopf, Kiefer, Lunte und Gliedmassen sind Lofts: senkrechte Masse aus der 1:50-Ansicht der Bone, seitliche Masse aus Grundriss und Vorderansicht (Rumpf 0,07 bis 0,08 m, Lunte bis 0,078 m, Unterbeine 0,016 bis 0,05 m neben der Mittellinie). Oberschenkel, Schulterblatt und Oberarm sind nicht einzeln gezeichnet; der ferne Oberschenkel übernimmt die nahe Kontur, Schulter und Oberarm sind kurze Röhren 0,04 m neben der Mittellinie.");
a.decide("Ears are thin upright lofts; eyes and nose are ellipsoids; the mouth line is drawn detail and not modelled.", "Die Ohren sind dünne aufrechte Lofts; Augen und Nase sind Ellipsoide; die Maullinie ist Zeichnungsdetail und nicht modelliert.");
a.write();
