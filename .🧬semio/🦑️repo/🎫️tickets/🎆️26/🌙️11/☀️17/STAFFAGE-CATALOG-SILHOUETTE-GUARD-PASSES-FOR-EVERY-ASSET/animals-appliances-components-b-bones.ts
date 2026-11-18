/** 🦴 Bone frames of an asset at its default pose: bun animals-appliances-components-b-bones.ts <asset> [scale] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [name, sc] = process.argv.slice(2);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc ?? 50) } }) as any;
const f = (v: number[]) => `[${v.map((x) => x.toFixed(3)).join(",")}]`;
for (const [id, b] of Object.entries<any>(state.bones)) console.log(id.padEnd(14), "o", f(b.frame.o), "u", f(b.frame.u), "v", f(b.frame.v), "w", f(b.frame.w));
