import { entryOf, meshesAt } from "./volumes-lib.ts";
import { readFileSync } from "fs";
const [spec, scale, ids] = [process.argv[2]!, Number(process.argv[3]), process.argv[4]!.split(",")];
const { asset } = entryOf(spec);
const src = JSON.parse(readFileSync(`volumes/${spec}.json`, "utf8")).volumes[String(scale)];
const solids = src.body.filter((s: any) => ids.some((i) => s.id.startsWith(i)));
for (const m of meshesAt(asset, scale, solids)) {
  const p = m.world; const mn = [1e9, 1e9, 1e9], mx = [-1e9, -1e9, -1e9];
  for (let i = 0; i < p.length; i += 3) for (let k = 0; k < 3; k++) { mn[k] = Math.min(mn[k]!, p[i + k]!); mx[k] = Math.max(mx[k]!, p[i + k]!); }
  console.log(m.id, "tris", m.indices.length / 3, "min", mn.map((x) => x.toFixed(3)).join(","), "max", mx.map((x) => x.toFixed(3)).join(","));
}
