/** 🥾️ Generator of `hiker-with-trekking-poles`: derived from the rig, outlines and volumes of `courier-with-hand-truck` (hand truck, parcels and reflective bands removed),
 * redressed for the mountains and given a 30 litre daypack and two telescopic trekking poles, its own poses and an uphill walk solved so that feet and pole tips stay planted.
 * bun hiker.ts
 * @see ../../📋️brief.md */
import { fitDimensions, pose, loadDoc, saveDoc, pruneBones, dropShapes, dropSolids, pruneUnused, bone, anchor, region, material, t, r, clone, revolvedShape, revolvedSolid, extrudedShape, extrudedSolid, roundedRect, SCALES, type Seg, type J } from "./lib.ts";
import { solve, stateOf } from "./ik.ts";
import { evaluate } from "./lib.ts";

const ID = "hiker-with-trekking-poles";
const doc = loadDoc("courier-with-hand-truck");
const a = doc.asset;
a.id = ID;
pruneBones(a, ["truck-reach"]);
dropShapes(a, (s) => s.id.startsWith("reflective-band"));
dropSolids(a, (s) => s.id?.startsWith("reflective-band") || s.of?.startsWith("reflective-band"));
a.poses = []; a.clips = [];

//#region materials
const mat = (id: string, en: string, de: string, emissivity: number, solarAbsorptivity: number, extra: J = {}, source = "authored-estimate", reference?: string) => ({ id, title: t(en, de), emissivity, solarAbsorptivity, ...extra, source, ...(reference ? { reference } : {}) });
const rename = (from: string, to: string) => {
  const walk = (x: J): void => { if (Array.isArray(x)) x.forEach(walk); else if (x && typeof x === "object") for (const k of Object.keys(x)) { if ((k === "material") && x[k] === from) x[k] = to; else walk(x[k]); } };
  walk(a);
};
rename("hivis-softshell", "softshell"); rename("cotton-workwear", "trekking-trouser"); rename("leather-work-boot", "leather-hiking-boot"); rename("fabric-glove", "liner-glove"); rename("wool-cap", "wool-cap");
const keep = (id: string) => a.materials.find((m: J) => m.id === id);
a.materials = [
  keep("skin"), keep("hair"), keep("cotton-underwear"), keep("cotton-sock"),
  mat("softshell", "Polyester-elastane softshell jacket, moss green", "Softshelljacke aus Polyester-Elasthan, moosgrün", 0.9, 0.72, { specificHeatJKgK: 1300 }),
  keep("fleece-liner"),
  mat("trekking-trouser", "Polyamide-elastane trekking trousers", "Trekkinghose aus Polyamid-Elasthan", 0.9, 0.7, { specificHeatJKgK: 1400 }),
  mat("leather-hiking-boot", "Nubuck leather hiking boots with rubber lug sole", "Wanderschuhe aus Nubukleder mit Profilsohle aus Gummi", 0.9, 0.78, { specificHeatJKgK: 1500 }),
  mat("liner-glove", "Thin polyester liner gloves", "Dünne Unterziehhandschuhe aus Polyester", 0.9, 0.75, { specificHeatJKgK: 1300 }),
  keep("wool-cap"),
  mat("pack-nylon", "Coated polyamide ripstop of the daypack", "Beschichtetes Polyamid-Ripstop des Tagesrucksacks", 0.88, 0.75, { densityKgM3: 1140, specificHeatJKgK: 1700 }),
  mat("pole-aluminium", "Anodised aluminium alloy 7075 pole tube", "Eloxiertes Aluminiumrohr (Legierung 7075) des Stocks", 0.8, 0.6, { densityKgM3: 2810, conductivityWmK: 130, specificHeatJKgK: 960, elasticModulusPa: 71.7e9, strengthPa: 503e6 }, "literature", "asm-7075"),
  mat("cork-grip", "Cork and EVA foam grip", "Griff aus Kork und EVA-Schaum", 0.9, 0.6, { densityKgM3: 240, conductivityWmK: 0.05, specificHeatJKgK: 1900 }),
  mat("carbide-tip", "Tungsten carbide tip and plastic basket", "Hartmetallspitze und Kunststoffteller", 0.6, 0.6, { densityKgM3: 15600, specificHeatJKgK: 300 }),
  keep("bone"),
].filter(Boolean);
for (const id of ["wool-cap"]) Object.assign(keep(id), { title: t("Knitted wool cap with peak", "Strickmütze aus Wolle mit Schirm") });
//#endregion

//#region rig additions
const POLE_L = 1.1;
const PACK = { u0: -0.005, u1: 0.5, v0: 0.118, half: 0.15 };
a.rig.bones.push(
  { id: "daypack", parent: "torso", title: t("Daypack", "Tagesrucksack"), x: 0, y: 0, angleDeg: 0, lengthM: 0.5, joint: { type: "fixed" }, mass: { kg: 7, comX: 0.22, comY: 0.215, distribution: "point", source: "authored-estimate" } },
);
for (const side of ["l", "r"] as const) {
  a.rig.bones.push({ id: `pole-${side}`, parent: `hand-${side}`, title: t(side === "l" ? "Left trekking pole" : "Right trekking pole", side === "l" ? "Linker Trekkingstock" : "Rechter Trekkingstock"), x: 0.05, y: 0.065, angleDeg: -90, lengthM: POLE_L, joint: { type: "revolute", minDeg: -70, maxDeg: 70 }, mass: { kg: 0.255, comX: 0.42, comY: 0, distribution: "point", source: "authored-estimate" }, section: { areaM2: 4.8e-5, secondMomentM4: 1.79e-9, extremeFibreM: 0.009, material: "pole-aluminium", title: t("Upper tube 18 x 0.9 mm", "Oberes Rohr 18 x 0,9 mm") } });
  a.anchors.push({ id: `pole-tip-${side}`, bone: `pole-${side}`, x: POLE_L, y: 0, kind: "contact", title: t(side === "l" ? "Left pole tip" : "Right pole tip", side === "l" ? "Linke Stockspitze" : "Rechte Stockspitze"), support: "contact", normalDeg: 90, footprintLengthM: 0.012, footprintWidthM: 0.012, frictionCoefficient: 0.7 });
  Object.assign(anchor(a, `grip-${side}`), { x: 0.05, y: 0, title: t(side === "l" ? "Left fist on the pole grip" : "Right fist on the pole grip", side === "l" ? "Linke Faust am Stockgriff" : "Rechte Faust am Stockgriff") });
}
//#endregion

//#region drawings and volumes of the props
const POLE_SEGS: Record<string, Seg[]> = {
  "50": [
    { id: "grip", u0: 0, u1: 0.13, r0: 0.0165, r1: 0.0155, region: "poles", material: "cork-grip", tone: 0.18 },
    { id: "upper-tube", u0: 0.13, u1: 0.6, r0: 0.009, region: "poles", material: "pole-aluminium", tone: 0.55 },
    { id: "lock-upper", u0: 0.6, u1: 0.632, r0: 0.0125, region: "poles", material: "pole-aluminium", tone: 0.3 },
    { id: "middle-tube", u0: 0.632, u1: 0.98, r0: 0.008, region: "poles", material: "pole-aluminium", tone: 0.55 },
    { id: "lock-lower", u0: 0.98, u1: 1.008, r0: 0.0115, region: "poles", material: "pole-aluminium", tone: 0.3 },
    { id: "lower-tube", u0: 1.008, u1: 1.09, r0: 0.007, region: "poles", material: "pole-aluminium", tone: 0.55 },
    { id: "basket", u0: 1.07, u1: 1.082, r0: 0.028, region: "poles", material: "carbide-tip", tone: 0.2 },
    { id: "tip", u0: 1.09, u1: 1.15, r0: 0.0045, r1: 0.004, region: "poles", material: "carbide-tip", tone: 0.1 },
  ],
  "100": [
    { id: "grip", u0: 0, u1: 0.13, r0: 0.0165, region: "poles", material: "cork-grip", tone: 0.18 },
    { id: "tube", u0: 0.13, u1: 1.07, r0: 0.0075, region: "poles", material: "pole-aluminium", tone: 0.55, penMm: 0.1 },
    { id: "basket", u0: 1.068, u1: 1.083, r0: 0.028, region: "poles", material: "carbide-tip", tone: 0.2 },
    { id: "tip", u0: 1.083, u1: 1.15, r0: 0.0075, region: "poles", material: "carbide-tip", tone: 0.1, penMm: 0.1 },
  ],
  "200": [
    { id: "grip", u0: 0, u1: 0.13, r0: 0.0165, region: "poles", material: "cork-grip", tone: 0.18 },
    { id: "tube", u0: 0.13, u1: 1.15, r0: 0.012, region: "poles", material: "pole-aluminium", tone: 0.55, penMm: 0.1 },
  ],
  "500": [
    { id: "tube", u0: 0, u1: 1.15, r0: 0.0375, region: "poles", material: "pole-aluminium", tone: 0.55, penMm: 0.13 },
  ],
};
const stretch = (u: number) => (u <= 0.13 ? u : r(0.13 + ((u - 0.13) * (POLE_L - 0.13)) / 1.02, 4));
for (const segs of Object.values(POLE_SEGS)) for (const sg of segs) { sg.u0 = stretch(sg.u0); sg.u1 = stretch(sg.u1); }
const TRUE_R: Record<string, number> = { "upper-tube": 0.009, "middle-tube": 0.008, "lower-tube": 0.007, tube: 0.008, grip: 0.016 };
const ZORDER = { elevation: { l: 0.65, r: 6.1 }, front: { l: 4.57, r: 4.57 }, plan: { l: 3.09, r: 3.09 } } as const;

