/** 🎋️ Round 3 bamboo clump: applies the root fixes (rhizome plate centred under the culms, lateral culm positions and leans, culm sections and masses from the modelled culms, barrier and bed materials, outlet anchors, plan symbol, texts), builds the hand-stated plant of every scale (`r3-bamboo-clump-plant.ts`) into `body`, `thermal` and `structure`, redraws every 2D shape as the true view of that scale's body (ruling E1) and writes the candidate document `🗑️generated/r3-bamboo-clump/🔣️.json` and the volumes source.
 * Usage: `bun r3-bamboo-clump-model.ts [--source] [--no-2d]`
 * @see ./round3-brief.md
 * @see ./r3-vegetation-bamboo-clump.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { GENERATED, STAFFAGE, TICKET, entryOf, type Frame, type Json, type P2, type V3, type View } from "./volumes-lib.ts";
import { contours, forward } from "./r3-project2d.ts";
import { simplifyRing } from "./volumes-lib.ts";
import { blobOutline, ellipsoidOf, hullOf, localPoint, loftOf, nearness, outline, pathOf, pointsOf, project, r4, r5, add, sub, mul, dot, sweep, tubePolygons, unit, len, type Tube } from "./r3-date-palm-kit.ts";
import { CULMS, LEAF_LMA, WALL, buildPlant, culmStations, type Part, type Plant, type Scale } from "./r3-bamboo-clump-plant.ts";

const { solveChain } = (await import(join(STAFFAGE, "🔨️modules/🦴️rig/🟦️.ts"))) as { solveChain: (rig: Json, offsets: Json, root: Json) => Record<string, { frame: Frame }> };

const SCALES: Scale[] = [50, 100, 200, 500];
const VIEWS: View[] = ["elevation", "front", "plan"];
const text = (en: string, de: string) => ({ en, de });
const say = (line: string) => process.stdout.write(`${line}\n`);
const ROOT_SHIFT = 0.9;
const LEAF_AREA = 13.6;

const { entry } = entryOf("vegetation/bamboo-clump");
const doc = JSON.parse(readFileSync(entry.path, "utf8"));
const base = JSON.parse(readFileSync(join(TICKET, "r3-bamboo-clump-baseline.json"), "utf8"));
const asset = doc.asset;
const clone = <T>(x: T): T => JSON.parse(JSON.stringify(x));

asset.rig = clone(base.rig);
asset.poses = clone(base.poses);
asset.anchors = clone(base.anchors);
asset.materials = clone(base.materials);
asset.symbols = clone(base.symbols);
asset.provenance = clone(base.provenance);
const boneOf = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);
boneOf("root-plate").x = -ROOT_SHIFT;
for (const c of CULMS) {
  const a = boneOf(`culm-${c.k}-a`);
  a.x = r4(a.x + ROOT_SHIFT);
  a.z = c.z0;
  const lean = Math.tan((c.leanDeg * Math.PI) / 180);
  for (const [child, parent] of [["b", "a"], ["c", "b"]]) boneOf(`culm-${c.k}-${child}`).z = r4(boneOf(`culm-${c.k}-${parent}`).lengthM * lean);
  boneOf(`fol-${c.k}`).z = r4(boneOf(`fol-${c.k}`).x * lean);
}
asset.anchors.find((a: Json) => a.id === "base-support").x = 0.8;
asset.materials.push(
  { id: "hdpe-root-barrier", title: text("HDPE rhizome barrier, 2 mm, 5.5 cm proud of the soil", "HDPE-Rhizomsperre, 2 mm, 5,5 cm über dem Boden"), emissivity: 0.92, solarAbsorptivity: 0.9, densityKgM3: 950, conductivityWmK: 0.45, specificHeatJKgK: 1900, elasticModulusPa: 900000000, strengthPa: 25000000, source: "literature", reference: "rhs-bamboo" },
  { id: "leaf-litter", title: text("Soil under a layer of shed bamboo leaves", "Boden unter einer Schicht abgeworfener Bambusblätter"), emissivity: 0.95, solarAbsorptivity: 0.85, densityKgM3: 1100, conductivityWmK: 0.4, specificHeatJKgK: 1500, source: "authored-estimate", reference: "liese-koehl-2015" },
);
for (const ref of [
  { id: "rhs-bamboo", citation: "Royal Horticultural Society. Bamboos: controlling spread. Running bamboos are contained by a continuous barrier of heavy-duty polyethylene sheet sunk 60 cm deep and left a few centimetres proud of the soil so that rhizomes cannot arch over it." },
  { id: "wang-2013-culm", citation: "Wang, H., Li, W., Ren, D., Yu, Z. and Yu, Y. (2013). A two-variable model for predicting the effects of moisture content and density on compressive strength parallel to the grain for moso bamboo; and the general culm form of Phyllostachys: internodes short at the base, longest at about a third of the height, wall about an eighth of the outer diameter (summarised in Liese and Koehl 2015, chapter 2)." },
  { id: "ohrnberger-1999", citation: "Ohrnberger, D. (1999). The Bamboos of the World. Elsevier, Amsterdam. Phyllostachys: two unequal branches per node from mid-culm upwards, alternating sides with the sulcus; leaves 2 to 4 per ultimate branchlet, lanceolate; P. aureosulcata culms to 8 m and 4 cm, leaves 5 to 15 by 1 to 2 cm." },
]) if (!asset.provenance.references.some((r: Json) => r.id === ref.id)) asset.provenance.references.push(ref);

asset.summary = text(
  "A clump of six culms of yellow-groove bamboo (Phyllostachys aureosulcata) grown inside an HDPE rhizome barrier, 2.4 to 4.5 m tall and 16 to 29 mm thick: green culms with short internodes at the ground and long ones at a third of the height, every node a double ring, the tops nodding outwards; from mid-culm up two unequal branches per node on alternating sides, with branchlets ending in fans of drooping lanceolate leaves that stream in the wind.",
  "Ein Horst aus sechs Halmen des Gelbrillenbambus (Phyllostachys aureosulcata) in einer HDPE-Rhizomsperre, 2,4 bis 4,5 m hoch und 16 bis 29 mm dick: grüne Halme mit kurzen Internodien am Boden und langen auf einem Drittel der Höhe, jeder Knoten ein Doppelring, die Spitzen nach außen nickend; ab halber Halmhöhe zwei ungleiche Äste je Knoten auf wechselnden Seiten, mit Zweiglein, die in Fächern hängender lanzettlicher Blätter enden, die im Wind wehen.",
);
asset.provenance.notes = text(
  "Six culms, each with a lower, middle and upper bone and a top bone that carries the nodding culm top with its foliage, are authored by hand; every rig joint sits on a node. Culm diameters follow the height-diameter relation of Phyllostachys (about 155 D), the wall is an eighth of the diameter, internodes are short at the ground and longest at a third of the height, branches start at 40 % of the culm length, two per node on alternating sides, with fans of two to four leaves (Liese 1985; Ohrnberger 1999; Liese and Koehl 2015). Masses and centres of the culm bones are integrated from the modelled culms (700 kg/m3), branches and the declared leaf area at 0.13 kg/m2 fresh leaf mass. Leaf area and conductance remain authored estimates guided by the cited literature.",
  "Sechs Halme mit je einem unteren, mittleren und oberen Knochen und einem Spitzenknochen, der die nickende Halmspitze mit ihrem Laub trägt, sind von Hand entworfen; jedes Riggelenk liegt auf einem Knoten. Die Halmdurchmesser folgen der Höhen-Durchmesser-Beziehung von Phyllostachys (etwa 155 D), die Wand ist ein Achtel des Durchmessers, die Internodien sind am Boden kurz und auf einem Drittel der Höhe am längsten, die Äste beginnen bei 40 % der Halmlänge, zwei je Knoten auf wechselnden Seiten, mit Fächern aus zwei bis vier Blättern (Liese 1985; Ohrnberger 1999; Liese und Köhl 2015). Massen und Schwerpunkte der Halmknochen sind aus den modellierten Halmen (700 kg/m3), Ästen und der deklarierten Blattfläche mit 0,13 kg/m2 Frischblattmasse integriert. Blattfläche und Leitfähigkeit bleiben Autorenschätzungen, angelehnt an die genannte Literatur.",
);
for (const k of [1, 2, 3, 4, 5, 6]) boneOf(`fol-${k}`).title = text(`Top of culm ${k} with its foliage`, `Spitze von Halm ${k} mit ihrem Laub`);
const calm = asset.poses.find((p: Json) => p.id === asset.presentation.defaultPose);
const solved = solveChain(asset.rig, calm.bones, { x: 0, y: 0, angleDeg: 0 });
const F: Record<string, Frame> = Object.fromEntries(Object.entries(solved).map(([id, s]) => [id, s.frame]));
const plants = Object.fromEntries(SCALES.map((s) => [s, buildPlant(s, F, asset.rig)])) as Record<Scale, Plant>;
for (const s of SCALES) say(`1:${s}: ${plants[s].parts.length} parts, ${plants[s].foliage.length} foliage, ${plants[s].leafCount} leaves`);

/** 🧮️ Volume, area and centre of a loft (sum over frusta of elliptic sections). */
function tubeStats(t: Tube): { vol: number; area: number; com: V3 } {
  let vol = 0;
  let area = 0;
  let com: V3 = [0, 0, 0];
  for (let i = 0; i + 1 < t.sections.length; i++) {
    const [a, b] = [t.sections[i]!, t.sections[i + 1]!];
    const h = len(sub(b.p, a.p));
    const v = (Math.PI * h * (a.a * a.b + b.a * b.b + Math.sqrt(a.a * a.b * b.a * b.b))) / 3;
    vol += v;
    area += Math.PI * h * (a.a + a.b + b.a + b.b) / 2;
    com = add(com, mul(add(a.p, b.p), v / 2));
  }
  return { vol, area, com: mul(com, 1 / Math.max(vol, 1e-12)) };
}

