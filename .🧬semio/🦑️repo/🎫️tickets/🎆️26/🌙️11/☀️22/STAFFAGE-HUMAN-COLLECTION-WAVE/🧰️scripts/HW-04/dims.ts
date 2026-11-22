/** 📦️ Prints the body volume extent (width ahead, height, depth lateral) of an asset JSON at every authored scale in a pose (default pose by default), as the silhouette guard measures it: bun dims.ts <asset.json> [pose] */
import { join, resolve } from "node:path";
const S = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const { worldSolids, bodyDimensions } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const [path, poseArg] = process.argv.slice(2);
const asset = (await Bun.file(path!).json()).asset;
const pose = poseArg ?? asset.presentation.defaultPose;
const bones = evaluate(asset, { kind: "pose", pose }).bones;
const out: Record<string, number[]> = {};
for (const sc of [50, 100, 200, 500]) out[sc] = bodyDimensions(worldSolids(asset, bones, sc)).map((x: number) => Math.round(x * 10000) / 10000);
console.log(JSON.stringify(out));
