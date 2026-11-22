/** 📐️ Prints `{heightM, widthM, depthM, massKg, powerW}` of the default pose of an asset JSON: height from the side and front elevations, width (ahead) from side elevation and plan, depth (lateral) from front elevation and plan, each the mean of its two views at 1:50; mass is the sum of the bone masses.
 *  bun dims.ts <asset.json> */
import { evaluate, totalPowerW } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const asset = (await Bun.file(process.argv[2]!).json()).asset;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } } as never);
const ext = (vp: "elevation" | "front" | "plan") => {
  let a = Infinity, b = -Infinity, c = Infinity, d = -Infinity;
  for (const s of state.shapes[vp]) for (const ring of s.rings) for (const p of ring) { a = Math.min(a, p.x); b = Math.max(b, p.x); c = Math.min(c, p.y); d = Math.max(d, p.y); }
  return { w: b - a, h: d - c };
};
const e = ext("elevation"), f = ext("front"), p = ext("plan");
const mass = asset.rig.bones.reduce((s: number, b: { mass?: { kg: number } }) => s + (b.mass?.kg ?? 0), 0);
const r3 = (v: number) => Math.round(v * 1000) / 1000;
console.log(JSON.stringify({ heightM: r3((e.h + f.h) / 2), widthM: r3((e.w + p.w) / 2), depthM: r3((f.w + p.h) / 2), massKg: Math.round(mass * 100) / 100, powerW: Math.round(totalPowerW(asset) * 10) / 10, views: { e, f, p } }));