const P50 = plants[50];
const modelledLeaf = P50.parts.reduce((s, p) => s + (p.fan ? p.leafArea ?? 0 : 0), 0);
const leafKgPerModelled = (LEAF_AREA * LEAF_LMA) / modelledLeaf;
const massReport: string[] = [];
const totals: Record<string, { kg: number; m: V3 }> = {};
const addMass = (bone: string, kg: number, at: V3) => {
  const e = (totals[bone] ??= { kg: 0, m: [0, 0, 0] });
  e.kg += kg;
  e.m = add(e.m, mul(at, kg));
};
for (const p of P50.parts) {
  if (!p.tube && !p.rod) continue;
  if (p.region === "culms" && p.tube && p.id === p.bone) {
    const c = P50.culms.find((x) => Object.values(x.bones).includes(p.bone))!;
    const st = tubeStats(p.tube);
    addMass(p.bone, st.vol * (1 - (1 - 2 * WALL) ** 2) * 700, st.com);
  } else if (p.region === "culms" && p.tube) {
    const st = tubeStats(p.tube);
    addMass(p.bone, st.vol * 700, st.com);
  } else if (p.rod) addMass(p.bone, Math.PI * p.rod.r0 * p.rod.r0 * len(sub(p.rod.p1, p.rod.p0)) * 700, mul(add(p.rod.p0, p.rod.p1), 0.5));
  else if (p.leafArea && p.fan) addMass(p.bone, p.leafArea * leafKgPerModelled, p.centre);
}
for (const c of P50.culms) {
  for (const [key, s] of [["a", 0], ["b", c.s1], ["c", c.s2], ["f", c.sf]] as [keyof typeof c.bones, number][]) {
    const bone = boneOf(c.bones[key]);
    const D = c.d(s);
    const d = D * (1 - 2 * WALL);
    const A = (Math.PI / 4) * (D * D - d * d);
    const I = (Math.PI / 64) * (D ** 4 - d ** 4);
    const old = bone.section ? `${(bone.section.extremeFibreM * 2000).toFixed(0)} mm` : "none";
    bone.section = { areaM2: Number(A.toPrecision(4)), secondMomentM4: Number(I.toPrecision(4)), extremeFibreM: r5(D / 2), material: "bamboo-culm", title: text(`Hollow culm, outer diameter ${(D * 1000).toFixed(0)} mm, wall ${(WALL * D * 1000).toFixed(1)} mm`, `Hohler Halm, Außendurchmesser ${(D * 1000).toFixed(0)} mm, Wand ${(WALL * D * 1000).toFixed(1).replace(".", ",")} mm`) };
    const t = totals[bone.id]!;
    const com = localPoint(F[bone.id]!, mul(t.m, 1 / t.kg));
    massReport.push(`${bone.id}: section ${old} -> ${(D * 1000).toFixed(1)} mm; mass ${bone.mass.kg} -> ${t.kg.toFixed(3)} kg, com (${bone.mass.comX}, ${bone.mass.comY}) -> (${com[0].toFixed(3)}, ${com[1].toFixed(3)})`);
    bone.mass = { ...bone.mass, kg: Number(t.kg.toFixed(3)), comX: r4(com[0]), comY: r4(com[1]) };
  }
}
asset.dimensions.massKg = Number(asset.rig.bones.reduce((s: number, b: Json) => s + (b.mass?.kg ?? 0), 0).toFixed(1));
for (const line of massReport) say(line);
say(`massKg ${base.dimensions.massKg} -> ${asset.dimensions.massKg}; modelled leaf area ${modelledLeaf.toFixed(2)} m2 of ${LEAF_AREA}`);

