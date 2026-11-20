/** 🌲️ Round 3 English oak: writes the volumes source (body, thermal, structure, decisions at 1:50, 1:100, 1:200, 1:500) from the hand-stated tree, redraws every 2D shape of every scale and view as the true orthographic view of that scale's body (ruling E1), moves the rig's lateral joint offsets and the transpiration anchors, and writes the candidate asset document.
 *
 * Usage: `bun r3-english-oak-model.ts` → `volumes/vegetation/english-oak.json`, `🗑️generated/r3-english-oak/🔣️.json` (candidate document), `🗑️generated/r3-english-oak/roots.json` (rig, anchors, representations for the splice).
 * @see ./r3-english-oak-tree.ts
 * @see ./r3-english-oak-splice.py */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { BONE_Z, axisOf, boughs, clumpTop, refine, restFrames, toLocal, treeOf, type Frame, type Lobe, type Station, type Tube, type V3 } from "./r3-english-oak-tree.ts";

type Json = any;
type P = [number, number];
type View = "elevation" | "front" | "plan";
const T = import.meta.dir;
const ASSET = join(T, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/english-oak/🔣️.json");
const OUT = join(T, "🗑️generated/r3-english-oak");
const SCALES = [50, 100, 200, 500] as const;
const FEATURE: Record<number, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };
const r5 = (v: number): number => Math.round(v * 1e5) / 1e5;
const r4 = (v: number): number => Math.round(v * 1e4) / 1e4;

const doc: Json = JSON.parse(readFileSync(ASSET, "utf8"));
const asset: Json = doc.asset;
for (const b of asset.rig.bones) {
  if (BONE_Z[b.id] !== undefined) b.z = BONE_Z[b.id];
  else delete b.z;
}
asset.rig.bones = asset.rig.bones.map((b: Json) => {
  const { id, parent, title, x, y, z, ...rest } = b;
  return z === undefined ? { id, parent, title, x, y, ...rest } : { id, parent, title, x, y, z, ...rest };
});
const FRAMES = restFrames(asset.rig.bones);
const PARENT: Record<string, string | null> = Object.fromEntries(asset.rig.bones.map((b: Json) => [b.id, b.parent]));
const BONE: Record<string, Json> = Object.fromEntries(asset.rig.bones.map((b: Json) => [b.id, b]));
const local = (bone: string, p: readonly number[]): V3 => toLocal(FRAMES[bone]!, p).map(r5) as V3;
const quat = (bone: string): [number, number, number, number] => {
  const a = -FRAMES[bone]!.ang / 2;
  return [0, 0, r5(Math.sin(a)), r5(Math.cos(a))];
};

const SECTION_R: Record<string, number> = { "root-plate": 0.46, trunk: 0.4, "trunk-upper": 0.3, leader: 0.19, "leader-top": 0.09, "limb-w-low": 0.18, "limb-e-low": 0.16, "limb-w-mid": 0.145, "limb-e-mid": 0.145, "br-w-low": 0.085, "br-e-low": 0.085, "br-w-mid": 0.085, "br-e-mid": 0.085, "br-leader-w": 0.07, "br-leader-e": 0.07, "br-low-w-up": 0.07, "br-low-e-up": 0.07 };
const SUN = ["w-low", "e-low", "w-mid", "e-mid", "leader-w", "leader-e", "top"];
const SUN_500 = ["crown-w-low", "crown-e-low", "crown-w-mid", "crown-e-mid", "crown-top", "crown-w-up", "crown-e-up"];

/** 🧭️ Section rotations of a tube without twist: every section plane is perpendicular to the path, its x axis is the one bone axis (y or z) that the tube travels least along, projected into the plane. The kernel's default frame projects the bone's x axis, which is the direction these tubes run in, so its rings turn against each other and wring the skin. */
function sectionRotations(at: readonly V3[]): [number, number, number, number][] {
  const travel = [0, 1, 2].map((k) => Math.abs(at[at.length - 1]![k]! - at[0]![k]!));
  const ref: V3 = travel[1]! <= travel[2]! ? [0, 1, 0] : [0, 0, 1];
  return at.map((_, i) => {
    const [p, q] = [at[Math.max(0, i - 1)]!, at[Math.min(at.length - 1, i + 1)]!];
    const d = [q[0] - p[0], q[1] - p[1], q[2] - p[2]];
    const len = Math.hypot(d[0]!, d[1]!, d[2]!);
    const t = d.map((v) => v / len);
    const dotp = ref[0] * t[0]! + ref[1] * t[1]! + ref[2] * t[2]!;
    const e = [ref[0] - dotp * t[0]!, ref[1] - dotp * t[1]!, ref[2] - dotp * t[2]!];
    const el = Math.hypot(e[0]!, e[1]!, e[2]!);
    const x = e.map((v) => v / el);
    const y = [t[1]! * x[2]! - t[2]! * x[1]!, t[2]! * x[0]! - t[0]! * x[2]!, t[0]! * x[1]! - t[1]! * x[0]!];
    const [m00, m11, m22] = [x[0]!, y[1]!, t[2]!];
    const trace = m00 + m11 + m22;
    let quaternion: [number, number, number, number];
    if (trace > 0) {
      const k = 2 * Math.sqrt(trace + 1);
      quaternion = [(y[2]! - t[1]!) / k, (t[0]! - x[2]!) / k, (x[1]! - y[0]!) / k, k / 4];
    } else if (m00 > m11 && m00 > m22) {
      const k = 2 * Math.sqrt(1 + m00 - m11 - m22);
      quaternion = [k / 4, (y[0]! + x[1]!) / k, (t[0]! + x[2]!) / k, (y[2]! - t[1]!) / k];
    } else if (m11 > m22) {
      const k = 2 * Math.sqrt(1 + m11 - m00 - m22);
      quaternion = [(y[0]! + x[1]!) / k, k / 4, (t[1]! + y[2]!) / k, (t[0]! - x[2]!) / k];
    } else {
      const k = 2 * Math.sqrt(1 + m22 - m00 - m11);
      quaternion = [(t[0]! + x[2]!) / k, (t[1]! + y[2]!) / k, k / 4, (x[1]! - y[0]!) / k];
    }
    return quaternion.map((v) => Math.round(v * 1e6) / 1e6) as [number, number, number, number];
  });
}
function loft(id: string, bone: string, material: string, role: string, tone: number, pts: readonly Station[]): Json {
  const at = pts.map((p) => local(bone, p));
  const rotation = sectionRotations(at);
  return { id, kind: "loft", bone, material, role, tone, sections: pts.map((p, i) => ({ at: at[i], rx: r5(p[3]), ry: r5(p[3]), exponent: 2, rotation: rotation[i] })), capStart: true, capEnd: true };
}
function ellipsoid(id: string, bone: string, material: string, role: string, tone: number, c: V3, r: V3): Json {
  return { id, kind: "ellipsoid", bone, material, role, tone, radii: r.map(r5), transform: { translation: local(bone, c), rotation: quat(bone) } };
}
function tubeSolid(t: Tube): Json {
  if (t.slender === true) return { id: t.id, kind: "capsule", bone: t.bone, material: "oak-bark", role: "detail", tone: 0.28, p0: local(t.bone, t.pts[0]!), p1: local(t.bone, t.pts[1]!), r0: t.pts[0]![3], r1: t.pts[1]![3], ...(t.texture === true ? { role: "texture", texture: true } : { slender: true }) };
  return loft(t.id, t.bone, "oak-bark", "body", t.region === "trunk-bark" ? 0.27 : 0.28, refine(t.pts, t.sub));
}
const lobeSolid = (l: Lobe): Json => ellipsoid(l.id, l.bone, l.shade ? "oak-leaf-shade" : "oak-leaf-sun", "body", l.shade ? 0.4 : l.main ? 0.55 : 0.6, l.c, l.r);

