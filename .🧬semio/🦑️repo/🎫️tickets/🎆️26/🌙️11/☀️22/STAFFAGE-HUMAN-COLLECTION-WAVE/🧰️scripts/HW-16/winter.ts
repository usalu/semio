/** ❄️ Builds `person-in-winter-coat`: an adult man of 1.78 m in a knee-length down parka, chunky scarf, knitted beanie, knitted gloves and lined winter boots, standing and walking with the hands in the parka pockets at −5 °C so that the breath condenses.
 *
 * Derived from the rig, body, gait and garment cut of `walking-commuter` (read from the catalog, never copied into the ticket): the bag is removed, the overcoat lofts are filled out by the down layer, the new beanie and boot shafts are lofted per scale, and every changed garment is drawn again as the projection of its own solids at every scale.
 * bun winter.ts
 *
 * @see ./lib.ts
 * @see ./garments.ts
 * @see ./winter-physics.ts
 */
import { clone, dropBones, dropParts, fitDimensions, r5, readDoc, remapMaterials, rigMass, writeDoc, SCALES, VIEWS, type Json } from "./lib.ts";
import { addGarment, envelopeSections, inflate, redraw, redrawByBone, roundLimbs } from "./garments.ts";
import { winterPhysics } from "./winter-physics.ts";

const SLUG = "person-in-winter-coat";
const base = await readDoc("walking-commuter");
const doc = clone(base);
const a = doc.asset;
a.id = SLUG;

dropBones(a, ["bag"]);
a.thermal.regions = a.thermal.regions.filter((r: Json) => r.id !== "bag-leather");
for (const p of a.poses) delete p.thermal.regions["bag-leather"], (p.loads = []);

const gone = /^(lapel-|coat-button|coat-pocket|coat-front-edge|coat-back-vent)/;
dropParts(a, (s) => gone.test(s.id), (s) => gone.test(s.id));
for (const sc of SCALES) a.volumes[sc].thermal = a.volumes[sc].thermal.filter((s: Json) => !gone.test(String(s.of ?? s.id).replace(/^th-/, "")));

const DOWN = 0.02;
const SLEEVE = 0.014;
const SCARF = 0.012;
const parka = (id: string) => /^(coat|coat-waist|coat-skirt-[lr]|coat-centre|collar)$/.test(id);
const sleeves = (id: string) => /^(sleeve|forearm)-[lr]$/.test(id);
const scarf = (id: string) => /^(scarf|scarf-front)$/.test(id);
inflate(a, (id) => (parka(id) ? DOWN : sleeves(id) ? SLEEVE : scarf(id) ? SCARF : 0));
for (const sc of SCALES)
  for (const s of a.volumes[sc].thermal) if (!("of" in s) && s.region === "coat" && /-(jacket|shirt|tee)$/.test(s.id)) for (const sec of s.sections ?? []) (sec.rx = r5(sec.rx + DOWN * 0.4)), (sec.ry = r5(sec.ry + DOWN * 0.4));

for (const sc of SCALES) for (const s of a.volumes[sc].body) if (s.material === "wool-coat") s.material = "down-parka";
redrawByBone(a, (s) => s.region === "coat" && s.role === "body", (sc) => a.volumes[sc].body.filter((s: Json) => parka(s.id) || sleeves(s.id)).map((s: Json) => s.id), 0.015, (bone) => (bone.endsWith("-l") ? -0.05 : bone.endsWith("-r") ? 0.05 : 0));
redraw(a, (s) => s.region === "scarf", (sc) => a.volumes[sc].body.filter((s: Json) => scarf(s.id)).map((s: Json) => s.id));