for (const e of P50.emitters) {
  const a = asset.anchors.find((x: Json) => x.id === e.anchor);
  const l = localPoint(F[e.bone]!, e.anchorPoint);
  a.bone = e.bone;
  a.x = r4(l[0]);
  a.y = r4(l[1]);
  a.z = r4(l[2]);
}

/** 🧊️ The solid of a part in its bone frame. */
function solidOf(p: Part, scale: Scale): Json {
  const f = F[p.bone]!;
  const extra = { ...(p.slender ? { slender: true } : {}), ...(p.texture ? { texture: true } : {}) };
  if (p.tube) return { ...loftOf({ ...p.tube, fields: { ...p.fields } }, f), ...extra };
  if (p.rod) return { id: p.id, kind: "capsule", bone: p.bone, ...p.fields, ...extra, p0: localPoint(f, p.rod.p0), p1: localPoint(f, p.rod.p1), r0: r5(p.rod.r0), r1: r5(p.rod.r1) };
  if (p.blob) return { ...ellipsoidOf(p.blob, f), ...extra };
  const e = p.extrude;
  const toLocal = (r: [number, number][]) => r.map(([x, z]) => [r5(z), r5(x + ROOT_SHIFT)]);
  void scale;
  return { id: p.id, kind: "extrude", bone: p.bone, ...p.fields, plane: "zx", from: 0, to: r5(e.y1), profile: toLocal(e.outer), ...(e.inner ? { holes: [toLocal(e.inner)] } : {}) };
}

/** 📏️ The mean outer diameter of a culm over an arclength stretch: the width the 2D draws that culm bone with, and the width the slender culm is compared with. */
const meanD = (c: Plant["culms"][number], s0: number, s1: number): number => Array.from({ length: 9 }, (_, q) => c.d(s0 + ((s1 - s0) * q) / 8)).reduce((a, b) => a + b, 0) / 9;

/** 🧊️ Body, thermal and structure lists of one scale. */
function volumesOf(scale: Scale, P: Plant): Json {
  const feature = (0.15 * scale) / 1000;
  const body = P.parts.filter((p) => p.body).map((p) => solidOf(p, scale));
  const thermal: Json[] = [];
  for (const p of P.parts) {
    if (!p.region || p.proxy) continue;
    if (p.body) thermal.push({ id: `th-${p.id}`, of: p.id, region: p.region });
    else thermal.push({ ...solidOf(p, scale), id: `th-${p.id}`, region: p.region });
  }
  const P50e = P.emitters;
  for (const e of P50e) thermal.push({ ...solidOf(e.part, scale), id: e.id, region: e.part.region, tags: ["emitter", e.emitter] });
  const structure: Json[] = [];
  for (const c of P.culms) {
    const ov = 0.012;
    const stretch: [string, number, number][] = [[c.bones.a, 0, c.s1 + ov], [c.bones.b, c.s1 - ov, c.s2 + ov], [c.bones.c, c.s2 - ov, c.sf + ov], [c.bones.f, c.sf - ov, c.L]];
    for (const [bone, s0, s1] of stretch) {
      const cap = (meanD(c, Math.max(0, s0), s1) / 2) * 0.95;
      const member: Tube = { id: `m-${bone}`, bone, fields: { material: "bamboo-culm", role: "body", tone: 0.5 }, sections: sweep(culmStations(c, s0, s1, scale).map(([s]) => ({ p: c.at(s), a: Math.min((c.d(s) / 2) * 0.95, cap) })), [0, 0, 1]) };
      structure.push({ ...loftOf(member, F[bone]!), slender: true, member: bone, part: "member" });
    }
    for (const [key, s] of [["a", 0], ["b", c.s1], ["c", c.s2], ["f", c.sf]] as [keyof typeof c.bones, number][]) {
      const bone = c.bones[key];
      const spans: Record<string, [number, number]> = { a: [0, c.s1], b: [c.s1, c.s2], c: [c.s2, c.sf], f: [c.sf, c.L] };
      const prev: Record<string, string> = { a: "a", b: "a", c: "b", f: "c" };
      const r = Math.max(0.5 * Math.min(meanD(c, ...spans[key]!), meanD(c, ...spans[prev[key]!]!)), feature * 0.5);
      const ru = key === "a" ? Math.min(r, feature * 0.9) : r;
      structure.push({ id: `j-${bone}`, kind: "ellipsoid", bone, material: "bamboo-culm", role: "detail", tone: 0.5, member: bone, part: "joint", radii: [r5(ru), r5(r), r5(r)] });
    }
  }
  const bed = P.parts.find((p) => p.id === "bed");
  const ringLocal = (ring: [number, number][], k: number) => ring.map(([x, z]) => [r5(z * k), r5((x + 0.1) * k - 0.1 + ROOT_SHIFT)]);
  if (bed) structure.push({ id: "m-root-plate", kind: "extrude", bone: "root-plate", material: "bamboo-culm", role: "body", tone: 0.5, member: "root-plate", part: "member", plane: "zx", from: scale === 200 ? 0.001 : 0.005, to: scale === 200 ? 0.031 : 0.025, profile: ringLocal(bed.extrude.outer, 0.96) });
  else structure.push({ id: "m-root-plate", kind: "box", bone: "root-plate", material: "bamboo-culm", role: "body", tone: 0.5, member: "root-plate", part: "member", size: [0.075, 0.075, 0.075], transform: { translation: [r5(0.02 + ROOT_SHIFT), 0.0375, 0.12] } });
  const t = feature;
  const barrier = plants[50].parts.find((p) => p.id === "barrier")!;
  structure.push({ id: "s-base-support", kind: "extrude", bone: "root-plate", material: "bamboo-culm", role: "detail", tone: 0.5, member: "root-plate", part: "support", anchor: "base-support", plane: "zx", from: r5(-t), to: 0, profile: ringLocal(barrier.extrude.outer, 1) });
  structure.push({ id: "mass-root-plate", kind: "ellipsoid", bone: "root-plate", material: "bamboo-culm", role: "detail", tone: 0.5, member: "root-plate", part: "mass", radii: scale === 500 ? [0.0375, 0.0375, 0.0375] : [Math.max(0.12, feature * 0.6), r5(feature * 0.9), Math.max(0.08, feature * 0.6)], transform: { translation: scale === 500 ? [r5(0.02 + ROOT_SHIFT), 0.0375, 0.12] : [0.8, 0, 0] } });
  return { body, thermal, structure };
}

