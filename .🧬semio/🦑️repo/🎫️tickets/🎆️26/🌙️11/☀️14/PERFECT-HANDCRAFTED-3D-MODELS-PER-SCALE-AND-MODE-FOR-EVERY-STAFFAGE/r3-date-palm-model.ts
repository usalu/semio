/** 🌴️ Round 3 date palm: applies the root fixes (rig, poses, clips, loads, anchors, masses, texts), builds the hand-stated palm of every scale (`r3-date-palm-parts.ts`) into `body`, `thermal` and `structure`, redraws every 2D shape as the true view of that scale's body (ruling E1) and writes the candidate document `🗑️generated/r3-date-palm/🔣️.json` and the source `volumes/vegetation/date-palm.json`.
 * Usage: `bun r3-date-palm-model.ts [--no-2d]`
 * @see ./round3-brief.md
 * @see ./r3-vegetation-date-palm.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { GENERATED, STAFFAGE, TICKET, entryOf, type Frame, type Json, type P2, type V3, type View } from "./volumes-lib.ts";
import { forward } from "./r3-project2d.ts";
import { blobOutline, ellipsoidOf, hullOf, localPoint, loftOf, nearness, outline, pathOf, pointsOf, project, r4, r5, sub, add, mul, dot, cross, unit, type Tube } from "./r3-date-palm-kit.ts";
import { fixRig } from "./r3-date-palm-rig-fix.ts";
import { FrondGeom, build, type Parts, type Scale } from "./r3-date-palm-parts.ts";

const { solveChain } = (await import(join(STAFFAGE, "🔨️modules/🦴️rig/🟦️.ts"))) as { solveChain: (rig: Json, offsets: Json, root: Json) => Record<string, { frame: Frame }> };

const SCALES: Scale[] = [50, 100, 200, 500];
const VIEWS: View[] = ["elevation", "front", "plan"];
const text = (en: string, de: string) => ({ en, de });
const say = (line: string) => process.stdout.write(`${line}\n`);

const { entry } = entryOf("vegetation/date-palm");
const doc = JSON.parse(readFileSync(entry.path, "utf8"));
const asset = doc.asset;
const { fronds, report } = fixRig(asset);
for (const line of report) say(line);
asset.thermal.regions.find((r: Json) => r.id === "stem-bark").areaM2 = 15.8;
const calm = asset.poses.find((p: Json) => p.id === asset.presentation.defaultPose);
const solved = solveChain(asset.rig, calm.bones, { x: 0, y: 0, angleDeg: 0 });
const F: Record<string, Frame> = Object.fromEntries(Object.entries(solved).map(([id, s]) => [id, s.frame]));
const anchorFronds = [...fronds.filter((f) => f.tier === "A" || f.tier === "B"), ...fronds.filter((f) => f.tier === "C").slice(0, 2)].map((f) => f.id);
const frondBone = (bone: string) => /^fr-[lr]\d+[ab]$/.test(bone);

const rotateAbout = (p: V3, o: V3, axis: V3, ang: number): V3 => {
  const d = sub(p, o);
  const k = unit(axis);
  const [c, s] = [Math.cos(ang), Math.sin(ang)];
  return add(o, add(add(mul(d, c), mul(cross(k, d), s)), mul(k, dot(k, d) * (1 - c))));
};

/** 🧱️ The world-space copies of a tube that carries a rotational repeat about its bone's u axis. */
function copies(t: Tube): Tube[] {
  const rep = t.fields.repeat;
  if (rep === undefined) return [t];
  const f = F[t.bone]!;
  const axis = add(add(mul(f.u, rep.rotationAxis[0]), mul(f.v, rep.rotationAxis[1])), mul(f.w, rep.rotationAxis[2]));
  return Array.from({ length: rep.count }, (_, k) => ({ ...t, sections: t.sections.map((s) => ({ ...s, p: rotateAbout(s.p, f.o, axis, rep.rotationRad * k), x: rotateAbout(s.x, [0, 0, 0], axis, rep.rotationRad * k), y: rotateAbout(s.y, [0, 0, 0], axis, rep.rotationRad * k) })) }));
}

const capsuleRing = (view: View, p0: V3, p1: V3, r0: number, r1: number): P2[] => {
  const ring = (c: V3, r: number) => Array.from({ length: 16 }, (_, i) => {
    const a = (2 * Math.PI * i) / 16 + 0.07;
    const q = project(view, c);
    return { x: q.x + r * Math.cos(a), y: q.y + r * Math.sin(a) };
  });
  return hullOf([...ring(p0, r0), ...ring(p1, r1)]);
};

