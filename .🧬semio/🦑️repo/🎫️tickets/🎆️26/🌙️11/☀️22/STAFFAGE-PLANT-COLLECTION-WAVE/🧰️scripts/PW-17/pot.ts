/** 🏺️ PW-17 pots and saucers: the hand-stated half profiles of a glazed stoneware pot with its saucer and the soil sheet, per scale, on a root bone whose axis u points up (u = height, r = radius). At 1:50 saucer, pot and compost are drawn apart (the saucer ring, the rolled rim and the compost inside it in plan); from 1:100 on, where the saucer band and the rim ring are thinner than the printable feature at the end of the band, pot and saucer are one turned outline and the plan is one disc that carries the three regions.
 * @see ./lib.ts */
import { BAND, FEATURE, T, ellipseRing, plainShape, r5, revolveSolid, revolvedShape, type Json, type P, type Placed, type Scale, type Text } from "./lib.ts";

/** 🧾️ A pot: saucer and pot outline profiles at 1:50 (u up from the floor, r radius; the pot outline closes through the axis at the compost), the closed loop of the pot shell with its drainage hole (the 3D pot of 1:50 and 1:100, so that the compost sheet closes the cavity of the pot), the compost fill inside the shell (a body solid that carries the roots, open profile closed through the axis), the soil level and radius, the inner and outer rim radii, the saucer radius and height. */
export type PotSpec = { saucer: P[]; pot: P[]; shell: P[]; fill: P[]; soilU: number; soilR: number; rimInnerR: number; rimTopU: number; potOuterR: number; saucerR: number; saucerTopU: number; materials: { pot: string; saucer: string; soil: string }; tones: { pot: number; saucer: number; soil: number } };

/** 🪜️ A profile reduced for a coarser scale: points kept by index. */
export const keep = (profile: readonly P[], idx: readonly number[]): P[] => idx.map((i) => profile[i]!);

/** 📐️ The radius of a profile at height u (linear between its points on the rising outer side). */
const radiusAt = (profile: readonly P[], u: number): number => {
  for (let i = 1; i < profile.length; i++) {
    const [a, b] = [profile[i - 1]!, profile[i]!];
    if (u >= a[0] && u <= b[0] && b[0] > a[0]) return a[1] + ((b[1] - a[1]) * (u - a[0])) / (b[0] - a[0]);
  }
  return profile[0]![1];
};

/** 🧱️ The pot parts of one scale: 2D shapes (elevation and front revolved, plan as rings and discs) and body solids. */
export function potParts(spec: PotSpec, scale: Scale, profiles: { saucer: P[]; pot: P[] }): { placed: Placed[]; plan: Placed[]; body: Json[]; thermal: Json[] } {
  const soilT = 0.006;
  const placed: Placed[] = [];
  const plan: Placed[] = [];
  const end = BAND[scale][1];
  const prints = (m: number) => (m * 1000) / end >= 0.22;
  const n = scale === 50 ? 44 : scale === 100 ? 32 : 24;
  const potMeta = { tone: spec.tones.pot, region: "pot-wall", material: spec.materials.pot };
  const apart = prints(spec.saucerR - spec.potOuterR) && prints(spec.rimTopU - spec.soilU) && prints(spec.potOuterR - spec.rimInnerR) && prints(profiles.saucer[profiles.saucer.length - 1]![0]);
  if (apart) {
    placed.push({ shape: revolvedShape("saucer", "pot", profiles.saucer, { tone: spec.tones.saucer, region: "saucer", material: spec.materials.saucer }), at: [0, 0.01, 0], rank: 1e6 });
    placed.push({ shape: revolvedShape("pot", "pot", profiles.pot, { tone: spec.tones.pot, regions: ["pot-wall", "soil-surface"], material: spec.materials.pot }), at: [0, 0.1, 0], rank: 2e6 });
    plan.push({ shape: plainShape("saucer", "pot", [ellipseRing(0, 0, spec.saucerR, spec.saucerR, n)], { tone: spec.tones.saucer, region: "saucer", material: spec.materials.saucer }), at: [0, -1, 0] });
    plan.push({ shape: plainShape("pot", "pot", [ellipseRing(0, 0, spec.potOuterR, spec.potOuterR, n), ellipseRing(0, 0, spec.rimInnerR, spec.rimInnerR, n)], potMeta), at: [0, -0.5, 0] });
    plan.push({ shape: plainShape("soil", "pot", [ellipseRing(0, 0, spec.rimInnerR + 0.002, spec.rimInnerR + 0.002, n)], { tone: spec.tones.soil, region: "soil-surface", material: spec.materials.soil }), at: [0, -0.9, 0] });
  } else {
    const top = profiles.saucer.reduce((m, p) => Math.max(m, p[0]), 0);
    const crest = profiles.pot.reduce((best, p, i) => (p[0] > profiles.pot[best]![0] ? i : best), 0);
    const outer: P[] = [[0, profiles.saucer[0]![1]], [top, spec.saucerR], [top, radiusAt(profiles.pot, top)], ...profiles.pot.slice(0, crest + 1).filter((p) => p[0] > top + 0.002)];
    placed.push({ shape: revolvedShape("pot", "pot", outer, { tone: spec.tones.pot, regions: ["pot-wall", "saucer", "soil-surface"], material: spec.materials.pot }), at: [0, 0.1, 0], rank: 2e6 });
    plan.push({ shape: plainShape("pot", "pot", [ellipseRing(0, 0, spec.saucerR, spec.saucerR, n)], { tone: spec.tones.pot, regions: ["pot-wall", "saucer", "soil-surface"], material: spec.materials.pot }), at: [0, -0.5, 0] });
  }
  const soilProfile: P[] = [[spec.soilU - soilT, spec.soilR], [spec.soilU, spec.soilR]];
  const fine = scale <= 100;
  const saucer = revolveSolid("saucer", "pot", profiles.saucer, { material: spec.materials.saucer, tone: spec.tones.saucer });
  const pot = fine ? { ...revolveSolid("pot", "pot", spec.shell, { material: spec.materials.pot, tone: spec.tones.pot }), profile: spec.shell.map(([u, r]) => [r5(u), r5(r)]) } : revolveSolid("pot", "pot", profiles.pot, { material: spec.materials.pot, tone: spec.tones.pot });
  const sheet = revolveSolid(fine ? "th-soil" : "soil", "pot", soilProfile, { material: spec.materials.soil, tone: spec.tones.soil, sheet: true });
  const body = fine ? [saucer, pot, revolveSolid("compost", "pot", spec.fill, { material: spec.materials.soil, tone: spec.tones.soil })] : [saucer, pot, sheet];
  const thermal = [{ id: "th-saucer", of: "saucer", region: "saucer" }, { id: "th-pot", of: "pot", region: "pot-wall" }, fine ? { ...sheet, region: "soil-surface" } : { id: "th-soil", of: "soil", region: "soil-surface" }];
  return { placed, plan, body, thermal };

}