const PACK_BODY = [[0.0, 0.118], [0.0, 0.27], [0.03, 0.305], [0.1, 0.316], [0.4, 0.292], [0.445, 0.262], [0.455, 0.118]];
const PACK_LID = [[0.4, 0.125], [0.425, 0.296], [0.47, 0.304], [0.505, 0.272], [0.517, 0.205], [0.505, 0.128]];
const PACK_ONE = [[0.0, 0.118], [0.0, 0.27], [0.03, 0.305], [0.1, 0.316], [0.42, 0.298], [0.49, 0.29], [0.512, 0.235], [0.505, 0.128]];
const PACK_POCKET = [[0.07, 0.3], [0.07, 0.334], [0.12, 0.338], [0.3, 0.322], [0.33, 0.3]];
const STRAP_PATH = [[0.44, 0.128], [0.474, 0.092], [0.493, 0.01], [0.474, -0.07], [0.41, -0.118], [0.3, -0.138], [0.16, -0.128]];
const offsetBand = (path: number[][], half: number): number[][] => {
  const n = path.length; const left: number[][] = []; const right: number[][] = [];
  for (let i = 0; i < n; i++) {
    const p = path[Math.max(0, i - 1)]!, q = path[Math.min(n - 1, i + 1)]!;
    const dx = q[0]! - p[0]!, dy = q[1]! - p[1]!, l = Math.hypot(dx, dy);
    const nx = -dy / l, ny = dx / l;
    left.push([path[i]![0]! + nx * half, path[i]![1]! + ny * half]); right.push([path[i]![0]! - nx * half, path[i]![1]! - ny * half]);
  }
  return [...left, ...right.reverse()];
};
const packExtra = (tone: number) => ({ tone, region: "daypack", material: "pack-nylon" });

for (const k of SCALES) {
  const rep = a.representations[k]; const vol = a.volumes[k];
  const g = rep.geometry;
  for (const side of ["l", "r"] as const) {
    const b = `pole-${side}`;
    for (const s of POLE_SEGS[k]!) {
      const seg = { ...s, id: `pole-${side}-${s.id}` };
      g.elevation.push(revolvedShape(b, seg, ZORDER.elevation[side] + (s.id === "grip" ? 0.02 : 0)));
      g.front.push(revolvedShape(b, seg, ZORDER.front[side]));
      g.plan.push(revolvedShape(b, seg, ZORDER.plan[side]));
      const trueR = TRUE_R[s.id];
      const solid = trueR !== undefined && s.id !== "grip" && s.id !== "lower-tube" ? { id: seg.id, kind: "capsule", bone: b, material: s.material, role: "body", tone: s.tone, p0: [r(s.u0 + trueR, 5), 0, 0], p1: [r(s.u1 - trueR, 5), 0, 0], r0: trueR, r1: trueR, slender: true } : revolvedSolid(b, seg, false);
      vol.body.push(solid);
      vol.thermal.push({ id: `th-${seg.id}`, of: seg.id, region: "poles" });
      if (["grip", "upper-tube", "middle-tube", "lower-tube", "tube"].includes(s.id)) vol.structure.push({ id: `member-${seg.id}`, of: seg.id, member: b, part: "member" });
    }
    vol.structure.push({ id: `mass-pole-${side}`, kind: "ellipsoid", bone: b, material: "bone", role: "detail", tone: 0.2, radii: [0.008, 0.008, 0.008].map((x) => x * ({ "50": 1, "100": 1.5, "200": 2, "500": 4.7 } as J)[k]), transform: { translation: [0.42, 0, 0] }, member: b, part: "mass" });
  }
  const parts: [string, number[][], number, number, number, number][] = k === "50" ? [["daypack-body", PACK_BODY, -0.15, 0.15, 0.3, 3.9], ["daypack-lid", PACK_LID, -0.13, 0.13, 0.24, 3.92], ["daypack-pocket", PACK_POCKET, -0.1, 0.1, 0.36, 3.93]]
    : k === "100" ? [["daypack-body", PACK_BODY, -0.15, 0.15, 0.3, 3.9], ["daypack-lid", PACK_LID, -0.13, 0.13, 0.24, 3.92]]
    : [["daypack", PACK_ONE, k === "500" ? -0.125 : -0.15, k === "500" ? 0.125 : 0.15, 0.3, 3.9]];
  for (const [id, pts, z0, z1, tone, zo] of parts) {
    g.elevation.push(extrudedShape(id, "daypack", pts, z0, z1, zo, packExtra(tone)));
    g.front.push(extrudedShape(id, "daypack", pts, z0, z1, 2 + (zo - 3.9), packExtra(tone)));
    g.plan.push(extrudedShape(id, "daypack", pts, z0, z1, zo + 0.07, packExtra(tone)));
    vol.body.push(extrudedSolid(id, "daypack", pts, z0, z1, { material: "pack-nylon", tone }));
    vol.thermal.push({ id: `th-${id}`, of: id, region: "daypack" });
  }
  vol.structure.push({ id: "member-daypack", of: parts[0]![0], member: "daypack", part: "member" });
  vol.structure.push({ id: "mass-daypack", kind: "ellipsoid", bone: "daypack", material: "bone", role: "detail", tone: 0.2, radii: [1, 1, 1].map(() => ({ "50": 0.03, "100": 0.035, "200": 0.04, "500": 0.075 } as J)[k]), transform: { translation: [0.22, 0.215, 0] }, member: "daypack", part: "mass" });
  if (k === "50" || k === "100") {
    for (const side of [1, -1]) {
      const zc = 0.1 * side, s = side > 0 ? "l" : "r";
      g.front.push({ id: `shoulder-strap-${s}`, bone: "torso", role: "detail", z: 4.2, filled: true, tone: 0.22, region: "daypack", material: "pack-nylon", kind: "poly", points: [[zc - 0.022 * side, 1.505], [zc + 0.022 * side, 1.505], [zc + 0.03 * side, 1.18], [zc - 0.016 * side, 1.18]].map(([x, y]) => [r(x!, 4), r(y!, 4)]), closed: true });
    }
    if (k === "50") g.front.push({ id: "sternum-strap", bone: "torso", role: "detail", z: 4.22, filled: true, tone: 0.22, region: "daypack", material: "pack-nylon", kind: "poly", points: [[-0.105, 1.34], [0.105, 1.34], [0.105, 1.358], [-0.105, 1.358]], closed: true });
  }
  if (k === "50") {
    g.elevation.push({ id: "shoulder-strap-r", bone: "torso", role: "body", z: 4.2, filled: true, tone: 0.22, region: "daypack", material: "pack-nylon", kind: "poly", points: offsetBand(STRAP_PATH, 0.007).map(([x, y]) => [r(x!, 4), r(y!, 4)]), closed: true });
    for (const side of [1, -1]) {
      const s = side > 0 ? "l" : "r";
      for (let i = 0; i + 1 < STRAP_PATH.length; i++) {
        const [p, q] = [STRAP_PATH[i]!, STRAP_PATH[i + 1]!];
        const zc = 0.1 * side;
        vol.body.push({ id: `shoulder-strap-${s}-${i}`, kind: "capsule", bone: "torso", material: "pack-nylon", role: "body", tone: 0.22, p0: [p[0], p[1], 0], p1: [q[0], q[1], 0], r0: 0.0055, r1: 0.0055, transform: { scale: [1, 1, 4], translation: [0, 0, zc] } });
        vol.thermal.push({ id: `th-shoulder-strap-${s}-${i}`, of: `shoulder-strap-${s}-${i}`, region: "daypack" });
      }
    }
  }
}
//#endregion