/** 🧊️ Body, thermal and structure lists of one scale. */
function volumesOf(scale: Scale, P: Parts): Json {
  const feature = (0.15 * scale) / 1000;
  const body: Json[] = [];
  for (const t of P.tubes) body.push(loftOf(t, F[t.bone]!));
  for (const r of P.rods) body.push({ id: r.id, kind: "capsule", bone: r.bone, ...r.fields, p0: localPoint(F[r.bone]!, r.p0), p1: localPoint(F[r.bone]!, r.p1), r0: r5(r.r0), r1: r5(r.r1) });
  for (const b of P.blobs) body.push(ellipsoidOf(b, F[b.bone]!));
  body.push(...P.repeats);
  const thermal: Json[] = Object.entries(P.regions).map(([id, region]) => ({ id: `th-${id}`, of: id, region }));
  for (const e of P.emitters) thermal.push({ ...loftOf(e.tube, F[e.bone]!), region: "fronds", tags: ["transpiration-fronds"] });
  const structure: Json[] = [];
  const member = (id: string, bone: string, sections: { u: number; r: number }[], slender = false) => {
    const f = F[bone]!;
    structure.push({ id, kind: "loft", bone, material: "palm-stem", role: "body", tone: 0.5, member: bone, part: "member", sections: sections.map((s) => ({ at: [r5(s.u), 0, 0], rx: r5(s.r), ry: r5(s.r), rotation: [0.70711, 0, 0.70711, 0].map((x) => x) })), capStart: true, capEnd: true, ...(slender ? { slender: true } : {}) });
    void f;
  };
  member("m-root-plate", "root-plate", [{ u: 0, r: 0.33 }, { u: 0.15, r: 0.29 }, { u: 0.3, r: 0.262 }]);
  member("m-stem-1", "stem-1", [{ u: 0, r: 0.25 }, { u: 3, r: 0.236 }]);
  member("m-stem-2", "stem-2", [{ u: 0, r: 0.232 }, { u: 2.7, r: 0.224 }]);
  member("m-stem-3", "stem-3", [{ u: 0, r: 0.222 }, { u: 2.4, r: 0.212 }]);
  member("m-crown", "crown", [{ u: 0, r: 0.2 }, { u: 0.6, r: 0.2 }]);
  for (const f of fronds) {
    const g = new FrondGeom(f, F[`${f.id}a`]!, F[`${f.id}b`]!);
    for (const half of ["a", "b"] as const) {
      const bone = `${f.id}${half}`;
      const [s0, s1] = half === "a" ? [scale >= 200 ? 0.3 : 0.12, f.La + 0.03] : [f.La - 0.03, g.L - 0.05];
      const n = scale >= 200 ? 3 : 7;
      const sections = Array.from({ length: n + 1 }, (_, i) => s0 + ((s1 - s0) * i) / n).map((s) => {
        const q = g.at(s, half);
        return { p: q.p, x: q.n, y: q.w, a: Math.max(0.7 * g.rd(s), feature * 0.55), b: Math.max(0.7 * g.rw(s), feature * 0.55) };
      });
      const tube: Tube = { id: `m-${bone}`, bone, sections, fields: { material: "palm-stem", role: "body", tone: 0.5 } };
      structure.push({ ...loftOf(tube, F[bone]!), member: bone, part: "member" });
    }
  }
  for (const bunch of ["bunch-l", "bunch-r"]) {
    const stalk = P.tubes.find((t) => t.id === `${bunch}-stalk`)!;
    const core: Tube = { id: `m-${bunch}`, bone: bunch, sections: stalk.sections.map((s) => ({ ...s, a: Math.min(s.a, Math.max(0.011, feature * 0.55)), b: Math.min(s.b, Math.max(0.016, feature * 0.55)) })), fields: { material: "palm-stem", role: "body", tone: 0.5 } };
    structure.push({ ...loftOf(core, F[bunch]!), member: bunch, part: "member" });
  }
  const joint = (id: string, bone: string, at: V3, radii: V3) => structure.push({ id, kind: "ellipsoid", bone, material: "palm-stem", role: "detail", tone: 0.5, member: bone, part: "joint", radii: radii.map(r5), transform: { translation: at.map(r5) } });
  for (const [bone, r] of [["stem-1", 0.25], ["stem-2", 0.232], ["stem-3", 0.222], ["crown", 0.2]] as [string, number][]) joint(`j-${bone}`, bone, [0, 0, 0], [Math.max(feature * 0.6, 0.04), r * 0.92, r * 0.92]);
  if (scale !== 500) {
    for (const f of fronds) {
      joint(`j-${f.id}a`, `${f.id}a`, [0, 0, 0], [Math.max(0.035, feature * 0.6), Math.max(0.035, feature * 0.6), Math.max(0.035, feature * 0.6)]);
      const rj = Math.max(0.02, feature * 0.55);
      joint(`j-${f.id}b`, `${f.id}b`, [0, 0, 0], [rj, rj, rj]);
    }
    for (const bunch of ["bunch-l", "bunch-r"]) joint(`j-${bunch}`, bunch, [0, 0, 0], [0.04, 0.04, 0.04].map((x) => Math.max(x, feature * 0.6)) as V3);
  }
  const t = Math.max(feature, 0.0075);
  structure.push({ id: "s-base-support", kind: "revolve", bone: "root-plate", material: "palm-stem", role: "ground", tone: 0.5, member: "root-plate", part: "support", anchor: "base-support", axis: [1, 0, 0], origin: [0, 0, 0], profile: [[-r5(t), 0], [-r5(t), 0.6], [0, 0.6], [0, 0]] });
  const massOf = (bone: string): Json => asset.rig.bones.find((b: Json) => b.id === bone).mass;
  for (const [bone, r] of [["root-plate", 0.13], ["stem-1", 0.15], ["stem-2", 0.145], ["stem-3", 0.14], ["crown", 0.12], ["bunch-l", 0.1], ["bunch-r", 0.1]] as [string, number][]) {
    const m = massOf(bone);
    structure.push({ id: `mass-${bone}`, kind: "ellipsoid", bone, material: "palm-stem", role: "detail", tone: 0.5, member: bone, part: "mass", radii: [r, r, r].map((x) => r5(Math.max(x, feature * 0.6))), transform: { translation: [r5(m.comX), r5(m.comY), 0] } });
  }
  return { body, thermal, structure };
}

