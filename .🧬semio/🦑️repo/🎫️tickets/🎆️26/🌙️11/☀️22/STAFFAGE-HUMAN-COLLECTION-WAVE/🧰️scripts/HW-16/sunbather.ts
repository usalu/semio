/** ☀️ Builds `sunbather-on-towel`: an adult of 1.69 m in a one-piece swimsuit lying on a 1.80 × 0.90 m beach towel in strong sun, on the back (rest and default), on the front and propped on the elbows, with a breathing loop and a loop of fanning the face.
 *
 * Derived from the rig and body of `standing-adult` (read from the catalog, never copied into the ticket): the clothing lofts are pared down to the body (sweatshirt to the trunk in a swimsuit, jeans to bare legs, sneakers to bare feet), the rig is re-rooted on a towel bone with a prismatic body lift so the rest pose lies on the back, the towel is a new thin solid, and the side elevation is redrawn bone by bone from the solids while the front and plan views are drawn again entirely from the solids of the lying body (limbs as revolved or prismatic solids that stay exact in every pose).
 * bun sunbather.ts
 *
 * @see ./lib.ts
 * @see ./garments.ts
 * @see ./sunbather-physics.ts
 */
import { clone, dropParts, fitDimensions, r5, readDoc, remapMaterials, rest, restBones, rigMass, shapesFrom, writeDoc, SCALES, VIEWS, type Json } from "./lib.ts";
import { handPrism, limbShapes, revolvedShape } from "./garments.ts";
import { regionOf, sunbatherPhysics } from "./sunbather-physics.ts";
import { worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const SLUG = "sunbather-on-towel";
const base = await readDoc("standing-adult");
const doc = clone(base);
const a = doc.asset;
a.id = SLUG;

const gone = /^(hood$|hood-roll|kangaroo-pocket|collar-rib|drawstring|hem-rib|cuff-rib|sock-|lace-|sole-|toe-sole|toe-cap|th-tee|th-briefs|th-sock)/;
dropParts(a, (s) => gone.test(s.id) || /^(skin-(thumb|index|middle|ring|little))/.test(s.id), (s) => gone.test(s.id));
for (const sc of SCALES) a.volumes[sc].thermal = a.volumes[sc].thermal.filter((s: Json) => !gone.test(s.id) && !gone.test(String(s.of ?? "")));

const PARE: Record<string, number> = { hoodie: 0.016, "sleeve-l": 0.014, "sleeve-r": 0.014, "forearm-l": 0.012, "forearm-r": 0.012, "jeans-seat": 0.008, "thigh-l": 0.011, "thigh-r": 0.011, "shank-l": 0.01, "shank-r": 0.01, "shoe-l": 0.012, "shoe-r": 0.012, "shoe-toe-l": 0.008, "shoe-toe-r": 0.008 };
const SKIN = new Set(["sleeve-l", "sleeve-r", "forearm-l", "forearm-r", "thigh-l", "thigh-r", "kneecap-l", "kneecap-r", "shank-l", "shank-r", "shoe-l", "shoe-r", "shoe-toe-l", "shoe-toe-r"]);
for (const sc of SCALES)
  for (const s of a.volumes[sc].body) {
    const d = PARE[s.id] ?? 0;
    const floor = sc === "500" ? 0.031 : sc === "200" ? 0.0155 : 0;
    if (d && s.kind === "loft") for (const sec of s.sections) (sec.rx = r5(Math.max(sec.rx * 0.55, sec.rx - d, Math.min(sec.rx, floor)))), (sec.ry = r5(Math.max(sec.ry * 0.55, sec.ry - d, Math.min(sec.ry, floor))));
    if (/^kneecap-/.test(s.id) && s.radii) s.radii = s.radii.map((r: number) => r5(r * 0.8));
    if (/^buttock-/.test(s.id) && s.radii) s.radii = s.radii.map((r: number) => r5(r * 0.96));
    if (SKIN.has(s.id)) s.material = "skin";
    else if (["hoodie", "jeans-seat", "buttock-l", "buttock-r"].includes(s.id)) s.material = "lycra-swimsuit";
  }

for (const side of ["l", "r"])
  for (const [sc, every, floor, toes] of [["200", 2, 0.0155, false], ["500", 3, 0.036, false]] as const) {
    const fine = a.volumes["50"].body;
    const vol = a.volumes[sc].body;
    const decimate = (id: string) => {
      const src = clone(fine.find((x: Json) => x.id === id));
      src.sections = src.sections.filter((_: Json, i: number) => i % every === 0 || i === src.sections.length - 1).map((sec: Json) => ({ ...sec, rx: r5(Math.max(sec.rx, floor)), ry: r5(Math.max(sec.ry, floor)) }));
      return src;
    };
    const keep = vol.filter((x: Json) => x.id !== `shoe-${side}` && x.id !== `shoe-toe-${side}`);
    a.volumes[sc].body = [...keep, decimate(`shoe-${side}`), ...(toes ? [decimate(`shoe-toe-${side}`)] : [])];
    a.volumes[sc].thermal = a.volumes[sc].thermal.filter((x: Json) => toes || x.of !== `shoe-toe-${side}`);
  }

const Xt = -0.9;
const towel = { id: "towel", parent: null, title: { en: "Beach towel on the sand", de: "Strandtuch auf dem Sand" }, x: Xt, y: 0, angleDeg: 0, lengthM: 1.8, joint: { type: "fixed" }, mass: { kg: 0.65, comX: 0.9, comY: 0.004, distribution: "point", source: "authored-estimate" } };
const lift = { id: "body-lift", parent: "towel", title: { en: "Body travel above the towel", de: "Körperverschiebung über dem Tuch" }, x: 0, y: 0, angleDeg: 0, lengthM: 0.01, joint: { type: "prismatic", axisDeg: 90, minM: -0.05, maxM: 0.3 } };
const pelvis = a.rig.bones.find((b: Json) => b.id === "pelvis");
pelvis.parent = "body-lift";
pelvis.angleDeg = 180;
pelvis.x = 0;
pelvis.y = 0;
pelvis.joint = { type: "revolute", minDeg: -20, maxDeg: 20 };
a.rig.root = "towel";
a.rig.bones.unshift(towel, lift);
const TOWEL_T = 0.01;
const place = (xp: number, yp: number) => ((lift.x = r5(xp - Xt)), (lift.y = r5(yp)));
place(-0.04, 0.12);
const lowest = () => {
  let lo = Infinity;
  for (const w of worldSolids(a, restBones(a) as never, 50, "body")) for (let i = 1; i < w.positions.length; i += 3) lo = Math.min(lo, w.positions[i]!);
  return lo;
};
place(-0.04, 0.12 + TOWEL_T - lowest());
console.log(`[DEBUG] pelvis rest at ${rest(a).pelvis!.o.map((v: number) => v.toFixed(3))}, lowest ${lowest().toFixed(4)}`);

for (const sc of SCALES) {
  a.volumes[sc].body.push({ id: "towel", kind: "box", bone: "towel", material: "cotton-terry", role: "body", tone: 0.85, size: [1.8, TOWEL_T, 0.9], cornerRadius: 0.002, transform: { translation: [0.9, TOWEL_T / 2, 0] }, sheet: true });
  a.volumes[sc].thermal.push({ id: "th-towel", of: "towel", region: "towel" });
}

const TONE: Record<string, number> = { skin: 0.74, hair: 0.25, swimsuit: 0.22, towel: 0.88 };
const fingers = (bone: string) => /^(thumb|index|middle|ring|little)-/.test(bone);
const TRUNK = new Set(["pelvis", "lumbar", "torso"]);
const ROUND = new Set(["arm-l", "forearm-l", "arm-r", "forearm-r"]);
const LEGS = new Set(["thigh-l", "shank-l", "thigh-r", "shank-r"]);
const FEET = new Set(["foot-l", "toe-l", "foot-r", "toe-r"]);
const PIECES: Record<string, number> = { "50": 12, "100": 9, "200": 6, "500": 4 };
const SAMPLES: Record<string, number> = { "50": 16, "100": 12, "200": 9, "500": 8 };

/** 🏖️ The towel drawn as what it is, a flat rectangle of its true size, in every view. */
const towelShape = (view: string, z: number, sc: string): Json => {
  const [x0, x1] = [Xt, Xt + 1.8];
  const points = view === "elevation" ? [[0, 0], [1.8, 0], [1.8, TOWEL_T], [0, TOWEL_T]] : view === "front" ? [[-0.45, 0], [0.45, 0], [0.45, TOWEL_T], [-0.45, TOWEL_T]] : [[x0, -0.45], [x1, -0.45], [x1, 0.45], [x0, 0.45]];
  return { id: "towel", bone: "towel", role: "body", z, filled: true, tone: TONE.towel, region: "towel", material: "cotton-terry", kind: "poly", points, closed: true, ...(sc === "500" ? { penMm: 0.2 } : {}) };
};
const stripes = (sc: string): Json[] => sc === "500" ? [] : [0.12, 0.2, 1.6, 1.68].map((u, i) => ({ id: `towel-stripe-${i + 1}`, bone: "towel", role: "detail", z: 0.2, filled: false, tone: 0.3, region: "towel", material: "cotton-terry", kind: "poly", points: [[Xt + u, -0.43], [Xt + u, 0.43]], closed: false }));

for (const sc of SCALES) {
  const r = a.representations[sc];
  const vol = a.volumes[sc];
  const groups = new Map<string, string[]>();
  for (const s of vol.body) {
    if (s.bone === "towel") continue;
    const bone = fingers(s.bone) ? s.bone.replace(/^(thumb|index|middle|ring|little)-(metacarpal|proximal|middle|distal)-/, "hand-") : s.bone;
    const key = `${bone}|${s.material === "hair" ? "hair" : s.material === "lycra-swimsuit" ? "lycra-swimsuit" : "skin"}`;
    groups.set(key, [...(groups.get(key) ?? []), s.id]);
  }
  const solidsAt = worldSolids(a, restBones(a) as never, Number(sc), "body");
  const extent = (ids: string[]) => {
    const set = new Set(ids);
    let [x1, y1, z0] = [-Infinity, -Infinity, Infinity];
    for (const w of solidsAt) if (set.has(w.id)) for (let i = 0; i < w.positions.length; i += 3) (x1 = Math.max(x1, w.positions[i]!)), (y1 = Math.max(y1, w.positions[i + 1]!)), (z0 = Math.min(z0, w.positions[i + 2]!));
    return { x1, y1, z0 };
  };
  const details = r.geometry.elevation.filter((s: Json) => s.role !== "body" && s.bone === "head" && !s.solid);
  for (const view of VIEWS) {
    const list: Json[] = [towelShape(view, 0.1, sc), ...(view === "plan" ? stripes(sc) : [])];
    for (const [key, ids] of groups) {
      const [bone, material] = key.split("|") as [string, string];
      const region = regionOf(bone, material);
      const e = extent(ids);
      const near = view === "elevation" ? -e.z0 : view === "front" ? e.x1 : e.y1;
      const meta = { id: material === "hair" ? `${bone}-hair` : bone, region, material, tone: TONE[region] ?? 0.74, z: r5(1 + near * 10 + (material === "hair" ? 0.05 : 0)) };
      const tol = sc === "50" ? 0.015 : 0.02;
      if (ROUND.has(bone)) {
        const made = revolvedShape(a, sc, bone, ids, meta, SAMPLES[sc]!, view === "elevation" ? "v" : view === "front" ? "v" : "w");
        if (made) list.push(made);
      } else if (/^hand-[lr]$/.test(bone) && view === "front") {
        list.push(...limbShapes(a, sc, bone, ids, meta, 1, 2.2));
      } else if (/^hand-[lr]$/.test(bone) || (FEET.has(bone) && view !== "elevation")) {
        const made = handPrism(a, sc, bone, ids, meta);
        if (made) list.push(made);
      } else if (TRUNK.has(bone) && view === "elevation") {
        const made = handPrism(a, sc, bone, ids, meta);
        if (made) list.push(made);
      } else if ((TRUNK.has(bone) || bone === "neck" || bone === "head") && view !== "elevation") {
        list.push(...limbShapes(a, sc, bone, ids, meta, bone === "neck" ? 2 : bone === "head" ? Math.max(3, PIECES[sc]! - 4) : PIECES[sc]!, 2.2));
      } else if (LEGS.has(bone) && view !== "elevation") {
        const made = revolvedShape(a, sc, bone, ids, meta, SAMPLES[sc]!, view === "front" ? "v" : "w");
        if (made) list.push(made);
      } else list.push(...shapesFrom(a, sc, view, ids, { ...meta, bone, role: "body" }, tol));
    }
    if (view === "elevation") for (const s of details) list.push({ ...s, z: r5(Math.max(...list.filter((q) => q.bone === s.bone).map((q) => q.z)) + 0.01) });
    r.geometry[view] = list.sort((p, q) => p.z - q.z);
  }
}

sunbatherPhysics(a, base.asset);
remapMaterials(a, { "fleece-sweatshirt": "lycra-swimsuit", denim: "skin", "canvas-sneaker": "skin", "cotton-sock": "skin" });
a.dimensions.massKg = rigMass(a);
console.log("[DEBUG] dimensions", fitDimensions(a), a.dimensions.massKg);
writeDoc(SLUG, doc);
console.log(`[DEBUG] wrote ${SLUG}`);