/** 💧️ The outlet patch of a transpiration emitter: a lens seated in the top of a lobe, its rim on the lobe surface (ruling D2). */
function lens(id: string, emitter: string, lobe: Lobe, rho: number, proud: number): Json {
  const a = (lobe.r[0] + lobe.r[2]) / 2;
  const k = Math.min(rho, 0.8 * a);
  const drop = lobe.r[1] * (1 - Math.sqrt(1 - (k * k) / (a * a)));
  return { ...ellipsoid(id, lobe.bone, lobe.shade ? "oak-leaf-shade" : "oak-leaf-sun", "detail", 0.5, [lobe.c[0], lobe.c[1] + lobe.r[1] - drop, lobe.c[2]], [k, drop + proud, k]), region: lobe.shade ? "leaf-shade" : "leaf-sun", tags: [emitter] };
}

function memberStations(bone: string, scale: number): Station[] {
  const axis = bone === "root-plate" ? ([[0, 0, 0, 0.5], [0, 0.4, 0, 0.5]] as Station[]) : axisOf(bone, scale)!;
  const pick = scale === 50 || axis.length < 3 || (scale >= 200 && /^(limb|br-[we]-)/.test(bone)) ? axis : scale === 500 ? [axis[0]!, axis[axis.length - 1]!] : [axis[0]!, axis[Math.floor((axis.length - 1) / 2)]!, axis[axis.length - 1]!];
  const sec = SECTION_R[bone]!;
  return pick.map((p, i) => [p[0], p[1], p[2], Math.max(scale === 500 ? 0.04 : 0.025, Math.min(0.95 * p[3], sec * (1 - (0.2 * i) / Math.max(1, pick.length - 1))))] as Station);
}

function volumeOf(scale: number): Json {
  const tree = treeOf(scale);
  const body: Json[] = [...tree.tubes.map(tubeSolid), ...tree.lobes.map(lobeSolid)];
  const thermal: Json[] = [...tree.tubes.filter((t) => t.texture !== true).map((t) => ({ id: `th-${t.id}`, of: t.id, region: t.region })), ...tree.lobes.map((l) => ({ id: `th-${l.id}`, of: l.id, region: l.shade ? "leaf-shade" : "leaf-sun" }))];
  const lobe = (id: string): Lobe => tree.lobes.find((l) => l.id === id)!;
  const proud = FEATURE[scale]! / 2;
  if (scale === 500) {
    SUN_500.forEach((id, i) => thermal.push(lens(`th-transpiration-sun-${i + 1}`, "transpiration-sun", lobe(id), 1.2, proud)));
    thermal.push(lens("th-transpiration-shade-1", "transpiration-shade", lobe("crown-core"), 1.0, proud));
  } else {
    SUN.forEach((id, i) => thermal.push(lens(`th-transpiration-sun-${i + 1}`, "transpiration-sun", lobe(`fol-${id}`), 1.2, proud)));
    if (scale === 200) thermal.push(lens("th-transpiration-shade-1", "transpiration-shade", lobe("fol-core"), 1.0, proud));
    else [...tree.lobes.filter((l) => l.id.startsWith("fol-core-")).slice(0, scale === 50 ? 4 : 2), lobe("fol-core")].forEach((l, i) => thermal.push(lens(`th-transpiration-shade-${i + 1}`, "transpiration-shade", l, 1.0, proud)));
  }
  const structure: Json[] = [];
  const feature = FEATURE[scale]!;
  for (const b of asset.rig.bones as Json[]) {
    if (SECTION_R[b.id] !== undefined) {
      const st = memberStations(b.id, scale);
      structure.push({ ...loft(`member-${b.id}`, b.id, "oak-wood", "body", 0.4, scale === 50 ? refine(st, 2) : st), member: b.id, part: "member" });
    } else {
      const f = FRAMES[b.id]!;
      const tip: V3 = [f.o[0] + b.lengthM * Math.cos(f.ang), f.o[1] + b.lengthM * Math.sin(f.ang), f.o[2]];
      const r = r5(Math.max(0.03, 0.6 * feature));
      structure.push({ id: `member-${b.id}`, kind: "capsule", bone: b.id, material: "oak-wood", role: "body", tone: 0.4, p0: local(b.id, f.o), p1: local(b.id, tip), r0: r, r1: r, member: b.id, part: "member" });
    }
  }
  if (scale <= 200)
    for (const t of boughs()) {
      const st = (scale === 50 ? t.pts : scale === 100 ? [t.pts[0]!, t.pts[2]!, t.pts[4]!] : [t.pts[0]!, t.pts[4]!]).map((p) => [p[0], p[1], p[2], Math.max(0.025, 0.8 * p[3])] as Station);
      structure.push({ ...loft(`member-${t.id}`, t.bone, "oak-wood", "body", 0.4, scale === 50 ? refine(st, 2) : st), member: t.bone, part: "member" });
    }
  if (scale <= 100)
    for (const b of asset.rig.bones as Json[]) {
      if (b.parent === null || (scale === 100 && SECTION_R[b.id] === undefined)) continue;
      const r = SECTION_R[b.id] === undefined ? 0.035 : Math.min(0.34, Math.max(0.06, 1.15 * memberStations(b.id, scale)[0]![3]));
      structure.push({ id: `joint-${b.id}`, kind: "ellipsoid", bone: b.id, material: "oak-wood", role: "detail", tone: 0.35, radii: [r5(r), r5(r), r5(r)], transform: { translation: [0, 0, 0] }, member: b.id, part: "joint" });
    }
  const thick = Math.max(0.04, feature);
  const [f0, f1] = [tree.tubes[0]!.pts[0]!, tree.tubes[0]!.pts[1]!];
  const plate = r5(f0[3] + ((f1[3] - f0[3]) * thick) / f1[1] - 0.01);
  structure.push({ id: "support-base", kind: "revolve", bone: "root-plate", material: "oak-wood", role: "ground", tone: 0.3, profile: [[0, 0], [0, plate], [thick, plate], [thick, 0]], axis: [1, 0, 0], origin: [0, 0, 0], member: "root-plate", part: "support", anchor: "base-support" });
  const mass = (bone: string, r: number): Json => ({ id: `mass-${bone}`, kind: "ellipsoid", bone, material: "oak-wood", role: "detail", tone: 0.3, radii: [r, r, r], transform: { translation: [BONE[bone].mass.comX, 0, 0] }, member: bone, part: "mass" });
  structure.push(mass("trunk", 0.33));
  if (scale <= 100) structure.push(mass("trunk-upper", 0.24));
  return { status: "authored", denominator: scale, body, thermal, structure, decisions: DECISIONS[scale]!(body.length, thermal.length, structure.length) };
}