/** 📝️ The decisions of each scale (en first, de second). */
const DECISIONS: Record<Scale, { en: string; de: string }[]> = {
  50: [
    text("1:50 shows the palm in full: a root collar with fourteen pencil roots, the stem with the spiral of old leaf bases (golden-angle phyllotaxis, every leaf base a small texture block), the fibrous boot under the crown with two whorls of cut petiole stubs, two spear leaves and sixteen fronds in four tiers. Each frond is a sheath, a spiny petiole (spines as texture) and a rachis arching over its two bones, with the two leaflet vanes of the induplicate V as thin lofts and every leaflet as a texture blade whose tip stands free beyond the vane. Two fruit bunches: a flattened stalk, eleven strands and the dates on them as texture.", "1:50 zeigt die Palme vollständig: einen Wurzelhals mit vierzehn Bleistiftwurzeln, den Stamm mit der Spirale alter Blattbasen (Phyllotaxis im goldenen Winkel, jede Blattbasis ein kleiner Texturblock), den faserigen Schaft unter der Krone mit zwei Wirteln abgeschnittener Blattstielstümpfe, zwei Speerblätter und sechzehn Wedel in vier Kränzen. Jeder Wedel ist Blattscheide, bedornter Blattstiel (Dornen als Textur) und eine über seine zwei Knochen gebogene Rhachis, mit den zwei Fiederflächen des V als dünne Lofts und jeder Fieder als Texturblatt, dessen Spitze frei über die Fläche hinausragt. Zwei Fruchtstände: ein abgeflachter Stiel, elf Stränge und die Datteln daran als Textur."),
    text("Thermal: stem-bark is the stem, collar, roots, boot and stubs by reference; fronds are every rachis, vane and spear leaf by reference plus twelve transpiration outlets as thin patches on the rachis of twelve fronds; fruit is stalks and strands. The texture solids carry no region. Structure: the stem as solid members of the declared sections, every rachis as a slender member along its arch, the bunch stalks, joints at every rig joint, the root ball support and the masses at their centres.", "Thermik: Stammrinde sind Stamm, Wurzelhals, Wurzeln, Schaft und Stümpfe per Verweis; Wedel sind jede Rhachis, Fiederfläche und jedes Speerblatt per Verweis sowie zwölf Transpirationsauslässe als dünne Flecken auf der Rhachis von zwölf Wedeln; Frucht sind Stiele und Stränge. Die Texturkörper tragen keine Region. Struktur: der Stamm als volle Glieder der deklarierten Querschnitte, jede Rhachis als schlankes Glied entlang ihres Bogens, die Fruchtstiele, Gelenke an jedem Riggelenk, die Auflagerung des Wurzelballens und die Massen in ihren Schwerpunkten."),
  ],
  100: [
    text("1:100 keeps the habit and merges the detail: eight roots, coarser leaf-base blocks, one spear leaf, two whorls of six stubs; fronds keep rachis and both vanes with fewer sections and leaflets every 9 cm as texture; spines and single dates are below what the scale can show (smaller than 15 mm), so each bunch is a stalk with seven strands.", "1:100 behält den Wuchs und fasst die Einzelheiten zusammen: acht Wurzeln, gröbere Blattbasenblöcke, ein Speerblatt, zwei Wirtel mit je sechs Stümpfen; die Wedel behalten Rhachis und beide Fiederflächen mit weniger Schnitten und Fiedern alle 9 cm als Textur; Dornen und einzelne Datteln liegen unter dem, was der Maßstab zeigen kann (kleiner als 15 mm), daher ist jeder Fruchtstand ein Stiel mit sieben Strängen."),
    text("Thermal and structure as at 1:50 with the merged solids; transpiration outlets on the same twelve fronds.", "Thermik und Struktur wie bei 1:50 mit den zusammengefassten Körpern; Transpirationsauslässe an denselben zwölf Wedeln."),
  ],
  200: [
    text("1:200 is a new abstraction: every frond half is one arched blade (the V of the leaflets merged into a flat blade of the scale's 30 mm minimum thickness, lifted to the middle of the V), the stem three lofts on a plain root collar, the boot with one whorl of six stubs, one spear leaf, each bunch a stalk and one hanging cluster. No texture: leaf bases and leaflets are below 30 mm.", "1:200 ist eine neue Abstraktion: jede Wedelhälfte ist ein gebogenes Blatt (das V der Fiedern zu einem flachen Blatt in der Mindestdicke des Maßstabs von 30 mm zusammengefasst, in die Mitte des V gehoben), der Stamm drei Lofts auf einem einfachen Wurzelhals, der Schaft mit einem Wirtel aus sechs Stümpfen, ein Speerblatt, jeder Fruchtstand ein Stiel und eine hängende Traube. Keine Textur: Blattbasen und Fiedern liegen unter 30 mm."),
    text("Thermal references the merged solids and keeps the twelve outlets; structure keeps the stem, the rachis members along the arches and the joints; frond base joints are 30 mm spheres.", "Die Thermik verweist auf die zusammengefassten Körper und behält die zwölf Auslässe; die Struktur behält Stamm, die Rhachisglieder entlang der Bögen und die Gelenke; die Wedelbasisgelenke sind Kugeln von 30 mm."),
  ],
  500: [
    text("1:500 keeps what reads at 75 mm: the stem as three lofts flaring into the ground, the boot as one rounded mass, the sixteen fronds as arched blades of 80 mm thickness that keep the drooping lower tier, the upright young fronds and the open star of the crown in plan, and each bunch as a stalk with one cluster. Roots, stubs, spear leaves and texture are omitted.", "1:500 behält, was bei 75 mm lesbar ist: den Stamm als drei Lofts, die in den Boden ausweiten, den Schaft als eine gerundete Masse, die sechzehn Wedel als gebogene Blätter von 80 mm Dicke, die den hängenden unteren Kranz, die aufrechten jungen Wedel und den offenen Stern der Krone im Grundriss bewahren, und jeden Fruchtstand als Stiel mit einer Traube. Wurzeln, Stümpfe, Speerblätter und Textur entfallen."),
    text("Thermal: the merged solids by reference and the twelve outlets; structure: stem, crown and rachis members, the stem joints, the support and the masses; frond joints are left out at this scale.", "Thermik: die zusammengefassten Körper per Verweis und die zwölf Auslässe; Struktur: Stamm-, Kronen- und Rhachisglieder, die Stammgelenke, das Auflager und die Massen; Wedelgelenke entfallen in diesem Maßstab."),
  ],
};

