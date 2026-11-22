/** 🏗️ PW-05 assembler: builds the complete asset document of a species design (see `tree-kit.ts`) and writes it to the worker's generated folder, the catalog (`--ship`) and the manifest row (`--ship`).
 * Usage: `repo tools exec bun -- assemble.ts <species> [--ship]` from this folder; species files are `<id>.ts` next to it.
 * @see ./tree-kit.ts
 * @see ../../📋️brief.md */
import { join } from "node:path";
import { FEATURE, SCALES, D2R, add, barkStrokes, bodySolid, boneTube, boxOf, clusterCentre, clusterLobes, designBones, dirOf, drawView, frames, massLobe, mul, outlineOf, pointTube, r1, r3, r4, regionOf, rigOf, ringsArea, structureList, sub, toLocal, tubeArea, tubeVolume, writeDoc, type Ctx, type DesignBone, type Json, type Part, type Scale, type Species, type Text, type V3, type View } from "./tree-kit.ts";


const T = join(import.meta.dir, "../..");
const S = join(T, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const RHO = 1.25;
const G = 9.80665;
const VIEWS: View[] = ["elevation", "front", "plan"];
const RMIN: Record<Scale, number> = { 50: 0.0065, 100: 0.0095, 200: 0.019, 500: 0.045 };
const deN = (x: number, d = 1): string => x.toFixed(d).replace(".", ",");
const enN = (x: number, d = 1): string => x.toFixed(d);
const cardinalEn = ["east", "north-east", "north", "north-west", "west", "south-west", "south", "south-east"];

/** 🪵️ The wood and foliage parts of one scale, in world rest metres. */
function partsOf(sp: Species, bones: DesignBone[], scale: Scale): Part[] {
  const out: Part[] = [];
  const W = sp.wood;
  const rr = (r: number): number => Math.max(r, RMIN[scale]);
  const root = bones[0]!;
  const flare = [[0, sp.root.flareR], [0.3, (sp.root.flareR + sp.root.topR) / 2 * 0.92], [1, sp.root.topR]];
  const st = scale === 500 ? [flare[0]!, flare[2]!] : flare;
  out.push({ kind: "tube", id: "root-plate", bone: "root-plate", material: W.bark, tone: 0.3, group: "root", points: st.map(([f, r]) => ({ p: [0, f! * root.len, 0] as V3, r: r4(r!) })) });
  if (scale <= 100) {
    const roots = scale === 50 ? sp.root.roots : sp.root.roots.filter((_, i) => i % 2 === 0);
    roots.forEach((q, i) => {
      const d: V3 = [Math.cos(q.az * D2R), 0, Math.sin(q.az * D2R)];
      const r0 = q.r * (scale === 50 ? 1 : 1.15);
      out.push(pointTube(`root-${i + 1}`, "root-plate", W.bark, 0.3, "root", add(mul(d, sp.root.topR * 0.6), [0, r0 + 0.18, 0]), add(mul(d, sp.root.topR * 0.6 + q.len), [0, rr(q.r * 0.35), 0]), r0, rr(q.r * 0.35), 2));
    });
  }
  const stations: Record<Scale, number> = { 50: 3, 100: 2, 200: 2, 500: 1 };
  for (const b of bones) {
    if (b.kind === "stem") out.push(boneTube(b.id, b, W.bark, 0.3, "stem", -0.04, 1.04 * b.len, rr(b.r0), rr(b.r1), stations[scale]));
    if (b.kind === "top" && scale <= 100) out.push(boneTube(b.id, b, W.barkYoung, 0.34, "shoot", -0.03, 0.7 * b.len, rr(b.r0), rr(b.r1), 1));
    if (b.kind === "limb" && !(scale === 500 && b.seg! >= 2)) out.push(boneTube(b.id, b, W.barkYoung, 0.34, "limb", b.seg === 0 ? 0 : -Math.min(0.5 * b.r0, 0.15), Math.max(1.03 * b.len, b.len + 0.3 * b.r1), rr(b.r0), rr(b.r1), scale === 50 ? 3 : 2));
    if (b.kind === "fol" && scale <= 100) out.push(boneTube(b.id, b, W.barkYoung, 0.34, "shoot", -Math.min(0.5 * b.r0, 0.1), 0.85 * b.len, rr(b.r0), rr(b.r1), 1));
  }
  if (scale <= 100) for (const b of bones.filter((x) => x.kind === "limb")) {
    (b.limb!.side ?? []).filter((s) => s.seg === b.seg && (scale === 50 || s.r >= 0.02)).forEach((s, i) => {
      const start = add(b.o, mul(b.u, s.at * b.len));
      const d = dirOf(b.az + s.dAz, s.el);
      out.push(pointTube(`side-${b.id}-${i + 1}`, b.id, W.barkYoung, 0.34, "side", add(start, mul(d, -0.5 * b.r0)), add(start, mul(d, s.len)), rr(s.r), rr(s.r * 0.45), 2));
    });
  }
  if (scale === 500 && sp.crown500.length > 0) {
    for (const m of sp.crown500) out.push(massLobe(m.id, m.bone, m.group === "sun" ? W.leafSun : W.leafShade, m.group === "sun" ? 0.6 : 0.44, m.group, m.c, m.radii, m.az));
  } else {
    for (const b of bones.filter((x) => x.kind === "fol")) out.push(...clusterLobes(sp, b, scale));
    const topB = bones.find((x) => x.kind === "top")!;
    const tc = add(topB.o, mul(topB.u, 0.55 * topB.len));
    const [A, B, C] = sp.top.size;
    const g = sp.coarse[scale];
    sp.templates[scale].forEach((t, i) => out.push(massLobe(`fol-top-l${i + 1}`, "fol-top", W.leafSun, 0.6, "sun", add(tc, [t[0] * A, t[1] * B, t[2] * C]), [r4(t[3] * A * g), r4(t[4] * B * g), r4(t[5] * C * g)], 90)));
  }
  for (const m of sp.shade[scale]) out.push(massLobe(m.id, m.bone, W.leafShade, 0.44, "shade", m.c, m.radii, m.az));
  if (sp.racemes) for (const b of bones.filter((x) => x.kind === "raceme")) {
    const L = sp.racemes.len, R = sp.racemes.r;
    if (scale <= 100) {
      const n = scale === 50 ? 5 : 3;
      for (let k = 0; k < n; k++) {
        const a = (k * 360) / n + 20;
        const off: V3 = [0.09 * Math.cos(a * D2R), 0, 0.09 * Math.sin(a * D2R)];
        const p0 = add(b.o, off);
        out.push(pointTube(`${b.id}-r${k + 1}`, b.id, W.flower!, 0.85, "raceme", add(p0, [0, -0.02, 0]), add(p0, [0, L * (0.85 + 0.15 * (k % 2)), 0]), rr(R * (scale === 50 ? 1 : 1.3)), rr(R * 0.45), 2));
      }
    } else out.push(massLobe(`${b.id}-bunch`, b.id, W.flower!, 0.85, "raceme", add(b.o, [0, 0.55 * L, 0]), [0.16, 0.55 * L + 0.02, 0.16].map(r4) as V3, 0));
  }
  return out;
}

/** 🧪️ Builds the asset. */
export function build(sp: Species): { doc: Json; row: Json; report: string[]; parts: Record<Scale, Part[]> } {
  const report: string[] = [];
  const bones = designBones(sp);
  const byId = new Map(bones.map((b) => [b.id, b]));
  if (sp.racemes) for (const limb of sp.limbs.filter((l) => l.raceme)) {
    const fol = byId.get(`fol-${limb.id}`)!;
    let host = fol;
    while (host.kind === "limb" || host.kind === "fol") host = byId.get(host.parent!)!;
    const c = clusterCentre(fol);
    const B = limb.size[1];
    const rb: DesignBone = { id: `raceme-${limb.id}`, parent: host.id, title: { en: `${limb.title.en}, flower racemes and pods`, de: `${limb.title.de}, Blütentrauben und Hülsen` }, o: add(c, [0, -0.92 * B, 0]), u: [0, 1, 0], w: [0, 0, 1], len: sp.racemes.len, r0: sp.racemes.r, r1: sp.racemes.r * 0.5, kind: "raceme", limit: [-2, 180], az: 0 };
    bones.push(rb);
    byId.set(rb.id, rb);
  }
  const parts = Object.fromEntries(SCALES.map((s) => [s, partsOf(sp, bones, s)])) as Record<Scale, Part[]>;
  const fine = parts[50];
  const wood = (p: Part): boolean => p.kind === "tube" && p.group !== "raceme";
  const woodKg: Record<string, number> = {}, woodMom: Record<string, number> = {};
  for (const p of fine.filter(wood)) {
    const b = byId.get(p.bone)!;
    const v = tubeVolume(p as Json);
    const mid = mul(add((p as Json).points[0].p, (p as Json).points.at(-1).p), 0.5);
    woodKg[p.bone] = (woodKg[p.bone] ?? 0) + v * sp.wood.density;
    woodMom[p.bone] = (woodMom[p.bone] ?? 0) + v * sp.wood.density * Math.max(0, Math.min(b.len, (mid[0] - b.o[0]) * b.u[0] + (mid[1] - b.o[1]) * b.u[1] + (mid[2] - b.o[2]) * b.u[2]));
  }
  const folBones = bones.filter((b) => b.kind === "fol" || b.kind === "top");
  const vol = (ps: Part[]): number => ps.reduce((s, p) => s + (p.kind === "lobe" ? (4 / 3) * Math.PI * p.radii[0] * p.radii[1] * p.radii[2] : 0), 0);
  const clusterVol = Object.fromEntries(folBones.map((b) => [b.id, vol(fine.filter((p) => p.bone === b.id && p.group === "sun"))]));
  const totalVol = Object.values(clusterVol).reduce((a, b) => a + b, 0);
  const crownPlan = ringsArea(outlineOf("plan", fine.filter((p) => p.group === "sun" || p.group === "shade"), 50));
  const leafTotal = sp.leaf.lai * crownPlan;
  const sunArea = leafTotal * sp.leaf.sunShare, shadeArea = leafTotal - sunArea;
  const shadeParts = fine.filter((p) => p.group === "shade");
  const shadeVol = vol(shadeParts);
  const leafAreas: Record<string, number> = {};
  for (const b of folBones) leafAreas[b.id] = (sunArea * clusterVol[b.id]!) / totalVol;
  const shadeByBone: Record<string, number> = {};
  for (const p of shadeParts) shadeByBone[p.bone] = (shadeByBone[p.bone] ?? 0) + (shadeArea * vol([p])) / shadeVol;
  const massOf = (b: DesignBone): { kg: number; comX: number } => {
    let kg = woodKg[b.id] ?? 0, mom = woodMom[b.id] ?? 0;
    if (b.kind === "fol" || b.kind === "top") {
      const c = add(b.o, mul(b.u, 0.55 * b.len));
      const leaf = leafAreas[b.id]! * sp.leaf.kgPerM2;
      kg += leaf;
      mom += leaf * 0.55 * b.len;
    }
    if (shadeByBone[b.id]) {
      const leaf = shadeByBone[b.id]! * sp.leaf.kgPerM2;
      kg += leaf;
      mom += leaf * 0.6 * b.len;
    }
    if (b.kind === "raceme") return { kg: sp.racemes!.kgPerCluster, comX: 0.5 * b.len };
    return { kg, comX: kg > 0 ? mom / kg : 0.5 * b.len };
  };
  const rig = rigOf(sp, bones, massOf);
  const massKg = r1(rig.bones.reduce((s: number, b: Json) => s + b.mass.kg, 0));
  const skeleton = { rig };
  const F = frames(skeleton);
  for (const b of bones) {
    const f = F[b.id]!;
    const tip = add(f.o, mul(f.u, b.len));
    const want = add(b.o, mul(b.u, b.len));
    const err = Math.hypot(...(sub(tip, want) as [number, number, number]));
    if (err > 2e-3) throw new Error(`bone ${b.id}: solved tip off by ${err.toFixed(4)} m`);
  }
  const box = boxOf(fine);
  const dims = { heightM: r3(box.max[1]), widthM: r3(box.max[0] - box.min[0]), depthM: r3(box.max[2] - box.min[2]), massKg };
  report.push(`dimensions ${JSON.stringify(dims)} box ${JSON.stringify(box)}`);
  for (const s of SCALES) {
    const b = boxOf(parts[s]);
    report.push(`1:${s} box x ${r3(b.min[0])}..${r3(b.max[0])} y ${r3(b.min[1])}..${r3(b.max[1])} z ${r3(b.min[2])}..${r3(b.max[2])}, parts ${parts[s].length}`);
  }
  const frontal: Record<string, number> = {}, planArea: Record<string, number> = {}, centroid: Record<string, V3> = {};
  for (const b of folBones) {
    const ps = fine.filter((p) => p.bone === b.id && p.group === "sun");
    frontal[b.id] = ringsArea(outlineOf("front", ps, 50));
    planArea[b.id] = ringsArea(outlineOf("plan", ps, 50));
    centroid[b.id] = add(b.o, mul(b.u, 0.55 * b.len));
  }
  const asset: Json = { id: sp.id, category: "vegetation", title: sp.title, summary: sp.summary, tags: sp.tags };
  const ctx: Ctx = { sp, bones, byId, F, asset, parts, leafAreas, frontal, planArea, centroid };
  asset.provenance = { author: "Semio staffage plant worker PW-05", method: "handcrafted", created: "2026-11-22", references: sp.references, notes: sp.notes };
  asset.dimensions = dims;
  asset.materials = sp.materials;
  asset.rig = rig;
  asset.anchors = anchorsOf(ctx);
  asset.thermal = thermalOf(ctx, sunArea, shadeArea, crownPlan);
  asset.structure = structureModel(sp);
  asset.poses = posesOf(ctx);
  asset.clips = clipsOf(ctx);
  asset.presentation = { defaultPose: "calm", defaultClip: "wind-sway", defaultScale: 200 };
  asset.representations = Object.fromEntries(SCALES.map((s) => [String(s), representationOf(ctx, s)]));
  asset.symbols = symbolsOf(ctx);
  asset.volumes = Object.fromEntries(SCALES.map((s) => [String(s), volumeOf(ctx, s)]));
  asset.exports = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"];
  asset.completion = "authored";
  const ordered: Json = {};
  for (const k of ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "volumes", "presentation", "exports", "completion"]) ordered[k] = asset[k];
  const doc = { $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset: ordered };
  const row = { id: sp.id, category: "vegetation", title: sp.title, heightM: dims.heightM, widthM: dims.widthM, powerW: 0, poses: asset.poses.map((p: Json) => p.id), clips: asset.clips.map((c: Json) => c.id), representations: { "50": "authored", "100": "authored", "200": "authored", "500": "authored" }, symbols: asset.symbols.map((s: Json) => s.id), analytics: ["thermal", "transport", "structural"], exports: asset.exports, distinction: sp.distinction, completion: "authored" };
  report.push(`mass ${massKg} kg, leaf area sun ${r1(sunArea)} shade ${r1(shadeArea)} m2, crown plan ${r1(crownPlan)} m2`);
  return { doc, row, report, parts };
}