const DECISIONS: Record<number, (b: number, t: number, s: number) => Json[]> = {
  50: (b, t, s) => [
    { en: `Body at 1:50 (${b} solids): the oak is built as it grows. A flared root collar with seven buttress roots carries a bole of 0.80 m diameter that swells into the fork at 3.4 m; the stem continues as upper trunk, sinuous leader and leader top. Four scaffold limbs leave the stem at the rig joints and swing out of the drawing plane (the low west limb and the upper east limb towards the left of the asset, the other two towards the right), each ending in a tapering branch; two ascending branches rise from the low limbs to the far sides, two branches fork from the leader, and one secondary bough leaves each scaffold limb towards the side its limb does not reach, so the twelve woody arms fill every direction of the plan. Eighteen bark ridges, 28 mm wide and 10 mm proud, stand on the bole between the fissures of old oak bark. Every limb is a smooth tapered loft through hand-stated stations and begins inside its parent, so sway never opens a joint. Twenty branchlets of 60 mm reach from the outer branches into the foliage. The crown is seventeen clumps in three tiers on the eight foliage carriers: six on the low limbs, four on the mid limbs, two on the front and back of the middle, three on the leader, around a shaded inner crown of two masses. Each clump is a main lobe, wider than tall, with six to eleven bosses, and tufts close the foliage around and below the branch tips; together they give the broad lobed dome, the tiers and the few sky windows of an open-grown oak.`, de: `Körper bei 1:50 (${b} Körper): Die Eiche ist so gebaut, wie sie wächst. Ein ausladender Wurzelhals mit sieben Wurzelanläufen trägt einen Schaft von 0,80 m Durchmesser, der zur Gabelung in 3,4 m Höhe anschwillt; der Stamm setzt sich als oberer Stamm, geschwungene Stammverlängerung und Wipfelstück fort. Vier Starkäste verlassen den Stamm an den Gelenken des Skeletts und schwenken aus der Zeichenebene (der untere Westast und der obere Ostast zur linken Seite des Assets, die beiden anderen zur rechten), jeder endet in einem sich verjüngenden Ast; zwei aufsteigende Äste wachsen von den unteren Starkästen zu den abgewandten Seiten, zwei Äste gabeln sich von der Stammverlängerung, und je ein Nebenast verlässt jeden Starkast zu der Seite, die dieser nicht erreicht, sodass die zwölf Holzarme jede Richtung des Grundrisses füllen. Achtzehn Borkenleisten, 28 mm breit und 10 mm vorstehend, sitzen am Schaft zwischen den Rissen alter Eichenborke. Jeder Ast ist ein glatter, verjüngter Loft durch von Hand gesetzte Stationen und beginnt im Inneren seines Trägers, sodass das Schwingen kein Gelenk öffnet. Zwanzig Zweige von 60 mm reichen von den äußeren Ästen ins Laub. Die Krone besteht aus siebzehn Laubballen in drei Etagen an den acht Laubträgern: sechs an den unteren Starkästen, vier an den mittleren, zwei vorn und hinten in der Mitte, drei an der Stammverlängerung, um eine beschattete Innenkrone aus zwei Massen. Jeder Ballen ist ein Hauptkörper, breiter als hoch, mit sechs bis elf Buckeln, und Büschel schließen das Laub um die Astspitzen und darunter; zusammen ergeben sie die breite gelappte Kuppel, die Etagen und die wenigen Himmelsfenster einer freistehenden Eiche.` },
    { en: `Thermal at 1:50 (${t} solids): the bark of root collar, buttress roots, bole, upper trunk and leader is the trunk-bark region, the bark of limbs, branches and branchlets the limb-bark region, each by reference to its body solid; the fifteen outer clumps are the sun leaves, the two inner masses are the shade leaves (exchange areas, coverage only). No region has layers. The transpiration emitters are twelve outlet patches: a lens seated flush in the top of the main clump of each sun-leaf carrier (2.4 m) and five lenses on the core clump (2.0 m, narrower where the lobe is smaller), at the anchors moved onto the new crown surface.`, de: `Thermik bei 1:50 (${t} Körper): Die Rinde von Wurzelhals, Wurzelanläufen, Schaft, oberem Stamm und Stammverlängerung ist die Region Stammrinde, die Rinde von Starkästen, Ästen und Zweigen die Region Astrinde, jeweils als Verweis auf den Körper; die fünfzehn äußeren Laubballen sind die Sonnenblätter, die zwei inneren Massen die Schattenblätter (Austauschflächen, nur Abdeckung). Keine Region hat Schichten. Die Transpirationsemitter sind zwölf Austrittsflächen: je eine bündig in die Oberseite des Hauptballens jedes Sonnenlaubträgers eingelassene Linse (2,4 m) und fünf Linsen auf dem Kernballen (2,0 m, schmaler, wo der Buckel kleiner ist), an den auf die neue Kronenoberfläche verschobenen Ankern.` },
    { en: `Structure at 1:50 (${s} solids): the load path is the wood itself, idealised: one tapered round member per bone along the true axis of that stem or limb with the section diameter of the rig (root collar 0.92 m, bole 0.80 m, upper trunk 0.60 m, leader 0.38 m, limbs 0.36 to 0.29 m, branches 0.17 and 0.14 m), never thicker than the bark surface, and the four secondary boughs as thinner members of their limb; each foliage carrier is a 60 mm rod from the branch tip to the centre of its foliage mass, where the wind load acts. Joint spheres sit at the rig joints, the fixed support is the root plate disc on the ground, and the two heaviest segments (bole 1430 kg, upper trunk 700 kg) carry a mass sphere at their centre of mass.`, de: `Tragwerk bei 1:50 (${s} Körper): Der Lastpfad ist das Holz selbst, idealisiert: je Knochen ein verjüngter Rundstab entlang der wahren Achse dieses Stammes oder Astes mit dem Querschnittsdurchmesser des Skeletts (Wurzelhals 0,92 m, Schaft 0,80 m, oberer Stamm 0,60 m, Stammverlängerung 0,38 m, Starkäste 0,36 bis 0,29 m, Äste 0,17 und 0,14 m), nie dicker als die Rindenoberfläche, dazu die vier Nebenäste als dünnere Stäbe ihres Starkastes; jeder Laubträger ist ein Stab von 60 mm von der Astspitze zum Mittelpunkt seiner Laubmasse, wo die Windlast angreift. Gelenkkugeln sitzen an den Gelenken des Skeletts, das eingespannte Auflager ist die Wurzeltellerscheibe auf dem Boden, und die beiden schwersten Abschnitte (Schaft 1430 kg, oberer Stamm 700 kg) tragen eine Massenkugel in ihrem Schwerpunkt.` },
  ],
  100: (b, t, s) => [
    { en: `Body at 1:100 (${b} solids), drawn anew for this scale: the buttress roots merge into one flared collar, bole, upper trunk and the leader with its top are four lofts with fewer stations, and each scaffold limb is one loft together with its branch; the four small branches and the four secondary boughs keep their stations with coarser sections, the branchlets and the bark ridges are gone. The roots keep their seven buttresses as three-station lofts. Every clump keeps its main lobe and the bosses that shape an outline of the crown in elevation, front or plan; the bosses that only fill the inside of the crown are left out (about one in three), so the drawn outline of 1:100 is the outline of 1:50 and the tufts at the branch tips still close the underside.`, de: `Körper bei 1:100 (${b} Körper), für diesen Maßstab neu gezeichnet: Die Wurzelanläufe verschmelzen zu einem ausladenden Wurzelhals, Schaft, oberer Stamm und die Stammverlängerung mit ihrem Wipfelstück sind vier Lofts mit weniger Stationen, und jeder Starkast ist zusammen mit seinem Ast ein Loft; die vier kleinen Äste und die vier Nebenäste behalten ihre Stationen mit gröberen Querschnitten, die Zweige und die Borkenleisten entfallen. Die Wurzeln behalten ihre sieben Anläufe als Lofts mit drei Stationen. Jeder Laubballen behält seinen Hauptkörper und die Buckel, die einen Umriss der Krone in Ansicht, Vorderansicht oder Draufsicht formen; die Buckel, die nur das Kroneninnere füllen, entfallen (etwa jeder dritte), sodass der gezeichnete Umriss von 1:100 der Umriss von 1:50 ist und die Büschel an den Astspitzen die Unterseite weiterhin schließen.` },
    { en: `Thermal at 1:100 (${t} solids): the four regions by reference to this scale's body solids (trunk bark, limb bark, sun leaves, shade leaves); the twelve transpiration outlets stay as lenses in the tops of the sun clumps and of the core clump.`, de: `Thermik bei 1:100 (${t} Körper): die vier Regionen als Verweise auf die Körper dieses Maßstabs (Stammrinde, Astrinde, Sonnenblätter, Schattenblätter); die zwölf Transpirationsaustritte bleiben als Linsen in den Oberseiten der Sonnenballen und des Kernballens.` },
    { en: `Structure at 1:100 (${s} solids): one round member per bone with three stations along the true axis and the rig's section diameter, three-station members for the secondary boughs, 60 mm rods for the foliage carriers, joint spheres only at the woody joints, the root plate disc as fixed support and the mass spheres of bole and upper trunk.`, de: `Tragwerk bei 1:100 (${s} Körper): je Knochen ein Rundstab mit drei Stationen entlang der wahren Achse und dem Querschnittsdurchmesser des Skeletts, Stäbe mit drei Stationen für die Nebenäste, Stäbe von 60 mm für die Laubträger, Gelenkkugeln nur an den Holzgelenken, die Wurzeltellerscheibe als eingespanntes Auflager und die Massenkugeln von Schaft und oberem Stamm.` },
  ],
  200: (b, t, s) => [
    { en: `Body at 1:200 (${b} solids), drawn anew: collar, bole, upper trunk and leader are four short lofts, the four scaffold limbs run with their branches as single lofts of four or five stations, the ascending branches and the secondary boughs are straight two-station lofts, the leader branches are folded into the foliage, and each of the seventeen clumps keeps its main lobe with only the bosses that carry the scallops of the outline: bosses were left out one at a time, smallest first, while each moved the outline by less than 0.2 m, so about half remain and the tufts close the five branch tips.`, de: `Körper bei 1:200 (${b} Körper), neu gezeichnet: Wurzelhals, Schaft, oberer Stamm und Stammverlängerung sind vier kurze Lofts, die vier Starkäste laufen mit ihren Ästen als einzelne Lofts mit vier oder fünf Stationen, die aufsteigenden Äste und die Nebenäste sind gerade Lofts mit zwei Stationen, die Äste der Stammverlängerung gehen im Laub auf, und jeder der siebzehn Laubballen behält seinen Hauptkörper mit nur den Buckeln, die die Wellen des Umrisses tragen: Buckel wurden einzeln, der kleinste zuerst, weggelassen, solange jeder den Umriss um weniger als 0,2 m verschob, sodass etwa die Hälfte bleibt und die Büschel die fünf Astspitzen schließen.` },
    { en: `Thermal at 1:200 (${t} solids): the four regions by reference to this scale's body solids; one outlet lens in the top of the main clump of each sun-leaf carrier and one in the top of the core clump stand for the two transpiration emitters.`, de: `Thermik bei 1:200 (${t} Körper): die vier Regionen als Verweise auf die Körper dieses Maßstabs; je eine Austrittslinse in der Oberseite des Hauptballens jedes Sonnenlaubträgers und eine in der Oberseite des Kernballens stehen für die beiden Transpirationsemitter.` },
    { en: `Structure at 1:200 (${s} solids): one round member per bone along this scale's own limb axis, two-station members for the secondary boughs, rods for the foliage carriers, joints merged into the members, the root plate disc as support and the mass sphere of the bole.`, de: `Tragwerk bei 1:200 (${s} Körper): je Knochen ein Rundstab entlang der Astachse dieses Maßstabs, Stäbe mit zwei Stationen für die Nebenäste, Stäbe für die Laubträger, Gelenke in den Stäben aufgegangen, die Wurzeltellerscheibe als Auflager und die Massenkugel des Schafts.` },
  ],
  500: (b, t, s) => [
    { en: `Body at 1:500 (${b} solids), drawn anew: one flared root loft and one stem loft into the crown, the scaffold limbs as lofts of their main stations, the two ascending branches and the four boughs as straight two-station lofts at least 80 mm thick where they show below the crown, and the crown as one mass on the leader: the seventeen clump lobes in three tiers with the few bosses that make the dozen scallops of its outline (each left-out boss moved the 1:200 outline by less than 0.4 m), around the shaded inner crown, whose upper mass is swollen to close the specks between the top clumps, while the three sky windows of the plan stay open; as wide as tall, not a ball.`, de: `Körper bei 1:500 (${b} Körper), neu gezeichnet: ein ausladender Wurzelloft und ein Stammloft bis in die Krone, die Starkäste als Lofts ihrer Hauptstationen, die zwei aufsteigenden Äste und die vier Nebenäste als gerade Lofts mit zwei Stationen und mindestens 80 mm Dicke, wo sie unter der Krone sichtbar sind, und die Krone als eine Masse an der Stammverlängerung: die siebzehn Hauptkörper der Laubballen in drei Etagen mit den wenigen Buckeln, die das Dutzend Wellen ihres Umrisses bilden (jeder weggelassene Buckel verschob den Umriss von 1:200 um weniger als 0,4 m), um die beschattete Innenkrone, deren obere Masse so vergrößert ist, dass sie die Fehlstellen zwischen den oberen Ballen schließt, während die drei Himmelsfenster der Draufsicht offen bleiben; so breit wie hoch, keine Kugel.` },
    { en: `Thermal at 1:500 (${t} solids): stem and limbs by reference as trunk bark and limb bark, the outer clump lobes and bosses as sun leaves and the lobes of the inner crown as shade leaves; one outlet lens on seven sun lobes and one on the core.`, de: `Thermik bei 1:500 (${t} Körper): Stamm und Äste als Verweise für Stammrinde und Astrinde, die Körper und Buckel der äußeren Laubballen als Sonnenblätter und die Körper der Innenkrone als Schattenblätter; je eine Austrittslinse auf sieben Sonnenkörpern und eine auf dem Kern.` },
    { en: `Structure at 1:500 (${s} solids): the model is already minimal: every bone the structural view loads keeps one straight round member between its real joints (at least 80 mm thick), with the root plate disc as support and the mass sphere of the bole; the skeleton stays the branching frame inside the crown, never the crown itself.`, de: `Tragwerk bei 1:500 (${s} Körper): Das Modell ist bereits minimal: Jeder Knochen, den die Tragwerksansicht belastet, behält einen geraden Rundstab zwischen seinen wirklichen Gelenken (mindestens 80 mm dick), mit der Wurzeltellerscheibe als Auflager und der Massenkugel des Schafts; das Skelett bleibt das verzweigte Gerüst in der Krone, nie die Krone selbst.` },
  ],
};

