/** 🧾 Evaluated rings of one drawn shape: bun vehicles-dump.ts <asset> <scale> <view> <id> */
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const [name, scaleArg, view, id] = process.argv.slice(2) as [string, string, string, string];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: any };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scaleArg) } }) as any;
for (const sh of state.shapes[view]) if (sh.shape.id === id) console.log(JSON.stringify({ bone: sh.shape.bone, rings: sh.rings.map((r: any) => r.map((p: any) => [+p.x.toFixed(4), +p.y.toFixed(4)])) }));
