/** 🌧️ Builds `person-in-rain-poncho`: an adult of 1.69 m walking through the rain under a hooded PVC rain poncho that hangs from the shoulders to above the knees over a fleece jacket, with wellington boots over the trousers.
 *
 * Derived from the rig, body and gait of `walking-commuter` (read from the catalog, never copied into the ticket), rescaled to the stature of this person; bag, overcoat skirt, scarf and hair are removed, the poncho is a thin revolved shell about the torso, the hood a loft about the head, the boot shafts lofts about the shanks, every scale authored on its own and every new part drawn as the projection of its own solids.
 * bun poncho.ts
 *
 * @see ./lib.ts
 * @see ./garments.ts
 * @see ./poncho-physics.ts
 */
import { clone, dropBones, dropParts, fitDimensions, r5, readDoc, remapMaterials, rest, rigMass, scaleUniform, writeDoc, SCALES, VIEWS, type Json } from "./lib.ts";
import { addGarment, redrawByBone, roundLimbs } from "./garments.ts";
import { ponchoPhysics } from "./poncho-physics.ts";

const SLUG = "person-in-rain-poncho";
export const STATURE = 1.03;
const base = await readDoc("walking-commuter");
const doc = clone(base);
const a = doc.asset;
a.id = SLUG;

dropBones(a, ["bag"]);
const gone = /^(lapel-|coat-button|coat-pocket|coat-front-edge|coat-back-vent|coat-skirt|coat-centre|scarf|hair|head-ear|th-hair|th-scarf)/;
dropParts(a, (s) => gone.test(s.id), (s) => gone.test(s.id));
for (const sc of SCALES) a.volumes[sc].thermal = a.volumes[sc].thermal.filter((s: Json) => !gone.test(String(s.of ?? s.id).replace(/^th-/, "")) && !["scarf", "hair", "bag-leather"].includes(s.region));
a.thermal.regions = a.thermal.regions.filter((r: Json) => !["bag-leather", "scarf", "hair"].includes(r.id));
for (const p of a.poses) for (const r of ["bag-leather", "scarf", "hair"]) delete p.thermal.regions[r];
scaleUniform(a, STATURE);

const S = STATURE;
const f = rest(a);
redrawByBone(a, (s) => s.region === "coat" && s.role === "body", (sc) => a.volumes[sc].body.filter((s: Json) => /^(coat|coat-waist|collar|sleeve-[lr]|forearm-[lr])$/.test(s.id)).map((s: Json) => s.id), 0.02);
const PONCHO: [number, number][] = [[0.3, 0.088], [0.285, 0.13], [0.255, 0.168], [0.215, 0.19], [0.15, 0.205], [0.05, 0.218], [-0.1, 0.23], [-0.3, 0.245], [-0.5, 0.258], [-0.6, 0.264]];
const LATERAL = 1.36;
const THICK = 0.006;
const ponchoSolid = (sc: string): Json => {
  const step = sc === "50" ? 1 : sc === "100" ? 1 : sc === "200" ? 2 : 3;
  const outer = PONCHO.filter((_, i) => i % step === 0 || i === PONCHO.length - 1).map(([u, r]) => [r5(u * S), r5(r * S)]);
  const inner = [...outer].reverse().map(([u, r]) => [r5(u!), r5(r! - THICK)]);
  const loop = [...outer, ...inner, outer[0]!];
  return { id: "poncho", kind: "revolve", bone: "torso", material: "pvc-poncho", role: "body", tone: 0.55, profile: loop, axis: [1, 0, 0], origin: [0, r5(-0.012 * S), 0], lateralScale: LATERAL };
};
addGarment(a, {
  scales: SCALES,
  solids: (sc) => [ponchoSolid(sc)],
  shape: { id: "poncho", bone: "torso", region: "coat", material: "pvc-poncho", tone: 0.55 },
  z: { elevation: 8.6, front: 5.5, plan: 5.1 },
  thermal: () => ({ id: "th-poncho", of: "poncho", region: "coat", layer: "poncho" }),
});

