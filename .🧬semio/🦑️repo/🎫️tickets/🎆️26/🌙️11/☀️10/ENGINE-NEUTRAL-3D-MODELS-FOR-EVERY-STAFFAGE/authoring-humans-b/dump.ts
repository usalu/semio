/** 🧾️ Dumps the authoring context of one asset: dimensions, rest bone frames, the 1:50 shapes of the three views in world metres, shape ids per scale. Output `🗑️generated/authoring-humans-b/<slug>.ctx.json`. */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, frames, loadAsset, resolveAsset, restState, round5 } from "../volume-lib.ts";

for (const spec of process.argv.slice(2)) {
  const entry = resolveAsset(spec);
  const asset = loadAsset(entry.path);
  const fr = frames(asset);
  const state = restState(asset, 50);
  const view = (list: any[]) => list.map((e) => ({ id: e.shape.id, bone: e.shape.bone, role: e.shape.role, kind: e.shape.kind, closed: e.closed, region: e.shape.region ?? null, tone: e.shape.tone ?? null, material: e.shape.material ?? null, solid: e.shape.solid ?? null, z: e.shape.z, rings: e.rings.map((r: any[]) => r.map((p) => [round5(p.x), round5(p.y)])) }));
  const scales: Record<string, string[]> = {};
  for (const [k, rep] of Object.entries<any>(asset.representations)) {
    const g = rep.geometry;
    if (!g) continue;
    const ids = new Set<string>();
    for (const v of ["elevation", "front", "plan"]) for (const s of g[v] ?? []) ids.add(s.id);
    scales[k] = [...ids];
  }
  const out = {
    id: asset.id, category: entry.category, dimensions: asset.dimensions, materials: asset.materials.map((m: any) => m.id),
    regions: asset.thermal.regions.map((r: any) => r.id),
    bones: asset.rig.bones.map((b: any) => ({ id: b.id, parent: b.parent, angleDeg: b.angleDeg, lengthM: b.lengthM, x: b.x, y: b.y, frame: fr[b.id] })),
    poses: asset.poses.map((p: any) => p.id), clips: asset.clips.map((c: any) => c.id),
    elevation: view(state.elevation), front: view(state.front), plan: view(state.plan), scales, degraded: state.degraded ?? null,
  };
  writeFileSync(join(GENERATED, "authoring-humans-b", `${asset.id}.ctx.json`), JSON.stringify(out));
  console.log(asset.id, "ok", out.elevation.length, out.front.length, out.plan.length, state.degraded ?? "");
}