const parts = Object.fromEntries(SCALES.map((s) => [s, build(s, F, fronds, anchorFronds)])) as Record<Scale, Parts>;

for (const [i, fid] of anchorFronds.entries()) {
  const e = parts[50].emitters[i]!;
  const anchor = asset.anchors.find((a: Json) => a.id === e.anchor);
  anchor.bone = e.bone;
  anchor.x = e.anchorX;
  anchor.y = e.anchorY;
  void fid;
}
for (const bunch of ["bunch-l", "bunch-r"]) {
  const strands = parts[50].rods.filter((r) => r.id.startsWith(`${bunch}-strand-`));
  const c = mul(strands.reduce((s, r) => add(s, mul(add(r.p0, r.p1), 0.5)), [0, 0, 0] as V3), 1 / strands.length);
  const l = localPoint(F[bunch]!, c);
  const bone = asset.rig.bones.find((b: Json) => b.id === bunch);
  bone.mass.comX = r4(l[0]);
  bone.mass.comY = r4(l[1]);
}
asset.summary = text(
  "A mature female date palm (Phoenix dactylifera) of about 12.5 m: a slender, unbranched stem covered in a spiral of old leaf bases, a fibrous boot with cut petiole stubs under the crown, sixteen grey-green feather fronds in four tiers (the lowest drooping, the youngest upright, two spear leaves in the centre), each with a spiny petiole and stiff leaflets in a V, and two heavy fruit bunches hanging on their stalks below the crown.",
  "Eine ausgewachsene weibliche Dattelpalme (Phoenix dactylifera) von etwa 12,5 m: ein schlanker, unverzweigter Stamm, bedeckt von einer Spirale alter Blattbasen, ein faseriger Schaft mit abgeschnittenen Blattstielstümpfen unter der Krone, sechzehn graugrüne Fiederwedel in vier Kränzen (der unterste hängend, der jüngste aufrecht, zwei Speerblätter in der Mitte), jeder mit bedorntem Blattstiel und steifen, V-förmig gestellten Fiedern, und zwei schwere Fruchtstände, die an ihren Stielen unter der Krone hängen.",
);
asset.provenance.notes = text(
  "One unbranched stem and sixteen fronds of two bones each (petiole with lower blade, upper blade) are authored by hand, in four tiers on the crown, spaced by the golden angle of the phyllotaxis; fronds fold to the lee in wind, which is how palms survive gales. Leaflets stand in a V (induplicate) at about 40 degrees to the rachis; spines replace the lowest leaflets. A real crown of 60 to 120 green fronds is represented by sixteen, carrying the leaf area of the whole crown. Measures after Zaid and de Wet (2002, FAO) and Barreveld (1993, FAO): fronds 3 to 6 m, leaflets 20 to 50 cm, stem 30 to 50 cm with leaf bases, fruit stalks 1 to 2 m.",
  "Ein unverzweigter Stamm und sechzehn Wedel aus je zwei Knochen (Blattstiel mit unterem Blatt, oberes Blatt) sind von Hand gezeichnet, in vier Kränzen an der Krone, im goldenen Winkel der Blattstellung verteilt; die Wedel legen sich im Wind nach Lee, so überstehen Palmen Stürme. Die Fiedern stehen V-förmig (induplikat) unter etwa 40 Grad zur Rhachis; Dornen ersetzen die untersten Fiedern. Eine reale Krone mit 60 bis 120 grünen Wedeln wird durch sechzehn dargestellt, denen die Blattfläche der ganzen Krone zugeordnet ist. Maße nach Zaid und de Wet (2002, FAO) und Barreveld (1993, FAO): Wedel 3 bis 6 m, Fiedern 20 bis 50 cm, Stamm 30 bis 50 cm mit Blattbasen, Fruchtstiele 1 bis 2 m.",
);
for (const ref of [
  { id: "zaid-de-wet-2002", citation: "Zaid, A. and de Wet, P. F. (2002). Botanical and systematic description of the date palm. In: Zaid, A. (ed.), Date Palm Cultivation, FAO Plant Production and Protection Paper 156 Rev. 1, chapter 1/3. Food and Agriculture Organization, Rome." },
  { id: "barreveld-1993", citation: "Barreveld, W. H. (1993). Date Palm Products. FAO Agricultural Services Bulletin 101. Food and Agriculture Organization, Rome. Chapter 1 (the date palm: stem, leaves, inflorescences and fruit bunches)." },
]) if (!asset.provenance.references.some((r: Json) => r.id === ref.id)) asset.provenance.references.push(ref);

