/** 📦️ Which solids set the bounding box? Usage: bun author-14-ext.ts <category>/<slug> <scale> [list] */
import { entryOf, loadSource, merged, scaleMeshes } from "./volumes-lib.ts";

const [spec, scaleArg, list = "body"] = process.argv.slice(2);
const scale = Number(scaleArg);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const sm = scaleMeshes(m, scale, m.volumes[String(scale)]) as any;
const ms = sm[list] as { id: string; world: Float64Array }[];
const ext = ms.map((x) => {
  const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < x.world.length; i += 3) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, x.world[i + k]!); hi[k] = Math.max(hi[k]!, x.world[i + k]!); }
  return { id: x.id, lo, hi };
});
for (let k = 0; k < 3; k++) {
  const a = [...ext].sort((p, q) => p.lo[k]! - q.lo[k]!).slice(0, 3).map((e) => `${e.id}:${e.lo[k]!.toFixed(3)}`);
  const b = [...ext].sort((p, q) => q.hi[k]! - p.hi[k]!).slice(0, 3).map((e) => `${e.id}:${e.hi[k]!.toFixed(3)}`);
  process.stdout.write(`axis ${"xyz"[k]} min ${a.join(" ")}  |  max ${b.join(" ")}\n`);
}