const count = (P: Plant, pred: (p: Part) => boolean) => P.parts.filter(pred).length;
/** 📝️ The decisions of each scale (en first, de second), with the counts of what was modelled. */
function decisionsOf(s: Scale, P: Plant): Json[] {
  const branches = count(P, (p) => p.region === "culms" && p.id !== p.bone && !p.rod);
  const twigs = count(P, (p) => !!p.rod);
  const foliage = P.foliage.length;
  if (s === 50) return [
    text(`1:50 shows the clump as it grows: the HDPE rhizome barrier 5.5 cm proud of the leaf-litter bed, six culms of 16 to 29 mm with every internode (short at the ground, longest at a third of the height) and every node as a sheath-scar ring under a supranodal ridge, the culm tops nodding outwards. From 40 % of the culm height every node carries two unequal branches on alternating sides (${branches} branches), each with branchlets (${twigs}) ending in fans of two to four drooping lanceolate leaves of 10 to 14 cm (${P.leafCount} leaves, each a thin blade arching down to its tip). Rig joints sit at nodes so that every pose bends the culm at a node ring.`, `1:50 zeigt den Horst, wie er wächst: die HDPE-Rhizomsperre 5,5 cm über dem Laubstreubeet, sechs Halme von 16 bis 29 mm mit jedem Internodium (am Boden kurz, auf einem Drittel der Höhe am längsten) und jedem Knoten als Scheidennarbenring unter einem Knotenwulst, die Halmspitzen nach außen nickend. Ab 40 % der Halmhöhe trägt jeder Knoten zwei ungleiche Äste auf wechselnden Seiten (${branches} Äste), jeder mit Zweiglein (${twigs}), die in Fächern aus zwei bis vier hängenden lanzettlichen Blättern von 10 bis 14 cm enden (${P.leafCount} Blätter, jedes ein dünnes Blatt, das sich zur Spitze hin abwärts wölbt). Die Gelenke des Rigs liegen an Knoten, so dass jede Pose den Halm an einem Knotenring biegt.`),
    text(`Thermal: culms are the six culms, branches and branchlets by reference; the leaves are split into sunlit (upper and outer, ${(5.4 / 13.6 * 100).toFixed(0)} % of the leaf area) and shaded foliage by reference; twelve transpiration outlets lie as thin patches on one sunlit and one shaded leaf of each culm top. The modelled leaves carry the declared leaf area of the whole clump (exchange area). Structure: every culm bone is a member that runs inside the culm along its own curve from node to node, at 95 % of the outer diameter, the nodding top included; the hollow section (wall an eighth of the diameter) is the section of the bone that the forces and moments are computed with; joints at every rig joint (at the nodes), the rhizome plate as a slab inside the barrier, the fixed support under it and the plate mass at its centre.`, `Thermik: Halme sind die sechs Halme, Äste und Zweiglein per Verweis; die Blätter sind in besonntes (oberes und äußeres, ${(5.4 / 13.6 * 100).toFixed(0)} % der Blattfläche) und beschattetes Laub per Verweis geteilt; zwölf Transpirationsauslässe liegen als dünne Flecken auf je einem besonnten und einem beschatteten Blatt jeder Halmspitze. Die modellierten Blätter tragen die deklarierte Blattfläche des ganzen Horsts (Austauschfläche). Struktur: jeder Halmknochen ist ein Glied, das im Halm entlang dessen eigener Krümmung von Knoten zu Knoten läuft, mit 95 % des Außendurchmessers, die nickende Spitze eingeschlossen; der hohle Querschnitt (Wand ein Achtel des Durchmessers) ist der Querschnitt des Knochens, mit dem Kräfte und Momente gerechnet werden; Gelenke an jedem Riggelenk (an den Knoten), die Rhizomplatte als Platte in der Sperre, das feste Auflager darunter und die Plattenmasse in ihrer Mitte.`),
  ];
  if (s === 100) return [
    text(`1:100 keeps the habit and merges what is below 15 mm: culms with every node as one ring, both unequal branches of every node (${branches} branches), the leaves of each fan merged into one drooping spray with the outline the splayed fan sweeps, 16 mm thick (${foliage} sprays); branchlets are below the scale and omitted. Barrier and bed as at 1:50.`, `1:100 behält den Wuchs und fasst zusammen, was unter 15 mm liegt: Halme mit jedem Knoten als ein Ring, beide ungleichen Äste jedes Knotens (${branches} Äste), die Blätter jedes Fächers zu einem hängenden Büschel mit dem Umriss des gespreizten Fächers zusammengefasst, 16 mm dick (${foliage} Büschel); Zweiglein liegen unter dem Maßstab und entfallen. Sperre und Beet wie bei 1:50.`),
    text("Thermal: culms and branches by reference, sprays split into sunlit and shaded foliage, twelve outlets on sprays of the culm tops. Structure as at 1:50: culm members along the curve of the culms with fewer stations, joints, slab, support and mass.", "Thermik: Halme und Äste per Verweis, Büschel in besonntes und beschattetes Laub geteilt, zwölf Auslässe auf Büscheln der Halmspitzen. Struktur wie bei 1:50: Halmglieder entlang der Krümmung der Halme mit weniger Stützstellen, Gelenke, Platte, Auflager und Masse."),
  ];
  if (s === 200) return [
    text(`1:200 is a new abstraction: nodes are below 30 mm and the culms are plain tapered lofts with their nodding tops; each node's pair of branches with all its leaves is one drooping tuft hanging from the culm along the branch (${foliage} tufts), the barrier a low rim around the bed.`, `1:200 ist eine neue Abstraktion: Knoten liegen unter 30 mm und die Halme sind einfache verjüngte Lofts mit nickenden Spitzen; das Astpaar jedes Knotens mit allen Blättern ist ein hängender Schopf, der entlang des Asts vom Halm hängt (${foliage} Schöpfe), die Sperre ein niedriger Rand um das Beet.`),
    text(`Thermal: culms by reference; the branches inside the tufts stay as slender thermal solids (${branches}) so that the culm area keeps its woody surface; tufts split into sunlit and shaded foliage; twelve outlets on tufts of the culm tops. Structure: plain tapered culm members, joints, slab, support and mass.`, `Thermik: Halme per Verweis; die Äste in den Schöpfen bleiben als schlanke Thermikkörper (${branches}), damit die Halmfläche ihre verholzte Oberfläche behält; Schöpfe in besonntes und beschattetes Laub geteilt; zwölf Auslässe auf Schöpfen der Halmspitzen. Struktur: einfache verjüngte Halmglieder, Gelenke, Platte, Auflager und Masse.`),
  ];
  return [
    text(`1:500 keeps what reads at 75 mm: six slender culms with their lean and nodding tops and on each culm three foliage masses (lower, middle, top) that follow the branch tiers and lean with the culm, so the clump reads as six narrow, plume-like crowns; barrier, bed and branches are below the scale.`, `1:500 behält, was bei 75 mm lesbar ist: sechs schlanke Halme mit ihrer Neigung und nickenden Spitzen und an jedem Halm drei Laubmassen (unten, Mitte, oben), die den Astetagen folgen und sich mit dem Halm neigen, so dass der Horst als sechs schmale, federbuschartige Kronen lesbar ist; Sperre, Beet und Äste liegen unter dem Maßstab.`),
    text(`Thermal: culms by reference, the branches as slender thermal solids inside the masses (${branches}), the top masses sunlit and the lower two shaded, twelve outlets on the masses of the culm tops. Structure: solid culm members, joints, the rhizome plate reduced to a 75 mm block at the central culm with its mass inside, and the support.`, `Thermik: Halme per Verweis, die Äste als schlanke Thermikkörper in den Massen (${branches}), die oberen Massen besonnt und die unteren beiden beschattet, zwölf Auslässe auf den Massen der Halmspitzen. Struktur: volle Halmglieder, Gelenke, die Rhizomplatte auf einen 75-mm-Block am mittleren Halm reduziert, mit ihrer Masse darin, und das Auflager.`),
  ];
}