const volumes: Json = {};
for (const s of SCALES) volumes[String(s)] = { status: "authored", denominator: s, ...volumesOf(s, parts[s]), decisions: DECISIONS[s] };

const flatGroups = (P: Parts) => Object.entries(P.outlines).filter(([, g]) => !frondBone(g.bone));
const polysOf = (P: Parts, view: View, ids: string[]): P2[][] => ids.flatMap((id) => {
  const t = P.tubes.find((x) => x.id === id);
  if (t) return copies(t).flatMap((c) => kitTube(view, c));
  const r = P.rods.find((x) => x.id === id);
  if (r) return [capsuleRing(view, r.p0, r.p1, r.r0, r.r1)];
  const b = P.blobs.find((x) => x.id === id);
  if (b) return [blobOutline(view, b)];
  throw new Error(`no part ${id}`);
});
import { tubePolygons as kitTube } from "./r3-date-palm-kit.ts";

/** ✏️ Affine map of the paper of a view into the frame of one shape, solved from three probe points placed by the product. */
function affineFor(cand: Json, scale: number, view: View, metas: Json[]): ((p: P2) => P2)[] {
  const probes = metas.map((meta) => ({ meta, points: [{ x: 0, y: 0 }, { x: 1, y: 0 }, { x: 0, y: 1 }] }));
  const placed = forward(cand, scale, view, probes);
  return placed.map(([o, a, b]) => {
    const [m00, m10, m01, m11] = [a!.x - o!.x, a!.y - o!.y, b!.x - o!.x, b!.y - o!.y];
    const det = m00 * m11 - m01 * m10;
    return (p: P2) => {
      const [dx, dy] = [p.x - o!.x, p.y - o!.y];
      return { x: (m11 * dx - m01 * dy) / det, y: (-m10 * dx + m00 * dy) / det };
    };
  });
}


