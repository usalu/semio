/** 📏️ Prints the bounding box of the body of animals/sheep in the default pose at a scale. Usage: bun r3-sheep-bbox.ts [scale] */
import { openCtx } from "./kit-lib.ts";
import { meshesAt } from "./volumes-lib.ts";
const scale = Number(process.argv[2] ?? 50);
const c = openCtx("animals/sheep", scale);
const lo = [Infinity, Infinity, Infinity];
const hi = [-Infinity, -Infinity, -Infinity];
for (const m of meshesAt(c.m, scale, c.vol.body)) for (let i = 0; i < m.world.length; i++) { lo[i % 3] = Math.min(lo[i % 3]!, m.world[i]!); hi[i % 3] = Math.max(hi[i % 3]!, m.world[i]!); }
console.log(`1:${scale} x ${lo[0]!.toFixed(4)}..${hi[0]!.toFixed(4)} (${(hi[0]! - lo[0]!).toFixed(4)}) y ${lo[1]!.toFixed(4)}..${hi[1]!.toFixed(4)} z ${lo[2]!.toFixed(4)}..${hi[2]!.toFixed(4)} (${(hi[2]! - lo[2]!).toFixed(4)})`);
