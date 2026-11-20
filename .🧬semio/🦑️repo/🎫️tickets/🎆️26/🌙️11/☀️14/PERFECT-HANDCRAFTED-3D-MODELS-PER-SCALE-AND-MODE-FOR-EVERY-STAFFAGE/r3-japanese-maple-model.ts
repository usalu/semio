/** 🍁️ Round-3 japanese maple: writes the hand-stated tree of `r3-japanese-maple-tree.ts` as bone-local solids per scale and list to `volumes/vegetation/japanese-maple.json`, and the root fixes (lateral bone offsets of the splayed stems, transpiration anchors on their outlets, bark areas, 2D drawings as true views of each scale's body, drawing decisions) to the candidate document `🗑️generated/r3-japanese-maple/🔣️.json`.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-japanese-maple.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, SOURCES, entryOf, frames, toLocal, toWorld, type Frame, type Json, type V3 } from "./volumes-lib.ts";
import { add, depthIn, dirOf, dot3, lobeOutline, mul, outline, pathOf, pointsOf, project, r4, r5, solidOf, sub, surfacePoint, tubePolygons, unit, type Lobe, type Part, type Tube, type View } from "./r3-japanese-maple-kit.ts";
import { STEMS, STEMS_100, STEMS_200, STEMS_500, SUN, SHADE, boneOffsets, sprays, stemPoint, stemV, wood, type Spray } from "./r3-japanese-maple-tree.ts";
import { draw } from "./r3-japanese-maple-draw.ts";
import { DECISIONS, DRAWN } from "./r3-japanese-maple-texts.ts";

const { entry } = entryOf("vegetation/japanese-maple");
const document: Json = JSON.parse(readFileSync(entry.path, "utf8"));
const asset: Json = document.asset;
const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);

const offsets = boneOffsets();
for (const b of asset.rig.bones) {
  delete b.z;
  if (offsets[b.id] !== undefined && offsets[b.id] !== 0) {
    const rel = offsets[b.id]! - (offsets[b.parent] ?? 0);
    const { id, parent, title, x, y, ...rest } = b;
    for (const k of Object.keys(b)) delete b[k];
    Object.assign(b, { id, parent, title, x, y, z: Math.round(rel * 1e4) / 1e4, ...rest });
  }
}
const F: Record<string, Frame> = frames(asset);

const AREAS: Record<string, number> = { "stem-bark": 3.45, "limb-bark": 2.32 };
for (const region of asset.thermal.regions) if (AREAS[region.id] !== undefined) region.areaM2 = AREAS[region.id];

type Outlet = [emitter: string, anchor: string | null, spray: string, az: number, el: number];
const OUTLETS: Record<number, Outlet[]> = {
  50: [
    ["transpiration-sun", "tp-sun-1", "t1-o7", 180, 90], ["transpiration-sun", "tp-sun-2", "t1-o1", 0, 90], ["transpiration-sun", "tp-sun-3", "t2-o6", 180, 90], ["transpiration-sun", "tp-sun-4", "t2-o1", 0, 90],
    ["transpiration-sun", "tp-sun-5", "t3-o5", 180, 90], ["transpiration-sun", "tp-sun-6", "t3-o1", 0, 90], ["transpiration-sun", "tp-sun-7", "top-cap", 0, 90],
    ["transpiration-shade", "tp-shade-1", "core-low-1", 0, 20], ["transpiration-shade", "tp-shade-2", "core-low-3", 120, 20], ["transpiration-shade", "tp-shade-3", "core-low-5", 240, 20],
    ["transpiration-shade", "tp-shade-4", "core-up-1", 45, 20], ["transpiration-shade", "tp-shade-5", "core-up-3", 225, 20],
  ],
  100: [["transpiration-sun", null, "t1-o7", 180, 90], ["transpiration-sun", null, "t2-o1", 0, 90], ["transpiration-sun", null, "t3-o3", 90, 90], ["transpiration-sun", null, "top-cap", 0, 90], ["transpiration-shade", null, "core-low-1", 30, 20], ["transpiration-shade", null, "core-up-1", 90, 20]],
  200: [["transpiration-sun", null, "t2-o4", 180, 90], ["transpiration-sun", null, "top-cap", 0, 90], ["transpiration-shade", null, "core-low-1", 90, 20]],
  500: [["transpiration-sun", null, "top-cap", 0, 90], ["transpiration-shade", null, "t1-x2", 90, -90]],
};
const EMITTER_D: Record<string, number> = Object.fromEntries(asset.thermal.emitters.map((e: Json) => [e.id, e.diameterM]));
const EMITTER_REGION: Record<string, string> = Object.fromEntries(asset.thermal.emitters.map((e: Json) => [e.id, e.region]));

type Model = { scale: number; wood: Tube[]; twigs: Tube[]; hidden: Tube[]; lobes: Spray[] };
function model(scale: number): Model {
  const lobes = sprays(scale);
  const { wood: all, twigs } = wood(scale, F, lobes);
  const hidden = scale === 500 ? all.filter((t) => t.group === "limb") : [];
  return { scale, wood: all.filter((t) => !hidden.includes(t)), twigs, hidden, lobes };
}
export const models = [50, 100, 200, 500].map(model);

for (const m of models) for (const t of [...m.wood, ...m.twigs]) {
  if (t.group !== "limb" && t.group !== "twig") continue;
  const tip = t.points[t.points.length - 1]!.p;
  const depth = Math.min(...m.lobes.map((l) => depthIn(l, tip)));
  if (depth > 0.8) console.log(`[DEBUG] 1:${m.scale} tip of ${t.id} at ${tip.map((v) => v.toFixed(2)).join(" ")} is ${depth > 1 ? "outside" : "shallow in"} the foliage (${depth.toFixed(2)})`);
}
const centroidOf = (bone: string): V3 => {
  const set = models[0]!.lobes.filter((l) => l.bone === bone);
  return mul(set.reduce((s, l) => add(s, l.c), [0, 0, 0] as V3), 1 / set.length);
};
const anchorsOut: Record<string, { bone: string; local: V3 }> = {};

function volumeOf(m: Model): Json {
  const s = (p: Part): Json => solidOf(p, F);
  const feature = (0.15 * m.scale) / 1000;
  const body: Json[] = [...m.wood.map(s), ...m.twigs.map(s), ...m.lobes.map(s)];
  const region = (p: Tube | Lobe): string => (p.kind === "lobe" ? (p.group === "shade" ? "leaf-shade" : "leaf-sun") : p.group === "limb" ? "limb-bark" : "stem-bark");
  const thermal: Json[] = [];
  for (const p of [...m.wood, ...m.lobes]) thermal.push({ id: `th-${p.id}`, of: p.id, region: region(p) });
  for (const p of m.hidden) thermal.push({ ...s({ ...p, id: `th-${p.id}` }), slender: true, region: region(p) });
  OUTLETS[m.scale]!.forEach(([emitter, anchor, sprayId, az, el], i) => {
    const l = m.lobes.find((x) => x.id === sprayId);
    if (!l) throw new Error(`1:${m.scale} outlet on unknown spray ${sprayId}`);
    const d = dirOf(az, el);
    const p = surfacePoint(l, d);
    const q = sub(p, l.c);
    const n = unit(l.axes.reduce((sum, axis, k) => add(sum, mul(axis, dot3(q, axis) / l.radii[k]! ** 2)), [0, 0, 0] as V3));
    const seed: V3 = Math.abs(n[1]) > 0.98 ? [1, 0, 0] : [n[2], 0, -n[0]];
    const side = unit(sub(seed, mul(n, dot3(seed, n))));
    const third: V3 = [side[1] * n[2] - side[2] * n[1], side[2] * n[0] - side[0] * n[2], side[0] * n[1] - side[1] * n[0]];
    const rho = Math.min(EMITTER_D[emitter]! / 2, 0.6 * Math.min(l.radii[0], l.radii[2]));
    const h = Math.max(feature, (rho * rho) / (2 * Math.min(...l.radii)) * 0.8, 0.06);
    const lens: Lobe = { ...l, id: `th-${anchor ?? `${emitter}-${i + 1}`}`, c: sub(p, mul(n, h - 0.02)), axes: [side, n, third], radii: [rho, h, rho] };
    thermal.push({ ...s(lens), role: "detail", region: EMITTER_REGION[emitter]!, tags: [emitter] });
    const out = add(p, mul(n, 0.03));
    const buried = m.lobes.find((o) => o !== l && Math.hypot(...o.axes.map((axis, k) => dot3(sub(out, o.c), axis) / o.radii[k]!)) < 1);
    if (buried) console.log(`[DEBUG] 1:${m.scale} outlet on ${sprayId} is buried in ${buried.id}`);
    if (anchor) anchorsOut[anchor] = { bone: l.bone, local: toLocal(F[l.bone]!, add(p, mul(n, 0.04))) };
  });
  const structure: Json[] = [];
  const bark = (r: number): number => 0.002 + 0.04 * r;
  const members = [...m.wood, ...m.hidden].filter((t) => t.group !== "twig" && t.id !== "root-1" && !/^root-\d/.test(t.id));
  for (const t of members) {
    const declared = Math.sqrt(bone(t.bone).section.areaM2 / Math.PI);
    const pts = m.scale === 500 && t.group !== "collar" ? [t.points[0]!, t.points[t.points.length - 1]!].map((q) => ({ p: q.p, r: declared })) : t.points.map((q) => ({ p: q.p, r: q.r - bark(q.r) }));
    const slender = m.scale === 500 && t.group === "limb" ? { slender: true } : undefined;
    structure.push({ ...s({ ...t, id: `member-${t.id}`, material: "maple-wood", tone: 0.4, points: pts, flags: slender } as Tube), member: t.bone, part: "member" });
  }
  for (const b of new Set(members.map((t) => t.bone))) {
    if (b === "root-plate") continue;
    const t = members.find((x) => x.bone === b && /^(limb-t\d[ew]|wood-.*)$/.test(x.id))!;
    const f = F[b]!;
    const first = t.points.find((q) => toLocal(f, q.p)[0] >= -1e-6) ?? t.points[0]!;
    const r = (m.scale === 500 ? Math.sqrt(bone(b).section.areaM2 / Math.PI) : first.r - bark(first.r)) * 1.12;
    if (2 * r < feature) continue;
    structure.push({ id: `joint-${b}`, kind: "ellipsoid", bone: b, material: "maple-wood", role: "detail", tone: 0.35, radii: [r5(r), r5(r), r5(r)], transform: { translation: toLocal(f, first.p).map(r5) }, member: b, part: "joint" });
  }
  const plate = Math.max(0.04, feature);
  structure.push({ id: "support-base-support", kind: "revolve", bone: "root-plate", material: "maple-wood", role: "detail", tone: 0.25, profile: [[r5(-plate), 0.6], [0, 0.6]], axis: [1, 0, 0], origin: [0, 0, 0], member: "root-plate", part: "support", anchor: "base-support" });
  for (const b of asset.rig.bones as Json[]) {
    if (!b.mass) continue;
    const f = F[b.id]!;
    if (b.id.startsWith("fol-")) {
      const r = Math.max(feature * 0.6, Math.cbrt((3 * b.mass.kg) / (4 * Math.PI * 600)));
      structure.push({ id: `mass-${b.id}`, kind: "ellipsoid", bone: b.id, material: b.id.startsWith("fol-core") ? SHADE : SUN, role: "detail", tone: 0.3, radii: [r5(r), r5(r * 0.6), r5(r)], transform: { translation: toLocal(f, centroidOf(b.id)).map(r5) }, member: b.parent, part: "mass" });
      continue;
    }
    const t = members.find((x) => x.bone === b.id && /^(limb-t\d[ew]|wood-.*|root-collar)$/.test(x.id));
    if (!t) continue;
    const com: V3 = STEMS[b.id] ? stemPoint(F, b.id, b.mass.comX) : toWorld(f, [b.mass.comX, b.mass.comY, 0]);
    const near = t.points.reduce((best, q) => (Math.hypot(...sub(q.p, com)) < Math.hypot(...sub(best.p, com)) ? q : best));
    const c: V3 = STEMS[b.id] ? com : near.p;
    const rr = Math.max(feature * 0.55, (near.r - bark(near.r)) * 0.8);
    structure.push({ id: `mass-${b.id}`, kind: "ellipsoid", bone: b.id, material: "maple-wood", role: "detail", tone: 0.3, radii: [r5(STEMS[b.id] ? Math.max(rr, Math.min(0.12, b.lengthM * 0.06)) : Math.min(rr * 1.6, b.lengthM * 0.4)), r5(rr), r5(rr)], transform: { translation: toLocal(f, c).map(r5) }, member: b.id, part: "mass" });
  }
  return { status: "authored", denominator: m.scale, body, thermal, structure, decisions: DECISIONS[m.scale]! };
}

const volumes: Json = {};
for (const m of models) volumes[String(m.scale)] = volumeOf(m);
writeFileSync(join(SOURCES, "vegetation", "japanese-maple.json"), JSON.stringify({ volumes }) + "\n");

for (const a of asset.anchors) {
  const moved = anchorsOut[a.id];
  if (!moved) continue;
  if (moved.bone !== a.bone) throw new Error(`anchor ${a.id} would change its bone (${a.bone} -> ${moved.bone})`);
  const { id, bone: b, x: _x, y: _y, z: _z, kind, ...rest } = a;
  for (const k of Object.keys(a)) delete a[k];
  Object.assign(a, { id, bone: b, x: Math.round(moved.local[0] * 1e4) / 1e4, y: Math.round(moved.local[1] * 1e4) / 1e4, kind, z: Math.round(moved.local[2] * 1e4) / 1e4, ...rest });
}
for (const [scale, texts] of Object.entries(DRAWN)) Object.assign(asset.representations[scale].decisions, texts);
asset.volumes = volumes;
const STEM_ROWS: Record<number, typeof STEMS> = { 50: STEMS, 100: STEMS_100, 200: STEMS_200, 500: STEMS_500 };
const drawnV = (scale: number) => (b: string, u: number): number => (STEM_ROWS[scale]![b] ? Math.sign(F[b]!.v[1] || -1) * stemV(STEM_ROWS[scale]![b]!, u) : 0);
const report = draw(asset, F, models.map((m) => ({ scale: m.scale, tubes: m.wood, twigs: m.twigs, lobes: m.lobes, stemV: drawnV(m.scale) })));
/** 🗺️ The true-extent plan symbol follows the 1:100 body: crown outlines of the three views, the stems at their real position and thickness, the six tier limbs as branch directions. */
const symbol = asset.symbols.find((x: Json) => x.id === "japanese-maple-symbol");
const m100 = models[1]!, m50 = models[0]!;
const ext: Record<string, number[]> = {};
for (const view of ["elevation", "front", "plan"] as View[]) {
  for (const shape of symbol.geometry[view] as Json[]) {
    if (shape.id === "crown") {
      const rings = outline(m100.lobes.map((l) => lobeOutline(view, l)), 0.02, 1).filter((r) => r.reduce((t, p, i) => t + p.x * r[(i + 1) % r.length]!.y - r[(i + 1) % r.length]!.x * p.y, 0) > 0);
      delete shape.points;
      delete shape.closed;
      shape.kind = "path";
      shape.d = pathOf(rings);
      const ring = rings.flat();
      ext[view] = [Math.min(...ring.map((p) => p.x)), Math.max(...ring.map((p) => p.x)), Math.min(...ring.map((p) => p.y)), Math.max(...ring.map((p) => p.y))];
    }
    const stem = { "stem-1": "stem-w", "stem-2": "stem-c", "stem-3": "stem-e" }[shape.id as string];
    if (stem && view !== "plan") shape.points = pointsOf(outline(m100.wood.filter((t) => t.bone === stem || (stem === "stem-c" && t.bone === "stem-c-top")).flatMap((t) => tubePolygons(view, t)), 0.01, 1)[0]!);
    if (stem && view === "plan") {
      const rows = STEMS[stem]!;
      const u = rows.reduce((best, r) => (Math.abs(stemPoint(F, stem, r[0])[1] - 1) < Math.abs(stemPoint(F, stem, best[0])[1] - 1) ? r : best));
      const c = project("plan", stemPoint(F, stem, u[0]));
      shape.points = Array.from({ length: 24 }, (_, i) => [r4(c.x + u[3] * Math.cos((i * Math.PI) / 12)), r4(c.y + u[3] * Math.sin((i * Math.PI) / 12))]);
    }
    const limb = /^scaffold-(\d)$/.exec(shape.id);
    if (limb) {
      const t = m50.wood.find((x) => x.id === `limb-${["t1e", "t2e", "t3w", "t1w", "t2w", "t3e"][Number(limb[1]) - 1]}`)!;
      shape.points = [t.points[0]!, t.points[t.points.length - 1]!].map((q) => { const p = project("plan", q.p); return [r4(p.x), r4(p.y)]; });
    }
  }
}
const [w, d, h] = [ext.elevation![1]! - ext.elevation![0]!, ext.plan![3]! - ext.plan![2]!, ext.elevation![3]!];
const de = (x: number): string => x.toFixed(2).replace(".", ",");
symbol.dimensionalNote = { en: `A conventional simplification of the plant drawn at its true extent in model metres in all three views (${w.toFixed(2)} m wide, ${d.toFixed(2)} m deep and ${h.toFixed(2)} m high): the outline is the crown, the three stems are drawn at their real position and thickness. It is a symbol, never used silently for a pictorial scale.`, de: `Eine konventionelle Vereinfachung der Pflanze in ihrer wahren Ausdehnung in Modellmetern in allen drei Ansichten (${de(w)} m breit, ${de(d)} m tief und ${de(h)} m hoch): Der Umriss ist die Krone, die drei Stämme sind in wirklicher Lage und Stärke gezeichnet. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.` };
symbol.description = { en: "A conventional plan sign for garden plans: the true plan outline of the crown, the three stem sections at breast height and the directions of the six tier limbs. It is a separate symbol and is never used silently for a pictorial scale.", de: "Ein konventionelles Grundrisszeichen für Gartenpläne: der wahre Grundrissumriss der Krone, die drei Stammquerschnitte in Brusthöhe und die Richtungen der sechs Etagenäste. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet." };
const out = join(GENERATED, "r3-japanese-maple");
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "🔣️.json"), JSON.stringify(document, null, 2) + "\n");
for (const m of models) console.log(`1:${m.scale} body ${volumes[m.scale].body.length} thermal ${volumes[m.scale].thermal.length} structure ${volumes[m.scale].structure.length}`);
console.log(report.join("\n"));