const proj = (view: View, p: readonly number[]): [number, number, number] => (view === "elevation" ? [p[0]!, p[1]!, -p[2]!] : view === "front" ? [p[2]!, p[1]!, p[0]!] : [p[0]!, p[2]!, p[1]!]);
type Ell = { a: number; b: number; ra: number; rb: number; d: number; rd: number; lobe: Lobe };
const ellOf = (view: View, l: Lobe): Ell => {
  const [a, b, d] = proj(view, l.c);
  const [ra, rb, rd] = view === "elevation" ? [l.r[0], l.r[1], l.r[2]] : view === "front" ? [l.r[2], l.r[1], l.r[0]] : [l.r[0], l.r[2], l.r[1]];
  return { a, b, ra, rb, d, rd, lobe: l };
};
const qOf = (e: Ell, x: number, y: number): number => ((x - e.a) / e.ra) ** 2 + ((y - e.b) / e.rb) ** 2;
type Cap = { x: number; y: number; ux: number; uy: number; r: number; k: number };
type Prim = { kind: "ell"; e: Ell } | { kind: "tube"; pts: [number, number, number][]; caps: Cap[] };

function field(prims: readonly Prim[], x: number, y: number, h: number): number {
  let best = -1e9;
  for (const p of prims) {
    if (p.kind === "ell") {
      const v = (1 - Math.sqrt(qOf(p.e, x, y))) * Math.min(p.e.ra, p.e.rb);
      if (v > best) best = v;
      continue;
    }
    const n = p.pts.length;
    let body = -1e9;
    for (let i = 0; i + 1 < n; i++) {
      const [ax, ay, ar] = p.pts[i]!;
      const [bx, by, br] = p.pts[i + 1]!;
      const [dx, dy] = [bx - ax, by - ay];
      const l2 = dx * dx + dy * dy;
      const tc = l2 < 1e-10 ? 0 : Math.max(0, Math.min(1, ((x - ax) * dx + (y - ay) * dy) / l2));
      body = Math.max(body, ar + (br - ar) * tc - Math.hypot(x - ax - dx * tc, y - ay - dy * tc));
    }
    for (const c of p.caps) {
      const along = (x - c.x) * c.ux + (y - c.y) * c.uy;
      const across = -(x - c.x) * c.uy + (y - c.y) * c.ux;
      body = Math.min(body, along);
      if (c.r * c.k > 2 * h) best = Math.max(best, (1 - Math.hypot(along / (c.r * c.k), across / c.r)) * c.r * c.k);
    }
    if (body > best) best = body;
  }
  return best;
}