/** 🖊️ All shapes of one scale and view; flat shapes in paper coordinates first, mapped into their frames by `affineFor`. */
function drawView(cand: Json, scale: Scale, view: View): Json[] {
  const P = parts[scale];
  const feature = (0.15 * scale) / 1000;
  const tol = feature / 3;
  type Pending = { meta: Json; rings: P2[][]; closed: boolean; depth: number; kind: "outline" | "lines" };
  const pending: Pending[] = [];
  for (const [id, g] of flatGroups(P)) {
    if (g.parts.length === 0) continue;
    const rings = outline(polysOf(P, view, g.parts), tol, feature * feature).filter((r, i) => i === 0 || Math.abs(r.reduce((s, p, k) => s + p.x * r[(k + 1) % r.length]!.y - r[(k + 1) % r.length]!.x * p.y, 0)) > 0);
    const outer = rings.filter((r) => r.reduce((s, p, k) => s + p.x * r[(k + 1) % r.length]!.y - r[(k + 1) % r.length]!.x * p.y, 0) > 0);
    if (outer.length === 0) continue;
    const pts = outer.flat();
    const cx = pts.reduce((s, p) => s + p.x, 0) / pts.length;
    const partCentres = g.parts.map((pid) => {
      const t = P.tubes.find((x) => x.id === pid);
      if (t) return t.sections[Math.floor(t.sections.length / 2)]!.p;
      const r = P.rods.find((x) => x.id === pid);
      if (r) return mul(add(r.p0, r.p1), 0.5);
      return P.blobs.find((x) => x.id === pid)!.c;
    });
    const depth = partCentres.reduce((s, c) => s + nearness(view, c), 0) / partCentres.length;
    void cx;
    pending.push({ meta: { id, bone: g.bone, role: g.role, filled: true, tone: g.tone, ...(g.region ? { region: g.region } : {}), ...(g.material ? { material: g.material } : {}) }, rings: outer, closed: true, depth, kind: "outline" });
  }
  if (view !== "plan") for (const tex of P.worldTexture) {
    const { lines, depth } = tex.polys(view);
    if (lines.length === 0) continue;
    const role = tex.kind === "strand" ? "detail" : "texture";
    pending.push({ meta: { id: tex.id, bone: tex.bone, role, filled: false, tone: 0.2 }, rings: lines, closed: tex.kind !== "strand", depth: depth.length ? depth.reduce((a, b) => a + b, 0) / depth.length : 0, kind: "lines" });
  }
  const depths: number[] = [...pending.map((p) => p.depth), ...P.plates.map((p) => nearness(view, p.centre))];
  const [dmin, dmax] = [Math.min(...depths), Math.max(...depths)];
  const zOf = (d: number) => r4(1 + (4 * (d - dmin)) / Math.max(1e-6, dmax - dmin));
  const ownerZ = (bone: string) => {
    const own = pending.find((p) => p.kind === "outline" && p.meta.bone === bone);
    return own ? zOf(own.depth) : 2;
  };
  const metas = pending.map((p) => ({ ...p.meta, z: p.kind === "outline" ? zOf(p.depth) : r4(ownerZ(p.meta.bone) + 0.003) }));
  const maps = affineFor(cand, scale, view, metas);
  const shapes: Json[] = pending.map((p, i) => {
    const local = p.rings.map((r) => r.map(maps[i]!));
    const meta = metas[i]!;
    if (p.kind === "outline" && local.length === 1) return { ...meta, kind: "poly", points: pointsOf(local[0]!), closed: true };
    return { ...meta, kind: "path", d: pathOf(local, p.closed) };
  });
  for (const pl of P.plates) {
    const fg = P.outlines[`${pl.bone}-blade`];
    shapes.push({ id: pl.id, bone: pl.bone, role: "body", z: zOf(nearness(view, pl.centre)), filled: true, tone: fg?.tone ?? 0.52, region: "fronds", material: "palm-frond", kind: "poly", points: pointsOf(pl.ring), closed: true, solid: { kind: "extruded", sweep: "v", z0: r5(pl.z0), z1: r5(pl.z1), zTip: [r5(pl.zTip[0]), r5(pl.zTip[1])] } });
  }
  for (const st of P.strokes) {
    const own = P.plates.filter((x) => x.bone.slice(0, -1) === st.bone.slice(0, -1));
    const z = own.length ? Math.max(...own.map((x) => zOf(nearness(view, x.centre)))) + 0.003 : 4;
    shapes.push({ id: st.id, bone: st.bone, role: st.role, z: r4(z), filled: false, tone: 0.25, kind: "path", d: pathOf(st.lines, false), solid: { kind: "extruded", sweep: "v", z0: r5(st.z0), z1: r5(st.z1), zTip: [r5(st.zTip[0]), r5(st.zTip[1])] } });
  }
  return shapes;
}

