import { STAFFAGE, catalogEntries, frames, loadAsset, analyse, meshes } from "./volume-lib.ts";
const { validateAsset } = await import(`${STAFFAGE}/🔨️modules/🗂️catalog/🟦️.ts`);
const { evaluate } = await import(`${STAFFAGE}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { carry } = await import(`${STAFFAGE}/🔨️modules/🦴️rig/🟦️.ts`);
const out: any[] = [];
for (const e of catalogEntries().filter((e) => e.category.endsWith("objects"))) {
  const a = loadAsset(e.path);
  const an = analyse(a, a.volume, 50);
  const d = a.dimensions, dims = [d.widthM, d.heightM, d.depthM];
  const rest = frames(a), list = meshes(a, a.volume.solids, Math.max(...dims));
  const pose = a.presentation?.defaultPose ?? a.poses[0].id;
  const st: any = evaluate(a, { kind: "pose", pose }, { kind: "scale", scale: { denominator: 50 } });
  const arr = (v: any) => (Array.isArray(v) ? v : [v.x, v.y, v.z]);
  const min = [1e9, 1e9, 1e9], max = [-1e9, -1e9, -1e9];
  list.forEach((m, i) => {
    const b = a.volume.solids[i].bone, f = st.bones[b].frame;
    const fr = { o: arr(f.o), u: arr(f.u), v: arr(f.v), w: arr(f.w) };
    for (let k = 0; k < m.world.length; k += 3) {
      const p = carry(rest[b] as any, fr as any, [m.world[k]!, m.world[k + 1]!, m.world[k + 2]!]);
      for (let j = 0; j < 3; j++) { min[j] = Math.min(min[j]!, p[j]!); max[j] = Math.max(max[j]!, p[j]!); }
    }
  });
  const dp = [0, 1, 2].map((j) => (max[j]! - min[j]!) / dims[j]! - 1);
  out.push({ slug: e.slug, solids: a.volume.solids.length, restErr: an.bboxRelativeError, defaultPose: pose, poseErr: dp, iou: [an.views.elevation.iou, an.views.front.iou, an.views.plan.iou], issues: validateAsset(a).filter((i: any) => i.code.startsWith("volume-")).length, poses: a.poses.map((p: any) => p.id) });
}
await Bun.write(import.meta.dir + "/🗑️generated/authoring-objects/report.json", JSON.stringify(out, null, 1));
for (const o of out) console.log(o.slug.padEnd(22), o.solids, "rest", o.restErr.map((x: number) => (x * 100).toFixed(1)).join("/"), "pose", o.defaultPose, o.poseErr.map((x: number) => (x * 100).toFixed(1)).join("/"), "iou", o.iou.map((x: number) => x.toFixed(2)).join("/"), "issues", o.issues);