//#region thermal model
const th = a.thermal;
th.environment = { airTempC: 9, meanRadiantTempC: 9, skyTempC: -8, groundTempC: 11, relativeHumidity: 0.65, windMS: 3, windDirectionDeg: 180, solarWM2: 550, diffuseFraction: 0.3, pressurePa: 90000 };
th.baseMet = 1.4;
Object.assign(th.body, { basalMetabolismW: 84.1, source: "authored-estimate", reference: "mifflin-1990" });
const lay = (rg: string, id: string, patch: J) => Object.assign(region(a, rg).layers.find((l: J) => l.id === id), patch);
Object.assign(region(a, "jacket"), { title: t("Merino shirt, fleece and softshell jacket on trunk and arms", "Merinoshirt, Fleece und Softshelljacke an Rumpf und Armen") });
lay("jacket", "shirt", { title: t("Merino wool long-sleeve base layer", "Langarm-Funktionsshirt aus Merinowolle") });
lay("jacket", "softshell", { title: t("Softshell jacket", "Softshelljacke") });
Object.assign(region(a, "trousers"), { title: t("Trekking trousers over the legs", "Trekkinghose über den Beinen") });
for (const k of SCALES) for (const x of a.volumes[k].thermal) if (x.layer === "workwear") x.layer = "trekking";
lay("trousers", "workwear", { id: "trekking", title: t("Polyamide-elastane trekking trousers", "Trekkinghose aus Polyamid-Elasthan"), resistanceM2KW: 0.0465, vapourResistanceM2PaW: 7.4 });
Object.assign(region(a, "boots"), { title: t("Socks and leather hiking boots", "Socken und Wanderschuhe aus Leder") });
lay("boots", "socks", { title: t("Wool-blend hiking socks", "Wandersocken aus Wollmischung"), resistanceM2KW: 0.0124 });
lay("boots", "boot", { title: t("Nubuck hiking boot, ankle high", "Knöchelhoher Wanderschuh aus Nubukleder"), resistanceM2KW: 0.0124 });
Object.assign(region(a, "gloves"), { title: t("Hands in liner gloves", "Hände in Unterziehhandschuhen") });
lay("gloves", "glove", { title: t("Thin liner glove", "Dünner Unterziehhandschuh"), resistanceM2KW: 0.0093, vapourResistanceM2PaW: 1.5 });
th.regions.push(
  { id: "daypack", title: t("Daypack with lid and shoulder straps", "Tagesrucksack mit Deckel und Schultergurten"), kind: "surface", model: "passive", material: "pack-nylon", areaM2: 0.62, characteristicLengthM: 0.35, geometry: "vertical-plate", skyViewFactor: 0.5, layers: [{ id: "pack-shell", title: t("Coated ripstop over the contents", "Beschichtetes Ripstop über dem Inhalt"), kind: "polymer", material: "pack-nylon", thicknessM: 0.0006, resistanceM2KW: 0.05, heatCapacityJM2K: 3600, source: "authored-estimate" }], heatCapacityJM2K: 3600, source: "authored-estimate", reference: "incropera-2011" },
  { id: "poles", title: t("Two trekking poles with grips, baskets and tips", "Zwei Trekkingstöcke mit Griffen, Tellern und Spitzen"), kind: "surface", model: "passive", material: "pole-aluminium", areaM2: 0.13, characteristicLengthM: 0.017, geometry: "cylinder", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 2400, source: "authored-estimate", reference: "incropera-2011" },
);
th.respiration.patterns = [
  { ...th.respiration.patterns[0], title: t("Standing at a rest stop", "Stehen bei einer Rast") },
  { id: "hiking-uphill", title: t("Climbing at a steady pace", "Gleichmäßiges Bergaufgehen"), rateBpm: 26, tidalVolumeL: 1.6, inhaleFraction: 0.45, pauseAfterInhaleFraction: 0.02, exhaleFraction: 0.5, pauseAfterExhaleFraction: 0.03, route: "nose-and-mouth", source: "authored-estimate", reference: "icrp-66-1994" },
  { id: "recovering", title: t("Catching breath on the poles", "Auf den Stöcken verschnaufen"), rateBpm: 22, tidalVolumeL: 1.3, inhaleFraction: 0.42, pauseAfterInhaleFraction: 0.02, exhaleFraction: 0.5, pauseAfterExhaleFraction: 0.06, route: "mouth", source: "authored-estimate", reference: "icrp-66-1994" },
];
th.respiration.motion = [{ bone: "torso", angleDeg: 0.25, dy: 0.002 }];
th.assumptions = [
  t("Skin area 2.034 m2 after DuBois and DuBois (1916) for 82 kg and 1.82 m; skin fractions 0.07 face and neck, 0.03 scalp under the cap, 0.05 hands, 0.49 trunk and arms, 0.30 legs, 0.06 feet (rule-of-nines proportions, authored).", "Hautfläche 2,034 m2 nach DuBois und DuBois (1916) für 82 kg und 1,82 m; Hautanteile 0,07 Gesicht und Hals, 0,03 Kopfhaut unter der Mütze, 0,05 Hände, 0,49 Rumpf und Arme, 0,30 Beine, 0,06 Füße (Neunerregel-Proportionen, geschätzt)."),
  t("Basal metabolism from the Mifflin-St Jeor equation (45 years, 82 kg, 182 cm): 84.1 W. Clothing in clo after ISO 9920 (1 clo = 0.155 m2K/W): merino base layer, fleece and softshell about 1.0 clo on the trunk, trekking trousers 0.35 clo, values authored.", "Grundumsatz aus der Mifflin-St-Jeor-Gleichung (45 Jahre, 82 kg, 182 cm): 84,1 W. Kleidung in clo nach ISO 9920 (1 clo = 0,155 m2K/W): Merino-Unterwäsche, Fleece und Softshell etwa 1,0 clo am Rumpf, Trekkinghose 0,35 clo, Werte geschätzt."),
  t("Activity after the 2011 Compendium of Physical Activities: hiking cross country is 6.0 MET (code 17080), uphill walking at 6 to 15 % grade 8.0 MET (code 17211); the steady climb at 0.83 m/s with a 7 kg pack is set to 7.0 MET in between, the stop on the poles to 2.0 MET while the breath recovers and standing with the pack to 1.4 MET (ISO 8996, standing relaxed 1.2 MET plus the load, authored).", "Aktivität nach dem Compendium of Physical Activities 2011: Wandern im Gelände 6,0 MET (Code 17080), Bergaufgehen mit 6 bis 15 % Steigung 8,0 MET (Code 17211); der gleichmäßige Aufstieg mit 0,83 m/s und 7 kg Rucksack liegt mit 7,0 MET dazwischen, das Verschnaufen auf den Stöcken mit 2,0 MET, das Stehen mit Rucksack mit 1,4 MET (ISO 8996, entspanntes Stehen 1,2 MET plus Last, geschätzt)."),
  t("External work of the climb is the lift of 89 kg at 0.83 m/s on a 12 % grade, about 87 W; the ground of the archive is level, so the gait carries the slope only in posture and timing. The daypack and the poles are passive surfaces; the pack contents (water, clothing, food) are lumped into its heat capacity.", "Äußere Arbeit des Aufstiegs ist das Heben von 89 kg mit 0,83 m/s auf 12 % Steigung, etwa 87 W; der Boden des Archivs ist eben, der Gang zeigt die Steigung nur in Haltung und Takt. Rucksack und Stöcke sind passive Flächen; der Rucksackinhalt (Wasser, Kleidung, Proviant) steckt in seiner Wärmekapazität."),
];
a.provenance.references.push(
  { id: "ainsworth-2011", citation: "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581 (codes 17080, 17211)." },
  { id: "asm-7075", citation: "ASM Handbook Vol. 2 (1990). Properties and Selection: Nonferrous Alloys. Aluminium alloy 7075-T6: density 2810 kg/m3, E 71.7 GPa, tensile strength 572 MPa, yield 503 MPa." },
  { id: "incropera-2011", citation: "Incropera, F. P., DeWitt, D. P. et al. (2011). Fundamentals of Heat and Mass Transfer, 7th ed., Wiley (material properties, convection correlations)." },
  { id: "schwameder-1999", citation: "Schwameder, H., Roithner, R., Mueller, E., Niessen, W. and Raschner, C. (1999). Knee joint forces during downhill walking with hiking poles. Journal of Sports Sciences 17(12), 969-978 (pole use, pole length and the share of load the poles take)." },
);
a.provenance.references = a.provenance.references.filter((x: J) => x.id !== "fetra-b1125l");
//#endregion