const REP_DECISIONS: Record<Scale, Json> = {
  50: {
    silhouette: text("At 1:50 (1 m is 20 mm) every drawing is the true orthographic view of the 1:50 model in the calm summer pose: the barrier rim, six culms with their taper and node rings, branches and branchlets as strokes, and every leaf as a lanceolate outline, sunlit and shaded leaves in two tones.", "Bei 1:50 (1 m sind 20 mm) ist jede Zeichnung die wahre Parallelprojektion des 1:50-Modells in der ruhigen Sommerpose: der Sperrenrand, sechs Halme mit Verjüngung und Knotenringen, Äste und Zweiglein als Striche und jedes Blatt als lanzettliche Kontur, besonnte und beschattete Blätter in zwei Tönen."),
    retained: text("Retained: every culm, node, branch, branchlet and leaf of the model.", "Beibehalten: jeder Halm, Knoten, Ast, jedes Zweiglein und Blatt des Modells."),
    merged: text("Merged: the leaves of one bone and light are one outline; overlapping leaves read as one patch.", "Zusammengefasst: die Blätter eines Knochens und einer Belichtung bilden eine Kontur; überlappende Blätter lesen sich als eine Fläche."),
    omitted: text("Omitted: culm sheaths (shed by the mature culm), the groove colour and the leaf veins, below 0.15 mm on paper.", "Weggelassen: Halmscheiden (vom reifen Halm abgeworfen), die Rillenfarbe und die Blattadern, unter 0,15 mm auf dem Papier."),
    thinElements: text("Node rings are the finest strokes (texture pen); branches and branchlets are detail strokes at their true position.", "Knotenringe sind die feinsten Striche (Texturstift); Äste und Zweiglein sind Detailstriche an ihrer wahren Lage."),
  },
  100: {
    silhouette: text("At 1:100 (1 m is 10 mm) the drawings are the true views of the 1:100 model: barrier rim, culms with node rings, both branches of every node as strokes, and every leaf fan as one spray, sunlit and shaded.", "Bei 1:100 (1 m sind 10 mm) sind die Zeichnungen die wahren Ansichten des 1:100-Modells: Sperrenrand, Halme mit Knotenringen, beide Äste jedes Knotens als Striche und jeder Blattfächer als ein Büschel, besonnt und beschattet."),
    retained: text("Retained: culms, nodes, both branches of every node and the leaf sprays.", "Beibehalten: Halme, Knoten, beide Äste jedes Knotens und die Blattbüschel."),
    merged: text("Merged: the leaves of a fan into one spray.", "Zusammengefasst: die Blätter eines Fächers zu einem Büschel."),
    omitted: text("Omitted: branchlets and single leaves (below 15 mm).", "Weggelassen: Zweiglein und einzelne Blätter (unter 15 mm)."),
    thinElements: text("Node rings are texture strokes, branches detail strokes at their true position.", "Knotenringe sind Texturstriche, Äste Detailstriche an ihrer wahren Lage."),
  },
  200: {
    silhouette: text("At 1:200 (1 m is 5 mm) the drawings are the true views of the 1:200 model: the barrier rim, six tapered culms and the drooping leaf tufts of every branch pair, sunlit and shaded.", "Bei 1:200 (1 m sind 5 mm) sind die Zeichnungen die wahren Ansichten des 1:200-Modells: der Sperrenrand, sechs verjüngte Halme und die hängenden Blattschöpfe jedes Astpaars, besonnt und beschattet."),
    retained: text("Retained: every culm and every branch tier as a tuft.", "Beibehalten: jeder Halm und jede Astetage als Schopf."),
    merged: text("Merged: branches and leaves of a node into one tuft.", "Zusammengefasst: Äste und Blätter eines Knotens zu einem Schopf."),
    omitted: text("Omitted: nodes and single branches (below 30 mm).", "Weggelassen: Knoten und einzelne Äste (unter 30 mm)."),
    thinElements: text("The culms are the thinnest filled shapes (0.15 mm on paper at the base, drawn with the body pen).", "Die Halme sind die dünnsten gefüllten Formen (0,15 mm auf dem Papier am Fuß, mit dem Körperstift gezeichnet)."),
  },
  500: {
    silhouette: text("At 1:500 (1 m is 2 mm) the drawings are the true views of the 1:500 model: six culms and three foliage masses on each, sunlit tops and shaded lower masses.", "Bei 1:500 (1 m sind 2 mm) sind die Zeichnungen die wahren Ansichten des 1:500-Modells: sechs Halme mit je drei Laubmassen, besonnte Spitzen und beschattete untere Massen."),
    retained: text("Retained: every culm and its crown as three masses.", "Beibehalten: jeder Halm und seine Krone als drei Massen."),
    merged: text("Merged: the branch tiers of a culm into three masses.", "Zusammengefasst: die Astetagen eines Halms zu drei Massen."),
    omitted: text("Omitted: barrier, bed, nodes and branches (below 75 mm).", "Weggelassen: Sperre, Beet, Knoten und Äste (unter 75 mm)."),
    thinElements: text("The culms are single body strokes on their true axes: at 2 mm per metre their 16 to 29 mm are below the pen.", "Die Halme sind einzelne Körperstriche auf ihren wahren Achsen: bei 2 mm je Meter liegen ihre 16 bis 29 mm unter dem Stift."),
  },
};

