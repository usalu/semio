import { Author } from "../authoring-animals-lib.ts";

const a = new Author("red-squirrel");

a.fit("rump", "rump", { zc: 0, zh: 0.054, p: 0.5, ex: 2 });
a.fit("chest", "chest", { zc: 0, zh: 0.045, p: 0.5, ex: 2 });
a.stripe("belly-patch", "chest", -1, 0.0, 0.099, 0.02, 0.006, ["belly-patch"]);
a.fit("neck", "neck", { zc: 0, zh: 0.028, p: 0.4, ex: 2 });
a.fit("head", "head", { zc: 0, zh: 0.03, p: 0.4, ex: 2 });
a.ell("nose", "head", [0.235, 0.0, 0], [0.006, 0.006, 0.007], 0, ["head"], { regions: ["nose"], mat: "bare-skin", tone: 0.1 });
a.fit("ear-near", "ear", { zc: -0.027, zh: 0.009, p: 0.4, ex: 2 });
a.fit("ear-far", "ear", { zc: 0.027, zh: 0.009, p: 0.4, ex: 2 });
a.fit("tail-1", "tail-1", { p: 0.3, ex: 2 });
a.fit("tail-2", "tail-2", { p: 0.3, ex: 2 });
for (const side of ["near", "far"]) {
  for (const p of ["arm", "forearm", "hand", "thigh", "shank", "foot"]) a.fit(`${side}-${p}`, `${side}-${p}`, { p: 0.5, ex: 2.2 });
}
a.decide("Rump and chest are round lofts (0.054 and 0.045 m radius), neck and head 0.056 and 0.06 m wide, both tail segments bushy lofts 0.086 to 0.094 m wide; arms and legs are lofts on their own bones with lateral offsets of 0.025 to 0.06 m from plan and front. The white belly patch is a thin stripe on the underside of the chest.", "Hinterleib und Brust sind runde Lofts (0,054 und 0,045 m Radius), Hals und Kopf 0,056 und 0,06 m breit, beide Schwanzsegmente buschige Lofts von 0,086 bis 0,094 m Breite; Arme und Beine sind Lofts auf eigenen Bones mit seitlichen Versätzen von 0,025 bis 0,06 m aus Grundriss und Vorderansicht. Der weisse Bauchfleck ist ein dünner Streifen an der Brustunterseite.");
a.decide("The authored rest elevation is 0.416 m long (tail tip at -0.175 m to nose at +0.241 m) and 0.262 m high, while dimensions say 0.21 m by 0.275 m, which describe the upright sitting pose; the volume follows the drawn rest shapes, so the x extent misses the 8 % gate by +98 % while the sitting pose measures 0.196 m by 0.281 m.", "Die gezeichnete Ruheansicht ist 0,416 m lang (Schwanzspitze bei -0,175 m bis Nase bei +0,241 m) und 0,262 m hoch, die Abmessungen nennen 0,21 m mal 0,275 m und beschreiben die aufrecht sitzende Pose; das Volumen folgt den gezeichneten Ruheformen, sodass die x-Ausdehnung die 8-%-Schranke um +98 % verfehlt, während die sitzende Pose 0,196 m mal 0,281 m misst.");
a.write();
