/** 📦️ World bbox per solid and optional centre-plane y-profile. Usage: bun author-32-bbox.ts cat/slug scale list [--source f] [--ids a,b] [--profile id --step 0.1] */
import { readFileSync } from "node:fs";
import { entryOf, loadSource, merged, scaleMeshes } from "./volumes-lib.ts";
const a = process.argv.slice(2);
const g = (n: string) => (a.includes(n) ? a[a.indexOf(n) + 1] : undefined);
const [spec, scaleArg, list = "body"] = a;
const scale = Number(scaleArg);
const { entry, asset } = entryOf(spec!);
const src = g("--source") ? JSON.parse(readFileSync(g("--source")!, "utf8")) : loadSource(entry);
const m = merged(asset, src);
const sm = scaleMeshes(m, scale, m.volumes[String(scale)]) as any;
const ms = sm[list] as { id: string; world: Float64Array; index?: Uint32Array }[];
const ids = g("--ids")?.split(",");
for (const x of ms) {
  if (ids && !ids.includes(x.id)) continue;
  const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < x.world.length; i += 3) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, x.world[i + k]!); hi[k] = Math.max(hi[k]!, x.world[i + k]!); }
  console.log(`${x.id.padEnd(28)} x ${lo[0]!.toFixed(3)}..${hi[0]!.toFixed(3)}  y ${lo[1]!.toFixed(3)}..${hi[1]!.toFixed(3)}  z ${lo[2]!.toFixed(3)}..${hi[2]!.toFixed(3)}`);
}
const pid = g("--profile");
if (pid) {
  const step = Number(g("--step") ?? 0.1);
  const x = ms.find((s) => s.id === pid)!;
  const bins = new Map<number, [number, number]>();
  for (let i = 0; i < x.world.length; i += 3) {
    if (Math.abs(x.world[i + 2]!) > Number(g("--zmax") ?? 0.1)) continue;
    const k = Math.round(x.world[i]! / step);
    const b = bins.get(k) ?? [Infinity, -Infinity];
    b[0] = Math.min(b[0], x.world[i + 1]!); b[1] = Math.max(b[1], x.world[i + 1]!);
    bins.set(k, b);
  }
  for (const k of [...bins.keys()].sort((p, q) => p - q)) console.log(`x=${(k * step).toFixed(2)}  y ${bins.get(k)![0].toFixed(3)} .. ${bins.get(k)![1].toFixed(3)}`);
}
