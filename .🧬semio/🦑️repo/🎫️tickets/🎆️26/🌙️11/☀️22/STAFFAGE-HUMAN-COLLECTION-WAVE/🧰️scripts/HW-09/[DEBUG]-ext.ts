import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { loadHuman } from "./kin.ts";
const [slug, scale] = process.argv.slice(2);
const { asset } = await loadHuman(slug!);
const s = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } });
for (const v of ["elevation", "plan"] as const) {
  const rows = s.shapes[v].map((e: any) => { const xs = e.rings.flat().map((p: any) => p.x); return [e.shape.id, Math.min(...xs), Math.max(...xs)] as const; });
  rows.sort((a: any, b: any) => a[1] - b[1]); console.log("[DEBUG]", v, "min", rows.slice(0, 3).map((r: any) => `${r[0]} ${r[1].toFixed(3)}`).join(", "));
  rows.sort((a: any, b: any) => b[2] - a[2]); console.log("[DEBUG]", v, "max", rows.slice(0, 3).map((r: any) => `${r[0]} ${r[2].toFixed(3)}`).join(", "));
}
