/** 📍️ R01: in which view and pose a shape fails the feature floor: measures one representation of an asset at a denominator with one view and one pose kept at a time (the elevation always kept because front and plan solids are generated from it, the other view replaced by one large far rectangle that cannot fail; a front or plan line therefore repeats the elevation findings), and prints the feature violations of the named shapes.
 * Usage: `bun r5-range-gaps-where.ts <category>/<slug> <scale> <denominator> [<shape id> ...]` */
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { loadAsset, resolveAsset, type Json } from "./volumes-lib.ts";

const [spec, scale, denominator, ...ids] = process.argv.slice(2);
const asset: Json = loadAsset(resolveAsset(spec!).path);
const rep = asset.representations[scale!];
const bone = asset.rig.bones[0].id;
const filler = { id: "r5-filler", bone, role: "body", z: -99, filled: true, tone: 0.5, kind: "rect", x: 50, y: 50, w: 5, h: 5 };
for (const pose of asset.poses.filter((p: Json) => !p.key)) {
  for (const view of ["elevation", "front", "plan"]) {
    const geometry = Object.fromEntries(["elevation", "front", "plan"].map((v) => [v, v === view || v === "elevation" ? rep.geometry[v] : [filler]]));
    const variant = { ...asset, poses: [pose], representations: { ...asset.representations, [scale!]: { ...rep, geometry, validRange: { minDenominator: Math.min(rep.validRange.minDenominator, Number(denominator)), maxDenominator: Math.max(rep.validRange.maxDenominator, Number(denominator)) } } } };
    const result = measureSuitability(variant, variant.representations[scale!], Number(denominator), DEFAULT_PRINT_PROFILE) as Json;
    for (const v of result.violations) if (ids.length === 0 || v.shapes.some((s: string) => ids.includes(s))) console.log(`${pose.id} ${view} ${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)}`);
  }
}
