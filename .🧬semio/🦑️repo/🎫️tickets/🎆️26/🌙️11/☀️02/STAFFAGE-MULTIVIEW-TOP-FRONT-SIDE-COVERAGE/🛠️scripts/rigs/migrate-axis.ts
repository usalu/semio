/** 🧭️ One-shot D24 migration of `Solid.axisX`/`axisZ` from absolute rest coordinates to offsets from the axis bone's own rest origin, with a before/after proof of the rest extents.
 * Usage: bun migrate-axis.ts <dry|write> [--only slug,slug] [--out report.json]. `before` is the evaluator copy `before.tmp.ts` with the old absolute semantics. */
import { existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import type { Asset, AuthoredRepresentation, Viewpoint } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { evaluate as after } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { evaluate as before } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/before.tmp.ts";
import { restFrames } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const ROOT = join(import.meta.dirname, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const args = process.argv.slice(2);
const mode = args[0];
const only = args.includes("--only") ? args[args.indexOf("--only") + 1]!.split(",") : undefined;
const out = args.includes("--out") ? args[args.indexOf("--out") + 1] : undefined;
const round = (v: number): number => {
  const r = Math.round(v * 1e6) / 1e6;
  return Object.is(r, -0) ? 0 : r;
};

type SolidJson = { kind: string; axisBone?: string; axisX?: number; axisZ?: number };
const report: { slug: string; solids: number; worstMm: number; worstWhere: string; changedAtRest: string[] }[] = [];
let totalSolids = 0;

function converted(asset: Asset): (solid: SolidJson) => { axisX: number; axisZ: number } {
  const rest = restFrames(asset.rig);
  return (solid) => {
    const o = rest[solid.axisBone!]!.frame.o;
    return { axisX: round((solid.axisX ?? 0) - o[0]), axisZ: round((solid.axisZ ?? 0) - o[2]) };
  };
}

function migrateAsset(asset: Asset): Asset {
  const convert = converted(asset);
  const copy = JSON.parse(JSON.stringify(asset)) as Asset;
  for (const representation of Object.values(copy.representations))
    if (representation.status === "authored")
      for (const view of ["front", "plan", "elevation"] as const)
        for (const shape of (representation as AuthoredRepresentation).geometry[view] ?? []) {
          const solid = shape.solid as SolidJson | undefined;
          if (solid?.kind !== "revolved") continue;
          const next = convert(solid);
          solid.axisX = next.axisX;
          solid.axisZ = next.axisZ;
        }
  for (const symbol of copy.symbols)
    for (const view of ["front", "plan", "elevation"] as const)
      for (const shape of (symbol.geometry[view] ?? []) as { solid?: SolidJson }[]) {
        const solid = shape.solid;
        if (solid?.kind !== "revolved") continue;
        const next = convert(solid);
        solid.axisX = next.axisX;
        solid.axisZ = next.axisZ;
      }
  return copy;
}

function rewrite(text: string, convert: ReturnType<typeof converted>): string {
  return text.replace(/"solid": \{[^{}]*\}/g, (block) => {
    const solid = JSON.parse(block.slice(block.indexOf("{"))) as SolidJson;
    if (solid.kind !== "revolved") return block;
    const next = convert(solid);
    let result = block.replace(/"axisX": [-0-9.eE+]+/, `"axisX": ${next.axisX}`);
    if (/"axisZ": /.test(result)) result = result.replace(/"axisZ": [-0-9.eE+]+/, `"axisZ": ${next.axisZ}`);
    else if (next.axisZ !== 0) result = result.replace(/("axisX": [-0-9.eE+]+,)(\r?\n)(\s*)/, `$1$2$3"axisZ": ${next.axisZ},$2$3`);
    return result;
  });
}

const sel = (d: number) => ({ kind: "scale", scale: { denominator: d } }) as const;
const views: Viewpoint[] = ["elevation", "front", "plan"];

for (const group of readdirSync(ROOT, { withFileTypes: true }).filter((e) => e.isDirectory()))
  for (const folder of readdirSync(join(ROOT, group.name), { withFileTypes: true }).filter((e) => e.isDirectory())) {
    const file = join(ROOT, group.name, folder.name, "🔣️.json");
    if (!existsSync(file) || (only !== undefined && !only.includes(folder.name))) continue;
    const text = readFileSync(file, "utf8");
    const asset = (JSON.parse(text) as { asset: Asset }).asset;
    const count = (text.match(/"kind": "revolved"/g) ?? []).length;
    totalSolids += count;
    if (count === 0) continue;
    const moved = migrateAsset(asset);
    let worst = 0;
    let where = "";
    const changed: string[] = [];
    const scales = Object.values(asset.representations).filter((r): r is AuthoredRepresentation => r.status === "authored").map((r) => r.denominator);
    for (const d of scales)
      for (const pose of asset.poses) {
        const a = before(asset, { kind: "pose", pose: pose.id }, sel(d));
        const b = after(moved, { kind: "pose", pose: pose.id }, sel(d));
        for (const view of views) {
          const pa = a.shapes[view].flatMap((s) => s.rings.flat());
          const pb = b.shapes[view].flatMap((s) => s.rings.flat());
          const box = (p: { x: number; y: number }[]) => [Math.min(...p.map((q) => q.x)), Math.max(...p.map((q) => q.x)), Math.min(...p.map((q) => q.y)), Math.max(...p.map((q) => q.y))];
          if (pa.length === 0 || pb.length === 0) continue;
          const [ba, bb] = [box(pa), box(pb)];
          const delta = Math.max(...ba.map((v, i) => Math.abs(v - bb[i]!))) * 1000;
          if (delta > worst) {
            worst = delta;
            where = `1:${d} ${pose.id} ${view}`;
          }
          if (delta > 0.1) changed.push(`1:${d} ${pose.id} ${view} ${delta.toFixed(2)} mm`);
        }
      }
    report.push({ slug: `${group.name}/${asset.id}`, solids: count, worstMm: worst, worstWhere: where, changedAtRest: changed.slice(0, 6) });
    if (mode === "write") writeFileSync(file, rewrite(text, converted(asset)));
    console.error(`[migrate] ${asset.id} solids=${count} worst=${worst.toFixed(3)} mm ${where}`);
  }
const result = { totalRevolvedSolids: totalSolids, assets: report.length, over0_1mm: report.filter((r) => r.worstMm > 0.1).length, report };
if (out !== undefined) writeFileSync(out, JSON.stringify(result, null, 1));
else console.log(JSON.stringify(result, null, 1));