const volumes: Json = {};
for (const s of SCALES) volumes[String(s)] = { status: "authored", denominator: s, ...volumesOf(s, plants[s]), decisions: decisionsOf(s, plants[s]) };

const cand = clone(asset);
cand.volumes = volumes;

/** ✏️ Affine maps of the paper of a view into the frames of the given shapes, solved from three probe points placed by the product. */
function affineFor(scale: number, view: View, metas: Json[]): ((p: P2) => P2)[] {
  const placed = forward(cand, scale, view, metas.map((meta) => ({ meta, points: [{ x: 0, y: 0 }, { x: 1, y: 0 }, { x: 0, y: 1 }] })));
  return placed.map(([o, a, b]) => {
    const [m00, m10, m01, m11] = [a!.x - o!.x, a!.y - o!.y, b!.x - o!.x, b!.y - o!.y];
    const det = m00 * m11 - m01 * m10;
    return (p: P2) => {
      const [dx, dy] = [p.x - o!.x, p.y - o!.y];
      return { x: (m11 * dx - m01 * dy) / det, y: (-m10 * dx + m00 * dy) / det };
    };
  });
}

const capsuleRing = (view: View, p0: V3, p1: V3, r0: number, r1: number): P2[] => {
  const ring = (c: V3, r: number) => Array.from({ length: 12 }, (_, i) => {
    const a = (2 * Math.PI * i) / 12 + 0.07;
    const q = project(view, c);
    return { x: q.x + r * Math.cos(a), y: q.y + r * Math.sin(a) };
  });
  return hullOf([...ring(p0, r0), ...ring(p1, r1)]);
};

/** 🔷️ The projected polygons of a part. */
function polysOf(view: View, p: Part): P2[][] {
  if (p.tube) return tubePolygons(view, p.tube);
  if (p.rod) return [capsuleRing(view, p.rod.p0, p.rod.p1, p.rod.r0, p.rod.r1)];
  if (p.blob) return [blobOutline(view, p.blob)];
  const e = p.extrude;
  const xs = e.outer.map((q: number[]) => q[0]);
  const zs = e.outer.map((q: number[]) => q[1]);
  if (view === "elevation") return [[{ x: Math.min(...xs), y: e.y0 }, { x: Math.max(...xs), y: e.y0 }, { x: Math.max(...xs), y: e.y1 }, { x: Math.min(...xs), y: e.y1 }].map((q) => project(view, [q.x, q.y, 0]))];
  if (view === "front") return [[{ z: Math.min(...zs), y: e.y0 }, { z: Math.max(...zs), y: e.y0 }, { z: Math.max(...zs), y: e.y1 }, { z: Math.min(...zs), y: e.y1 }].map((q) => project(view, [0, q.y, q.z]))];
  return [e.outer.map((q: number[]) => project(view, [q[0]!, e.y1, q[1]!]))];
}

/** 🧱️ The outline rings of the union of polygons on a raster of `cell`: every polygon filled by scanline, traced (outer rings counter-clockwise, holes clockwise), simplified within `tolerance`, rings under `minArea` dropped. */
function rasterOutline(polys: P2[][], cell: number, tolerance: number, minArea: number, inset = 0): P2[][] {
  const pts = polys.flat();
  if (!pts.length) return [];
  const box = { minX: Math.min(...pts.map((p) => p.x)) - 2 * cell, minY: Math.min(...pts.map((p) => p.y)) - 2 * cell, maxX: Math.max(...pts.map((p) => p.x)) + 2 * cell, maxY: Math.max(...pts.map((p) => p.y)) + 2 * cell };
  const width = Math.ceil((box.maxX - box.minX) / cell);
  const height = Math.ceil((box.maxY - box.minY) / cell);
  const cells = new Uint8Array(width * height);
  for (const poly of polys) {
    if (poly.length < 3) continue;
    const ys = poly.map((p) => (p.y - box.minY) / cell - 0.5);
    const y0 = Math.max(0, Math.ceil(Math.min(...ys)));
    const y1 = Math.min(height - 1, Math.floor(Math.max(...ys)));
    for (let y = y0; y <= y1; y++) {
      const xs: number[] = [];
      for (let i = 0; i < poly.length; i++) {
        const [a, b] = [poly[i]!, poly[(i + 1) % poly.length]!];
        const [ya, yb] = [ys[i]!, ys[(i + 1) % poly.length]!];
        if ((ya <= y && yb > y) || (yb <= y && ya > y)) xs.push((a.x - box.minX) / cell - 0.5 + ((y - ya) / (yb - ya)) * ((b.x - a.x) / cell));
      }
      xs.sort((m, n) => m - n);
      for (let k = 0; k + 1 < xs.length; k += 2) for (let x = Math.max(0, Math.ceil(xs[k]!)); x <= Math.min(width - 1, Math.floor(xs[k + 1]!)); x++) cells[y * width + x] = 1;
    }
  }
  const steps = Math.round(inset / cell);
  const grow = (src: Uint8Array, value: 0 | 1): Uint8Array => {
    const out = src.slice();
    for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) {
      if (src[y * width + x] === value) continue;
      for (const [dx, dy] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) {
        const [nx, ny] = [x + dx!, y + dy!];
        if (nx >= 0 && ny >= 0 && nx < width && ny < height && src[ny * width + nx] === value) {
          out[y * width + x] = value;
          break;
        }
      }
    }
    return out;
  };
  let core = cells;
  for (let i = 0; i < steps; i++) core = grow(core, 0);
  let opened = core;
  for (let i = 0; i < steps; i++) opened = grow(opened, 1);
  const kept = new Uint8Array(cells.length);
  for (let i = 0; i < cells.length; i++) kept[i] = core[i]! | (cells[i]! & (opened[i]! ^ 1) & thinAt(i));
  function thinAt(i: number): number {
    const [x, y] = [i % width, Math.floor(i / width)];
    for (let dy = -steps; dy <= steps; dy++) for (let dx = -steps; dx <= steps; dx++) {
      const [nx, ny] = [x + dx, y + dy];
      if (nx >= 0 && ny >= 0 && nx < width && ny < height && core[ny * width + nx] === 1) return 0;
    }
    return 1;
  }
  return contours(steps > 0 ? kept : cells, width, height, box, cell).map((r) => simplifyRing(r, tolerance)).filter((r) => r.length >= 3 && Math.abs(ringArea(r)) >= minArea).sort((a, b) => ringArea(b) - ringArea(a));
}

