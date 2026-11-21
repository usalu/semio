/** 🌳️ Round-3 multi-stem birch: writes the hand-stated tree of `r3-multi-stem-birch-tree.ts` as bone-local solids per scale and list to `volumes/vegetation/multi-stem-birch.json`, and the root fixes (stool and lateral bone offsets of the tripod of stems, transpiration anchors on their outlets, bark areas, 2D drawings as true views of each scale's body, drawing decisions, plan symbol) to the candidate document `🗑️generated/r3-multi-stem-birch/🔣️.json`.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-multi-stem-birch.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, SOURCES, entryOf, frames, toLocal, type Frame, type Json, type V3 } from "./volumes-lib.ts";
import { add, bounds, cross, dirToLocalSafe, markSolid, mul, quaternion, r5, solidOf, sub, unit, type Part, type Sec } from "./r3-multi-stem-birch-kit.ts";
import { LEAF, WOOD, applyRig, stemAt, tree, type Tree } from "./r3-multi-stem-birch-tree.ts";
import { draw } from "./r3-multi-stem-birch-draw.ts";
import { DECISIONS, DRAWN } from "./r3-multi-stem-birch-texts.ts";

const { entry } = entryOf("vegetation/multi-stem-birch");
const document: Json = JSON.parse(readFileSync(entry.path, "utf8"));
const asset: Json = document.asset;
const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);

applyRig(asset.rig);
const F: Record<string, Frame> = frames(asset);

const AREAS: Record<string, number> = { "stem-bark": 14, "branch-bark": 4 };
for (const region of asset.thermal.regions) if (AREAS[region.id] !== undefined) region.areaM2 = AREAS[region.id];

/** 💨️ Outlet patches of the transpiration emitter per scale: anchor (1:50 only) and the plume that carries the patch on its back. */
const OUTLETS: Record<number, [anchor: string | null, plume: string][]> = {
  50: [["tp-crown-1", "leaf-l-b-t-s"], ["tp-crown-2", "leaf-r-b-t-s"], ["tp-crown-3", "leaf-l-a-t-s"], ["tp-crown-4", "leaf-r-a-t-s"], ["tp-crown-5", "leaf-c-a-t-s"], ["tp-crown-6", "leaf-fol-l-top-s"], ["tp-crown-7", "leaf-fol-r-top-s"], ["tp-crown-8", "leaf-c-b-t-s"], ["tp-crown-9", "leaf-fol-c-top-s"]],
  100: [[null, "leaf-l-a-t-s"], [null, "leaf-r-a-t-s"], [null, "leaf-l-b-t-s"], [null, "leaf-c-b-t-s"], [null, "leaf-fol-c-top-s"]],
  200: [[null, "leaf-l-a-t-s"], [null, "leaf-r-a-t-s"], [null, "leaf-fol-c-top-s"]],
  500: [[null, "leaf-l-a-t-s"], [null, "leaf-r-a-t-s"], [null, "leaf-fol-c-top-s"]],
};
const emitter = asset.thermal.emitters[0];

export const models: Tree[] = [50, 100, 200, 500].map((scale) => tree(scale, F, asset));
const anchorsOut: Record<string, { bone: string; local: V3 }> = {};
const bark = (r: number): number => 0.002 + 0.04 * r;

