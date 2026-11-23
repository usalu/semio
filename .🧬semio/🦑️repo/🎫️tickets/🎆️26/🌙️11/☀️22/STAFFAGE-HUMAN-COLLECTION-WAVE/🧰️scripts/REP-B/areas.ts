/** 🌡️ Declares the thermal regions that `volumes-thermal-coverage` reports at the surface the volume has: `areaM2` becomes the outer surface measured at 1:50 (the definition of the README: the faces the air reaches) and, unless `--keep-exposure` names the region, the `exposedFraction` of every pose shrinks by the same factor so that the exchanging area of the pose stays what the author stated. Merges the ops into the repair file of the person.
 *  bun areas.ts <slug> [--file <asset.json>] [--keep-exposure a,b] [--only a,b] [--dry] */
import { parseArgs } from "node:util";
import { checkVolumes, measureVolumes } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { applyOp, applyOps, mergeOps, readOps, writeOps, type Op } from "./ops.ts";
import { catalogFile, opsFile } from "./people.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { file: { type: "string" }, "keep-exposure": { type: "string" }, only: { type: "string" }, dry: { type: "boolean" } } });
const slug = positionals[0]!;
const asset = (await Bun.file(values.file ?? catalogFile(slug)).json()).asset;
const existing = await readOps(opsFile(slug));
applyOps(asset, existing);
const keep = new Set((values["keep-exposure"] ?? "").split(",").filter(Boolean));
const only = values.only === undefined ? undefined : new Set(values.only.split(","));
const failing = new Set<string>();
const scales = [50, 100, 200, 500].filter((d) => asset.volumes[String(d)]?.status === "authored");
let totals = false;
for (const scale of scales) for (const f of checkVolumes(asset, { scale: scale as never, rules: ["volumes-thermal-coverage"] as never })) {
  const region = (f as any).data?.region as string | undefined;
  if (region !== undefined) failing.add(region); else if (/all regions/u.test(f.message.en)) totals = true;
  console.log(`[DEBUG] ${f.message.en}`);
}
const fine = measureVolumes(asset, { scale: scales[0] as never, rules: ["volumes-thermal-coverage"] as never });
const measured = (id: string, basis: string | undefined): number | undefined => (fine.find((m: any) => m.name === `info:${basis === "total" ? "totalAreaM2" : "outerAreaM2"}:${id}`) as any)?.value;
const ops: Op[] = [];
for (const region of asset.thermal.regions) {
  const value = measured(region.id, region.areaBasis);
  if (value === undefined || (only !== undefined ? !only.has(region.id) : !(failing.has(region.id) || (totals && Math.abs(value / region.areaM2 - 1) > 0.15)))) continue;
  const coarse = scales.length > 1 ? (measureVolumes(asset, { scale: scales[1] as never, rules: ["volumes-thermal-coverage"] as never }).find((m: any) => m.name === `info:${region.areaBasis === "total" ? "totalAreaM2" : "outerAreaM2"}:${region.id}`) as any)?.value as number | undefined : undefined;
  const fineLimit = region.model === "body" ? 0.3 : 0.15;
  let fitted = value;
  if (coarse !== undefined && Math.abs(coarse / value - 1) > 0.28) {
    let bestCost = Infinity;
    for (let i = 0; i <= 400; i++) { const v = Math.min(value, coarse) + (Math.abs(value - coarse) * i) / 400; const cost = Math.max(Math.abs(value / v - 1) / fineLimit, Math.abs(coarse / v - 1) / 0.3); if (cost < bestCost) { bestCost = cost; fitted = v; } }
    console.log(`[DEBUG] ${region.id}: 1:${scales[0]} ${value.toFixed(4)} m², 1:${scales[1]} ${coarse.toFixed(4)} m²: ${fitted.toFixed(4)} m² keeps both within their limits (${(bestCost * 100).toFixed(0)} % of them)`);
  }
  const next = Number(fitted.toPrecision(4));
  ops.push({ op: "region-set", id: region.id, set: { areaM2: next } });
  console.log(`${slug} ${region.id}: areaM2 ${region.areaM2} -> ${next}`);
}
const final: Op[] = [];
final.push(...ops);
for (const o of ops.filter((x) => x.op === "region-set")) {
  const region = asset.thermal.regions.find((r: any) => r.id === o.id);
  if (keep.has(region.id) || o.set.areaM2 <= region.areaM2) continue;
  asset.poses.forEach((pose: any) => {
    const binding = pose.thermal?.regions?.[region.id];
    if (binding === undefined) return;
    const fraction = Number(Math.min(1, (binding.exposedFraction * region.areaM2) / o.set.areaM2).toPrecision(4));
    final.push({ op: "set", path: ["poses", { id: pose.id }, "thermal", "regions", region.id, "exposedFraction"], value: fraction });
  });
}
if (!values.dry && final.length > 0) await writeOps(opsFile(slug), mergeOps(existing, final));
console.log(`${slug}: ${final.filter((o) => o.op === "region-set").length} regions redeclared${values.dry ? " (dry)" : ""}`);