/** 📍️ Base support and the emitter anchors: seven sun anchors on the tops of foliage clusters spread round the crown and five shade anchors on the shade cores, 4 cm above the 1:50 surface. */
function anchorsOf(ctx: Ctx): Json[] {
  const { sp, bones, F, parts } = ctx;
  const out: Json[] = [{ id: "base-support", bone: "root-plate", x: 0, y: 0, kind: "support", title: { en: "Root plate anchorage", de: "Verankerung des Wurzeltellers" }, support: "fixed", normalDeg: 90, footprintLengthM: r3(2 * sp.root.flareR), footprintWidthM: r3(2 * sp.root.flareR), frictionCoefficient: 0.6 }];
  const fols = bones.filter((b) => b.kind === "fol").sort((a, b) => a.az - b.az || a.o[1] - b.o[1]);
  const pick = Array.from({ length: 7 }, (_, i) => fols[Math.floor(((i + 0.5) * fols.length) / 7)]!);
  pick.forEach((b, i) => {
    const main = parts[50].find((p) => p.id === `${b.id}-l1`) as Json;
    const top: V3 = [main.c[0], main.c[1] + main.radii[1] + 0.04, main.c[2]];
    const l = toLocal(F[b.id]!, top);
    out.push({ id: `tp-sun-${i + 1}`, bone: b.id, x: r4(l[0]), y: r4(l[1]), z: r4(l[2]), kind: "emitter", title: { en: `Transpiration of the sun leaves, anchor ${i + 1}`, de: `Transpiration der Sonnenblätter, Anker ${i + 1}` } });
  });
  const shades = sp.shade[50];
  const sp5 = Array.from({ length: 5 }, (_, i) => shades[Math.floor(((i + 0.5) * shades.length) / 5)]!);
  sp5.forEach((m, i) => {
    const top: V3 = [m.c[0], m.c[1] + m.radii[1] + 0.04, m.c[2]];
    const l = toLocal(F[m.bone]!, top);
    out.push({ id: `tp-shade-${i + 1}`, bone: m.bone, x: r4(l[0]), y: r4(l[1]), z: r4(l[2]), kind: "emitter", title: { en: `Transpiration of the shade leaves, anchor ${i + 1}`, de: `Transpiration der Schattenblätter, Anker ${i + 1}` } });
  });
  return out;
}