function volumeOf(m: Tree): Json {
  const s = (p: Part): Json => solidOf(p, F);
  const feature = (0.15 * m.scale) / 1000;
  const body: Json[] = [...m.wood.map(s), ...m.strands.map(s), ...m.marks.map((k) => markSolid(k, F)), ...m.leaves.map(s)];
  const region = (p: Part): string => (p.group === "leaf" ? "leaf-crown" : p.group === "branch" || p.group === "tip" ? "branch-bark" : "stem-bark");
  const thermal: Json[] = [];
  for (const p of [...m.wood, ...m.leaves]) if (p.group !== "twig") thermal.push({ id: `th-${p.id}`, of: p.id, region: region(p) });
  for (const p of m.hidden) thermal.push({ ...s({ ...p, id: `th-${p.id}` }), region: region(p) });
  OUTLETS[m.scale]!.forEach(([anchor, id], i) => {
    const l = m.leaves.find((x) => x.id === id);
    if (!l) throw new Error(`1:${m.scale} outlet on unknown plume ${id}`);
    const sec: Sec = l.sections[Math.round(0.45 * (l.sections.length - 1))]!;
    const n = sec.y[1] >= 0 ? sec.y : mul(sec.y, -1);
    const p = add(sec.p, mul(n, sec.ry));
    const rho = Math.min(emitter.diameterM / 2, 0.62 * sec.rx);
    const h = Math.max(feature, 0.4 * sec.ry, 0.05);
    const third = cross(sec.x, n);
    const f = F[l.bone]!;
    const cols = [sec.x, n, third].map((a) => unit(dirToLocalSafe(f, a))) as [V3, V3, V3];
    thermal.push({ id: `th-${anchor ?? `${emitter.id}-${i + 1}`}`, kind: "ellipsoid", bone: l.bone, material: LEAF, role: "detail", tone: 0.5, radii: [r5(rho), r5(h), r5(rho * 1.3)], transform: { translation: toLocal(f, sub(p, mul(n, h - 0.02))).map(r5), rotation: quaternion(cols[0], cols[1], cols[2]) }, region: emitter.region, tags: [emitter.id] });
    if (anchor) anchorsOut[anchor] = { bone: l.bone, local: toLocal(f, add(p, mul(n, 0.04))) };
  });
  const structure: Json[] = [];
  const members: Part[] = [...m.cores, ...m.wood.filter((t) => t.group !== "root" && t.group !== "base" && (m.scale === 50 || t.group !== "twig") && !m.cores.some((c) => c.id === t.id)), ...m.hidden];
  for (const t of members) {
    const sections = t.group === "stool" ? t.sections.map((q) => ({ ...q, rx: q.rx - 0.02, ry: q.ry - 0.02 })) : t.sections.map((q) => ({ ...q, rx: q.rx - bark(q.rx), ry: q.ry - bark(q.ry) }));
    const thin = 2 * Math.max(...sections.map((q) => q.rx)) < feature;
    structure.push({ ...s({ ...t, id: `member-${t.id}`, material: WOOD, tone: 0.4, sections, flags: thin ? { slender: true } : t.flags }), member: t.bone, part: "member" });
  }
  for (const b of asset.rig.bones as Json[]) {
    if (b.parent === null) continue;
    const t = members.find((x) => x.bone === b.id && /^(stem|leader|tip|br-)/.test(x.id));
    if (!t) continue;
    const f = F[b.id]!;
    const first = t.sections.find((q) => toLocal(f, q.p)[0] >= -1e-6) ?? t.sections[0]!;
    const r = (first.rx - bark(first.rx)) * 1.12;
    if (2 * r < feature) continue;
    structure.push({ id: `joint-${b.id}`, kind: "ellipsoid", bone: b.id, material: WOOD, role: "detail", tone: 0.35, radii: [r5(r), r5(r), r5(r)], transform: { translation: toLocal(f, first.p).map(r5) }, member: b.id, part: "joint" });
  }
  const plate = Math.max(0.04, feature);
  structure.push({ id: "support-base-support", kind: "revolve", bone: "root-plate", material: WOOD, role: "detail", tone: 0.25, profile: [[r5(-plate), 0.5], [0, 0.5]], axis: [1, 0, 0], origin: [0, 0, 0], member: "root-plate", part: "support", anchor: "base-support" });
  for (const b of asset.rig.bones as Json[]) {
    if (!b.mass) continue;
    const f = F[b.id]!;
    if (b.id === "root-plate") {
      structure.push({ id: "mass-root-plate", kind: "ellipsoid", bone: b.id, material: WOOD, role: "detail", tone: 0.3, radii: [0.1, 0.15, 0.15], transform: { translation: [0.16, 0, 0] }, member: b.id, part: "mass" });
      continue;
    }
    if (/^fol-.-[ab]$|^fol-.-top$/.test(b.id)) {
      const set = m.leaves.filter((l) => l.bone === b.id);
      if (set.length === 0) throw new Error(`1:${m.scale}: no foliage on ${b.id}`);
      const l = set.reduce((best, x) => (Math.max(...x.sections.map((q) => q.rx)) > Math.max(...best.sections.map((q) => q.rx)) ? x : best));
      const sec = l.sections[Math.round(0.55 * (l.sections.length - 1))]!;
      const r = Math.cbrt((3 * b.mass.kg) / (4 * Math.PI * 600));
      const cols = [sec.x, sec.y, cross(sec.x, sec.y)].map((a) => unit(dirToLocalSafe(f, a))) as [V3, V3, V3];
      structure.push({ id: `mass-${b.id}`, kind: "ellipsoid", bone: b.id, material: LEAF, role: "detail", tone: 0.3, radii: [r5(Math.min(r, 0.8 * sec.rx)), r5(Math.min(0.6 * r, 0.8 * sec.ry)), r5(r)], transform: { translation: toLocal(f, sec.p).map(r5), rotation: quaternion(cols[0], cols[1], cols[2]) }, member: b.id, part: "mass" });
      continue;
    }
    const t = members.find((x) => x.bone === b.id && /^(stem|br-)/.test(x.id));
    if (!t) continue;
    const isStem = b.id.startsWith("stem-");
    const com: V3 = isStem ? stemAt(F, asset, b.id, b.mass.comX).p : t.sections.reduce((best, q) => (Math.abs(toLocal(f, q.p)[0] - b.mass.comX) < Math.abs(toLocal(f, best.p)[0] - b.mass.comX) ? q : best)).p;
    const near = t.sections.reduce((best, q) => (Math.hypot(...sub(q.p, com)) < Math.hypot(...sub(best.p, com)) ? q : best));
    const rr = Math.max(feature * 0.55, (near.rx - bark(near.rx)) * 0.8);
    structure.push({ id: `mass-${b.id}`, kind: "ellipsoid", bone: b.id, material: WOOD, role: "detail", tone: 0.3, radii: [r5(isStem ? Math.max(rr, Math.min(0.14, b.lengthM * 0.03)) : Math.min(rr * 1.8, b.lengthM * 0.4)), r5(rr), r5(rr)], transform: { translation: toLocal(f, com).map(r5) }, member: b.id, part: "mass" });
  }
  return { status: "authored", denominator: m.scale, body, thermal, structure, decisions: DECISIONS[m.scale]! };
}

