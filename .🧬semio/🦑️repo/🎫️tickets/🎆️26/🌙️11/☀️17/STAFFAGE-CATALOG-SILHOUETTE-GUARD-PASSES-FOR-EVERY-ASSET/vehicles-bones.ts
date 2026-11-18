/** 🦴 Default-pose frame of every bone of an asset as JSON {bone: [ox, oy, angleDeg]}: bun vehicles-bones.ts <asset> <scale> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const [name, scaleArg] = process.argv.slice(2) as [string, string];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scaleArg) } }) as any;
const out: Record<string, number[]> = {};
for (const [id, b] of Object.entries(state.bones as Record<string, any>)) out[id] = [+b.frame.o.x.toFixed(5), +b.frame.o.y.toFixed(5), +((Math.atan2(b.frame.u.y, b.frame.u.x) * 180) / Math.PI).toFixed(4)];
console.log(JSON.stringify(out));
