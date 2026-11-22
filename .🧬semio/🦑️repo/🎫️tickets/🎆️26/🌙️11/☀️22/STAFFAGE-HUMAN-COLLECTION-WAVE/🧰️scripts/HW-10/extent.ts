/** 📏️ Default-pose extents of every authored scale as the validator measures them: bun extent.ts <file> [pose] */
import { evaluate, loadAsset, viewExtent } from "./lib.ts";

const [file, poseId] = process.argv.slice(2);
const asset = await loadAsset(file!);
const out: Record<string, unknown> = {};
for (const d of [50, 100, 200, 500]) {
  const rep = (asset.representations as any)[`${d}`];
  if (rep?.status !== "authored") continue;
  const s = evaluate(asset, { kind: "pose", pose: poseId ?? asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } } as any);
  const f = viewExtent(s, "front")!, p = viewExtent(s, "plan")!;
  out[d] = { height: +(s.bounds.maxY - s.bounds.minY).toFixed(4), width: +(s.bounds.maxX - s.bounds.minX).toFixed(4), minY: +s.bounds.minY.toFixed(4), frontH: +f.horizontal.toFixed(4), frontV: +f.vertical.toFixed(4), planH: +p.horizontal.toFixed(4), planV: +p.vertical.toFixed(4) };
}
console.log(JSON.stringify(out));