const worldOfAnchor = (ctx: Ctx, a: Json): V3 => {
  const f = ctx.F[a.bone]!;
  return add(add(add(f.o, mul(f.u, a.x)), mul(f.v, a.y)), mul(f.w, a.z ?? 0));
};

/** 🌡️ Thermal model: bark of stem and limbs passive, sun and shade leaves (exchange areas from the leaf-area index), racemes where the species has them; transpiration emitters over the anchors. */
function thermalOf(ctx: Ctx, sunArea: number, shadeArea: number, crownPlan: number): Json {
  const { sp, parts, F } = ctx;
  const area = (region: string): number => parts[50].filter((p) => p.kind === "tube" && regionOf(p.group) === region).reduce((s, p) => s + tubeArea(p as Json), 0);
  const trunkD = 2 * sp.stem[0]!.r0;
  const regions: Json[] = [
    { id: "trunk-bark", title: { en: "Bark of root flare and stem", de: "Rinde von Wurzelanlauf und Stamm" }, kind: "bark", model: "passive", material: sp.wood.bark, areaM2: r1(area("trunk-bark") * 0.93), characteristicLengthM: r3(trunkD), geometry: "cylinder", skyViewFactor: 0.35, layers: [], heatCapacityJM2K: 80000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "limb-bark", title: { en: "Bark of limbs, branches and end shoots", de: "Rinde von Ästen, Zweigen und Endtrieben" }, kind: "bark", model: "passive", material: sp.wood.barkYoung, areaM2: r1(area("limb-bark") * 0.95), characteristicLengthM: 0.15, geometry: "cylinder", skyViewFactor: 0.45, layers: [], heatCapacityJM2K: 45000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "leaf-sun", title: { en: "Sun leaves of the outer crown", de: "Sonnenblätter der Außenkrone" }, kind: "foliage", model: "leaf", material: sp.wood.leafSun, areaM2: r1(sunArea), areaBasis: "exchange", characteristicLengthM: sp.leaf.size, geometry: "vertical-plate", skyViewFactor: 0.7, layers: [], heatCapacityJM2K: 900, leaf: { stomatalConductanceMaxMS: sp.leaf.sunGs, leafAreaIndex: r3(sunArea / crownPlan), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
    { id: "leaf-shade", title: { en: "Shade leaves of the inner crown", de: "Schattenblätter der Innenkrone" }, kind: "foliage", model: "leaf", material: sp.wood.leafShade, areaM2: r1(shadeArea), areaBasis: "exchange", characteristicLengthM: sp.leaf.size, geometry: "vertical-plate", skyViewFactor: 0.3, layers: [], heatCapacityJM2K: 700, leaf: { stomatalConductanceMaxMS: sp.leaf.shadeGs, leafAreaIndex: r3(shadeArea / crownPlan), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
  ];
  if (sp.racemes) regions.push({ id: "racemes", title: { en: "Flower racemes and pods", de: "Blütentrauben und Hülsen" }, kind: "foliage", model: "passive", material: sp.wood.flower, areaM2: r1(sp.limbs.filter((l) => l.raceme).length * sp.racemes.areaPerCluster), areaBasis: "exchange", characteristicLengthM: 0.02, geometry: "vertical-plate", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 900, source: "authored-estimate", reference: "jones-2014" });
  const dirFor = (bone: string): number => {
    const f = F[bone]!;
    return r1((Math.atan2(f.v[1], f.u[1]) * 180) / Math.PI);
  };
  const anchors = ctx.asset.anchors as Json[];
  const sun = anchors.filter((a) => a.id.startsWith("tp-sun-")).map((a) => a.id);
  const shade = anchors.filter((a) => a.id.startsWith("tp-shade-")).map((a) => a.id);
  return {
    environment: { airTempC: 22, meanRadiantTempC: 25, relativeHumidity: 0.55, windMS: 2, windDirectionDeg: 0, solarWM2: 700, pressurePa: 101325, diffuseFraction: 0.15 },
    regions,
    emitters: [
      { id: "transpiration-sun", kind: "transpiration", title: { en: "Transpiration of the sun leaves (distributed over the crown)", de: "Transpiration der Sonnenblätter (über die Krone verteilt)" }, anchor: sun[0], anchors: sun, directionDeg: dirFor(anchors.find((a) => a.id === sun[0])!.bone), diameterM: 2, region: "leaf-sun", source: "authored-estimate", reference: "monteith-unsworth-2013" },
      { id: "transpiration-shade", kind: "transpiration", title: { en: "Transpiration of the shade leaves (distributed over the crown)", de: "Transpiration der Schattenblätter (über die Krone verteilt)" }, anchor: shade[0], anchors: shade, directionDeg: dirFor(anchors.find((a) => a.id === shade[0])!.bone), diameterM: 1.5, region: "leaf-shade", source: "authored-estimate", reference: "monteith-unsworth-2013" },
    ],
    assumptions: [
      { en: `Leaf regions use one-sided leaf area: a leaf-area index of ${enN(sp.leaf.lai)} over the crown projection of ${enN(crownPlan, 0)} m2 (an authored estimate for an open-grown ${sp.latin}), split ${Math.round(sp.leaf.sunShare * 100)} % sun and ${Math.round((1 - sp.leaf.sunShare) * 100)} % shade leaves. The maximum stomatal conductance is an authored estimate for the species type, guided by Jones (2014). Photosynthesis type: C3. Aperture and night closure are the engine's business, not authored here.`, de: `Blattregionen verwenden die einseitige Blattfläche: ein Blattflächenindex von ${deN(sp.leaf.lai)} über der Kronenprojektion von ${deN(crownPlan, 0)} m2 (eine Autorenschätzung für einen frei gewachsenen ${sp.latin}), aufgeteilt in ${Math.round(sp.leaf.sunShare * 100)} % Sonnen- und ${Math.round((1 - sp.leaf.sunShare) * 100)} % Schattenblätter. Die maximale Spaltöffnungsleitfähigkeit ist eine Autorenschätzung für den Artentyp, angelehnt an Jones (2014). Photosynthesetyp: C3. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben.` },
      { en: "Bark regions are passive. Their areal heat capacity is the volumetric heat capacity of green wood (about 2.7 MJ per m3 and K) times an effective depth of 1.5 to 3 cm, an authored estimate guided by the Wood Handbook; their areas are the outer surface of the bark solids of the 1:50 model. No surface temperature is authored.", de: "Rindenregionen sind passiv. Ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz (etwa 2,7 MJ je m3 und K) mal einer wirksamen Tiefe von 1,5 bis 3 cm, eine Autorenschätzung angelehnt an das Wood Handbook; ihre Flächen sind die Außenfläche der Rindenkörper des Modells 1:50. Es werden keine Oberflächentemperaturen vorgegeben." },
      { en: "Transpiration emitters release the latent flux of their foliage region as vapour, spread over seven anchors on the sun clusters and five on the shaded inner crown; the direction is the vertical measured in the frame of the first anchor's bone, since the release is diffuse.", de: "Transpirationsemitter geben den latenten Strom ihrer Laubregion als Dampf ab, verteilt auf sieben Anker an den Sonnenbüscheln und fünf in der beschatteten Innenkrone; die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers, da die Abgabe diffus ist." },
      ...(sp.racemes ? [{ en: "The flower racemes are part of the model in every pose: outside the flowering pose they are folded up into the foliage masses on their own bones and stand for the young pods there; only the flowering pose lets them hang below the leaves. Their region is passive with the exchange area of the petals.", de: "Die Blütentrauben gehören in jeder Pose zum Modell: außerhalb der Blütenpose sind sie an eigenen Knochen in die Laubmassen hochgeklappt und stehen dort für die jungen Hülsen; nur die Blütenpose lässt sie unter den Blättern hängen. Ihre Region ist passiv mit der Austauschfläche der Blütenblätter." }] : []),
    ],
    operating: [],
    passengers: [],
  };
}

function structureModel(sp: Species): Json {
  const mat = sp.materials.find((m: Json) => m.id === sp.wood.material);
  return {
    gravityMS2: 9.80665,
    dynamics: "quasi-static",
    supports: ["base-support"],
    assumptions: [
      { en: "The tree is one rigid body per evaluated pose, fixed at the root plate; wind drag (EN 1991-1-4, F = 0.5 rho cd A v^2 with the power-law profile v(z) = v10 (z/10)^0.22) acts at the centre of every foliage cluster and on the stem as a line load, gravity on every bone mass. The base moment follows from statics at the posed geometry; root-soil failure and dynamic amplification are not modelled.", de: "Der Baum ist je ausgewerteter Pose ein starrer Körper, am Wurzelteller eingespannt; Winddruck (EN 1991-1-4, F = 0,5 rho cd A v^2 mit dem Potenzprofil v(z) = v10 (z/10)^0,22) wirkt im Mittelpunkt jedes Laubbüschels und am Stamm als Linienlast, Schwerkraft auf jede Knochenmasse. Das Fußmoment folgt aus der Statik der gezeigten Geometrie; Wurzel-Boden-Versagen und dynamische Überhöhung werden nicht modelliert." },
      { en: `Section properties are solid circles of green ${mat.title.en.replace(/^Green /, "").toLowerCase()} under the bark (0.92 of the bark radius; E ${enN(mat.elasticModulusPa / 1e9)} GPa, modulus of rupture ${enN(mat.strengthPa / 1e6, 0)} MPa, Wood Handbook green values); hollow or decayed stems are not represented.`, de: `Querschnittswerte sind Vollkreise aus ${mat.title.de.replace(/^Frisches /, "frischem ").replace(/^frisches /, "frischem ")} unter der Rinde (0,92 des Rindenradius; E ${deN(mat.elasticModulusPa / 1e9)} GPa, Biegefestigkeit ${deN(mat.strengthPa / 1e6, 0)} MPa, Grünholzwerte des Wood Handbook); hohle oder faule Stämme werden nicht abgebildet.` },
    ],
    clinical: false,
  };
}

type PoseKind = "calm" | "windy" | "storm" | "snow" | "flowering";
const WIND: Record<"calm" | "windy" | "storm", { v: number; cd: number; ref: string }> = { calm: { v: 1.5, cd: 0.45, ref: "en-1991-1-4" }, windy: { v: 8, cd: 0.35, ref: "en-1991-1-4" }, storm: { v: 22, cd: 0.22, ref: "vogel-1989" } };

/** 🧲️ Wind loads of a wind case: drag on every foliage cluster at its centre and on the stem as line loads. */
function windLoads(ctx: Ctx, kind: "calm" | "windy" | "storm"): Json[] {
  const { sp, bones, F } = ctx;
  const w = WIND[kind];
  const v = (z: number): number => w.v * Math.pow(Math.max(z, 1) / 10, 0.22);
  const out: Json[] = [];
  for (const b of bones.filter((x) => x.kind === "fol" || x.kind === "top")) {
    const c = ctx.centroid[b.id]!;
    const f = 0.5 * RHO * w.cd * ctx.frontal[b.id]! * sp.leaf.porosity * v(c[1]) ** 2;
    const l = toLocal(F[b.id]!, c);
    out.push({ id: `wind-${b.id}`, kind: "wind", title: { en: `Drag on the foliage: ${b.title.en.replace(/, end shoots with foliage$/, "")}`, de: `Windwiderstand am Laub: ${b.title.de.replace(/, Endtriebe mit Laub$/, "")}` }, bone: b.id, x: r3(l[0]), y: r3(l[1]), z: r3(l[2]), fxN: r1(f), fyN: 0, source: "literature", reference: w.ref });
  }
  for (const b of bones.filter((x) => x.kind === "stem")) {
    const mid = b.o[1] + 0.5 * b.len * b.u[1];
    const d = b.r0 + b.r1;
    out.push({ id: `wind-${b.id}`, kind: "wind", title: { en: `Drag on the stem: ${b.title.en}`, de: `Widerstand am Stamm: ${b.title.de}` }, fxN: 0, fyN: 0, distributed: { bone: b.id, fromM: 0, toM: r4(b.len), wxNPerM: Number((0.5 * RHO * 0.8 * d * v(mid) ** 2).toFixed(2)), wyNPerM: 0 }, source: "literature", reference: w.ref });
  }
  return out;
}

/** 🦴️ Bone offsets of a lean: stem segments share `lean` degrees downwind in growing steps, limbs lean with the cosine of their azimuth, end shoots most. */
function leanOffsets(ctx: Ctx, lean: number, limb: number): Json {
  const out: Json = {};
  const stems = ctx.bones.filter((b) => b.kind === "stem");
  stems.forEach((b, i) => (out[b.id] = { angleDeg: r3(-lean * (0.6 + (0.8 * i) / Math.max(1, stems.length - 1)) / stems.length) }));
  out["fol-top"] = { angleDeg: r3(-limb * 1.3) };
  for (const b of ctx.bones.filter((x) => x.kind === "limb" || x.kind === "fol")) {
    const k = b.kind === "fol" ? 1.2 : [0.5, 0.8, 1][b.seg!]!;
    out[b.id] = { angleDeg: r3(-limb * k * Math.cos(b.az * D2R)) };
  }
  return out;
}

function droopOffsets(ctx: Ctx, droop: number): Json {
  const out: Json = {};
  for (const b of ctx.bones.filter((x) => x.kind === "limb" || x.kind === "fol")) out[b.id] = { angleDeg: r3(-droop * (b.kind === "fol" ? 1.5 : [0.35, 0.7, 1][b.seg!]!)) };
  out["fol-top"] = { angleDeg: 0 };
  return out;
}

function regionBinding(ctx: Ctx, kind: PoseKind): Json {
  const base: Record<string, Json> = {
    calm: { "trunk-bark": [1, 0.35], "limb-bark": [1, 0.25], "leaf-sun": [1, 0.9], "leaf-shade": [1, 0.08], racemes: [0.2, 0.05] },
    windy: { "trunk-bark": [1, 0.35], "limb-bark": [1, 0.25], "leaf-sun": [1, 0.9], "leaf-shade": [1, 0.1], racemes: [0.2, 0.05] },
    storm: { "trunk-bark": [1, 0.1, 0.4], "limb-bark": [1, 0.1, 0.4], "leaf-sun": [1, 0.5, 0.4], "leaf-shade": [1, 0, 0.4], racemes: [0.2, 0, 0.4] },
    snow: { "trunk-bark": [1, 0, 0.3], "limb-bark": [1, 0, 0.3], "leaf-sun": [1, 0, 0.5], "leaf-shade": [1, 0, 0.5], racemes: [0.2, 0, 0.5] },
    flowering: { "trunk-bark": [1, 0.35], "limb-bark": [1, 0.25], "leaf-sun": [1, 0.85], "leaf-shade": [1, 0.08], racemes: [1, 0.6] },
  };
  const out: Json = {};
  for (const r of ctx.asset.thermal.regions as Json[]) {
    const [e, s, wet] = base[kind]![r.id];
    out[r.id] = { exposedFraction: e, sunlitFraction: s, ...(wet ? { wettedness: wet } : {}) };
  }
  return out;
}

function poseOf(ctx: Ctx, id: string, kind: PoseKind, bones: Json, loads: Json[], emit: number): Json {
  const t = ctx.sp.poseTexts[id]!;
  return { id, title: t.title, description: t.description, bones, ground: "authored", contacts: [], thermal: { regions: regionBinding(ctx, kind), emitters: { "transpiration-sun": emit, "transpiration-shade": emit } }, loads };
}

/** 🧍️ The named poses: calm (rest), fresh breeze, gale, wet snow where the species has it, and the species' own extra poses. */
function posesOf(ctx: Ctx): Json[] {
  const { sp, bones, F } = ctx;
  const out: Json[] = [];
  out.push(poseOf(ctx, "calm", "calm", {}, windLoads(ctx, "calm"), 0.8));
  out.push(poseOf(ctx, "windy", "windy", leanOffsets(ctx, sp.sway.windyLean, 3), windLoads(ctx, "windy"), 1));
  out.push(poseOf(ctx, "storm", "storm", leanOffsets(ctx, sp.sway.stormLean, 7), windLoads(ctx, "storm"), 0.5));
  if (sp.poseTexts["wet-snow"]) {
    const loads = windLoads(ctx, "calm").filter((l) => l.distributed);
    for (const b of bones.filter((x) => x.kind === "fol" || x.kind === "top")) {
      const l = toLocal(F[b.id]!, ctx.centroid[b.id]!);
      loads.push({ id: `snow-${b.id}`, kind: "snow", title: { en: `Snow on the foliage: ${b.title.en.replace(/, end shoots with foliage$/, "")}`, de: `Schnee auf dem Laub: ${b.title.de.replace(/, Endtriebe mit Laub$/, "")}` }, bone: b.id, x: r3(l[0]), y: r3(l[1]), z: r3(l[2]), fxN: 0, fyN: r1(-G * sp.leaf.snowKgM2 * ctx.planArea[b.id]!), source: "authored-estimate", reference: "en-1991-1-3" });
    }
    out.push(poseOf(ctx, "wet-snow", "snow", droopOffsets(ctx, 4), loads, 0));
  }
  if (sp.racemes && sp.poseTexts["flowering"]) {
    const rb: Json = {};
    for (const b of bones.filter((x) => x.kind === "raceme")) rb[b.id] = { angleDeg: 178 };
    const droop = droopOffsets(ctx, 1);
    out.push(poseOf(ctx, "flowering", "flowering", { ...droop, ...rb }, windLoads(ctx, "calm"), 0.9));
  }
  return out;
}

/** 🎞️ Clips: a looping, periodic wind sway about the fresh-breeze lean, a single gust with load tracks, and a day of transpiration. */
function clipsOf(ctx: Ctx): Json[] {
  const { sp, bones } = ctx;
  const P = sp.sway.periodS;
  const windy = leanOffsets(ctx, sp.sway.windyLean, 3);
  const level = (id: string): number => {
    const b = ctx.byId.get(id)!;
    return b.kind === "stem" ? 0 : b.kind === "top" ? 1.4 : b.kind === "fol" ? 1.3 : 0.35 * (b.seg! + 1);
  };
  const swayKeys = Array.from({ length: 9 }, (_, i) => {
    const t = (P * i) / 8;
    const bonesAt: Json = {};
    for (const [id, o] of Object.entries(windy) as [string, Json][]) {
      const b = ctx.byId.get(id)!;
      const amp = b.kind === "stem" ? 0.35 * Math.abs(o.angleDeg) + 0.08 : 0.35 * Math.abs(o.angleDeg) + (b.kind === "fol" || b.kind === "top" ? 1.2 : 0.5);
      bonesAt[id] = { angleDeg: r3(o.angleDeg - amp * Math.sin((2 * Math.PI * t) / P - level(id))) };
    }
    return { t: r3(t), pose: "windy", ease: "smooth", bones: bonesAt };
  });
  const storm = leanOffsets(ctx, sp.sway.stormLean, 7);
  const gustT = [0, 1.2, 2.5, 3.6, 5, 8], gustLoad = [0.05, 0.45, 2.75, 1.4, 0.5, 0.05], gustLean = [0.04, 0.2, 0.7, 0.5, 0.22, 0.04];
  const gustKeys = gustT.map((t, i) => ({ t, pose: "windy", ease: "smooth", bones: Object.fromEntries(Object.entries(storm).map(([id, o]: [string, Json]) => [id, { angleDeg: r3(o.angleDeg * gustLean[i]!) }])) }));
  const windLoadIds = (ctx.asset.poses[1].loads as Json[]).map((l) => l.id);
  const sunTable = [[0, 0], [6, 0], [8, 0.43], [10, 0.79], [12, 1], [14, 0.79], [16, 0.36], [18, 0], [24, 0]];
  const calmSun: Record<string, number> = { "trunk-bark": 0.35, "limb-bark": 0.25, "leaf-sun": 0.9, "leaf-shade": 0.08, racemes: 0.05 };
  return [
    { id: "wind-sway", title: { en: "Wind sway", de: "Windschwingung" }, description: { en: `A deterministic ${enN(P)} s sway cycle about the fresh-breeze lean (first bending mode of the stem, estimated ${enN(1 / P, 2)} Hz for a ${enN(ctx.asset.dimensions.heightM, 0)} m ${sp.latin}). Every level lags the one below: limbs and end shoots follow the stem with growing phase and amplitude, foliage clusters swing up to about ${enN(1.2 + 0.35 * 3.6, 1)} degrees about the mean.`, de: `Ein deterministischer Schwingungszyklus von ${deN(P)} s um die Neigung der frischen Brise (erste Biegeeigenform des Stamms, geschätzt ${deN(1 / P, 2)} Hz für einen ${deN(ctx.asset.dimensions.heightM, 0)} m hohen ${sp.latin}). Jede Ebene hinkt der darunter nach: Äste und Endtriebe folgen dem Stamm mit wachsender Phase und Amplitude, die Laubbüschel schwingen bis zu etwa ${deN(1.2 + 0.35 * 3.6, 1)} Grad um den Mittelwert.` }, durationS: P, fps: 30, loop: true, rootMotion: "none", keys: swayKeys, contacts: [], phases: [{ id: "sway", fromS: 0, toS: P, title: { en: "Sway cycle", de: "Schwingungszyklus" } }], events: [{ t: 0, kind: "motion", title: { en: "Cycle start, stem at mean lean", de: "Zyklusbeginn, Stamm in mittlerer Neigung" } }], tracks: [{ target: "emitter:transpiration-sun", ease: "smooth", keys: [{ t: 0, v: 1 }, { t: r3(P / 2), v: 0.9 }, { t: P, v: 1 }] }, { target: "emitter:transpiration-shade", ease: "smooth", keys: [{ t: 0, v: 1 }, { t: r3(P / 2), v: 0.9 }, { t: P, v: 1 }] }] },
    { id: "gust", title: { en: "Gust", de: "Windböe" }, description: { en: "A single gust over 8 s: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load (peak velocity pressure (1 + 7 Iv) q_m with Iv 0.25, EN 1991-1-4) at 2.5 s and decay again; the crown leans up to 70 % of the gale lean and recovers slowly.", de: "Eine einzelne Böe über 8 s: Die Windlast-Spuren steigen von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzengeschwindigkeitsdruck (1 + 7 Iv) q_m mit Iv 0,25, EN 1991-1-4) bei 2,5 s und klingen wieder ab; die Krone neigt sich bis zu 70 % der Sturmneigung und richtet sich langsam wieder auf." }, durationS: 8, fps: 30, loop: false, rootMotion: "none", keys: gustKeys, contacts: [], phases: [{ id: "rise", fromS: 0, toS: 2.5, title: { en: "Gust front", de: "Böenfront" } }, { id: "peak", fromS: 2.5, toS: 3.6, title: { en: "Peak", de: "Spitze" } }, { id: "decay", fromS: 3.6, toS: 8, title: { en: "Decay and recovery", de: "Abklingen und Erholung" } }], events: [{ t: 2.5, kind: "load-on", title: { en: "Peak gust load", de: "Böenspitze" } }, { t: 5, kind: "load-off", title: { en: "Load back below the mean", de: "Last wieder unter dem Mittelwert" } }], tracks: windLoadIds.map((id) => ({ target: `load:${id}`, ease: "smooth", keys: gustT.map((t, i) => ({ t, v: gustLoad[i] })) })) },
    { id: "diurnal-transpiration", title: { en: "Day of transpiration", de: "Transpirationstag" }, description: { en: "A 24 h cycle compressed to 24 s (1 s = 1 h, 00:00 to 24:00), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun, stomata nearly closed at night; leaf and bark temperatures are not authored but follow from the thermal solver.", de: "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) bei ruhigem Wetter. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Stomata fast geschlossen; Blatt- und Rindentemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser." }, durationS: 24, fps: 10, loop: true, rootMotion: "none", keys: [{ t: 0, pose: "calm", ease: "smooth" }, { t: 12, pose: "calm", ease: "smooth" }, { t: 24, pose: "calm", ease: "smooth" }], contacts: [], phases: [{ id: "night", fromS: 0, toS: 6, title: { en: "Night", de: "Nacht" } }, { id: "morning", fromS: 6, toS: 12, title: { en: "Morning to noon", de: "Morgen bis Mittag" } }, { id: "afternoon", fromS: 12, toS: 18, title: { en: "Afternoon", de: "Nachmittag" } }, { id: "evening", fromS: 18, toS: 24, title: { en: "Evening and night", de: "Abend und Nacht" } }], events: [{ t: 6, kind: "thermal", title: { en: "Sunrise", de: "Sonnenaufgang" } }, { t: 12, kind: "thermal", title: { en: "Solar noon, peak transpiration", de: "Sonnenhöchststand, maximale Transpiration" } }, { t: 18, kind: "thermal", title: { en: "Sunset", de: "Sonnenuntergang" } }], tracks: [
      { target: "emitter:transpiration-sun", ease: "smooth", keys: [[0, 0.03], [5, 0.05], [7, 0.3], [9, 0.7], [12, 1], [15, 0.9], [18, 0.35], [20, 0.08], [24, 0.03]].map(([t, v]) => ({ t, v })) },
      { target: "emitter:transpiration-shade", ease: "smooth", keys: [[0, 0.03], [5, 0.05], [7, 0.45], [9, 0.8], [12, 0.95], [15, 0.7], [18, 0.2], [20, 0.06], [24, 0.03]].map(([t, v]) => ({ t, v })) },
      ...(ctx.asset.thermal.regions as Json[]).map((r) => ({ target: `region:${r.id}:sunlitFraction`, ease: "smooth", keys: sunTable.map(([t, f]) => ({ t, v: r3(f! * calmSun[r.id]!) })) })),
    ] },
  ];
}

const PENS: Record<Scale, Json> = { 500: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 }, 200: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 }, 100: { body: 0.2, detail: 0.13, texture: 0.1, glazing: null, ground: 0.3 }, 50: { body: 0.25, detail: 0.18, texture: 0.13, glazing: 0.1, ground: 0.35 } };
const RANGE: Record<Scale, [number, number]> = { 50: [20, 75], 100: [75, 150], 200: [150, 350], 500: [350, 1000] };

/** 🖨️ One authored representation: the three views drawn from the scale's body, pens, hatches, valid range and the decisions. */
function representationOf(ctx: Ctx, scale: Scale): Json {
  const { sp } = ctx;
  const geometry: Json = {};
  for (const v of VIEWS) geometry[v] = [...drawView(ctx, scale, v), ...barkStrokes(ctx, scale, v)];
  const el = geometry.elevation as Json[];
  const woodN = el.filter((s) => s.region === "trunk-bark" || s.region === "limb-bark").length;
  const folN = el.filter((s) => s.region === "leaf-sun" || s.region === "leaf-shade").length;
  const strokes = el.filter((s) => s.role === "texture").length;
  const mm = 1000 / scale;
  const thin = 2 * RMIN[scale] * 1000;
  const hatches = scale <= 100 ? [{ id: "shade-lines", title: { en: "Shade line hatch", de: "Schattenschraffur" }, pattern: "lines", angleDeg: 45, spacingMm: scale === 50 ? 0.8 : 0.6, penMm: 0.1 }] : [];
  const p = PENS[scale];
  const T = sp.scaleTexts[scale];
  const name = sp.title;
  return {
    status: "authored",
    denominator: scale,
    validRange: { minDenominator: RANGE[scale][0], maxDenominator: RANGE[scale][1] },
    pens: p,
    hatches,
    geometry,
    decisions: {
      silhouette: { en: `At 1:${scale} (1 m is ${enN(mm, mm < 5 ? 0 : 0)} mm) the ${name.en} is drawn as the projection of its model of the same scale: ${T.silhouette.en} Side elevation, front elevation and plan are three views of the same solids.`, de: `Bei 1:${scale} (1 m sind ${deN(mm, 0)} mm) ist die Zeichnung (${name.de}) die Projektion des Modells desselben Maßstabs: ${T.silhouette.de} Seitenansicht, Vorderansicht und Draufsicht sind drei Ansichten derselben Körper.` },
      retained: { en: `Retained: ${woodN} wood outlines, one per bone and bark region, and ${folN} foliage outlines, painted in the depth order of the view${strokes > 0 ? `, plus ${strokes} bark ridge strokes on the stem` : ""}.`, de: `Beibehalten: ${woodN} Holzumrisse, je Knochen und Rindenregion einer, und ${folN} Laubumrisse, in der Tiefenfolge der Ansicht gemalt${strokes > 0 ? `, dazu ${strokes} Borkenrippen-Striche am Stamm` : ""}.` },
      merged: scale === 500 ? { en: "The crown is generalised into a few large masses bound to the stem segments, merged into one outline per stem segment and leaf region; the limbs of the crown interior disappear in it.", de: "Die Krone ist zu wenigen großen, an die Stammabschnitte gebundenen Massen verallgemeinert, je Stammabschnitt und Blattregion zu einem Umriss verschmolzen; die Äste im Kroneninneren gehen darin auf." } : { en: "The wood solids of one bone are merged into one outline; the foliage masses of one limb system are merged into one lobed outline, bound in the side elevation to the stem segment that carries the limb and in front and plan to the limb's own end shoot.", de: "Die Holzkörper eines Knochens sind zu einem Umriss verschmolzen; die Laubmassen eines Astsystems sind zu einem gelappten Umriss verschmolzen, in der Seitenansicht an den tragenden Stammabschnitt gebunden, in Vorderansicht und Draufsicht an den eigenen Endtrieb des Asts." },
      omitted: { en: `Omitted: wood thinner than ${enN(thin, 0)} mm, single leaves${scale >= 200 ? ", side branches, end shoots and the buttress roots" : ""}; the relief of foliage is carried by the lobed outlines themselves.`, de: `Weggelassen: Holz dünner als ${deN(thin, 0)} mm, einzelne Blätter${scale >= 200 ? ", Seitenzweige, Endtriebe und die Wurzelanläufe" : ""}; das Relief des Laubs tragen die gelappten Umrisse selbst.` },
      thinElements: { en: `Wood tapers to ${enN(thin, 0)} mm, the smallest section this scale models, and is never drawn thinner; nothing of the drawing is thickened beyond its model.`, de: `Holz verjüngt sich bis ${deN(thin, 0)} mm, den kleinsten Querschnitt, den dieser Maßstab modelliert, und wird nie dünner gezeichnet; nichts der Zeichnung ist über ihr Modell hinaus verdickt.` },
      hatching: scale <= 100 ? { en: "The shaded inner foliage carries the shade line hatch; no other hatching.", de: "Das beschattete innere Laub trägt die Schattenschraffur; sonst keine Schraffur." } : { en: "No hatching: at this scale tone alone separates sun and shade foliage.", de: "Keine Schraffur: In diesem Maßstab trennt allein der Ton Sonnen- und Schattenlaub." },
      styles: { en: `Light line: ${enN(p.body, 2)} mm outlines${p.detail ? `, ${enN(p.detail, 2)} mm detail` : ""}${p.texture ? `, ${enN(p.texture, 2)} mm texture` : ""}. Dark solid: tonal masses, bark darker than limbs, sun foliage lighter than shade foliage.`, de: `Helle Linie: Umrisse ${deN(p.body, 2)} mm${p.detail ? `, Detail ${deN(p.detail, 2)} mm` : ""}${p.texture ? `, Textur ${deN(p.texture, 2)} mm` : ""}. Dunkle Fläche: getönte Massen, Rinde dunkler als Äste, Sonnenlaub heller als Schattenlaub.` },
      overlays: { en: "Every thermal region keeps its own shapes, so temperatures act on the drawn parts; wind and snow loads act at the foliage clusters and on the stem, where the structural views draw them.", de: "Jede thermische Region behält ihre eigenen Formen, sodass Temperaturen an den gezeichneten Teilen wirken; Wind- und Schneelasten greifen an den Laubbüscheln und am Stamm an, wo die Tragwerksansichten sie zeichnen." },
    },
  };
}

/** 🔣️ The true-extent plan sign (crown outline, stem, scaffold directions in all three views) and the nominal circle-and-dot tree sign. */
function symbolsOf(ctx: Ctx): Json[] {
  const { sp, bones } = ctx;
  const parts = ctx.parts[500];
  const crown = parts.filter((p) => p.group === "sun" || p.group === "shade");
  const stem = parts.filter((p) => p.group === "root" || p.group === "stem");
  const pts = (ring: { x: number; y: number }[]): number[][] => ring.map((q) => [r4(q.x), r4(q.y)]);
  const geom: Json = {};
  const outer = (ring: { x: number; y: number }[]): boolean => ring.reduce((a, q, i) => a + q.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * q.y, 0) > 0;
  for (const v of VIEWS) {
    const c = outlineOf(v, crown, 500);
    const s = v === "plan" ? outlineOf(v, stem.slice(0, 2), 500) : outlineOf(v, stem, 500);
    geom[v] = [
      ...c.filter(outer).map((ring, i) => ({ id: i === 0 ? "crown" : `crown-${i + 1}`, bone: "root-plate", role: "body", z: 2, filled: false, tone: 0.5, kind: "poly", points: pts(ring), closed: true })),
      { id: "stem-1", bone: "root-plate", role: "body", z: 4, filled: true, tone: 0.3, kind: "poly", points: pts(s[0]!), closed: true },
    ];
  }
  bones.filter((b) => b.kind === "limb" && b.seg === 0).forEach((b, i) => {
    const tip = add(b.o, mul(b.u, b.len));
    geom.plan.push({ id: `scaffold-${i + 1}`, bone: "root-plate", role: "detail", z: 3, filled: false, tone: 0.35, kind: "poly", points: [[r4(b.o[0]), r4(b.o[2])], [r4(tip[0]), r4(tip[2])]], closed: false });
  });
  const d = ctx.asset.dimensions;
  const circleEl = "M1.8 3.42C1.8 3.72 1.709 4.06 1.559 4.32C1.409 4.58 1.16 4.829 0.9 4.979C0.64 5.129 0.3 5.22 0 5.22C-0.3 5.22 -0.64 5.129 -0.9 4.979C-1.16 4.829 -1.409 4.58 -1.559 4.32C-1.709 4.06 -1.8 3.72 -1.8 3.42C-1.8 3.12 -1.709 2.78 -1.559 2.52C-1.409 2.26 -1.16 2.011 -0.9 1.861C-0.64 1.711 -0.3 1.62 0 1.62C0.3 1.62 0.64 1.711 0.9 1.861C1.16 2.011 1.409 2.26 1.559 2.52C1.709 2.78 1.8 3.12 1.8 3.42Z";
  const dotEl = "M0.35 3.42C0.35 3.502 0.306 3.609 0.247 3.667C0.189 3.726 0.082 3.77 0 3.77C-0.082 3.77 -0.189 3.726 -0.247 3.667C-0.306 3.609 -0.35 3.502 -0.35 3.42C-0.35 3.338 -0.306 3.231 -0.247 3.173C-0.189 3.114 -0.082 3.07 0 3.07C0.082 3.07 0.189 3.114 0.247 3.173C0.306 3.231 0.35 3.338 0.35 3.42Z";
  const circlePl = "M1.8 0C1.8 0.3 1.709 0.64 1.559 0.9C1.409 1.16 1.16 1.409 0.9 1.559C0.64 1.709 0.3 1.8 0 1.8C-0.3 1.8 -0.64 1.709 -0.9 1.559C-1.16 1.409 -1.409 1.16 -1.559 0.9C-1.709 0.64 -1.8 0.3 -1.8 0C-1.8 -0.3 -1.709 -0.64 -1.559 -0.9C-1.409 -1.16 -1.16 -1.409 -0.9 -1.559C-0.64 -1.709 -0.3 -1.8 0 -1.8C0.3 -1.8 0.64 -1.709 0.9 -1.559C1.16 -1.409 1.409 -1.16 1.559 -0.9C1.709 -0.64 1.8 -0.3 1.8 0Z";
  const dotPl = "M0.35 0C0.35 0.082 0.306 0.189 0.247 0.247C0.189 0.306 0.082 0.35 0 0.35C-0.082 0.35 -0.189 0.306 -0.247 0.247C-0.306 0.189 -0.35 0.082 -0.35 0C-0.35 -0.082 -0.306 -0.189 -0.247 -0.247C-0.189 -0.306 -0.082 -0.35 0 -0.35C0.082 -0.35 0.189 -0.306 0.247 -0.247C0.306 -0.189 0.35 -0.082 0.35 0Z";
  const sign = (v: "elevation" | "front" | "plan"): Json[] => v === "plan" ? [{ id: "sign-circle", bone: "root-plate", role: "body", z: 1, filled: false, tone: 0.5, kind: "path", d: circlePl }, { id: "sign-dot", bone: "root-plate", role: "body", z: 2, filled: true, tone: 0.3, kind: "path", d: dotPl }] : [{ id: "sign-circle", bone: "root-plate", role: "body", z: 1, filled: false, tone: 0.5, kind: "path", d: circleEl }, { id: "sign-dot", bone: "root-plate", role: "body", z: 2, filled: true, tone: 0.3, kind: "path", d: dotEl }, { id: "sign-stem", bone: "root-plate", role: "body", z: 1, filled: false, tone: 0.3, kind: "path", d: "M0 0L0 1.62" }];
  return [
    { id: `${sp.id}-symbol`, title: { en: "Plan symbol with true crown extent", de: "Grundrisssymbol mit wahrer Kronenausdehnung" }, description: { en: "A conventional plan sign for site and landscape plans: the true outline of the crown, the scaffold limb directions and the stem section. It is a separate symbol and is never used silently for a pictorial scale.", de: "Ein konventionelles Grundrisszeichen für Lage- und Freiraumpläne: der wahre Umriss der Krone, die Richtungen der Leitäste und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet." }, semantics: "true-extent", units: "model-m", denominators: [500, 200, 100], pens: { body: 0.2, detail: 0.13, texture: 0.1, glazing: null, ground: 0.3 }, hatches: [], geometry: geom, dimensionalNote: { en: `A conventional simplification of the tree drawn at its true extent in model metres in all three views (${enN(d.widthM, 2)} m wide, ${enN(d.depthM, 2)} m deep and ${enN(d.heightM, 2)} m high): the outline is the crown, the stem is drawn at its real position and thickness. It is a symbol, never used silently for a pictorial scale.`, de: `Eine konventionelle Vereinfachung des Baums in seiner wahren Ausdehnung in Modellmetern in allen drei Ansichten (${deN(d.widthM, 2)} m breit, ${deN(d.depthM, 2)} m tief und ${deN(d.heightM, 2)} m hoch): Der Umriss ist die Krone, der Stamm ist in wirklicher Lage und Stärke gezeichnet. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.` } },
    { id: "tree-sign", title: { en: "Nominal tree sign with trunk centre", de: "Nominelles Baumzeichen mit Stammmitte" }, description: { en: "The conventional circle-and-dot tree sign of small-scale site plans: the dot marks the trunk centre, the circle is a fixed paper size.", de: "Das übliche Kreis-und-Punkt-Baumzeichen kleinmaßstäblicher Lagepläne: Der Punkt markiert die Stammmitte, der Kreis hat eine feste Papiergröße." }, semantics: "nominal", units: "paper-mm", denominators: [500, 200], pens: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 }, hatches: [], geometry: { elevation: sign("elevation"), front: sign("front"), plan: sign("plan") }, dimensionalNote: { en: "A conventional symbol in paper millimetres, not to scale: a fixed 3.6 mm tree sign, drawn as plan circle, side lollipop and front lollipop. Its size carries no dimension; side and front are the same sign.", de: "Ein konventionelles Symbol in Papiermillimetern, nicht maßstäblich: ein festes Baumzeichen von 3,6 mm, gezeichnet als Grundrisskreis sowie als Lutscher in Seiten- und Vorderansicht. Seine Größe trägt keine Abmessung; Seiten- und Vorderansicht sind dasselbe Zeichen." } },
  ];
}

/** 🧊️ The authored volume of a scale: body solids of the parts, thermal references plus transpiration outlets, the structural list and the decisions. */
function volumeOf(ctx: Ctx, scale: Scale): Json {
  const { sp, F, parts } = ctx;
  const body = parts[scale].map((p) => bodySolid(p, F));
  const thermal: Json[] = parts[scale].map((p) => ({ id: `th-${p.id}`, of: p.id, region: regionOf(p.group) }));
  const anchors = (ctx.asset.anchors as Json[]).filter((a) => a.kind === "emitter");
  const keep = scale <= 100 ? anchors : scale === 200 ? anchors.filter((a) => /-(1|3|5)$/.test(a.id) && !/shade-5/.test(a.id)) : anchors.filter((a) => /-1$/.test(a.id));
  const f = FEATURE[scale];
  for (const a of keep) {
    const sun = a.id.startsWith("tp-sun");
    const w = add(worldOfAnchor(ctx, a), [0, -0.04, 0]);
    const lens = massLobe(`th-${a.id}`, a.bone, sun ? sp.wood.leafSun : sp.wood.leafShade, sun ? 0.6 : 0.44, sun ? "sun" : "shade", w, [0.22 + 3 * f, 0.05 + f, 0.22 + 3 * f].map(r4) as V3, 0);
    const s = bodySolid(lens, F);
    s.role = "detail";
    thermal.push({ ...s, region: sun ? "leaf-sun" : "leaf-shade", tags: [sun ? "transpiration-sun" : "transpiration-shade"] });
  }
  const structure = structureList(ctx, scale);
  const T = sp.scaleTexts[scale];
  const woodN = parts[scale].filter((p) => p.kind === "tube").length;
  const folN = parts[scale].filter((p) => p.kind === "lobe").length;
  return {
    status: "authored",
    denominator: scale,
    body,
    thermal,
    structure,
    decisions: [
      { en: `Body at 1:${scale} (${body.length} solids, smallest feature ${enN(f * 1000)} mm): ${T.body.en} The wood is ${woodN} tapered lofts along their bones, none thinner than ${enN(2 * RMIN[scale] * 1000, 0)} mm, each starting inside its parent so sway never opens a joint; the foliage is ${folN} masses that follow the bone that carries them.`, de: `Körper bei 1:${scale} (${body.length} Körper, kleinstes Merkmal ${deN(f * 1000)} mm): ${T.body.de} Das Holz besteht aus ${woodN} verjüngten Lofts entlang ihrer Knochen, keiner dünner als ${deN(2 * RMIN[scale] * 1000, 0)} mm, jeder im Mutterknochen beginnend, sodass kein Gelenk beim Schwingen aufgeht; das Laub aus ${folN} Massen, die dem tragenden Knochen folgen.` },
      { en: `Thermal at 1:${scale} (${thermal.length} solids): every bark and foliage surface is its region by reference to the body solid, so the thermal view is the same tree; no region has layers. ${keep.length} transpiration outlets are low lenses seated on the tops of the foliage masses that carry their anchors.`, de: `Thermik bei 1:${scale} (${thermal.length} Körper): Jede Rinden- und Laubfläche ist ihre Region als Verweis auf den Körper, die thermische Ansicht zeigt also denselben Baum; keine Region hat Schichten. ${keep.length} Transpirationsauslässe sind flache Linsen auf den Oberseiten der Laubmassen, die ihre Anker tragen.` },
      { en: `Structure at 1:${scale} (${structure.length} solids): the load path is the wood itself. Every stem, limb and end-shoot bone is a round member along its axis at 0.8 of the bark radius, starting inside its parent down to the root collar; ${structure.filter((s) => s.part === "joint").length} joint spheres sit at the rig joints this scale shows, the fixed support is the root plate disc of ${enN(1.8 * sp.root.flareR, 2)} m on the ground, and the stem segments carry mass spheres on their axis.`, de: `Tragwerk bei 1:${scale} (${structure.length} Körper): Der Lastpfad ist das Holz selbst. Jeder Stamm-, Ast- und Endtriebknochen ist ein Rundstab entlang seiner Achse mit dem 0,8-fachen Rindenradius, im Mutterknochen beginnend bis hinab zum Wurzelhals; ${structure.filter((s) => s.part === "joint").length} Gelenkkugeln sitzen an den Gelenken, die dieser Maßstab zeigt, das eingespannte Auflager ist die Wurzeltellerscheibe von ${deN(1.8 * sp.root.flareR, 2)} m auf dem Boden, und die Stammabschnitte tragen Massenkugeln auf ihrer Achse.` },
    ],
  };
}

if (import.meta.main) {
  const name = process.argv[2];
  if (!name) throw new Error("usage: assemble.ts <species> [--ship]");
  const sp: Species = (await import(`./${name}.ts`)).species;
  const { doc, row, report } = build(sp);
  const out = join(T, "🗑️generated/PW-05", sp.id, "🔣️.json");
  writeDoc(out, doc);
  console.log(report.join("\n"));
  console.log(`[DEBUG] wrote ${out}`);
  if (process.argv.includes("--ship")) {
    writeDoc(join(S, "🗂️catalog/🌳️vegetation", sp.id, "🔣️.json"), doc);
    writeDoc(join(T, "📥️manifest-rows", `${sp.id}.json`), row);
    console.log("[DEBUG] shipped to catalog and manifest row");
  }
}