const ringArea = (r: P2[]) => r.reduce((s, p, k) => s + p.x * r[(k + 1) % r.length]!.y - r[(k + 1) % r.length]!.x * p.y, 0) / 2;

/** 🖊️ All shapes of one scale and view: every body shape group as its true outline, culm nodes and branches as strokes. */
function drawView(scale: Scale, view: View): Json[] {
  const P = plants[scale];
  const feature = (0.15 * scale) / 1000;
  const tol = feature / 4;
  type Pending = { meta: Json; rings: P2[][]; closed: boolean; depth: number; owner?: string };
  const pending: Pending[] = [];
  const groups = new Map<string, Part[]>();
  for (const p of P.parts) if (p.body && p.shape && !p.slender || (p.body && p.region === "culms" && p.id === p.bone)) {
    const key = p.shape!;
    if (!groups.has(key)) groups.set(key, []);
    groups.get(key)!.push(p);
  }
  for (const [id, parts] of groups) {
    if (id === "bed" && view !== "plan") continue;
    const p0 = parts[0]!;
    let rings: P2[][];
    if (id === "barrier" && view === "plan") {
      const e = p0.extrude;
      const o = e.outer.map((q: number[]) => project(view, [q[0]!, e.y1, q[1]!]));
      const i = e.inner.map((q: number[]) => project(view, [q[0]!, e.y1, q[1]!]));
      rings = scale >= 200 ? [o] : [ringArea(o) > 0 ? o : o.reverse(), ringArea(i) < 0 ? i : i.reverse()];
      if (scale >= 200) {
        pending.push({ meta: { id, bone: p0.bone, role: "detail", filled: false, tone: p0.fields.tone, material: p0.fields.material }, rings, closed: true, depth: nearness(view, p0.centre) });
        continue;
      }
    } else {
      const role = p0.fields.role === "texture" ? "detail" : p0.fields.role;
      const penMm = p0.fields.role === "texture" ? 0.13 : (cand.representations[String(scale)].pens[role] as number);
      rings = rasterOutline(parts.flatMap((p) => polysOf(view, p)), feature / 12, feature / 14, feature * feature * 0.25);
    }
    if (!rings.length) continue;
    const depth = parts.reduce((s, p) => s + nearness(view, p.centre), 0) / parts.length;
    const region = p0.region;
    const isCulm = p0.id === p0.bone && region === "culms";
    if (isCulm) {
      const c = P.culms.find((x) => Object.values(x.bones).includes(p0.bone))!;
      const s0 = p0.bone === c.bones.a ? 0 : p0.bone === c.bones.b ? c.s1 : p0.bone === c.bones.c ? c.s2 : c.sf;
      const s1 = p0.bone === c.bones.a ? c.s1 : p0.bone === c.bones.b ? c.s2 : p0.bone === c.bones.c ? c.sf : c.L;
      const line = Array.from({ length: 9 }, (_, q) => project(view, c.at(s0 + ((s1 - s0) * q) / 8)));
      const dMean = meanD(c, s0, s1);
      const penMm = Math.max(0.13, Math.round((dMean * 100000) / scale) / 100);
      pending.push({ meta: { id, bone: p0.bone, role: "body", filled: false, tone: p0.fields.tone, region: "culms", material: p0.fields.material, penMm }, rings: [line], closed: false, depth: scale === 500 ? depth - 10 : depth + 0.02 });
      continue;
    }
    pending.push({ meta: { id, bone: p0.bone, role: p0.fields.role === "texture" ? "detail" : p0.fields.role, ...(p0.fields.role === "texture" ? { penMm: 0.13 } : {}), filled: true, tone: p0.fields.tone, ...(region ? { region } : {}), material: p0.fields.material }, rings, closed: true, depth: isCulm ? depth + 0.02 : depth });
  }
  for (const [id, parts] of new Map([...P.parts.filter((p) => p.body && p.slender && p.shape?.endsWith("-branches")).reduce((m, p) => m.set(p.shape!, [...(m.get(p.shape!) ?? []), p]), new Map<string, Part[]>())])) {
    const lines = parts.map((p) => (p.tube ? p.tube.sections.map((s) => project(view, s.p)) : [project(view, p.rod!.p0), project(view, p.rod!.p1)]));
    pending.push({ meta: { id, bone: parts[0]!.bone, role: "detail", filled: false, tone: 0.3 }, rings: lines, closed: false, depth: 0, owner: parts[0]!.bone });
  }
  if (scale <= 100) for (const c of P.culms) {
    const stretches: [string, number, number][] = [[c.bones.a, 0, c.s1], [c.bones.b, c.s1, c.s2], [c.bones.c, c.s2, c.sf], [c.bones.f, c.sf, c.L]];
    for (const [bone, s0, s1] of stretches) {
      const lines = c.nodes.filter((n) => n > s0 + 0.015 && n < s1 - 0.015).map((n) => {
        const [q0, q1] = [project(view, c.at(n - 0.01)), project(view, c.at(n + 0.01))];
        const t = { x: q1.x - q0.x, y: q1.y - q0.y };
        const l = Math.hypot(t.x, t.y);
        if (l < 0.004) return null;
        const q = project(view, c.at(n));
        const r = (c.d(n) / 2) * 1.07;
        return [{ x: q.x - (t.y / l) * r, y: q.y + (t.x / l) * r }, { x: q.x + (t.y / l) * r, y: q.y - (t.x / l) * r }];
      }).filter((x): x is P2[] => x !== null);
      if (lines.length) pending.push({ meta: { id: `${bone}-nodes`, bone, role: "texture", filled: false, tone: 0.2 }, rings: lines, closed: false, depth: 0, owner: bone });
    }
  }
  const filled = pending.filter((p) => p.owner === undefined);
  void filled;
  const depths = filled.map((p) => p.depth);
  const [dmin, dmax] = [Math.min(...depths), Math.max(...depths)];
  const order = [...filled].sort((a, b) => a.depth - b.depth);
  const zOf = new Map(order.map((p, i) => [p, r4(1 + (4 * i) / Math.max(1, order.length - 1))]));
  void dmin;
  void dmax;
  const ownerZ = (bone: string) => zOf.get(filled.find((p) => p.meta.id === bone)!) ?? 3;
  const metas = pending.map((p) => ({ ...p.meta, z: p.owner === undefined ? zOf.get(p)! : r4(ownerZ(p.owner) + (p.meta.role === "texture" ? 0.002 : -0.001)) }));
  const maps = affineFor(scale, view, metas);
  return pending.map((p, i) => {
    const local = p.rings.map((r) => r.map(maps[i]!));
    const meta = metas[i]!;
    if (p.closed && local.length === 1) return { ...meta, kind: "poly", points: pointsOf(local[0]!), closed: true };
    return { ...meta, kind: "path", d: pathOf(local, p.closed) };
  });
}