/** 🗺️ The zero contour of a field as closed rings (marching squares with linear interpolation). */
function contours(prims: readonly Prim[], h: number): P[][] {
  let [x0, y0, x1, y1] = [1e9, 1e9, -1e9, -1e9];
  for (const p of prims) {
    const box = p.kind === "ell" ? [[p.e.a - p.e.ra, p.e.b - p.e.rb], [p.e.a + p.e.ra, p.e.b + p.e.rb]] : p.pts.flatMap(([x, y, r]) => [[x - 1.1 * r, y - 1.1 * r], [x + 1.1 * r, y + 1.1 * r]]);
    for (const [x, y] of box) [x0, y0, x1, y1] = [Math.min(x0, x!), Math.min(y0, y!), Math.max(x1, x!), Math.max(y1, y!)];
  }
  x0 -= 2.5 * h;
  y0 -= 2.5 * h;
  const nx = Math.ceil((x1 - x0) / h) + 3;
  const ny = Math.ceil((y1 - y0) / h) + 3;
  const w = nx + 1;
  const val = new Float64Array(w * (ny + 1));
  for (let j = 0; j <= ny; j++) for (let i = 0; i <= nx; i++) val[j * w + i] = field(prims, x0 + i * h, y0 + j * h, h);
  const pts = new Map<number, P>();
  const link = new Map<number, number[]>();
  const edge = (i: number, j: number, vertical: boolean): number => {
    const key = (j * w + i) * 2 + (vertical ? 1 : 0);
    if (!pts.has(key)) {
      const a = val[j * w + i]!;
      const b = vertical ? val[(j + 1) * w + i]! : val[j * w + i + 1]!;
      const t = a / (a - b);
      pts.set(key, vertical ? [x0 + i * h, y0 + (j + t) * h] : [x0 + (i + t) * h, y0 + j * h]);
    }
    return key;
  };
  const join2 = (a: number, b: number): void => {
    link.set(a, [...(link.get(a) ?? []), b]);
    link.set(b, [...(link.get(b) ?? []), a]);
  };
  for (let j = 0; j < ny; j++)
    for (let i = 0; i < nx; i++) {
      const c = (val[j * w + i]! > 0 ? 1 : 0) | (val[j * w + i + 1]! > 0 ? 2 : 0) | (val[(j + 1) * w + i + 1]! > 0 ? 4 : 0) | (val[(j + 1) * w + i]! > 0 ? 8 : 0);
      if (c === 0 || c === 15) continue;
      const [B, R, Tp, L] = [(): number => edge(i, j, false), (): number => edge(i + 1, j, true), (): number => edge(i, j + 1, false), (): number => edge(i, j, true)];
      if (c === 1 || c === 14) join2(L(), B());
      else if (c === 2 || c === 13) join2(B(), R());
      else if (c === 3 || c === 12) join2(L(), R());
      else if (c === 4 || c === 11) join2(R(), Tp());
      else if (c === 6 || c === 9) join2(B(), Tp());
      else if (c === 7 || c === 8) join2(L(), Tp());
      else if (c === 5) {
        join2(L(), Tp());
        join2(B(), R());
      } else {
        join2(L(), B());
        join2(R(), Tp());
      }
    }
  const seen = new Set<number>();
  const rings: P[][] = [];
  for (const start of link.keys()) {
    if (seen.has(start)) continue;
    const ring: P[] = [];
    let [prev, cur] = [-1, start];
    while (!seen.has(cur)) {
      seen.add(cur);
      ring.push(pts.get(cur)!);
      const next = (link.get(cur) ?? []).find((k) => k !== prev && !seen.has(k));
      if (next === undefined) break;
      [prev, cur] = [cur, next];
    }
    if (ring.length >= 8) rings.push(ring);
  }
  return rings;
}

