/** 📏️ Prints the default-pose body extents [width, height, depth] of every authored volume scale of an asset file as JSON: bun dims.ts <asset.json> */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { bodyDimensions, worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const asset = (await Bun.file(process.argv[2]!).json()).asset;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose });
const out: Record<string, number[]> = {};
for (const scale of [50, 100, 200, 500]) if (asset.volumes[`${scale}`].status === "authored") out[scale] = bodyDimensions(worldSolids(asset, state.bones as never, scale));
console.log(JSON.stringify(out));