//#region poses
const FEET = ["heel-l", "ball-l", "heel-r", "ball-r"];
const REGIONS = ["face-skin", "cap", "gloves", "jacket", "trousers", "boots", "daypack", "poles"];
const binding = (met: number, breathing: string, plume: number, sun: number, feet: number) => ({
  activityMet: met, breathing,
  regions: Object.fromEntries(REGIONS.map((id) => [id, { exposedFraction: id === "jacket" ? 0.84 : id === "daypack" ? 0.86 : 1, sunlitFraction: r(sun * ({ "face-skin": 0.55, cap: 0.8, gloves: 0.5, jacket: 0.45, trousers: 0.4, boots: 0.35, daypack: 0.5, poles: 0.4 } as J)[id], 3), ...(id === "boots" && feet > 0 ? { contact: { fraction: feet, tempC: 11 } } : {}) }])),
  emitters: { "body-plume": plume },
});
const mkPose = (id: string, title: J, description: J, bones: J, contacts: string[], thermal: J, key?: boolean) => ({ id, title, description, ...(key ? { key: true } : {}), bones, ground: "auto", contacts, thermal, loads: [] });
const LEGS_STAND = { "thigh-l": { angleDeg: 6.6956 }, "shank-l": { angleDeg: -16.2204 }, "foot-l": { angleDeg: 9.5248 }, "toe-l": { angleDeg: 0 }, "thigh-r": { angleDeg: 6.6956 }, "shank-r": { angleDeg: -16.2204 }, "foot-r": { angleDeg: 9.5248 }, "toe-r": { angleDeg: 0 } };
/** 🦯️ Solves shoulder flexion and pole angle of both sides so that the pole tips reach (x, ground). */
function plantPoles(bones: J, contacts: string[], tipX: { l: number; r: number }, sides: ("l" | "r")[] = ["l", "r"], tipY = 0.0015): J {
  let out = bones;
  for (const s of sides) {
    const sol = solve(a, out, contacts, [{ bone: `arm-${s}`, field: "angleDeg" }, { bone: `pole-${s}`, field: "angleDeg" }], [{ kind: "tip", bone: `pole-${s}`, x: tipX[s], y: tipY }]);
    if (sol.error > 1e-3) console.log(`[DEBUG] plantPoles ${s} error ${sol.error}`);
    out = sol.bones;
  }
  return out;
}
const ABD = { l: -Number(process.env.ABD ?? 4), r: Number(process.env.ABD ?? 4) };
const standBones = plantPoles({ ...LEGS_STAND, torso: { angleDeg: -4 }, neck: { angleDeg: 3 }, head: { angleDeg: 0 }, "arm-l": { angleDeg: 20, abductionDeg: ABD.l }, "forearm-l": { angleDeg: 62 }, "hand-l": { angleDeg: 8 }, "arm-r": { angleDeg: 20, abductionDeg: ABD.r }, "forearm-r": { angleDeg: 62 }, "hand-r": { angleDeg: 8 }, "pole-l": { angleDeg: 0 }, "pole-r": { angleDeg: 0 } }, FEET, { l: 0.36, r: 0.36 });
const restBones = plantPoles({ ...LEGS_STAND, "thigh-l": { angleDeg: 14 }, "shank-l": { angleDeg: -24 }, "foot-l": { angleDeg: 10 }, "thigh-r": { angleDeg: 14 }, "shank-r": { angleDeg: -24 }, "foot-r": { angleDeg: 10 }, torso: { angleDeg: -26 }, neck: { angleDeg: 14 }, head: { angleDeg: 9 }, "arm-l": { angleDeg: 50 }, "forearm-l": { angleDeg: 25 }, "hand-l": { angleDeg: 10 }, "arm-r": { angleDeg: 50 }, "forearm-r": { angleDeg: 25 }, "hand-r": { angleDeg: 10 }, "pole-l": { angleDeg: 0 }, "pole-r": { angleDeg: 0 } }, FEET, { l: 0.78, r: 0.78 });
const ALL = [...FEET, "pole-tip-l", "pole-tip-r"];
a.poses.push(
  mkPose("standing-with-poles", t("Standing with poles", "Stehen mit Stöcken"), t("A rest on the trail: feet together, knees soft, both poles planted a little ahead of the boots, fists on the grips at hip height, the daypack on the back.", "Rast am Weg: Füße beieinander, Knie locker, beide Stöcke etwas vor den Schuhen aufgesetzt, die Fäuste an den Griffen auf Hüfthöhe, der Rucksack auf dem Rücken."), standBones, ALL, binding(1.4, "rest-standing", 0.6, 1, 0.3)),
  mkPose("resting-on-poles", t("Catching breath on the poles", "Auf den Stöcken verschnaufen"), t("After a steep pitch: the trunk leans forward over both poles planted half a step ahead, arms nearly straight, part of the weight on the poles, mouth breathing.", "Nach einem steilen Stück: Der Oberkörper lehnt sich über beide Stöcke, die einen halben Schritt voraus stehen, Arme fast gestreckt, ein Teil des Gewichts auf den Stöcken, Mundatmung."), restBones, ALL, binding(2.0, "recovering", 0.8, 1, 0.3)),
);
//#endregion