const areaOf = (r: readonly P[]): number => r.reduce((s, p, i) => s + p[0] * r[(i + 1) % r.length]![1] - r[(i + 1) % r.length]![0] * p[1], 0) / 2;
function simplifyOpen(pts: readonly P[], tol: number): P[] {
  if (pts.length < 3) return [...pts];
  const [a, b] = [pts[0]!, pts[pts.length - 1]!];
  const [dx, dy] = [b[0] - a[0], b[1] - a[1]];
  const len = Math.hypot(dx, dy) || 1e-12;
  let [worst, at] = [0, 0];
  for (let i = 1; i < pts.length - 1; i++) {
    const d = Math.abs((pts[i]![0] - a[0]) * dy - (pts[i]![1] - a[1]) * dx) / len;
    if (d > worst) [worst, at] = [d, i];
  }
  if (worst <= tol) return [a, b];
  return [...simplifyOpen(pts.slice(0, at + 1), tol).slice(0, -1), ...simplifyOpen(pts.slice(at), tol)];
}
function simplifyRing(ring: readonly P[], tol: number): P[] {
  let far = 0;
  ring.forEach((p, i) => {
    if (Math.hypot(p[0] - ring[0]![0], p[1] - ring[0]![1]) > Math.hypot(ring[far]![0] - ring[0]![0], ring[far]![1] - ring[0]![1])) far = i;
  });
  return [...simplifyOpen(ring.slice(0, far + 1), tol).slice(0, -1), ...simplifyOpen([...ring.slice(far), ring[0]!], tol).slice(0, -1)];
}
function outline(prims: readonly Prim[], scale: number): P[][] {
  const h = FEATURE[scale]! / 2;
  const rings = contours(prims, h)
    .filter((r) => Math.abs(areaOf(r)) > 40 * h * h)
    .map((r) => simplifyRing(r, 0.4 * FEATURE[scale]!));
  return rings.sort((p, q) => Math.abs(areaOf(q)) - Math.abs(areaOf(p)));
}

type Scene = { scale: number; tubes: Tube[]; lobes: Lobe[] };
function tubePrim(view: View, t: Tube): Prim {
  const st = refine(t.pts, t.sub);
  const cap = (a: Station, b: Station): Cap => {
    const [pa, pb] = [proj(view, a), proj(view, b)];
    const d = [pb[0] - pa[0], pb[1] - pa[1], pb[2] - pa[2]];
    const flat = Math.hypot(d[0]!, d[1]!) || 1e-9;
    return { x: pa[0], y: pa[1], ux: d[0]! / flat, uy: d[1]! / flat, r: a[3], k: Math.abs(d[2]!) / Math.hypot(d[0]!, d[1]!, d[2]!) };
  };
  return { kind: "tube", pts: st.map((p) => [proj(view, p)[0], proj(view, p)[1], p[3]]), caps: [cap(st[0]!, st[1]!), cap(st[st.length - 1]!, st[st.length - 2]!)] };
}

/** ✒️ The interior arcs of a foliage mass in a view: rims of its lobes seen against a lobe behind and hidden by none in front. */
function arcsOf(scene: Scene, view: View, bone: string): P[][] {
  const all = scene.lobes.map((l) => ellOf(view, l));
  const runs: P[][] = [];
  for (const e of all.filter((x) => x.lobe.bone === bone)) {
    let run: P[] = [];
    const flush = (): void => {
      if (run.length >= 12) {
        const cut = Math.max(0, Math.floor((run.length - 34) / 2));
        runs.push(run.slice(cut, run.length - cut).filter((_, i) => i % 3 === 0));
      }
      run = [];
    };
    for (let k = 0; k <= 480; k++) {
      const t = ((k % 360) * Math.PI) / 180 + 2.1;
      const [x, y] = [e.a + e.ra * Math.cos(t), e.b + e.rb * Math.sin(t)];
      let [interior, hidden] = [false, false];
      for (const o of all) {
        if (o === e) continue;
        const q = qOf(o, x, y);
        if (q >= 1) continue;
        if (o.d + o.rd * Math.sqrt(1 - q) > e.d + 0.02) hidden = true;
        else if (q < 0.92) interior = true;
      }
      if (interior && !hidden && k < 360 + run.length) run.push([x, y]);
      else flush();
      if (k >= 360 && run.length === 0) break;
    }
    flush();
  }
  return runs.sort((p, q) => q.length - p.length);
}

const PATTERN_A: P[] = [[-0.5, 0.3], [0.2, 0.5], [0.5, 0], [-0.15, -0.35], [0.35, -0.45], [-0.55, -0.15]];
const PATTERN_B: P[] = [[0, 0.1], [-0.3, 0.6], [0.6, 0.35], [0.1, -0.6], [-0.65, 0.1], [0.3, 0.15]];
/** 🍃️ Leaf marks of a foliage mass in a view: small strokes at stated places on its lobes, kept where that lobe is the nearest surface. */
function marksOf(scene: Scene, view: View, bone: string, pattern: readonly P[], count: number, size: number): P[][] {
  const all = scene.lobes.map((l) => ellOf(view, l));
  const own = all.filter((x) => x.lobe.bone === bone);
  const out: P[][] = [];
  for (let k = 0; k < pattern.length && out.length < count; k++)
    own.forEach((e, n) => {
      if (out.length >= count) return;
      const [fx, fy] = pattern[(k + n) % pattern.length]!;
      const [x, y] = [e.a + fx * e.ra, e.b + fy * e.rb];
      const depth = e.d + e.rd * Math.sqrt(Math.max(0, 1 - qOf(e, x, y)));
      for (const o of all) {
        const q = qOf(o, x, y);
        if (o !== e && q < 1 && o.d + o.rd * Math.sqrt(1 - q) > depth + 1e-6) return;
      }
      const s = Math.min(size, 0.3 * Math.min(e.ra, e.rb));
      const turn = ((k * 50 + n * 110) * Math.PI) / 180;
      const at = (u: number, v: number): P => [x + u * Math.cos(turn) - v * Math.sin(turn), y + u * Math.sin(turn) + v * Math.cos(turn)];
      out.push([at(-s, 0), at(0, 0.7 * s), at(s, 0)]);
    });
  return out;
}
const quadratic = (m: readonly P[], n: number): P[] => Array.from({ length: n }, (_, i) => {
  const t = i / (n - 1);
  return [(1 - t) ** 2 * m[0]![0] + 2 * t * (1 - t) * m[1]![0] + t * t * m[2]![0], (1 - t) ** 2 * m[0]![1] + 2 * t * (1 - t) * m[1]![1] + t * t * m[2]![1]] as P;
});