const REP_DECISIONS: Record<Scale, Json> = {
  50: {
    silhouette: text("At 1:50 (1 m is 20 mm) every drawing is the true orthographic view of the 1:50 model in the calm pose: stem with root collar and pencil roots, the spiral of leaf bases as small outlines, the boot with its petiole stubs, two spear leaves, sixteen fronds as tilted plates (one per third of each frond half) with every leaflet and spine as a stroke, two bunches with strands and dates.", "Bei 1:50 (1 m sind 20 mm) ist jede Zeichnung die wahre Parallelprojektion des 1:50-Modells in der ruhigen Pose: Stamm mit Wurzelhals und Bleistiftwurzeln, die Spirale der Blattbasen als kleine Umrisse, der Schaft mit seinen Blattstielstümpfen, zwei Speerblätter, sechzehn Wedel als geneigte Platten (eine je Drittel jeder Wedelhälfte) mit jeder Fieder und jedem Dorn als Strich, zwei Fruchtstände mit Strängen und Datteln."),
    retained: text("Retained: every frond, leaflet, spine, leaf base visible from the viewpoint, root, stub, strand and date.", "Beibehalten: jeder Wedel, jede Fieder, jeder Dorn, jede vom Standpunkt sichtbare Blattbasis, Wurzel, jeder Stumpf, Strang und jede Dattel."),
    merged: text("The leaflets of each side of a frond are merged into one vane, drawn as the plate of that frond part; the fibre of the boot is merged into its outline.", "Die Fiedern jeder Wedelseite sind zu einer Fläche zusammengefasst, gezeichnet als Platte dieses Wedelteils; die Fasern des Schafts sind in seinen Umriss aufgenommen."),
    omitted: text("Omitted: single fibres of the boot and the flowers, below 0.15 mm on paper.", "Weggelassen: einzelne Fasern des Schafts und die Blüten, unter 0,15 mm auf dem Papier."),
    thinElements: text("Leaflets, spines, leaf bases and dates are the finest strokes (0.13 mm).", "Fiedern, Dornen, Blattbasen und Datteln sind die feinsten Striche (0,13 mm)."),
  },
  100: {
    silhouette: text("At 1:100 the drawings are the true views of the 1:100 model: stem, collar with eight roots, coarser leaf bases, boot with stubs, one spear leaf, sixteen fronds as two plates per frond half with leaflets every 9 cm as strokes, bunches with seven strands.", "Bei 1:100 sind die Zeichnungen die wahren Ansichten des 1:100-Modells: Stamm, Wurzelhals mit acht Wurzeln, gröbere Blattbasen, Schaft mit Stümpfen, ein Speerblatt, sechzehn Wedel als zwei Platten je Wedelhälfte mit Fiedern alle 9 cm als Striche, Fruchtstände mit sieben Strängen."),
    retained: text("Retained: every frond, the leaflets as strokes, leaf bases, roots, stubs and strands.", "Beibehalten: jeder Wedel, die Fiedern als Striche, Blattbasen, Wurzeln, Stümpfe und Stränge."),
    omitted: text("Omitted: spines and single dates (below 15 mm, 0.15 mm on paper).", "Weggelassen: Dornen und einzelne Datteln (unter 15 mm, 0,15 mm auf dem Papier)."),
  },
  200: {
    silhouette: text("At 1:200 the drawings are the true views of the 1:200 model: stem on a plain collar, boot with one whorl of stubs, one spear leaf, sixteen fronds as one arched plate per frond half, each bunch a stalk and a cluster.", "Bei 1:200 sind die Zeichnungen die wahren Ansichten des 1:200-Modells: Stamm auf einfachem Wurzelhals, Schaft mit einem Wirtel Stümpfe, ein Speerblatt, sechzehn Wedel als ein gebogenes Blatt je Wedelhälfte, jeder Fruchtstand ein Stiel und eine Traube."),
    retained: text("Retained: every frond, the boot and the bunches.", "Beibehalten: jeder Wedel, der Schaft und die Fruchtstände."),
  },
  500: {
    silhouette: text("At 1:500 the drawings are the true views of the 1:500 model: stem, boot, sixteen frond blades and two bunch masses.", "Bei 1:500 sind die Zeichnungen die wahren Ansichten des 1:500-Modells: Stamm, Schaft, sechzehn Wedelblätter und zwei Fruchtmassen."),
    retained: text("Retained: every frond as a blade and the bunches.", "Beibehalten: jeder Wedel als Blatt und die Fruchtstände."),
  },
};

