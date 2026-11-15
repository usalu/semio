/** 📏️ Prints per bone the thinnest drawn 2D extent (C13 limit) and each solid's smallest extent: bun author-24-limits.ts <asset> <scale> [body|thermal|structure] */
import { entryOf, loadSource, merged, minExtent } from "./volumes-lib.ts";
import { drawnMinExtents } from "./volumes-rulings.ts";

const [spec, sc, list = "body"] = process.argv.slice(2);
const scale = Number(sc);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const lim = drawnMinExtents(m, scale);
const feature = (0.15 * scale) / 1000;
const vol = loadSource(entry)!.volumes[String(scale)];
for (const s of vol[list]) {
  if (s.of) continue;
  const d = minExtent(m, scale, s);
  const l = Math.min(feature, lim.get(s.bone) ?? Infinity);
  console.log(`${s.id.padEnd(28)} ${s.bone.padEnd(12)} min=${(d * 1000).toFixed(1)}mm limit=${(l * 1000).toFixed(1)}mm${d < l - 1e-9 ? "  BELOW" : ""}`);
}