const env = (loft: Json, u: number) => {
  const s = loft.sections;
  for (let i = 0; i + 1 < s.length; i++) {
    const [p, q] = [s[i], s[i + 1]];
    if ((p.at[0] - u) * (q.at[0] - u) <= 0 && p.at[0] !== q.at[0]) {
      const t = (u - p.at[0]) / (q.at[0] - p.at[0]);
      const c = p.at[1] + t * (q.at[1] - p.at[1]);
      const ry = p.ry + t * (q.ry - p.ry);
      return { v0: c - ry, v1: c + ry, w: p.rx + t * (q.rx - p.rx) };
    }
  }
  return null;
};
const HOOD: [number, number, number][] = [[-0.115, -0.015, 0.1], [-0.085, -0.04, 0.115], [-0.04, -0.05, 0.128], [0.01, -0.052, 0.132], [0.055, -0.06, 0.133], [0.092, -0.104, 0.13], [0.125, -0.092, 0.118], [0.15, -0.062, 0.095], [0.168, -0.028, 0.06], [0.176, -0.006, 0.025]];
const hoodSolid = (sc: string): Json => {
  const head = a.volumes["50"].body.find((s: Json) => s.id === "head");
  const pick = sc === "50" ? HOOD : sc === "100" ? HOOD.filter((_, i) => i !== 3 && i !== 7) : sc === "200" ? HOOD.filter((_, i) => [0, 2, 4, 5, 7, 9].includes(i)) : HOOD.filter((_, i) => [0, 2, 5, 7, 9].includes(i));
  const sections = pick.map(([u, front, w]) => {
    const e = env(head, u * S);
    const back = e ? e.v1 + 0.024 * S : 0.11 * S;
    const fr = front * S;
    return { at: [r5(u * S), r5((fr + back) / 2), 0], rx: r5(Math.max(w * S, (e?.w ?? 0) + 0.022 * S)), ry: r5((back - fr) / 2), exponent: 2.3, rotation: head.sections[0].rotation };
  });
  return { id: "hood", kind: "loft", bone: "head", material: "pvc-poncho", role: "body", tone: 0.55, sections, capStart: true, capEnd: true };
};
addGarment(a, {
  scales: SCALES,
  solids: (sc) => [hoodSolid(sc)],
  shape: { id: "hood", bone: "head", region: "hood", material: "pvc-poncho", tone: 0.55 },
  z: { elevation: 8.8, front: 5.9, plan: 5.6 },
  thermal: () => ({ id: "th-hood", of: "hood", region: "hood", layer: "hood" }),
});

for (const side of ["l", "r"] as const) {
  addGarment(a, {
    scales: SCALES,
    solids: (sc) => {
      const shank = a.volumes[sc].body.find((s: Json) => s.id === `shank-${side}`);
      const top = 0.1 * S;
      const first = shank.sections.find((x: Json) => x.at[0] >= top);
      const secs = [{ ...first, at: [top, first.at[1], first.at[2]] }, ...shank.sections.filter((x: Json) => x.at[0] > top)].map((x: Json) => ({ ...x, rx: r5(x.rx + 0.014 * S), ry: r5(x.ry + 0.016 * S) }));
      const end = secs[secs.length - 1];
      secs.push({ ...end, at: [r5(Math.min(end.at[0] + 0.012, 0.44 * S)), end.at[1], end.at[2]] });
      return [{ id: `welly-shaft-${side}`, kind: "loft", bone: `shank-${side}`, material: "rubber-boot", role: "body", tone: 0.62, sections: secs, capStart: true, capEnd: true }];
    },
    shape: { id: `welly-shaft-${side}`, bone: `shank-${side}`, region: "shoes", material: "rubber-boot", tone: 0.62 },
    z: side === "l" ? { elevation: 2.1, front: 2.3, plan: 1.7 } : { elevation: 4.1, front: 2.3, plan: 1.7 },
    thermal: (_sc, s) => ({ id: `th-${s.id}`, of: s.id, region: "shoes", layer: "shoe" }),
  });
}

roundLimbs(a, ["arm-l", "forearm-l", "hand-l", "arm-r", "forearm-r", "hand-r"], VIEWS, (sc) => (sc === "50" ? 16 : sc === "100" ? 12 : sc === "200" ? 9 : 8));
for (const sc of SCALES)
  for (const v of VIEWS) {
    const g = a.representations[sc].geometry[v];
    const poncho = g.find((s: Json) => s.id === "poncho").z;
    for (const s of g) if (/^(arm|forearm|hand)-[lr]$/.test(s.bone) && s.z > poncho) s.z = r5(poncho - 0.3);
    g.sort((p: Json, q: Json) => p.z - q.z);
  }

ponchoPhysics(a, base.asset);
remapMaterials(a, { "wool-coat": "fleece-jacket", "leather-shoe": "rubber-boot", "cotton-sock": "wool-sock", "wool-suit": "fleece-jacket" });
a.dimensions.massKg = rigMass(a);
console.log("[DEBUG] dimensions", fitDimensions(a), a.dimensions.massKg);
writeDoc(SLUG, doc);
console.log(`[DEBUG] wrote ${SLUG}`);