const loftOf = (sc: string, id: string) => a.volumes[sc].body.find((s: Json) => s.id === id);
const BEANIE_Z = { elevation: 6.4, front: 6.4, plan: 6.4 };
addGarment(a, {
  scales: SCALES,
  solids: (sc) => {
    const n = sc === "50" ? 14 : sc === "100" ? 9 : sc === "200" ? 6 : 5;
    const crown = envelopeSections(loftOf(sc, "head"), loftOf(sc, "hair"), 0.052, 0.148, n, 0.009);
    const top = crown[crown.length - 1];
    const cap = [...crown, { ...top, at: [r5(top.at[0] + 0.012), top.at[1], top.at[2]], rx: r5(top.rx * 0.45), ry: r5(top.ry * 0.45) }];
    const solids: Json[] = [{ id: "beanie", kind: "loft", bone: "head", material: "knit-wool", role: "body", tone: 0.42, sections: cap, capStart: true, capEnd: true }];
    if (sc === "50" || sc === "100") {
      const cuff = envelopeSections(loftOf(sc, "head"), loftOf(sc, "hair"), 0.05, 0.088, sc === "50" ? 5 : 3, 0.015);
      solids.push({ id: "beanie-cuff", kind: "loft", bone: "head", material: "knit-wool", role: "detail", tone: 0.36, sections: cuff, capStart: true, capEnd: true });
    }
    return solids;
  },
  shape: { id: "beanie", bone: "head", region: "beanie", material: "knit-wool", tone: 0.42 },
  z: BEANIE_Z,
  thermal: (_sc, s) => ({ id: `th-${s.id}`, of: s.id, region: "beanie", layer: "beanie" }),
});

for (const side of ["l", "r"] as const) {
  addGarment(a, {
    scales: SCALES,
    solids: (sc) => {
      const shank = loftOf(sc, `shank-${side}`);
      const secs = shank.sections.filter((x: Json) => x.at[0] >= 0.24);
      const first = shank.sections.find((x: Json) => x.at[0] >= 0.24);
      const s0 = { ...first, at: [0.24, first.at[1], first.at[2]] };
      const body = [s0, ...secs.filter((x: Json) => x.at[0] > 0.24)].map((x: Json) => ({ ...x, rx: r5(x.rx + 0.013), ry: r5(x.ry + 0.013) }));
      const end = body[body.length - 1];
      body.push({ ...end, at: [r5(Math.min(end.at[0] + 0.012, 0.44)), end.at[1], end.at[2]] });
      return [{ id: `boot-shaft-${side}`, kind: "loft", bone: `shank-${side}`, material: "nubuck-boot", role: "body", tone: 0.18, sections: body, capStart: true, capEnd: true }];
    },
    shape: { id: `boot-shaft-${side}`, bone: `shank-${side}`, region: "shoes", material: "nubuck-boot", tone: 0.18 },
    z: side === "l" ? { elevation: 2.1, front: 2.3, plan: 1.7 } : { elevation: 4.1, front: 2.3, plan: 1.7 },
    thermal: (_sc, s) => ({ id: `th-${s.id}`, of: s.id, region: "shoes", layer: "shoe" }),
  });
}

for (const sc of SCALES) {
  for (const s of a.volumes[sc].body) {
    if (s.material === "leather-shoe") s.material = "nubuck-boot";
    if (s.material === "skin" && /^(hand-|wrist-|skin-)/.test(s.id)) s.material = "knit-glove";
  }
  for (const v of VIEWS)
    for (const s of a.representations[sc].geometry[v]) {
      if (s.material === "wool-coat") s.material = "down-parka";
      if (s.material === "leather-shoe") s.material = "nubuck-boot";
      if (s.region === "hand-skin") s.material = "knit-glove";
    }
}

roundLimbs(a, ["arm-l", "forearm-l", "hand-l", "arm-r", "forearm-r", "hand-r"], VIEWS, (sc) => (sc === "50" ? 16 : sc === "100" ? 12 : sc === "200" ? 9 : 8));
for (const sc of SCALES) for (const v of ["front", "plan"] as const) {
  const g = a.representations[sc].geometry[v];
  const low = Math.min(...g.filter((s: Json) => s.region === "coat").map((s: Json) => s.z));
  for (const s of g) if (/^hand-[lr]$/.test(s.bone)) s.z = Math.round((low - 0.1) * 100) / 100;
  g.sort((p: Json, q: Json) => p.z - q.z);
}
winterPhysics(a, base.asset);
remapMaterials(a, { "wool-coat": "down-parka", "leather-shoe": "nubuck-boot", "cotton-sock": "wool-sock", "wool-suit": "wool-sweater" });
a.dimensions.massKg = rigMass(a);
console.log("[DEBUG] dimensions", fitDimensions(a), a.dimensions.massKg);
writeDoc(SLUG, doc);
console.log(`[DEBUG] wrote ${SLUG}`);
