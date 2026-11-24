/** 🔢️ How the depth order `z` of the drawn shapes of a plant relates to the place of their bones along the viewing axis (elevation: world z, front: world x, plan: world y), per scale and view: the correlation over the bones and the range of `z`, for body and for texture shapes.
 *  bun zorder.ts <id|file.json>… */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { fileOf } from "./plants.ts";
for (const arg of process.argv.slice(2)) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  for (const [scale, rep] of Object.entries(asset.representations) as [string, any][]) {
    if (rep.status !== "authored") continue;
    const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } } as never);
    for (const view of ["elevation", "front", "plan"]) {
      const axis = view === "elevation" ? "z" : view === "front" ? "x" : "y";
      for (const roles of [["body", "detail"], ["texture"]]) {
        const per = new Map<string, number[]>();
        for (const s of rep.geometry[view]) if (roles.includes(s.role)) (per.get(s.bone) ?? per.set(s.bone, []).get(s.bone)!).push(s.z);
        const rows = [...per].map(([b, zs]) => { const f = state.bones[b].frame; return [f.o[axis] + f.u[axis] * (asset.rig.bones.find((x: any) => x.id === b).lengthM ?? 0) / 2, zs.reduce((a, c) => a + c, 0) / zs.length, Math.min(...zs), Math.max(...zs)] as const; });
        if (rows.length < 3) continue;
        const [mx, my] = [rows.reduce((a, r) => a + r[0], 0) / rows.length, rows.reduce((a, r) => a + r[1], 0) / rows.length];
        const [sxy, sxx, syy] = [rows.reduce((a, r) => a + (r[0] - mx) * (r[1] - my), 0), rows.reduce((a, r) => a + (r[0] - mx) ** 2, 0), rows.reduce((a, r) => a + (r[1] - my) ** 2, 0)];
        console.log(`${asset.id} 1:${scale} ${view} ${roles[0]}: correlation of z with world ${axis} ${(sxy / Math.sqrt(sxx * syy || 1)).toFixed(2)} over ${rows.length} bones, z ${Math.min(...rows.map((r) => r[2])).toFixed(3)}..${Math.max(...rows.map((r) => r[3])).toFixed(3)}, widest z range within a bone ${Math.max(...rows.map((r) => r[3] - r[2])).toFixed(3)}`);
      }
    }
  }
}
