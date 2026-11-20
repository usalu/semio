/** 📦 Round 3 dairy cow: world bounding box of the body solids whose id contains one of the given words, at one scale; with `--slice X` the lowest and highest point of each within 2 cm of that x.
 *
 * Usage: bun r3-dairy-cow-bbox.ts 50 barrel waist [--slice 0.95] [--list structure] */
import { entryOf, loadSource, merged, meshesAt, type Json } from "./volumes-lib.ts";

const args = process.argv.slice(2);
const scale = Number(args[0]);
const cut = args.includes("--slice") ? Number(args[args.indexOf("--slice") + 1]) : undefined;
const list = args.includes("--list") ? args[args.indexOf("--list") + 1]! : "body";
const words = args.slice(1).filter((a, i, all) => !a.startsWith("--") && !all[i - 1]?.startsWith("--"));
const { entry, asset: catalog } = entryOf("animals/dairy-cow");
const asset = merged(catalog, loadSource(entry)!);
const solids: Json[] = (asset.volumes[String(scale)][list] as Json[]).filter((s) => s.kind);
for (const m of meshesAt(asset, scale, solids)) {
  if (words.length > 0 && !words.some((w) => m.id.includes(w))) continue;
  const lo = [Infinity, Infinity, Infinity];
  const hi = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < m.world.length; i += 3) {
    if (cut !== undefined && Math.abs(m.world[i]! - cut) > 0.02) continue;
    for (let j = 0; j < 3; j++) { lo[j] = Math.min(lo[j]!, m.world[i + j]!); hi[j] = Math.max(hi[j]!, m.world[i + j]!); }
  }
  console.log(m.id.padEnd(26), lo.map((c) => c.toFixed(3)).join(" "), "|", hi.map((c) => c.toFixed(3)).join(" "));
}
