/** 📏️ Whether the foliage lofts of a plant reach as far as the ellipsoids they stand for: `bun short.ts <id|file.json> [--scales 100,200] [--views front,plan]` evaluates every pose at every authored scale and compares, per bone that draws `loft` shapes in a view, the box the engine draws for those lofts with the exact box of the ellipsoids of that bone in `volumes.<scale>.body` under the evaluated frame of the bone (centre plus or minus the root of the squared projections of the three half axes). It prints per scale and view the largest shortfall (the loft box inside the ellipsoid box) and overshoot with bone and pose, and every bone short by more than 10 mm (the `LOST` threshold of `../MV-L1/loft.ts`).
 * @see ../MV-L1/loft.ts — the tool whose first version took the contour of an ellipsoid across Aᵀn instead of A⁻¹n */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";

const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { project } = await import(join(S, "🔨️modules", "🎨️depiction", "🔭️views", "🟦️.ts"));

type V3 = [number, number, number];
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const asset = JSON.parse(readFileSync(fileOf(args[0]!), "utf8")).asset;
const scales = flag("scales")?.split(",");
const views = flag("views")?.split(",") ?? ["elevation", "front", "plan"];
const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined && (scales === undefined || scales.includes(key)));
const poses: string[] = (Array.isArray(asset.poses) ? asset.poses : Object.values(asset.poses)).map((pose: any) => pose.id);

/** 🥚️ The centre and the three half axes of an ellipsoid solid in the frame of its bone. */
function ellipsoidOf(solid: any): { c: V3; a: V3[] } {
  const [x, y, z, w] = (solid.transform?.rotation ?? [0, 0, 0, 1]) as number[];
  const scale = (solid.transform?.scale ?? [1, 1, 1]) as V3;
  const rotate = (p: V3): V3 => [(1 - 2 * (y! * y! + z! * z!)) * p[0] + 2 * (x! * y! - z! * w!) * p[1] + 2 * (x! * z! + y! * w!) * p[2], 2 * (x! * y! + z! * w!) * p[0] + (1 - 2 * (x! * x! + z! * z!)) * p[1] + 2 * (y! * z! - x! * w!) * p[2], 2 * (x! * z! - y! * w!) * p[0] + 2 * (y! * z! + x! * w!) * p[1] + (1 - 2 * (x! * x! + y! * y!)) * p[2]];
  const [r0, r1, r2] = solid.radii as V3;
  return { c: (solid.transform?.translation ?? [0, 0, 0]) as V3, a: [rotate([r0 * scale[0], 0, 0]), rotate([0, r1 * scale[1], 0]), rotate([0, 0, r2 * scale[2]])] };
}

let lost = 0;
for (const key of authored) {
  const solids: any[] = (asset.volumes?.[key]?.body ?? []).filter((solid: any) => solid.kind === "ellipsoid");
  for (const view of views) {
    let short = { mm: 0, at: "" }, over = { mm: 0, at: "" }, bones = 0;
    const named = new Map<string, number>();
    for (const pose of poses) {
      const state = evaluate(asset, { kind: "pose", pose }, engine.scaleSelector(Number(key)));
      const drawn = new Map<string, { x: number; y: number }[]>();
      for (const entry of state.shapes[view]) if (entry.shape.kind === "loft") drawn.set(entry.shape.bone, [...(drawn.get(entry.shape.bone) ?? []), ...entry.rings.flat()]);
      if (pose === poses[0]) bones = drawn.size;
      for (const [bone, points] of drawn) {
        const own = solids.filter((solid) => solid.bone === bone);
        if (own.length === 0 || points.length === 0) continue;
        const f = state.bones[bone].frame;
        const world = (p: V3): any => ({ x: f.o.x + p[0] * f.u.x + p[1] * f.v.x + p[2] * f.w.x, y: f.o.y + p[0] * f.u.y + p[1] * f.v.y + p[2] * f.w.y, z: f.o.z + p[0] * f.u.z + p[1] * f.v.z + p[2] * f.w.z });
        const box = [Infinity, -Infinity, Infinity, -Infinity];
        for (const solid of own) {
          const e = ellipsoidOf(solid);
          const c = project(view, world(e.c));
          const tips = e.a.map((axis) => project(view, world([e.c[0] + axis[0], e.c[1] + axis[1], e.c[2] + axis[2]])));
          const [rx, ry] = [Math.hypot(...tips.map((tip: any) => tip.x - c.x)), Math.hypot(...tips.map((tip: any) => tip.y - c.y))];
          box[0] = Math.min(box[0]!, c.x - rx); box[1] = Math.max(box[1]!, c.x + rx); box[2] = Math.min(box[2]!, c.y - ry); box[3] = Math.max(box[3]!, c.y + ry);
        }
        const got = [Math.min(...points.map((p) => p.x)), Math.max(...points.map((p) => p.x)), Math.min(...points.map((p) => p.y)), Math.max(...points.map((p) => p.y))];
        const inside = Math.max(got[0]! - box[0]!, box[1]! - got[1]!, got[2]! - box[2]!, box[3]! - got[3]!);
        const outside = Math.max(box[0]! - got[0]!, got[1]! - box[1]!, box[2]! - got[2]!, got[3]! - box[3]!);
        if (inside > short.mm) short = { mm: inside, at: `${bone} in ${pose}` };
        if (outside > over.mm) over = { mm: outside, at: `${bone} in ${pose}` };
        if (inside > 0.01) named.set(bone, Math.max(named.get(bone) ?? 0, inside));
      }
    }
    if (bones === 0) continue;
    lost += named.size;
    console.log(`1:${key} ${view}: ${bones} bones with lofts over ${poses.length} poses; short of the ellipsoids by at most ${(short.mm * 1000).toFixed(0)} mm (${short.at}), beyond them by at most ${(over.mm * 1000).toFixed(0)} mm (${over.at})${named.size === 0 ? "" : `; SHORT by more than 10 mm: ${[...named].map(([bone, value]) => `${bone} ${(value * 1000).toFixed(0)}`).join(", ")}`}`);
  }
}
console.log(`${asset.id}: ${lost} bone views short by more than 10 mm`);