function redrawScale(scale: number): void {
  const tree = treeOf(scale);
  const scene: Scene = { scale, tubes: tree.tubes, lobes: tree.lobes };
  const rep = asset.representations[String(scale)];
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const fmt = (p: P, bone: string): P => {
      const q = view === "elevation" ? toLocal(FRAMES[bone]!, [p[0], p[1], 0]) : p;
      return [r4(q[0]!), r4(q[1]!)];
    };
    const path = (lines: readonly (readonly P[])[], bone: string, close: boolean): string => lines.map((l) => l.map((p, i) => `${i === 0 ? "M" : "L"} ${fmt(p, bone).join(" ")}`).join(" ") + (close ? " Z" : "")).join(" ");
    const curves = (marks: readonly (readonly P[])[], bone: string): string => marks.map((m) => `M ${fmt(m[0]!, bone).join(" ")} Q ${fmt(m[1]!, bone).join(" ")} ${fmt(m[2]!, bone).join(" ")}`).join(" ");
    const shapes: Json[] = rep.geometry[view].filter((s: Json) => !/plug[01]$/.test(s.id) && !/^crown-fill-/.test(s.id));
    const filled = (s: Json): boolean => s.role === "body" && s.filled === true && s.kind !== "ellipse";
    const drawnBones = new Set<string>(shapes.filter(filled).map((s) => s.bone));
    const owner = (bone: string): string | null => {
      let b: string | null = bone;
      while (b !== null && !drawnBones.has(b)) b = PARENT[b] ?? null;
      return b ?? (bone === "root-plate" && drawnBones.has("trunk") ? "trunk" : null);
    };
    const first = new Map<string, string>();
    for (const s of shapes.filter(filled)) if (!first.has(s.bone)) first.set(s.bone, s.id);
    const arcs = new Map<string, P[][]>();
    const arcsFor = (bone: string): P[][] => arcs.get(bone) ?? arcs.set(bone, arcsOf(scene, view, bone)).get(bone)!;
    const setGeometry = (s: Json, geometry: Json): Json => {
      const { d: _d, points: _p, closed: _c, penMm: _pen, ...rest } = s;
      const keepPen = s.role !== "body" && s.penMm !== undefined ? { penMm: s.penMm } : {};
      return { ...rest, ...geometry, ...keepPen, ...(s.followTip === true ? { followTip: true } : {}) };
    };
    const open = (s: Json, lines: readonly (readonly P[])[]): Json => (s.kind === "poly" ? setGeometry(s, { points: lines[0]!.map((p) => fmt(p, s.bone)), closed: false }) : setGeometry(s, { d: path(lines, s.bone, false) }));
    const next: Json[] = [];
    for (const s of shapes) {
      const fol = String(s.bone).startsWith("fol-") || (scale === 500 && s.id === "crown");
      if (filled(s)) {
        if (first.get(s.bone) !== s.id) continue;
        const prims: Prim[] = [...scene.lobes.filter((l) => l.bone === s.bone).map((l) => ({ kind: "ell", e: ellOf(view, l) }) as Prim), ...scene.tubes.filter((t) => t.slender !== true && owner(t.bone) === s.bone).map((t) => tubePrim(view, t))];
        const rings = outline(prims, scale);
        if (rings.length === 0) throw new Error(`${scale} ${view} ${s.id}: no outline`);
        const region = s.region ?? (s.regions === undefined ? (String(s.bone).startsWith("br-") || String(s.bone).startsWith("limb-") ? "limb-bark" : "trunk-bark") : undefined);
        const base = region === undefined ? s : { ...s, region };
        next.push(s.kind === "poly" && (s.followTip === true || rings.length === 1) ? setGeometry(base, { points: rings[0]!.map((p) => fmt(p, s.bone)), closed: true }) : setGeometry({ ...base, kind: "path" }, { d: path(rings, s.bone, true) }));
        continue;
      }
      const arcN = /-arc-?(\d)(-100)?$/.exec(s.id);
      const leafN = /-leaves-(\d)$/.exec(s.id);
      if (fol && /-arcs$/.test(s.id)) {
        if (arcsFor(s.bone).length > 0) next.push(open(s, arcsFor(s.bone).slice(0, 5)));
      }
      else if (fol && arcN !== null) {
        const a = arcsFor(s.bone)[Number(arcN[1]) - 1];
        if (a !== undefined) next.push(open(s, [a]));
      } else if (fol && leafN !== null) {
        const m = marksOf(scene, view, s.bone, PATTERN_A, 4, 0.4)[Number(leafN[1]) - 1];
        if (m !== undefined) next.push(open(s, [quadratic(m, 6)]));
      } else if (fol && /-leaves$/.test(s.id)) {
        if (marksOf(scene, view, s.bone, PATTERN_A, 12, 0.17).length > 0) next.push(setGeometry(s, { d: curves(marksOf(scene, view, s.bone, PATTERN_A, 12, 0.17), s.bone) }));
      }
      else if (fol && /-fine$/.test(s.id)) {
        if (marksOf(scene, view, s.bone, PATTERN_B, 12, 0.11).length > 0) next.push(setGeometry(s, { d: curves(marksOf(scene, view, s.bone, PATTERN_B, 12, 0.11), s.bone) }));
      }
      else if (/^twigs-/.test(s.id)) next.push(open(s, scene.tubes.filter((t) => t.slender === true && t.texture !== true && t.bone === s.bone).map((t) => t.pts.map((p) => [proj(view, p)[0], proj(view, p)[1]] as P))));
      else if (/^bark-limb-/.test(s.id)) {
        const axis = refine(axisOf(s.bone)!, 3).map((p) => [proj(view, p)[0], proj(view, p)[1], p[3]] as [number, number, number]);
        const stroke = (f0: number, f1: number, side: number): P[] => [f0, (f0 + f1) / 2, f1].map((f, i) => {
          const k = Math.round(f * (axis.length - 1));
          const [a, b] = [axis[Math.max(0, k - 1)]!, axis[Math.min(axis.length - 1, k + 1)]!];
          const len = Math.hypot(b[0] - a[0], b[1] - a[1]) || 1;
          const off = side * axis[k]![2] * (i === 1 ? 0.5 : 0.3);
          return [axis[k]![0] - ((b[1] - a[1]) / len) * off, axis[k]![1] + ((b[0] - a[0]) / len) * off] as P;
        });
        next.push(setGeometry(s, { d: curves([stroke(0.18, 0.42, 1), stroke(0.5, 0.74, -1), stroke(0.66, 0.9, 1)], s.bone) }));
      } else if (view === "elevation" && (s.id === "bark-trunk" || s.id === "bark-trunk-fine" || s.id === "bark-upper")) {
        const k = s.id === "bark-upper" ? 0.8 : 0.88;
        let n = 0;
        next.push({ ...s, d: String(s.d).replace(/-?\d+(\.\d+)?/g, (v) => (n++ % 2 === 1 ? String(r4(Number(v) * k)) : v)) });
      } else if (s.id === "root-ridges") next.push({ ...s, d: "M 0.42 0.37 C 0.3 0.42 0.15 0.6 0.06 0.97 M 0.42 -0.37 C 0.3 -0.43 0.16 -0.62 0.06 -0.98 M 0.43 0.04 C 0.3 0.07 0.16 0.05 0.03 0.1" });
      else if (/^hint-/.test(s.id)) next.push(open(s, [refine(axisOf(s.bone)!, 2).map((p) => [p[0], p[2]] as P)]));
      else if (s.kind === "capsule" && /^plan-/.test(s.id)) {
        const axis = axisOf(s.bone)!;
        const [a, b] = [axis[0]!, axis[axis.length - 1]!];
        next.push({ ...s, x0: r4(a[0]), y0: r4(a[2]), x1: r4(b[0]), y1: r4(b[2]), r0: r4(a[3]), r1: r4(b[3]) });
      } else next.push(s);
    }
    const masses = next.filter((s) => filled(s) && String(s.bone).startsWith("fol-"));
    const depthOf = (bone: string): number => {
      const ls = scene.lobes.filter((l) => l.bone === bone);
      return ls.reduce((sum, l) => sum + proj(view, l.c)[2], 0) / Math.max(1, ls.length);
    };
    const zs = masses.map((s) => s.z as number).sort((a, b) => a - b);
    const order = [...masses].sort((a, b) => depthOf(a.bone) - depthOf(b.bone));
    const shift = new Map<string, number>();
    order.forEach((s, i) => shift.set(s.bone, zs[i]! - s.z));
    for (const s of next) {
      const d = shift.get(s.bone);
      if (d === undefined || d === 0) continue;
      if (filled(s) || (view === "elevation" && scale === 50 && s.z < 5)) s.z = r4(s.z + d);
    }
    rep.geometry[view] = next;
  }
}