/** 🗺️ The plan symbol redrawn from the 1:200 model: the true foliage outline and the six culms. */
function drawSymbol(): Json[] {
  const sym = cand.symbols[0];
  const P = plants[200];
  const out: Record<View, Json[]> = { elevation: [], front: [], plan: [] };
  for (const view of VIEWS) {
    const crown = rasterOutline(P.parts.filter((p) => p.region === "leaves-sun" || p.region === "leaves-shade").flatMap((p) => polysOf(view, p)), 0.01, 0.03, 0.0001);
    const hull = hullOf([...crown.flat(), ...P.culms.map((c) => project(view, c.at(c.L)))]);
    const items: { meta: Json; ring: P2[]; closed: boolean }[] = [{ meta: { id: "crown", bone: "root-plate", role: "body", z: 2, filled: false, tone: 0.5 }, ring: hull, closed: true }];
    for (const c of P.culms) {
      if (view === "plan") {
        const q = project(view, c.at(0));
        const r = c.d(0) / 2;
        items.push({ meta: { id: `culm-${c.spec.k}`, bone: "root-plate", role: "body", z: 4, filled: true, tone: 0.25 }, ring: Array.from({ length: 16 }, (_, i) => ({ x: q.x + r * Math.cos((i * Math.PI) / 8), y: q.y + r * Math.sin((i * Math.PI) / 8) })), closed: true });
      } else items.push({ meta: { id: `culm-${c.spec.k}`, bone: "root-plate", role: "detail", z: 3, filled: false, tone: 0.3 }, ring: [c.at(0), c.at(c.s1), c.at(c.s2), c.at(c.sf), c.at(c.L)].map((p) => project(view, p)), closed: false });
    }
    const maps = affineFor(200, view, items.map((x) => x.meta));
    out[view] = items.map((x, i) => ({ ...x.meta, kind: "poly", points: pointsOf(x.ring.map(maps[i]!)), closed: x.closed }));
  }
  sym.geometry = out;
  const crownPlan = plants[200].parts.filter((p) => p.region?.startsWith("leaves")).flatMap((p) => polysOf("plan", p).flat());
  const xs = crownPlan.map((p) => p.x);
  const ys = crownPlan.map((p) => p.y);
  const [w, d] = [Math.max(...xs) - Math.min(...xs), Math.max(...ys) - Math.min(...ys)];
  sym.description = text(`Plan sign for landscape plans: the true foliage extent (${w.toFixed(1)} m by ${d.toFixed(1)} m) with the six culm sections; a separate symbol, never used silently for a pictorial scale.`, `Grundrisszeichen für Freiraumpläne: die wahre Laubausdehnung (${w.toFixed(1).replace(".", ",")} m mal ${d.toFixed(1).replace(".", ",")} m) mit den sechs Halmquerschnitten; ein eigenes Symbol, nie stillschweigend für einen Bildmaßstab verwendet.`);
  return out.plan;
}

if (!process.argv.includes("--no-2d")) {
  for (const s of SCALES) {
    const rep = cand.representations[String(s)];
    for (const view of VIEWS) {
      rep.geometry[view] = drawView(s, view);
      say(`2D 1:${s} ${view}: ${rep.geometry[view].length} shapes`);
    }
    const keep = Object.fromEntries(Object.entries(rep.decisions).filter(([k]) => !["silhouette", "retained", "merged", "omitted", "thinElements"].includes(k)));
    rep.decisions = { ...keep, ...REP_DECISIONS[s] };
  }
  drawSymbol();
}

const ext = (P: Plant) => {
  const e = { min: [Infinity, Infinity, Infinity], max: [-Infinity, -Infinity, -Infinity] };
  for (const p of P.parts.filter((x) => x.body)) for (const q of polysOf("elevation", p).flat()) {
    e.min[0] = Math.min(e.min[0]!, q.x);
    e.max[0] = Math.max(e.max[0]!, q.x);
    e.min[1] = Math.min(e.min[1]!, q.y);
    e.max[1] = Math.max(e.max[1]!, q.y);
  }
  for (const p of P.parts.filter((x) => x.body)) for (const q of polysOf("front", p).flat()) {
    e.min[2] = Math.min(e.min[2]!, q.x);
    e.max[2] = Math.max(e.max[2]!, q.x);
  }
  return e;
};
for (const s of SCALES) {
  const e = ext(plants[s]);
  say(`1:${s} extent elevation-x ${(e.max[0]! - e.min[0]!).toFixed(3)} (${e.min[0]!.toFixed(3)}..${e.max[0]!.toFixed(3)}) height ${e.max[1]!.toFixed(3)} front-x ${(e.max[2]! - e.min[2]!).toFixed(3)} (${e.min[2]!.toFixed(3)}..${e.max[2]!.toFixed(3)}); dimensions ${asset.dimensions.widthM} x ${asset.dimensions.heightM} x ${asset.dimensions.depthM}`);
}

const out = { ...doc, asset: cand };
const candPath = join(GENERATED, "r3-bamboo-clump", "🔣️.json");
mkdirSync(dirname(candPath), { recursive: true });
writeFileSync(candPath, `${JSON.stringify(out, null, 2)}\n`);
const srcPath = process.argv.includes("--source") ? join(TICKET, "volumes", "vegetation", "bamboo-clump.json") : join(GENERATED, "r3-bamboo-clump", "volumes.json");
writeFileSync(srcPath, `${JSON.stringify({ volumes }, null, 2)}\n`);
for (const s of SCALES) {
  const v = volumes[String(s)];
  say(`1:${s}: body ${v.body.length}, thermal ${v.thermal.length}, structure ${v.structure.length}`);
}
say(`written ${candPath}\nwritten ${srcPath}`);