/** 🏗️ The pot's structure solids of one scale: the pot member (by reference), the support pad on the floor and the mass marker of pot, saucer and soil. */
export function potStructure(spec: PotSpec, scale: Scale): Json[] {
  const f = FEATURE[scale];
  const bottom = spec.shell.find((p) => p[1] > 0)![0];
  const padT = Math.max(bottom + 0.004, f + 0.001);
  const [ri, ro] = [Math.min(...spec.shell.map((p) => p[1])) + 0.008, spec.pot[0]![1] * 0.8];
  const pad = scale <= 100 ? { profile: [[0, ri], [0, ro], [r5(padT), ro], [r5(padT), ri], [0, ri]] } : { profile: [[0, r5(ro)], [r5(padT), r5(ro)]] };
  const markR = Math.max(0.03, f / 2 + 0.002);
  return [
    { id: "m-pot", of: "pot", member: "pot", part: "member" },
    { id: "s-base-support", kind: "revolve", bone: "pot", material: spec.materials.saucer, role: "ground", tone: 0.5, axis: [1, 0, 0], origin: [0, 0, 0], ...pad, member: "pot", part: "support", anchor: "base-support" },
    { id: "k-pot", kind: "ellipsoid", bone: "pot", material: spec.materials.soil, role: "body", tone: 0.5, radii: [r5(markR), r5(markR), r5(markR)], transform: { translation: [r5(spec.soilU * 0.5), 0, 0] }, member: "pot", part: "mass" },
  ];
}

/** 🏷️ The pot materials shared by the three plants: glazed stoneware (ISO 10456 ceramic values) and a moist potting compost. */
export function potMaterials(glaze: Text, glazeAbs: number): Json[] {
  return [
    { id: "glazed-stoneware", title: glaze, emissivity: 0.92, solarAbsorptivity: glazeAbs, densityKgM3: 2300, conductivityWmK: 1.3, specificHeatJKgK: 840, source: "literature", reference: "iso-10456" },
    { id: "potting-compost", title: T("Moist potting compost (bark, coir, perlite)", "Feuchte Topferde (Rinde, Kokos, Perlit)"), emissivity: 0.95, solarAbsorptivity: 0.9, densityKgM3: 650, conductivityWmK: 0.35, specificHeatJKgK: 2000, source: "authored-estimate", reference: "en-13040" },
  ];
}

/** 🧮️ Thermal areas of a pot from its 1:50 profiles, for `areaM2`: the outer wall with rim and the inner wall above the compost (pot), the outer wall, rim and floor ring of the saucer, the compost disc. */
export function potAreas(spec: PotSpec): { pot: number; saucer: number; soil: number } {
  const lateral = (pr: readonly P[]) => pr.reduce((s, p, i) => (i === 0 ? s : s + Math.PI * (p[1] + pr[i - 1]![1]) * Math.hypot(p[0] - pr[i - 1]![0], p[1] - pr[i - 1]![1])), 0);
  return { pot: lateral(spec.pot), saucer: lateral(spec.saucer), soil: Math.PI * spec.soilR ** 2 };
}

