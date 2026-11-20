/** 📐️ Round 3 tram: raw mesh area of every thermal solid of one scale, summed per region and per solid-id family (`bun r3-tram-areas.ts 50 [region] [--solids]`). */
import { entryOf, loadSource, merged, meshArea, scaleMeshes } from "./volumes-lib.ts";

const args = process.argv.slice(2);
const scale = Number(args[0] ?? 50);
const only = args[1] && !args[1].startsWith("--") ? args[1] : undefined;
const { entry, asset } = entryOf("vehicles/tram");
const a = merged(asset, loadSource(entry)!);
const sm = scaleMeshes(a, scale, a.volumes[String(scale)]);
const sum: Record<string, number> = {};
const fam: Record<string, number> = {};
sm.solids.thermal.forEach((s, i) => {
  const m = sm.thermal[i];
  if (!m) return;
  const area = meshArea(m);
  const key = `${s.region}/${s.layer ?? "-"}`;
  sum[key] = (sum[key] ?? 0) + area;
  if (only && s.region !== only) return;
  const f = `${s.region}/${s.layer ?? "-"} ${String(s.id).replace(/-(l|r|a|b|\d+)(?=-|$)/g, "-*")}`;
  fam[f] = (fam[f] ?? 0) + area;
  if (args.includes("--solids")) console.log(`[DEBUG] ${s.id} ${area.toFixed(3)} ${JSON.stringify(s.sectionFaces ?? [])}`);
});
for (const [k, v] of Object.entries(sum)) console.log(`[DEBUG] region ${k} ${v.toFixed(2)}`);
for (const [k, v] of Object.entries(fam).sort((x, y) => y[1] - x[1])) console.log(`[DEBUG]   ${k} ${v.toFixed(2)}`);