mkdirSync(OUT, { recursive: true });
const volumes: Json = {};
for (const scale of SCALES) volumes[String(scale)] = volumeOf(scale);
writeFileSync(join(T, "volumes/vegetation/english-oak.json"), JSON.stringify({ volumes }));
for (const scale of SCALES) redrawScale(scale);
asset.representations["500"].decisions.silhouette = { en: "The English oak is composed as a broad, lobed dome crown in three tiers whose underside rises over the bole between the two low clumps; 1 m is 2 mm, so only the overall form is drawn, with one restrained closed outline and no internal structure. Front and plan: Front: the bole with the low limbs, their ascending branches and boughs under one crown outline of about a dozen scallops. Plan: one scalloped crown outline from above; bole and limbs are hidden.", de: "Die Stieleiche ist als breite, gelappte Kuppelkrone in drei Etagen komponiert, deren Unterseite zwischen den zwei unteren Laubballen über dem Schaft ansteigt; 1 m sind 2 mm, daher wird nur die Gesamtform mit einem zurückhaltenden geschlossenen Umriss und ohne innere Struktur gezeichnet. Vorder- und Draufsicht: Vorderansicht: der Schaft mit den unteren Starkästen, ihren aufsteigenden Ästen und Nebenästen unter einem Kronenumriss aus etwa einem Dutzend Wellen. Draufsicht: ein gewellter Kronenumriss von oben; Schaft und Äste sind verdeckt." };
asset.representations["500"].decisions.retained = { en: "Retained: the bole with its root flare, the two lowest limbs and the ascending branches and boughs where they show below the crown.", de: "Beibehalten: der Schaft mit Wurzelanlauf, die zwei untersten Starkäste sowie die aufsteigenden Äste und Nebenäste, soweit sie unter der Krone sichtbar sind." };
asset.representations["50"].decisions.merged = { en: "Nothing is merged at this scale: every woody part begins inside its parent, so the joints stay closed in every pose without plug pieces; each foliage mass is the outline of its clump of lobes.", de: "In diesem Maßstab wird nichts zusammengefasst: Jedes Holzteil beginnt im Inneren seines Trägers, sodass die Gelenke in jeder Pose ohne Passstücke geschlossen bleiben; jede Laubmasse ist der Umriss ihres Ballens aus Buckeln." };

const anchorAt = (id: string, p: V3): void => {
  const a = asset.anchors.find((x: Json) => x.id === id);
  const q = toLocal(FRAMES[a.bone]!, p);
  [a.x, a.y] = [r4(q[0]), r4(q[1])];
  if (Math.abs(q[2]) > 1e-9) a.z = r4(q[2]);
  else delete a.z;
};
SUN.forEach((id, i) => anchorAt(`tp-sun-${i + 1}`, clumpTop(id).top));
[1, 2, 3, 4, 0].forEach((k, i) => anchorAt(`tp-shade-${i + 1}`, clumpTop("core", k).top));

asset.volumes = volumes;
writeFileSync(join(OUT, "🔣️.json"), JSON.stringify(doc, null, 2));
writeFileSync(join(OUT, "roots.json"), JSON.stringify({ rig: asset.rig, anchors: asset.anchors, representations: asset.representations }));

for (const scale of SCALES) {
  const tree = treeOf(scale);
  const lo = [1e9, 1e9, 1e9];
  const hi = [-1e9, -1e9, -1e9];
  for (const l of tree.lobes) for (let k = 0; k < 3; k++) [lo[k], hi[k]] = [Math.min(lo[k]!, l.c[k]! - l.r[k]!), Math.max(hi[k]!, l.c[k]! + l.r[k]!)];
  for (const t of tree.tubes) for (const p of t.pts) for (let k = 0; k < 3; k++) [lo[k], hi[k]] = [Math.min(lo[k]!, p[k]! - (k === 1 ? 0 : p[3])), Math.max(hi[k]!, p[k]! + (k === 1 ? 0 : p[3]))];
  const area = (region: string): number => tree.tubes.filter((t) => t.region === region).reduce((sum, t) => sum + refine(t.pts, 3).slice(1).reduce((a, p, i, arr) => {
    const q = i === 0 ? refine(t.pts, 3)[0]! : arr[i - 1]!;
    return a + Math.PI * (p[3] + q[3]) * Math.hypot(p[0] - q[0], p[1] - q[1], p[2] - q[2]);
  }, 0), 0);
  const v = volumes[String(scale)];
  console.log(`1:${scale} body ${v.body.length} thermal ${v.thermal.length} structure ${v.structure.length} | x ${lo[0]!.toFixed(2)}..${hi[0]!.toFixed(2)} (-9.09..9.26) y ${lo[1]!.toFixed(2)}..${hi[1]!.toFixed(2)} (0..18.226) z ${lo[2]!.toFixed(2)}..${hi[2]!.toFixed(2)} (-7.50..7.53) | tube area trunk ${area("trunk-bark").toFixed(1)} limb ${area("limb-bark").toFixed(1)}`);
  for (const view of ["elevation", "front", "plan"]) console.log(`   ${view}: ${asset.representations[String(scale)].geometry[view].length} shapes, ${JSON.stringify(asset.representations[String(scale)].geometry[view]).length} chars`);
}