//#region uphill walk
const base = loadDoc("courier-with-hand-truck").asset.clips.find((c: J) => c.id === "push-walk");
const STRETCH = 1.15;
const OUT = { l: Number(process.env.OUT ?? 8), r: -Number(process.env.OUT ?? 8) };
const T = r(base.durationS * STRETCH, 4);
const N = base.keys.length - 1;
const bump = (t0: number, t1: number, t: number) => (t > t0 && t < t1 ? Math.sin((Math.PI * (t - t0)) / (t1 - t0)) ** 2 : 0);
const legKeys = base.keys.map((k: J, i: number) => {
  const tb = k.t;
  const bl = bump(0.768, 1.2, tb), br = bump(0.168, 0.6, tb);
  const o: J = {};
  for (const [side, b] of [["l", bl], ["r", br]] as const) {
    o[`thigh-${side}`] = { angleDeg: r(k.bones[`thigh-${side}`].angleDeg + 13 * b, 4) };
    o[`shank-${side}`] = { angleDeg: r(k.bones[`shank-${side}`].angleDeg - 17 * b, 4) };
    o[`foot-${side}`] = { angleDeg: r(k.bones[`foot-${side}`].angleDeg + 6 * b, 4) };
    o[`toe-${side}`] = { angleDeg: k.bones[`toe-${side}`].angleDeg };
  }
  return { t: r((i * T) / N, 5), bones: o };
});
const footSpans = base.contacts.map((c: J) => ({ anchor: c.anchor, fromS: r(c.fromS * STRETCH, 4), toS: r(c.toS * STRETCH, 4) }));
const upper = (i: number) => ({ torso: { angleDeg: -11 }, neck: { angleDeg: 6 }, head: { angleDeg: 3 }, "hand-l": { angleDeg: 8 }, "hand-r": { angleDeg: 8 } });
const elbow = (side: "l" | "r", t: number) => r(60 + 20 * Math.cos((2 * Math.PI * (t - (side === "r" ? 0 : T / 2))) / T), 4);
const strideTitle = t("Uphill stride", "Bergauf-Schritt");
const strideDesc = t("Mid-stance of the uphill walk: the trunk leans 11 degrees into the slope, the left leg carries the body, the right pole pushes beside the left boot while the left pole swings forward with the right leg.", "Mittlere Standphase des Bergaufgehens: Der Oberkörper neigt sich 11 Grad in den Hang, das linke Bein trägt, der rechte Stock schiebt neben dem linken Schuh, während der linke Stock mit dem rechten Bein nach vorn schwingt.");
a.poses.push(mkPose("uphill-stride", strideTitle, strideDesc, {}, ["heel-l", "ball-l", "pole-tip-r"], { ...binding(7.0, "hiking-uphill", 0.9, 1, 0.15), externalWorkW: 87 }));
const walk: J = {
  id: "uphill-walk", title: t("Uphill walk with poles", "Bergaufgehen mit Stöcken"),
  description: t("A steady climb at 0.83 m/s and 87 steps per minute on what would be a 12 % grade: trunk leaning into the slope, higher knee lift in swing, diagonal pole technique (the right pole is planted with the left heel and pushes until the right heel strikes, then the left one); feet and pole tips stay planted without sliding.", "Gleichmäßiger Aufstieg mit 0,83 m/s und 87 Schritten pro Minute, wie auf 12 % Steigung: Oberkörper in den Hang geneigt, höher angehobenes Knie in der Schwungphase, diagonale Stocktechnik (der rechte Stock setzt mit der linken Ferse auf und schiebt bis zum Aufsetzen der rechten Ferse, dann der linke); Füße und Stockspitzen gleiten nicht."),
  durationS: T, fps: 30, loop: true, rootMotion: "plant", advanceM: base.advanceM,
  keys: legKeys.map((k: J, i: number) => ({ t: k.t, pose: "uphill-stride", ease: "smooth", bones: { ...k.bones, ...upper(i), "arm-l": { angleDeg: 0, abductionDeg: OUT.l }, "forearm-l": { angleDeg: elbow("l", k.t) }, "arm-r": { angleDeg: 0, abductionDeg: OUT.r }, "forearm-r": { angleDeg: elbow("r", k.t) }, "pole-l": { angleDeg: 0 }, "pole-r": { angleDeg: 0 } } })),
  contacts: footSpans, phases: base.phases.map((p: J) => ({ ...p, fromS: r(p.fromS * STRETCH, 4), toS: r(p.toS * STRETCH, 4) })),
  events: base.events.map((e: J) => ({ ...e, t: r(e.t * STRETCH, 4) })),
  tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 7 }, { t: T, v: 7 }] }, { target: "breathing", ease: "step", keys: [{ t: 0, v: 1 }, { t: T, v: 1 }] }, { target: "externalWorkW", ease: "smooth", keys: [{ t: 0, v: 87 }, { t: T, v: 87 }] }, { target: "emitter:body-plume", ease: "smooth", keys: [{ t: 0, v: 0.9 }, { t: T, v: 0.9 }] }],
};
a.clips.push(walk);
const rootAt = (time: number) => evaluate(a, { kind: "clip", clip: "uphill-walk", timeS: time }).root.x;
const rootEnd = rootAt(T - 1e-4) + (rootAt(T - 1e-4) - rootAt(T - 2e-4));
const ROOT = walk.keys.map((k: J, i: number) => (i === N ? rootEnd : rootAt(k.t)));
const contactsAt = (time: number) => [...new Set(footSpans.filter((c: J) => time >= c.fromS && time < c.toS).map((c: J) => c.anchor as string))];
const DPLANT = 0.24;
const LAG = 4;
const plant = { r: { from: LAG, to: N / 2 - 2, x: ROOT[LAG] + DPLANT }, l: { from: N / 2 + LAG, to: N - 2, x: ROOT[N / 2 + LAG] + DPLANT } };
const arm: Record<string, number[]> = { l: [], r: [] };
const fore: Record<string, number[]> = { l: [], r: [] };
const pole: Record<string, number[]> = { l: [], r: [] };
const SWING = Number(process.env.SWING ?? 0);
const LIFT = Number(process.env.LIFT ?? 0.06);
const WRIST = [Number(process.env.W0 ?? 0.32), Number(process.env.W1 ?? 0.04)];
for (const side of ["r", "l"] as const) {
  const w = plant[side];
  for (let i = w.from; i <= w.to; i++) {
    const k = walk.keys[i]; const sp = (i - w.from) / (w.to - w.from);
    const phi = i > w.from ? pole[side][i - 1]! : 0;
    const bones = { ...k.bones, [`arm-${side}`]: { angleDeg: i > w.from ? arm[side][i - 1]! : 20, abductionDeg: OUT[side] }, [`forearm-${side}`]: { angleDeg: i > w.from ? fore[side][i - 1]! : 60 }, [`pole-${side}`]: { angleDeg: phi } };
    const wrist = WRIST[0]! + (WRIST[1]! - WRIST[0]!) * (1 - Math.cos(Math.PI * sp)) / 2;
    const sol = solve(a, bones, contactsAt(k.t), [{ bone: `arm-${side}`, field: "angleDeg" }, { bone: `forearm-${side}`, field: "angleDeg" }, { bone: `pole-${side}`, field: "angleDeg" }], [{ kind: "tip", bone: `pole-${side}`, x: w.x - ROOT[i], y: 0.0015 }, { kind: "tip", bone: `forearm-${side}`, x: wrist }]);
    if (sol.error > 1e-3) console.log(`[DEBUG] plant ${side} key ${i} error ${sol.error.toFixed(4)}`);
    arm[side][i] = sol.bones[`arm-${side}`].angleDeg; fore[side][i] = sol.bones[`forearm-${side}`].angleDeg; pole[side][i] = sol.bones[`pole-${side}`].angleDeg;
    arm[side][i % N] = arm[side][i]!; fore[side][i % N] = fore[side][i]!; pole[side][i % N] = pole[side][i]!;
  }
}
const ADV = rootEnd - ROOT[0]!;
for (const side of ["r", "l"] as const) {
  const w = plant[side];
  const s0 = w.to, s1 = w.from + N;
  for (let j = s0 + 1; j < s1; j++) {
    const i = j % N; const sp = (j - s0) / (s1 - s0); const e = (1 - Math.cos(Math.PI * sp)) / 2;
    const lerp = (q: Record<string, number[]>) => r(q[side][s0 % N]! + (q[side][w.from]! - q[side][s0 % N]!) * e, 4);
    fore[side][i] = r(lerp(fore) + 10 * Math.sin(Math.PI * sp), 4);
    const k = walk.keys[i];
    const rootNow = j >= N ? ROOT[i]! + ADV : ROOT[i]!;
    const tipX = w.x + ADV * e - rootNow;
    const bones = { ...k.bones, [`arm-${side}`]: { angleDeg: j > s0 + 1 ? arm[side][(j - 1) % N]! : arm[side][s0 % N]!, abductionDeg: OUT[side] }, [`forearm-${side}`]: { angleDeg: fore[side][i] }, [`pole-${side}`]: { angleDeg: j > s0 + 1 ? pole[side][(j - 1) % N]! : pole[side][s0 % N]! } };
    const sol = solve(a, bones, contactsAt(k.t), [{ bone: `arm-${side}`, field: "angleDeg" }, { bone: `pole-${side}`, field: "angleDeg" }], [{ kind: "tip", bone: `pole-${side}`, x: tipX, y: 0.0015 + LIFT * Math.sin(Math.PI * sp) }]);
    if (sol.error > 2e-3) console.log(`[DEBUG] swing ${side} key ${i} error ${sol.error.toFixed(4)}`);
    arm[side][i] = sol.bones[`arm-${side}`].angleDeg; pole[side][i] = sol.bones[`pole-${side}`].angleDeg;
  }
}
walk.keys.forEach((k: J, i: number) => { for (const side of ["l", "r"]) { k.bones[`arm-${side}`] = { angleDeg: arm[side][i % N], abductionDeg: OUT[side as "l" | "r"] }; k.bones[`forearm-${side}`] = { angleDeg: fore[side][i % N] }; k.bones[`pole-${side}`] = { angleDeg: pole[side][i % N] }; } });
walk.keys[N].bones = structuredClone(walk.keys[0].bones);
const tl = walk.keys[LAG].t, tm = walk.keys[N / 2 - 2].t, tr = walk.keys[N / 2 + LAG].t;
walk.contacts = [...footSpans, { anchor: "pole-tip-r", fromS: tl, toS: tm }, { anchor: "pole-tip-l", fromS: tr, toS: walk.keys[N - 2].t }];
walk.events.push({ t: tl, kind: "contact-on", title: t("Right pole planted", "Rechter Stock setzt auf"), target: "pole-tip-r" }, { t: tm, kind: "contact-off", title: t("Right pole lifts", "Rechter Stock hebt ab"), target: "pole-tip-r" }, { t: tr, kind: "contact-on", title: t("Left pole planted", "Linker Stock setzt auf"), target: "pole-tip-l" }, { t: walk.keys[N - 2].t, kind: "contact-off", title: t("Left pole lifts", "Linker Stock hebt ab"), target: "pole-tip-l" });
walk.events.sort((p: J, q: J) => p.t - q.t);
const STRIDE_KEY = 8;
pose(a, "uphill-stride").bones = structuredClone(walk.keys[STRIDE_KEY].bones);
pose(a, "uphill-stride").contacts = [...contactsAt(walk.keys[STRIDE_KEY].t), "pole-tip-r"];
console.log("[DEBUG] arms", arm.r.map((x) => x.toFixed(1)).join(" "), "\n[DEBUG] fore", fore.r.map((x) => x.toFixed(1)).join(" "), "\n[DEBUG] poles", pole.r.map((x) => x.toFixed(1)).join(" "));
for (let i = 0; i < N; i++) { const st = evaluate(a, { kind: "clip", clip: "uphill-walk", timeS: walk.keys[i].t }); const lo = Math.min(...["l", "r"].map((s) => st.bones[`pole-${s}`].tip.y)); if (lo < -0.002) console.log(`[DEBUG] key ${i} pole tip below ground ${lo.toFixed(4)}`); }
//#endregion

