/** 🌡️ The 1:50 thermal model of `humans/person-cooking` in space: every region of the asset on the new body, the layered regions as nested shells with their declared thicknesses (T-shirt under the fleece on trunk and upper arms, briefs under the jeans at the seat, socks inside the sneakers), the face as the head ring below the hair line, the unit, hob, pot with its water and spoon as their own surfaces, and the four outlets: nostrils, mouth, the crown of the head and the water surface in the pot.
 * @see ./r3-person-cooking-s50.ts
 * @see ./design.md (B2, C18, C20, C29, D2) */
import type { Json } from "./volumes-lib.ts";
import { box, dense, ellipsoid, inset, loft, pa, sa, thOf, type St } from "./r3-person-cooking-kit.ts";
import { C, MAT, SLEEVE_L, TONE, hoodieStations } from "./r3-person-cooking-s50.ts";
import { SEAT, pelvisRow, type Slice } from "./r3-person-cooking-forms.ts";
import { POT, UNIT } from "./r3-person-cooking-data.ts";

/** 🧅 A loft of the body list moved inwards by `by` (an inner layer shell), as a thermal solid of `region` and `layer`. */
export const shell = (src: Json, id: string, region: string, layer: string, by: number, material: string): Json => ({ ...src, id, material, tone: 0.8, role: "body", region, layer, sections: src.sections.map((q: Json) => ({ ...q, rx: Math.max(0.002, q.rx - by), ry: Math.max(0.002, q.ry - by) })) });
/** 🏷️ The region of a body solid of the unit, the pot or the spoon. */
export function propRegion(id: string): string | null {
  if (/^(hob-|zone-)/.test(id)) return "hob";
  if (/^(carcass-|drawer-front-|handle-|plinth|worktop)/.test(id)) return "counter";
  if (/^pot-/.test(id)) return "pot";
  if (/^spoon-/.test(id)) return "spoon";
  return null;
}

/** 🗄️ Body solids of the unit that bound its closed cavity (C31): the thermal surface of the unit is the closed envelope below instead, the hob tray hangs inside it. */
export const ENCLOSED = /^(carcass-|worktop$|hob-tray$)/;
/** 📦 The closed envelope of the carcass and the worktop without its cut-out: the faces the room air reaches. */
export function unitEnvelope(): Json[] {
  const top = UNIT.plinth + UNIT.carcass;
  const x0 = UNIT.front + UNIT.door + 0.001;
  return [
    { ...box(C("th-carcass", "counter", MAT.board, TONE.carcass), [UNIT.depth - x0, UNIT.carcass, UNIT.width], [(x0 + UNIT.depth) / 2, UNIT.plinth + UNIT.carcass / 2, 0]), region: "counter" },
    { ...box(C("th-worktop", "counter", MAT.board, TONE.worktop), [UNIT.depth, UNIT.worktop, UNIT.width], [UNIT.depth / 2, top + UNIT.worktop / 2, 0]), region: "counter" },
  ];
}

/** 👖 The jeans region at the seat ends at the hem of the sweatshirt (0.866 m): above it the trunk region with its own layers covers the body, so the thermal surface of the jeans is the seat below the hem, open at the top. */
export const seatBelowHem = (grow = 0): St[] => ([...SEAT.slice(0, 3), [0.866, -0.1305, 0.083, 0.1755, 2.37]] as Slice[]).map((s) => { const [a, f, hw, hd, e] = pelvisRow(s); return sa("pelvis", a, f, 0, hw + grow, hd + grow, e); });

export function thermal50(b: Json[]): Json[] {
  const out: Json[] = [];
  const by = new Map(b.map((s) => [s.id, s]));
  const head = by.get("head");
  out.push({ ...head, id: "th-face", region: "face-skin", sections: head.sections.filter((q: Json) => q.at[0] <= 0.1), capEnd: false });
  for (const id of ["nose", "lips", "chin", "eye-l", "eye-r", "brow-l", "brow-r", "ear-l", "ear-r", "neck"]) out.push(thOf(id, "face-skin"));
  out.push(thOf("hair", "hair", "hair"));
  for (const s of b.filter((q) => /^hand-[lr]/.test(q.id))) out.push(thOf(s.id, "hand-skin"));
  for (const s of b.filter((q) => q.material === MAT.fleece)) out.push(thOf(s.id, "hoodie", "sweatshirt"));
  out.push({ ...loft(C("th-tee", "torso", MAT.cotton, 0.8), dense(hoodieStations(-0.0048), 2)), region: "hoodie", layer: "tee" });
  const teeSleeve = (side: "l" | "r"): Json => {
    const stn: St[] = inset(SLEEVE_L.slice(0, 5), 0.0048).map((q) => (side === "l" ? q : { ...q, at: [q.at[0], q.at[1], -q.at[2]] }));
    return { ...loft(C(`th-tee-sleeve-${side}`, `arm-${side}`, MAT.cotton, 0.8), stn), region: "hoodie", layer: "tee" };
  };
  out.push(teeSleeve("l"), teeSleeve("r"));
  for (const s of b.filter((q) => q.material === MAT.denim && q.id !== "hips")) out.push(thOf(s.id, "jeans", "jeans"));
  out.push({ ...loft(C("th-hips", "pelvis", MAT.denim, TONE.jeans), dense(seatBelowHem(), 2), [true, true], { sectionFaces: ["+u"] }), region: "jeans", layer: "jeans" });
  out.push({ ...loft(C("th-briefs", "pelvis", MAT.cotton, 0.8), dense(seatBelowHem(-0.0012).slice(1), 2)), region: "jeans", layer: "briefs" });
  for (const s of b.filter((q) => q.material === MAT.sneaker)) out.push(thOf(s.id, "sneakers", "sneaker"));
  for (const s of ["l", "r"]) {
    out.push(shell(by.get(`shoe-${s}`), `th-sock-${s}`, "sneakers", "socks", 0.004, MAT.sock));
    out.push(shell(by.get(`toe-cap-${s}`), `th-sock-toe-${s}`, "sneakers", "socks", 0.004, MAT.sock));
  }
  for (const s of b) { const r = propRegion(s.id); if (r && !ENCLOSED.test(s.id)) out.push(thOf(s.id, r)); }
  out.push(...unitEnvelope());
  out.push({ ...ellipsoid(C("th-nose-breath", "head", MAT.skin, TONE.skin, "detail"), [0.0039, 0.0039, 0.0085], pa("head", -0.002, 0.105, 0)), region: "face-skin", tags: ["emitter", "nose-breath"] });
  out.push({ ...ellipsoid(C("th-mouth-breath", "head", MAT.skin, TONE.lips, "detail"), [0.0045, 0.004, 0.0115], pa("head", -0.022, 0.104, 0)), region: "face-skin", tags: ["emitter", "mouth-breath"] });
  out.push({ ...ellipsoid(C("th-body-plume", "head", MAT.hair, TONE.hair, "detail"), [0.004, 0.06, 0.05], pa("head", 0.155, 0.0, 0)), region: "hoodie", tags: ["emitter", "body-plume"] });
  out.push({ ...ellipsoid(C("th-steam", "pot", MAT.water, TONE.water, "detail"), [0.1, 0.004, 0.1], [0, POT.water, 0]), region: "pot", tags: ["emitter", "steam"] });
  return out;
}
