/** 📏 Drawn bbox per scale and view (HEAD vs current) with declared dimensions and volume extents. bun humansa-extents2.ts <asset> [scale...] */
import { execFileSync } from "node:child_process";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { VIEWPOINTS, loadAsset, stateOf } from "./humansa-lib.ts";
import { boundsOf, drawnFor, extentOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, ...sc] = process.argv.slice(2);
const cur = await loadAsset(name!);
const head = JSON.parse(execFileSync("git", ["-C", "/Users/ueli/Documents/semio", "show", `HEAD:🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/${name}/🔣️.json`], { maxBuffer: 1 << 28 }).toString("utf8")).asset as Asset;
const d = cur.dimensions;
console.log(name, "dims w", d.widthM, "h", d.heightM, "d", d.depthM, "(head", head.dimensions.widthM, head.dimensions.heightM, head.dimensions.depthM, ")");
const box = (a: Asset, scale: number, v: any): string => { const state = stateOf(a, scale); let b = [Infinity, -Infinity, Infinity, -Infinity]; for (const s of drawnFor(a, state, v, scale)) { const bb = boundsOf(s.rings); b = [Math.min(b[0]!, bb[0]), Math.max(b[1]!, bb[1]), Math.min(b[2]!, bb[2]), Math.max(b[3]!, bb[3])]; } return `x ${b[0]!.toFixed(3)}..${b[1]!.toFixed(3)} (${(b[1]! - b[0]!).toFixed(3)}) y ${b[2]!.toFixed(3)}..${b[3]!.toFixed(3)} (${(b[3]! - b[2]!).toFixed(3)})`; };
for (const scale of (sc.length ? sc.map(Number) : [500, 200, 100, 50])) {
  if ((cur.volumes as any)[String(scale)]?.status !== "authored") continue;
  const e = extentOf(worldSolids(cur, stateOf(cur, scale).bones as never, scale).filter((w) => w.texture !== true));
  console.log(`1:${scale} vol x ${(e.max[0]! - e.min[0]!).toFixed(3)} y ${(e.max[1]! - e.min[1]!).toFixed(3)} z ${(e.max[2]! - e.min[2]!).toFixed(3)}`);
  for (const v of VIEWPOINTS) console.log(`   ${v[0]} head ${box(head, scale, v)} | now ${box(cur, scale, v)}`);
}
