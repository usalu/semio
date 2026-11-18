/** 📐 Bone frames (world origin and unit axes) of the default pose: bun furniture-objects-frames.ts <asset> <scale> [bone-substring] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [name, scaleArg, q] = process.argv.slice(2) as [string, string, string | undefined];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scaleArg) } }) as any;
const r = (p: any) => [p.x, p.y, p.z].map((v: number) => +v.toFixed(4));
for (const [id, b] of Object.entries(state.bones as Record<string, any>)) if (q === undefined || id.includes(q)) console.log(id.padEnd(16), "o", r(b.frame.o), "u", r(b.frame.u), "v", r(b.frame.v), "w", r(b.frame.w));
