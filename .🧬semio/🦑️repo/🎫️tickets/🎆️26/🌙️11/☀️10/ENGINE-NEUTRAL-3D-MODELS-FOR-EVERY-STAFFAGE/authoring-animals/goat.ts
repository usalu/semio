import { Author } from "../authoring-animals-lib.ts";

const a = new Author("goat");

a.fit("hindbody", "hindbody", { zc: 0, zh: 0.16, p: 0.3, ex: 2.4 });
a.fit("chest", "chest", { zc: 0, zh: 0.17, p: 0.3, ex: 2.3 });
a.fit("neck", "neck", { zc: 0, zh: 0.095, p: 0.5, ex: 2.2 });
a.fit("head", "head", { zc: 0, zh: 0.06, p: 0.35, ex: 2.3 });
a.fit("jaw", "jaw", { zc: 0, zh: 0.04, p: 0.4, ex: 2.3 });
a.fit("beard", "beard", { from: "beard", p: 0.4, ex: 2 });
for (const [n, z] of [["n", -1], ["f", 1]] as const) {
  const side = n === "n" ? "near" : "far";
  a.ell(`ear-${n}`, `ear-${n}`, [1.095, 0.303, z * 0.093], [0.056, 0.036, 0.071], 0, [`ear-${n}`]);
  a.ell(`eye-${n}`, "head", [1.075, 0.243, z * 0.045], [0.011, 0.011, 0.008], 0, [`eye-${n}`, "eye"]);
  a.tube(`horn-${n}`, "head", [
    { p: [1.043, 0.298, z * 0.032], rz: 0.024, ry: 0.024 },
    { p: [1.052, 0.345, z * 0.037], rz: 0.022, ry: 0.022 },
    { p: [1.032, 0.388, z * 0.046], rz: 0.018, ry: 0.018 },
    { p: [0.992, 0.417, z * 0.058], rz: 0.013, ry: 0.013 },
    { p: [0.952, 0.436, z * 0.068], rz: 0.009, ry: 0.009 },
    { p: [0.92, 0.442, z * 0.077], rz: 0.003, ry: 0.003 },
  ], [`horn-${n}`]);
  a.tube(`${side}-humerus`, `humerus-f${n}`, [
    { p: [0.748, -0.04, z * 0.09], rz: 0.048, ry: 0.05 },
    { p: [0.71, -0.12, z * 0.09], rz: 0.043, ry: 0.044 },
    { p: [0.672, -0.2, z * 0.09], rz: 0.036, ry: 0.038 },
  ], ["near-humerus"], { region: "coat-upper-legs", tone: 0.72 });
  a.fit(`${side}-thigh`, "thigh-n", { bone: `thigh-h${n}`, from: `${side}-thigh`, p: 0.5, ex: 2.2 }, { tone: n === "n" ? 0.72 : 0.5 });
  a.fit(`${side}-shoulder`, "shoulder-n", { bone: `scapula-f${n}`, from: `${side}-shoulder`, p: 0.5, ex: 2.2 }, { tone: n === "n" ? 0.72 : 0.5 });
  a.fit(`gaskin-${n}`, `gaskin-${n}`, { from: `${side}-gaskin`, p: 0.5, ex: 2.2 });
  a.fit(`cannon-h-${n}`, `cannon-h-${n}`, { from: `${side}-cannon-h`, p: 0.2 });
  a.fit(`hoof-h-${n}`, `hoof-h-${n}`, { from: `${side}-hoof-h`, p: 0.5, ex: 2.6 });
  a.fit(`forearm-${n}`, `forearm-${n}`, { from: `${side}-forearm`, p: 0.4, ex: 2.2 });
  a.fit(`cannon-f-${n}`, `cannon-f-${n}`, { from: `${side}-cannon-f`, p: 0.2 });
  a.fit(`hoof-f-${n}`, `hoof-f-${n}`, { from: `${side}-hoof-f`, p: 0.5, ex: 2.6 });
}
a.fit("tail-root", "tail-root", { p: 0.4, ex: 2 });
a.fit("tail-tip", "tail-tip", { p: 0.4, ex: 2 });
a.decide("Trunk, neck, head, limb segments and tail are lofts sized from the elevation outline of each bone with lateral half-widths from the same shapes in the front and plan views (barrel 0.16 to 0.17 m, thighs 0.07 m, cannons 0.025 m); the far thigh and far shoulder reuse the near outline on their own bones because the elevation hides them.", "Rumpf, Hals, Kopf, Beinsegmente und Schwanz sind Lofts, deren senkrechte Masse aus der Ansicht jeder Bone und deren seitliche Halbbreiten aus denselben Formen in Vorderansicht und Grundriss stammen (Rumpf 0,16 bis 0,17 m, Oberschenkel 0,07 m, Röhrbeine 0,025 m); fernes Oberschenkel- und Schulterstück übernehmen die nahe Kontur auf ihren Bones, weil die Ansicht sie verdeckt.");
a.decide("The humerus has no elevation outline; it is a tapered tube between shoulder and elbow. Horns are tapering tubes following the drawn sickle, splayed outwards; ears are flat ellipsoids pointing sideways; engraved lines (mouth, hock) are not modelled.", "Der Oberarm hat keine Kontur in der Ansicht; er ist eine sich verjüngende Röhre zwischen Schulter und Ellbogen. Die Hörner sind sich verjüngende Röhren entlang der gezeichneten Sichel, nach aussen gespreizt; die Ohren sind flache, seitlich weisende Ellipsoide; Gravurlinien (Maul, Sprunggelenk) sind nicht modelliert.");
a.write();