//#region idle clip, presentation, save
a.clips.push({
  id: "idle-breathing", title: t("Idle breathing at a rest stop", "Ruhiges Atmen bei der Rast"), description: t("Standing on both poles at a rest stop, the chest and shoulders rising with each breath, the head turning a little to look along the trail.", "Stehen auf beiden Stöcken bei einer Rast, Brustkorb und Schultern heben sich mit jedem Atemzug, der Kopf dreht sich leicht und schaut den Weg entlang."),
  durationS: 4.6, fps: 30, loop: true, rootMotion: "none",
  keys: [{ t: 0, pose: "standing-with-poles", ease: "smooth" }, { t: 2.3, pose: "standing-with-poles", ease: "smooth", bones: { torso: { angleDeg: -3.2 }, neck: { angleDeg: 1.5 }, head: { angleDeg: 2.5 } } }, { t: 4.6, pose: "standing-with-poles", ease: "smooth" }],
  contacts: ALL.map((anchor) => ({ anchor, fromS: 0, toS: 4.6 })), phases: [{ id: "rest-stop", fromS: 0, toS: 4.6, title: t("Rest stop", "Rast") }], events: [],
  tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 4.6, v: 0 }] }, { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.4 }, { t: 4.6, v: 1.4 }] }],
});
//#region 1:500 sleeves generalised: 20 % (upper arm) and 10 % (forearm) fuller so that the arms keep their printable width beside the jacket and the drawn front matches the 1:50 body
const FULL = (bone: string) => (bone.startsWith("arm-") ? 1.2 : bone.startsWith("forearm-") ? 1.1 : 1);
for (const v of ["elevation", "front", "plan"]) for (const sh of a.representations["500"].geometry[v]) if (FULL(sh.bone) !== 1 && sh.solid?.kind === "revolved" && sh.kind === "poly") sh.points = sh.points.map(([u, w]: number[]) => [u, r(w! * FULL(sh.bone), 4)]);
for (const v of ["elevation", "front", "plan"]) for (const sh of a.representations["50"].geometry[v]) if (/^hand-[lr]$/.test(sh.bone) && sh.solid?.kind === "extruded") sh.solid = { ...sh.solid, z0: r(sh.solid.z0 + 0.012, 4), z1: r(sh.solid.z1 - 0.012, 4), zTip: [sh.solid.z0, sh.solid.z1] };
for (const so of a.volumes["500"].body) if (so.bone.startsWith("hand-") && so.kind === "ellipsoid") so.radii = [r(so.radii[0] * 1.1, 4), so.radii[1], r(so.radii[2] * 1.1, 4)];
for (const so of a.volumes["500"].body) if (FULL(so.bone) !== 1 && so.kind === "loft") for (const sec of so.sections) { sec.rx = r(sec.rx * FULL(so.bone), 4); sec.ry = r(sec.ry * FULL(so.bone), 4); }
//#endregion
//#region texts
a.title = t("Hiker with Trekking Poles", "Wanderer mit Trekkingstöcken");
a.summary = t("A 45-year-old hiker of 1.82 m in a moss-green softshell jacket, trekking trousers, ankle-high nubuck boots, liner gloves and a peaked wool cap, carrying a 30 litre daypack (7 kg) and walking with two telescopic aluminium trekking poles set to 1.10 m: standing at a rest stop, catching breath on the poles after a steep pitch and a steady uphill walk in diagonal pole technique at 0.83 m/s with feet and pole tips planted; the poles take part of the weight as third and fourth ground contacts.", "Ein 45-jähriger Wanderer von 1,82 m in moosgrüner Softshelljacke, Trekkinghose, knöchelhohen Wanderschuhen aus Nubukleder, Unterziehhandschuhen und Wollmütze mit Schirm, mit 30-Liter-Tagesrucksack (7 kg) und zwei auf 1,10 m eingestellten Teleskop-Trekkingstöcken aus Aluminium: Rast im Stehen, Verschnaufen auf den Stöcken nach einem steilen Stück und gleichmäßiges Bergaufgehen in diagonaler Stocktechnik mit 0,83 m/s, Füße und Stockspitzen ohne Gleiten; die Stöcke tragen als dritter und vierter Bodenkontakt einen Teil des Gewichts.");
a.tags = ["human", "adult", "man", "hiker", "hiking", "trekking-poles", "daypack", "outdoor", "sports-and-leisure", "walking", "uphill"];
a.provenance = { ...a.provenance, author: "Semio staffage human worker HW-08", created: "2026-11-22", notes: t("Rig, outlines and volumes derived by hand from the archive's courier-with-hand-truck (1.82 m, de Leva male fractions): hand truck, parcels and reflective bands removed, garments renamed and retoned for the mountains, a daypack bone on the trunk and two pole bones in the fists added with their own drawings at every scale and their own solids; poses solved with the library's own evaluation so that the pole tips meet the ground; the uphill walk keeps the planted feet of the courier's gait, stretched to 87 steps per minute, with higher knee lift in swing, an 11 degree trunk lean and diagonal pole plants solved key by key. Generator: ticket STAFFAGE-HUMAN-COLLECTION-WAVE, scripts HW-08/hiker.ts.", "Rig, Umrisse und Volumen von Hand aus dem Kurier mit Sackkarre des Archivs abgeleitet (1,82 m, männliche Anteile nach de Leva): Sackkarre, Pakete und Reflexstreifen entfernt, Kleidung für die Berge umbenannt und umgetönt, ein Rucksackknochen am Rumpf und zwei Stockknochen in den Fäusten mit eigenen Zeichnungen in jedem Maßstab und eigenen Volumen ergänzt; die Posen mit der bibliothekseigenen Auswertung gelöst, sodass die Stockspitzen den Boden treffen; das Bergaufgehen übernimmt die ortsfesten Füße des Kuriergangs, auf 87 Schritte pro Minute gedehnt, mit höherem Kniehub im Schwung, 11 Grad Rumpfneigung und diagonalen Stockeinsätzen, Schlüssel für Schlüssel gelöst. Generator: Ticket STAFFAGE-HUMAN-COLLECTION-WAVE, Skripte HW-08/hiker.ts.") };
a.structure.assumptions = [
  t("Segments are point masses at the de Leva centres; the 7 kg daypack is a point mass on its own bone at the centre of its contents and the two 255 g poles are point masses at 0.42 m below the grip top.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva; der 7 kg schwere Rucksack ist eine Punktmasse auf eigenem Knochen im Schwerpunkt seines Inhalts, die beiden 255 g schweren Stöcke sind Punktmassen 0,42 m unter dem Griffende."),
  t("Contacts are heel and ball of each boot and the carbide tip of each pole; a planted pole tip is a unilateral contact with rubber-on-rock friction, its share of the load follows the elastic-support model of the solver (about 5 to 10 % of body weight per pole in the literature on pole walking) and is not a clinical measurement.", "Kontakte sind Ferse und Ballen jedes Schuhs und die Hartmetallspitze jedes Stocks; eine aufgesetzte Stockspitze ist ein einseitiger Kontakt mit Reibung auf Fels, ihr Lastanteil folgt dem Elastizitätsmodell des Lösers (in der Literatur zum Stockgehen etwa 5 bis 10 % des Körpergewichts je Stock) und ist keine klinische Messung."),
];
const DEC: Record<string, J> = {
  "50": {
    silhouette: t("The true side, front and top view of the 1:50 model standing with poles (36.2 mm tall): peaked wool cap, head with nose, chin and ear, stand-up collar, hip-length softshell with sleeves and cuff straps, gloved fists on the cork grips, trekking trousers with thigh pocket, ankle boots with heel, sole and toe cap; the daypack with lid and stretch pocket stands behind the back, the right shoulder strap runs over the shoulder, and both poles reach from the fists to the ground with grip, two locks, three tubes, basket and tip. Trunk, head, legs and boots are outlines bound to their bones; arms, fists, pack and poles are revolved or extruded profiles, so the three views are one definition in every pose.", "Wahre Seiten-, Vorder- und Draufsicht des 1:50-Modells beim Stehen mit Stöcken (36,2 mm hoch): Wollmütze mit Schirm, Kopf mit Nase, Kinn und Ohr, Stehkragen, hüftlange Softshelljacke mit Ärmeln und Ärmelriegeln, behandschuhte Fäuste an den Korkgriffen, Trekkinghose mit Beintasche, knöchelhohe Schuhe mit Absatz, Sohle und Zehenkappe; der Rucksack mit Deckel und Stretchtasche steht hinter dem Rücken, der rechte Schultergurt läuft über die Schulter, und beide Stöcke reichen von den Fäusten bis zum Boden mit Griff, zwei Verschlüssen, drei Rohren, Teller und Spitze. Rumpf, Kopf, Beine und Schuhe sind an ihre Knochen gebundene Umrisse; Arme, Fäuste, Rucksack und Stöcke sind gedrehte oder extrudierte Profile, sodass die drei Ansichten in jeder Pose eine Definition sind."),
    retained: t("Each garment with its own region, cuff straps, thigh pocket, boot shaft, sole and toe cap, hair at the nape, ear, eye, zip, collar line, mouth, cap seam and laces; on the pack the lid, the stretch pocket, both shoulder straps and the sternum strap in front; on each pole the grip, both flick locks, the three tube diameters (18, 16 and 14 mm), the 56 mm basket and the carbide tip.", "Jedes Kleidungsstück mit eigener Region, Ärmelriegel, Beintasche, Schuhschaft, Sohle und Zehenkappe, Haar im Nacken, Ohr, Auge, Reißverschluss, Kragenlinie, Mund, Mützennaht und Schnürung; am Rucksack Deckel, Stretchtasche, beide Schultergurte und vorn der Brustgurt; an jedem Stock Griff, beide Klemmverschlüsse, die drei Rohrdurchmesser (18, 16 und 14 mm), der 56-mm-Teller und die Hartmetallspitze."),
    merged: t("Fingers and thumb merge into one fist outline around the grip; chin, brow and nose belong to the head outline in the side view; the wrist strap of the pole lies inside the fist; the weave is a hatch.", "Finger und Daumen verschmelzen zu einem Faustumriss um den Griff; Kinn, Stirn und Nase gehören in der Seitenansicht zum Kopfumriss; die Handschlaufe liegt in der Faust; das Gewebe ist eine Schraffur."),
    omitted: t("Finger separations, hip belt webbing, compression straps, zip pullers, bottle in the side pocket, labels on the tubes and the tread of the soles; in plan the face and the laces.", "Fingertrennungen, Hüftgurtband, Kompressionsriemen, Reißverschlusszipper, Flasche in der Seitentasche, Aufdrucke auf den Rohren und das Sohlenprofil; im Grundriss Gesicht und Schnürung."),
    thinElements: t("The pole tubes are drawn at their true 14 to 18 mm (0.28 to 0.36 mm), the 12 mm basket as a filled disc, the shoulder straps as filled 44 mm bands; zip, collar line, mouth, cap seam and laces are open strokes.", "Die Stockrohre sind in ihren wahren 14 bis 18 mm gezeichnet (0,28 bis 0,36 mm), der 12 mm dicke Teller als gefüllte Scheibe, die Schultergurte als gefüllte 44-mm-Bänder; Reißverschluss, Kragenlinie, Mund, Mützennaht und Schnürung sind offene Striche."),
    hatching: t("Jacket and sleeves: 45 degree weave lines at 0.7 mm; trousers: vertical twill at 0.6 mm; pack and poles plain.", "Jacke und Ärmel: Gewebelinien unter 45 Grad mit 0,7 mm; Hose: senkrechter Köper mit 0,6 mm; Rucksack und Stöcke ohne Schraffur."),
    styles: t("Body 0.25 mm, detail 0.13 mm, texture 0.10 mm; jacket mid-grey, trousers, cap and pack dark, boots dark with near-black soles, skin light, pole tubes mid-grey with dark grips and locks.", "Körper 0,25 mm, Detail 0,13 mm, Textur 0,10 mm; Jacke mittelgrau, Hose, Mütze und Rucksack dunkel, Schuhe dunkel mit fast schwarzen Sohlen, Haut hell, Stockrohre mittelgrau mit dunklen Griffen und Verschlüssen."),
    overlays: t("One region per shape: jacket, trousers, boots, gloves, cap, face skin, daypack and poles; straps, lid and pocket carry the pack region, grips, locks, basket and tip the pole region, so thermal, structural and transport views read the same surfaces in all three views.", "Eine Region je Form: Jacke, Hose, Schuhe, Handschuhe, Mütze, Gesichtshaut, Rucksack und Stöcke; Gurte, Deckel und Tasche tragen die Rucksackregion, Griffe, Verschlüsse, Teller und Spitze die Stockregion, sodass Thermik-, Tragwerks- und Transportansichten in allen drei Ansichten dieselben Flächen lesen."),
  },
  "100": {
    silhouette: t("The true three views of the 1:100 model (18.1 mm tall): cap with brim, head with nose, jacket with collar, sleeves, mitten fists on the grips, trouser legs, boots with sole and toe cap, the daypack as body and lid behind the back, and each pole as grip, one tube, basket and tip from the fist to the ground.", "Die wahren drei Ansichten des 1:100-Modells (18,1 mm hoch): Mütze mit Schirm, Kopf mit Nase, Jacke mit Kragen, Ärmel, Fäustlinge an den Griffen, Hosenbeine, Schuhe mit Sohle und Zehenkappe, der Rucksack als Körper und Deckel hinter dem Rücken und jeder Stock als Griff, ein Rohr, Teller und Spitze von der Faust bis zum Boden."),
    retained: t("Cap brim, collar, boot sole and toe cap, the pack lid, the two shoulder straps in front, the pole basket and the grip.", "Mützenschirm, Kragen, Schuhsohle und Zehenkappe, der Rucksackdeckel, die beiden Schultergurte vorn, der Stockteller und der Griff."),
    merged: t("The three pole tubes and the locks merge into one 15 mm tube; stretch pocket and pack body into one outline; fingers into the mitten fist.", "Die drei Stockrohre und die Verschlüsse verschmelzen zu einem 15-mm-Rohr; Stretchtasche und Rucksackkörper zu einem Umriss; die Finger zur Fausthandschuhform."),
    omitted: t("Sternum strap, cuff straps, laces, the shoulder strap in the side view, ear and eye details, thigh pocket.", "Brustgurt, Ärmelriegel, Schnürung, der Schultergurt in der Seitenansicht, Ohr- und Augendetails, Beintasche."),
    thinElements: t("The tube is drawn 15 mm wide (0.15 mm) with a 0.10 mm pen, the basket 15 mm thick, the tip as wide as the tube; nothing thinner than the profile's feature remains.", "Das Rohr ist 15 mm breit gezeichnet (0,15 mm) mit 0,10-mm-Stift, der Teller 15 mm dick, die Spitze so breit wie das Rohr; nichts Dünneres als das Profilmerkmal bleibt."),
    hatching: t("Jacket weave at 45 degrees and trouser twill, both at the spacing of the 1:50 drawing in paper; pack and poles plain.", "Jackengewebe unter 45 Grad und Hosenköper, beide im Papierabstand der 1:50-Zeichnung; Rucksack und Stöcke ohne Schraffur."),
    styles: t("Body 0.18 mm, detail 0.1 mm; the same tones as at 1:50.", "Körper 0,18 mm, Detail 0,1 mm; dieselben Töne wie bei 1:50."),
    overlays: t("The same eight regions as at 1:50; the merged tube carries the pole region.", "Dieselben acht Regionen wie bei 1:50; das verschmolzene Rohr trägt die Stockregion."),
  },
  "200": {
    silhouette: t("The true three views of the 1:200 model (9.1 mm tall) in main masses: head under the cap, jacket with collar, sleeve, forearm and fist per arm, thigh, shank and boot per leg, the daypack as one block with its lid, and each pole as grip and one tube.", "Die wahren drei Ansichten des 1:200-Modells (9,1 mm hoch) in Hauptmassen: Kopf unter der Mütze, Jacke mit Kragen, Ärmel, Unterarm und Faust je Arm, Oberschenkel, Unterschenkel und Schuh je Bein, der Rucksack als ein Block mit Deckel und jeder Stock als Griff und ein Rohr."),
    retained: t("The pack depth behind the back that marks the hiker, both poles reaching the ground and the dark grips in the fists.", "Die Rucksacktiefe hinter dem Rücken, die den Wanderer kennzeichnet, beide Stöcke bis zum Boden und die dunklen Griffe in den Fäusten."),
    merged: t("Pack body and lid into one outline; tubes, locks and tip into one tube; collar into the jacket.", "Rucksackkörper und Deckel zu einem Umriss; Rohre, Verschlüsse und Spitze zu einem Rohr; Kragen in die Jacke."),
    omitted: t("Basket, straps, pockets, brim details, laces, face features.", "Teller, Gurte, Taschen, Schirmdetails, Schnürung, Gesichtszüge."),
    thinElements: t("The tube is drawn 24 mm wide (0.12 mm) with a 0.10 mm pen so that it prints; the 3D tube keeps its true section as a slender member.", "Das Rohr ist 24 mm breit gezeichnet (0,12 mm) mit 0,10-mm-Stift, damit es druckt; das 3D-Rohr behält als schlankes Bauteil seinen wahren Querschnitt."),
    hatching: t("None.", "Keine."),
    styles: t("Body 0.13 mm, detail 0.1 mm; mid jacket, dark trousers, pack and cap, light skin, mid-grey poles.", "Körper 0,13 mm, Detail 0,1 mm; mittlere Jacke, dunkle Hose, dunkler Rucksack und dunkle Mütze, helle Haut, mittelgraue Stöcke."),
    overlays: t("All eight regions keep their own shapes.", "Alle acht Regionen behalten ihre eigenen Formen."),
  },
  "500": {
    silhouette: t("The true three views of the 1:500 model (3.6 mm tall): cap head, neck, one jacket mass, two-part arms with fists ahead of the body, two-part legs with boots, the daypack as one block on the back and the two poles as strips from the fists to the ground. Every shape sits on the bones of the side view, so limbs, pack and poles move with every pose and clip.", "Die wahren drei Ansichten des 1:500-Modells (3,6 mm hoch): Mützenkopf, Hals, eine Jackenmasse, zweiteilige Arme mit Fäusten vor dem Körper, zweiteilige Beine mit Schuhen, der Rucksack als ein Block auf dem Rücken und die beiden Stöcke als Streifen von den Fäusten zum Boden. Jede Form sitzt auf den Knochen der Seitenansicht, sodass Glieder, Rucksack und Stöcke jeder Pose und jedem Clip folgen."),
    retained: t("The pack hump on the back and the two poles: the features that make the figure read as a hiker at this size; the exact stature.", "Der Rucksackbuckel auf dem Rücken und die beiden Stöcke: die Merkmale, an denen die Figur in dieser Größe als Wanderer zu erkennen ist; die exakte Körperhöhe."),
    merged: t("Grip and tube into one pole strip; collar and jacket into the trunk; nose, chin and face into the head; lid and pack into one block.", "Griff und Rohr zu einem Stockstreifen; Kragen und Jacke in den Rumpf; Nase, Kinn und Gesicht in den Kopf; Deckel und Rucksack zu einem Block."),
    omitted: t("Basket, tip, straps, pockets, brim, laces; the pack is drawn 0.25 m instead of 0.30 m wide so that an arm swung back beside it stays a printable strip in plan.", "Teller, Spitze, Gurte, Taschen, Schirm, Schnürung; der Rucksack ist 0,25 m statt 0,30 m breit gezeichnet, damit ein nach hinten geschwungener Arm daneben im Grundriss ein druckbarer Streifen bleibt."),
    thinElements: t("Each 16 mm pole is drawn as a 75 mm strip (0.15 mm with the 0.13 mm pen), the thinnest area the print profile keeps from 1:350 to 1:1000; the volume keeps the true tube as a slender member.", "Jeder 16-mm-Stock ist als 75-mm-Streifen gezeichnet (0,15 mm mit 0,13-mm-Stift), die dünnste Fläche, die das Druckprofil von 1:350 bis 1:1000 hält; das Volumen behält das wahre Rohr als schlankes Bauteil."),
    hatching: t("None.", "Keine."),
    styles: t("Pen 0.13 mm only; mid jacket, dark trousers, boots, cap and pack, light skin, mid-grey poles; light mode dark lines on white, dark mode inverted.", "Nur Stift 0,13 mm; mittlere Jacke, dunkle Hose, Schuhe, Mütze und Rucksack, helle Haut, mittelgraue Stöcke; heller Modus dunkle Linien auf Weiß, dunkler Modus invertiert."),
    overlays: t("Head and cap, jacket, trousers, boots, gloves, pack and poles keep their own region each, so the overlays are painted from the same regions as at the finer scales.", "Kopf und Mütze, Jacke, Hose, Schuhe, Handschuhe, Rucksack und Stöcke behalten je ihre eigene Region, sodass die Überlagerungen aus denselben Regionen gemalt werden wie bei den feineren Maßstäben."),
  },
};
const VOLDEC: Record<string, J[]> = {
  "50": [
    t("Body 1:50: the courier's garments over the body, retoned for the mountains: hip-length softshell with stand-up collar and set-in sleeves, trekking trousers with thigh pockets, ankle-high nubuck boots with heel, arch and toe cap, a peaked wool cap; the gloves are fists closed around the pole grips.", "Körper 1:50: die Kleidung des Kuriers über dem Körper, für die Berge umgetönt: hüftlange Softshelljacke mit Stehkragen und eingesetzten Ärmeln, Trekkinghose mit Beintaschen, knöchelhohe Nubuk-Wanderschuhe mit Absatz, Gewölbe und Zehenkappe, Wollmütze mit Schirm; die Handschuhe sind Fäuste um die Stockgriffe."),
    t("Daypack of 30 litres (0.46 x 0.30 x 0.20 m, 7 kg packed) as an extruded body leaning on the shoulder blades, an extruded lid and a stretch pocket; the shoulder straps are flat 44 x 11 mm bands from the top of the pack over the shoulders down the chest. Poles after common telescopic aluminium trekking poles set to 1.10 m: cork grip 33 mm, tubes of 18, 16 and 14 mm as slender members, two flick locks, a 56 mm basket and a carbide tip.", "Tagesrucksack mit 30 Litern (0,46 x 0,30 x 0,20 m, gepackt 7 kg) als extrudierter Körper, der an den Schulterblättern anliegt, mit extrudiertem Deckel und Stretchtasche; die Schultergurte sind flache 44 x 11 mm Bänder vom oberen Rand des Rucksacks über die Schultern die Brust hinab. Stöcke nach üblichen Teleskop-Trekkingstöcken aus Aluminium, auf 1,10 m eingestellt: Korkgriff 33 mm, Rohre von 18, 16 und 14 mm als schlanke Bauteile, zwei Klemmverschlüsse, ein 56-mm-Teller und eine Hartmetallspitze."),
    t("Thermal 1:50: the courier's nested clothing shells (base layer, fleece, softshell; briefs under the trousers; socks in the boots) with the pack and the poles as passive surfaces referencing their body solids. Structure 1:50: the courier's skeleton as load path, the poles as members from the fist to a support at the tip and the pack as a mass at the centre of its contents.", "Thermik 1:50: die verschachtelten Kleidungsschalen des Kuriers (Funktionsshirt, Fleece, Softshell; Unterhose unter der Hose; Socken in den Schuhen), Rucksack und Stöcke als passive Flächen, die ihre Körpervolumen referenzieren. Tragwerk 1:50: das Skelett des Kuriers als Lastpfad, die Stöcke als Bauteile von der Faust bis zu einem Auflager an der Spitze und der Rucksack als Masse im Schwerpunkt seines Inhalts."),
  ],
  "100": [
    t("Body 1:100, authored for a 15 mm feature: one mass per garment segment, the boot as upper, sole and toe; pack body and lid as two extrusions; each pole as grip, one 16 mm tube (slender), a 15 mm thick basket and the tip merged with the tube.", "Körper 1:100 für ein 15-mm-Merkmal: eine Masse je Kleidungsabschnitt, der Schuh als Schaft, Sohle und Spitze; Rucksackkörper und Deckel als zwei Extrusionen; jeder Stock als Griff, ein 16-mm-Rohr (schlank), ein 15 mm dicker Teller und die Spitze mit dem Rohr verschmolzen."),
    t("Thermal and structure 1:100 reference the body solids; the poles are members from the fist to the tip support, the pack a mass.", "Thermik und Tragwerk 1:100 referenzieren die Körpervolumen; die Stöcke sind Bauteile von der Faust bis zum Spitzenauflager, der Rucksack eine Masse."),
  ],
  "200": [
    t("Body 1:200, authored for a 30 mm feature: the courier's main masses; the pack as one extrusion with the lid in its outline; each pole as grip and one slender tube; the basket is below the feature and omitted.", "Körper 1:200 für ein 30-mm-Merkmal: die Hauptmassen des Kuriers; der Rucksack als eine Extrusion mit dem Deckel im Umriss; jeder Stock als Griff und ein schlankes Rohr; der Teller liegt unter dem Merkmal und entfällt."),
    t("Thermal and structure 1:200 as at 1:100 with the coarser skeleton of the base figure.", "Thermik und Tragwerk 1:200 wie bei 1:100 mit dem gröberen Skelett der Ausgangsfigur."),
  ],
  "500": [
    t("Body 1:500, authored for a 75 mm feature: the courier's simple masses, the pack as one block, each pole as one slender member from the grip to the tip at its true section; the drawing widens the poles to 75 mm strips.", "Körper 1:500 für ein 75-mm-Merkmal: die einfachen Massen des Kuriers, der Rucksack als ein Block, jeder Stock als ein schlankes Bauteil vom Griff bis zur Spitze in seinem wahren Querschnitt; die Zeichnung verbreitert die Stöcke zu 75-mm-Streifen."),
    t("Thermal 1:500: one surface solid per region. Structure 1:500: the stick skeleton of the base figure with the poles as members ending in tip supports and the pack as a mass.", "Thermik 1:500: ein Flächenvolumen je Region. Tragwerk 1:500: das Stabskelett der Ausgangsfigur mit den Stöcken als Bauteilen, die in Spitzenauflagern enden, und dem Rucksack als Masse."),
  ],
};
for (const k of SCALES) {
  a.representations[k].decisions = DEC[k];
  a.volumes[k].decisions = [...VOLDEC[k]!, ...a.volumes[k].decisions.filter((d: J) => d.en.startsWith("Rework") && !/truck|handle|cartons|parcel/i.test(d.en))];
}
//#endregion
a.presentation = { defaultPose: "standing-with-poles", defaultClip: "uphill-walk", defaultScale: 100 };
pruneUnused(a);
fitDimensions(a);
console.log("[DEBUG] dimensions", JSON.stringify(a.dimensions));
saveDoc(ID, doc);
//#endregion
