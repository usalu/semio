/** 🦴️ Prints the rest frames (origin, u, v, w in world metres) of every bone of an asset as JSON: bun frames.ts <asset.json> */
import { restFrames } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
const asset = (await Bun.file(process.argv[2]!).json()).asset;
const frames = restFrames(asset.rig);
console.log(JSON.stringify(Object.fromEntries(Object.entries(frames).map(([id, f]) => [id, f.frame]))));
