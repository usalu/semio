/** 🔎 Tessellates every solid of one list and scale of the ceiling-diffuser source and prints bounds, triangle count and mesh area per solid. */
import { readFileSync } from "node:fs";
import { localGeometry, meshesAt, meshArea, entryOf, merged, loadSource, resolveList } from "./volumes-lib.ts";
const [scale, list] = [Number(process.argv[2] ?? 50), (process.argv[3] ?? "body") as "body"];
const { entry, asset } = entryOf("appliances/ceiling-diffuser");
const a = merged(asset, loadSource(entry)!);
const solids = resolveList(a.volumes[String(scale)], list);
for (const m of meshesAt(a, scale, solids)) {
  const p = m.world; let lo = [1e9, 1e9, 1e9], hi = [-1e9, -1e9, -1e9];
  for (let i = 0; i < p.length; i += 3) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, p[i + k]!); hi[k] = Math.max(hi[k]!, p[i + k]!); }
  console.log(m.id, "tris", m.indices.length / 3, "area", meshArea(m).toFixed(4), "lo", lo.map((v) => v.toFixed(4)).join(","), "hi", hi.map((v) => v.toFixed(4)).join(","));
}