const volumes: Json = {};
for (const m of models) volumes[String(m.scale)] = volumeOf(m);
writeFileSync(join(SOURCES, "vegetation", "multi-stem-birch.json"), JSON.stringify({ volumes }) + "\n");

for (const a of asset.anchors) {
  const moved = anchorsOut[a.id];
  if (!moved) continue;
  if (moved.bone !== a.bone) throw new Error(`anchor ${a.id} would change its bone (${a.bone} -> ${moved.bone})`);
  const { id, bone: b, x: _x, y: _y, z: _z, kind, title, ...rest } = a;
  for (const k of Object.keys(a)) delete a[k];
  Object.assign(a, { id, bone: b, x: Math.round(moved.local[0] * 1e4) / 1e4, y: Math.round(moved.local[1] * 1e4) / 1e4, kind, title, z: Math.round(moved.local[2] * 1e4) / 1e4, ...rest });
}
asset.volumes = volumes;
for (const [scale, texts] of Object.entries(DRAWN)) Object.assign(asset.representations[scale].decisions, texts);
console.log(draw(asset, F, models).join("\n"));
const out = join(GENERATED, "r3-multi-stem-birch");
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "🔣️.json"), JSON.stringify(document, null, 2) + "\n");
for (const m of models) {
  const b = bounds([...m.wood, ...m.leaves]);
  const leafB = bounds(m.leaves);
  const lateral = (set: Part[]): number => set.reduce((sum, p) => sum + p.sections.slice(1).reduce((s, q, i) => s + Math.PI * (q.rx + p.sections[i]!.rx) * Math.hypot(...sub(q.p, p.sections[i]!.p)), 0), 0);
  console.log(`  wood ${m.wood.length} strands ${m.strands.length} marks ${m.marks.length} leaves ${m.leaves.length}; lateral bark m2: stems ${lateral(m.wood.filter((p) => ["stool", "root", "base", "stem", "leader"].includes(p.group))).toFixed(2)} scaffold ${lateral([...m.wood, ...m.hidden].filter((p) => p.group === "branch" || p.group === "tip")).toFixed(2)} twigs ${lateral(m.wood.filter((p) => p.group === "twig")).toFixed(2)}`);
  console.log(`1:${m.scale} body ${volumes[m.scale].body.length} thermal ${volumes[m.scale].thermal.length} structure ${volumes[m.scale].structure.length}; x ${b.min[0].toFixed(3)}..${b.max[0].toFixed(3)} (${(b.max[0] - b.min[0]).toFixed(3)}) y ${b.min[1].toFixed(3)}..${b.max[1].toFixed(3)} z ${b.min[2].toFixed(3)}..${b.max[2].toFixed(3)} (${(b.max[2] - b.min[2]).toFixed(3)}); foliage from y ${leafB.min[1].toFixed(2)}`);
}
void bone;
