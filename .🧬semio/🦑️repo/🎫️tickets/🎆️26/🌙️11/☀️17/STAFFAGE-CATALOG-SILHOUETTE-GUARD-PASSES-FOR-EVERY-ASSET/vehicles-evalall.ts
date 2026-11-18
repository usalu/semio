/** 🧾 Evaluated first rings of every shape of one view as JSON {"id bone": ring}: bun vehicles-evalall.ts <asset> <scale> <view> */
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const [name, scaleArg, view] = process.argv.slice(2) as [string, string, string];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: any };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scaleArg) } }) as any;
const out: Record<string, number[][]> = {};
for (const sh of state.shapes[view]) if (sh.rings.length > 0 && out[sh.shape.id + " " + sh.shape.bone] === undefined) out[sh.shape.id + " " + sh.shape.bone] = sh.rings[0].map((p: any) => [p.x, p.y]);
console.log("[JSON]" + JSON.stringify(out));
