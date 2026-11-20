/** 🌡️ The 1:50 thermal model of `humans/pregnant-person` in space: every region of the asset on the new body, the layered regions as nested shells with their declared thicknesses (T-shirt under the knit on trunk, upper arms and abdomen, briefs under the leggings at the seat, socks inside the boots), the face as the head ring below the hair line, and the three outlets on the skin and the crown.
 * @see ./r3-pregnant-person-s50.ts
 * @see ./design.md (B2, C18, C20, C29, D2) */
import type { Json } from "./volumes-lib.ts";
import { dense, ellipsoid, inset, loft, pa, sa, thOf, type St } from "./r3-pregnant-person-kit.ts";
import { C, MAT, SLEEVE_L, TONE, bellyStations, seatStations, sweaterStations } from "./r3-pregnant-person-s50.ts";

/** 🧅 A loft of the body list moved inwards by `by` (an inner layer shell), as a thermal solid of `region` and `layer`. */
export const shell = (src: Json, id: string, region: string, layer: string, by: number, material: string): Json => ({ ...src, id, material, tone: 0.8, region, layer, sections: src.sections.map((q: Json) => ({ ...q, rx: Math.max(0.002, q.rx - by), ry: Math.max(0.002, q.ry - by) })) });

export function thermal50(b: Json[]): Json[] {
  const out: Json[] = [];
  const by = new Map(b.map((s) => [s.id, s]));
  const head = by.get("head");
  out.push({ ...head, id: "th-face", region: "face-skin", sections: head.sections.filter((q: Json) => q.at[0] <= 0.1), capEnd: false });
  for (const id of ["nose", "lips", "chin", "eye-l", "eye-r", "brow-l", "brow-r", "ear-l", "ear-r", "neck"]) out.push(thOf(id, "face-skin"));
  out.push(thOf("hair", "hair", "hair"));
  for (const s of b.filter((q) => /^hand-[lr]/.test(q.id))) out.push(thOf(s.id, "hand-skin"));
  for (const id of ["sweater", "sweater-hem-rib", "sweater-collar", "sleeve-l", "sleeve-r", "forearm-l", "forearm-r", "cuff-rib-l", "cuff-rib-r"]) out.push(thOf(id, "sweater", "knit"));
  out.push({ ...loft(C("th-tee", "torso", MAT.cotton, 0.8), dense(sweaterStations(-0.013), 2)), region: "sweater", layer: "tee" });
  const teeSleeve = (side: "l" | "r"): Json => {
    const st: St[] = inset(SLEEVE_L.slice(0, 4), 0.014).map((q) => (side === "l" ? q : { ...q, at: [q.at[0], q.at[1], -q.at[2]] }));
    return { ...loft(C(`th-tee-sleeve-${side}`, `arm-${side}`, MAT.cotton, 0.8), st), region: "sweater", layer: "tee" };
  };
  out.push(teeSleeve("l"), teeSleeve("r"));
  out.push(thOf("belly-cover", "belly-sweater", "knit"));
  out.push({ ...loft(C("th-belly-tee", "belly", MAT.cotton, 0.8), dense(bellyStations(-0.0056), 2), [false, true]), region: "belly-sweater", layer: "tee" });
  for (const id of ["leggings-seat", "thigh-l", "thigh-r", "knee-l", "knee-r", "shank-l", "shank-r"]) out.push(thOf(id, "leggings", "leggings"));
  out.push({ ...loft(C("th-briefs", "pelvis", MAT.cotton, 0.8), dense(seatStations(-0.001).slice(1, 6), 2)), region: "leggings", layer: "briefs" });
  for (const s of ["l", "r"]) {
    for (const id of [`boot-${s}`, `boot-vamp-${s}`, `boot-toe-${s}`, `boot-heel-${s}`, `boot-sole-${s}`, `boot-toe-sole-${s}`, `boot-rim-${s}`, `boot-zip-${s}`, `boot-zip-pull-${s}`]) out.push(thOf(id, "ankle-boots", "boot"));
    out.push(shell(by.get(`boot-${s}`), `th-sock-${s}`, "ankle-boots", "socks", 0.004, MAT.sock));
    out.push(shell(by.get(`boot-vamp-${s}`), `th-sock-vamp-${s}`, "ankle-boots", "socks", 0.004, MAT.sock));
    out.push(shell(by.get(`boot-toe-${s}`), `th-sock-toe-${s}`, "ankle-boots", "socks", 0.004, MAT.sock));
  }
  out.push({ ...ellipsoid(C("th-nose-breath", "head", MAT.skin, TONE.skin, "detail"), [0.0039, 0.0039, 0.0085], pa("head", -0.002, 0.105, 0)), region: "face-skin", tags: ["emitter", "nose-breath"] });
  out.push({ ...ellipsoid(C("th-mouth-breath", "head", MAT.skin, TONE.lips, "detail"), [0.0045, 0.004, 0.0115], pa("head", -0.022, 0.104, 0)), region: "face-skin", tags: ["emitter", "mouth-breath"] });
  out.push({ ...ellipsoid(C("th-body-plume", "head", MAT.hair, TONE.hair, "detail"), [0.004, 0.06, 0.05], pa("head", 0.158, 0.0, 0)), region: "sweater", tags: ["emitter", "body-plume"] });
  void sa;
  return out;
}