const cand = JSON.parse(JSON.stringify(asset));
cand.volumes = volumes;
if (!process.argv.includes("--no-2d")) {
  for (const s of SCALES) for (const view of VIEWS) cand.representations[String(s)].geometry[view] = [];
  for (const s of SCALES) {
    const rep = cand.representations[String(s)];
    for (const view of VIEWS) {
      rep.geometry[view] = drawView(cand, s, view);
      say(`2D 1:${s} ${view}: ${rep.geometry[view].length} shapes`);
    }
    const { silhouette, retained, merged, omitted, thinElements } = REP_DECISIONS[s];
    rep.decisions = { ...rep.decisions, silhouette, retained, ...(merged ? { merged } : {}), ...(omitted ? { omitted } : {}), ...(thinElements ? { thinElements } : {}) };
  }
}
const out = { ...doc, asset: cand };
const candPath = join(GENERATED, "r3-date-palm", "🔣️.json");
mkdirSync(dirname(candPath), { recursive: true });
writeFileSync(candPath, `${JSON.stringify(out, null, 2)}\n`);
const srcPath = process.argv.includes("--source") ? join(TICKET, "volumes", "vegetation", "date-palm.json") : join(GENERATED, "r3-date-palm", "volumes.json");
writeFileSync(srcPath, `${JSON.stringify({ volumes }, null, 2)}\n`);
for (const s of SCALES) {
  const v = volumes[String(s)];
  const expanded = (l: Json[]) => l.reduce((n, x) => n + (x.repeat?.count ?? 1), 0);
  say(`1:${s}: body ${v.body.length} (${expanded(v.body)} expanded), thermal ${v.thermal.length}, structure ${v.structure.length}`);
}
const ext = { min: [Infinity, Infinity, Infinity], max: [-Infinity, -Infinity, -Infinity] };
for (const t of parts[50].tubes) for (const c of copies(t)) for (const sct of c.sections) for (const k of [0, 1, 2]) {
  const r = Math.max(sct.a, sct.b);
  ext.min[k] = Math.min(ext.min[k]!, sct.p[k]! - r);
  ext.max[k] = Math.max(ext.max[k]!, sct.p[k]! + r);
}
say(`1:50 tube extent x ${ext.min[0]!.toFixed(3)}..${ext.max[0]!.toFixed(3)} y ${ext.min[1]!.toFixed(3)}..${ext.max[1]!.toFixed(3)} z ${ext.min[2]!.toFixed(3)}..${ext.max[2]!.toFixed(3)} (dimensions ${asset.dimensions.widthM} x ${asset.dimensions.heightM} x ${asset.dimensions.depthM})`);
say(`written ${candPath}\nwritten ${srcPath}`);
