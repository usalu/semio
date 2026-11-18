/** 📏 Moves one extreme of the volume of an asset at one scale by shifting the `at` of one loft section of the solid that owns it: bun vegetation-extent.ts <asset> <scale> <axis 0|1|2> <min|max> <shift m> [exclude solid ids, comma separated]
 *  Prints the edit as one JSON line {scale, solid, section, at, got} (got: the achieved shift of the extreme). */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { extentOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, axisArg, side, shiftArg, excludeArg] = process.argv.slice(2) as string[];
const [scale, axis, shift] = [Number(scaleArg), Number(axisArg), Number(shiftArg)];
const exclude = new Set((excludeArg ?? "").split(",").filter((x) => x !== ""));
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const body = (asset.volumes as any)[String(scale)].body as any[];
const measure = (): { min: number[]; max: number[]; owner: string } => {
  const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
  const e = extentOf(solids);
  let owner = "";
  let best = side === "max" ? -Infinity : Infinity;
  for (const s of solids) {
    if (exclude.has(s.id)) continue;
    for (let i = axis; i < s.positions.length; i += 3) {
      const v = s.positions[i]!;
      if (side === "max" ? v > best : v < best) {
        best = v;
        owner = s.id;
      }
    }
  }
  return { min: [...e.min], max: [...e.max], owner };
};
const base = measure();
const at0 = base[side as "min" | "max"][axis]!;
const owner = body.find((s) => s.id === base.owner);
if (owner === undefined) {
  console.log(JSON.stringify({ error: "no owner", owner: base.owner }));
  process.exit(0);
}
const f = state.bones[owner.bone].frame;
const dir = [f.u, f.v, f.w].map((v: any) => [v.x, v.y, v.z][axis]!);
if (owner.sections === undefined) {
  console.log(JSON.stringify({ scale, solid: base.owner, kind: owner.kind, translate: dir.map((d: number) => +(d * shift).toFixed(5)), from: at0 }));
  process.exit(0);
}
let bestEdit: { section: number; at: number[]; got: number } | undefined;
for (let j = 0; j < owner.sections.length; j++) {
  const orig = [...owner.sections[j].at] as number[];
  let delta = shift;
  let got = 0;
  for (let it = 0; it < 6; it++) {
    owner.sections[j].at = orig.map((v, k) => v + delta * dir[k]!);
    got = measure()[side as "min" | "max"][axis]! - at0;
    if (Math.abs(got - shift) < 1e-4) break;
    if (Math.abs(got) < 1e-9) break;
    delta *= shift / got;
  }
  const at = [...owner.sections[j].at] as number[];
  owner.sections[j].at = orig;
  if (Math.abs(got - shift) < 5e-4) {
    const move = Math.abs(delta);
    if (bestEdit === undefined || move < Math.abs(bestEdit.at.reduce((s, v, k) => s + Math.abs(v - orig[k]!), 0))) bestEdit = { section: j, at, got };
  }
}
console.log(JSON.stringify({ scale, solid: base.owner, ...(bestEdit ?? { kind: "loft", translate: dir.map((d: number) => +(d * shift).toFixed(5)) }), from: at0 }));
