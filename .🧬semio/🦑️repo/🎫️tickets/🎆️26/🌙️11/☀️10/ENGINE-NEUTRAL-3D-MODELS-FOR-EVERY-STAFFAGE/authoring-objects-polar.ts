import { STAFFAGE, frames, loadAsset, resolveAsset, meshes } from "./volume-lib.ts";
const { evaluate } = await import(`${STAFFAGE}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { carry } = await import(`${STAFFAGE}/🔨️modules/🦴️rig/🟦️.ts`);
const [slug, poseId, prefix] = process.argv.slice(2);
const asset = loadAsset(resolveAsset(slug!).path);
const solids = asset.volume.solids as any[];
const d = asset.dimensions;
const rest = frames(asset);
const list = meshes(asset, solids, Math.max(d.widthM, d.heightM, d.depthM));
const state = evaluate(asset, { kind: "pose", pose: poseId }, { kind: "scale", scale: { denominator: 50 } });
const now: Record<string, any> = {};
for (const [b, e] of Object.entries<any>(state.bones)) now[b] = e.frame;
const arr = (v: any) => (Array.isArray(v) ? v : [v.x, v.y, v.z]);
const fr = (f: any) => ({ o: arr(f.o), u: arr(f.u), v: arr(f.v), w: arr(f.w) });
const bins = new Array(24).fill(0), low = new Array(24).fill(9), high = new Array(24).fill(-9);
list.forEach((m, i) => {
  if (!solids[i].id.startsWith(prefix ?? "panel")) return;
  for (let k = 0; k < m.world.length; k += 3) {
    const p = carry(rest[solids[i].bone] as any, fr(now[solids[i].bone]) as any, [m.world[k]!, m.world[k + 1]!, m.world[k + 2]!]);
    const a = Math.atan2(p[2]!, p[0]!), r = Math.hypot(p[0]!, p[2]!);
    const b = Math.floor(((a + Math.PI) / (2 * Math.PI)) * 24) % 24;
    bins[b] = Math.max(bins[b], r);
    low[b] = Math.min(low[b], p[1]!); high[b] = Math.max(high[b], p[1]!);
  }
});
console.log(poseId, "max radius per 15deg bin from -180:", bins.map((x: number) => x.toFixed(2)).join(" "));
console.log("y min per bin:", low.map((x: number) => x.toFixed(2)).join(" "));
console.log("y max per bin:", high.map((x: number) => x.toFixed(2)).join(" "));
