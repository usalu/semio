import { writeFileSync } from "node:fs";
import { entryOf, restState, poseState, frames, memberBones, memberShapes, shapeBones, GENERATED } from "./volumes-lib.ts";
const spec = process.argv[2]!;
const { entry, asset } = entryOf(spec);
const out: any = { frames: frames(asset), scales: {}, dims: asset.dimensions, presentation: asset.presentation, bones: asset.rig.bones.map((b: any) => ({ id: b.id, parent: b.parent, x: b.x, y: b.y, z: b.z, angleDeg: b.angleDeg, lengthM: b.lengthM, section: b.section, mass: b.mass?.kg, comX: b.mass?.comX, joint: b.joint?.type })), thermal: { regions: asset.thermal.regions.map((r: any) => ({ id: r.id, kind: r.kind, material: r.material, areaM2: r.areaM2, hidden: r.hidden, layers: (r.layers ?? []).map((l: any) => ({ id: l.id, thicknessM: l.thicknessM, resistanceM2KW: l.resistanceM2KW, material: l.material })), body: r.body, geometry: r.geometry })), emitters: asset.thermal.emitters }, supports: asset.structure.supports, materials: asset.materials.map((m: any) => ({ id: m.id })) };
for (const s of [50, 100, 200, 500]) {
  const rep = asset.representations[String(s)];
  if (rep?.status !== "authored") { out.scales[s] = { status: "unavailable" }; continue; }
  const st = poseState(asset, s); out.scales[s + "frames"] = st.frames;
  const views: any = {};
  for (const v of ["elevation", "front", "plan"] as const) views[v] = st[v].map((e: any) => ({ id: e.shape.id, role: e.shape.role, closed: e.closed, bone: e.shape.bone ?? e.shape.solid?.bone ?? null, kind: e.shape.kind, rings: e.rings.map((r: any) => r.map((p: any) => [Math.round(p.x * 1e4) / 1e4, Math.round(p.y * 1e4) / 1e4])) }));
  out.scales[s] = { status: "authored", views, memberBones: memberBones(asset, s), shapeBones: [...shapeBones(asset, s)], members: memberShapes(asset, s).map((m: any) => ({ id: m.id, from: m.from, to: m.to, section: m.section, bone: m.bone })) };
}
writeFileSync(`${GENERATED}/author10/${entry.slug}.json`, JSON.stringify(out));
console.log(entry.slug, "ok");
