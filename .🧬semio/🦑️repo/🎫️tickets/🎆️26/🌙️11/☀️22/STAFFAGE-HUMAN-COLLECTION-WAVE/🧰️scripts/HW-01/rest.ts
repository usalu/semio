/** 🦴️ Prints the rest-pose world frames `{bone: {o, u, v, w}}` (root at the origin) of the asset JSON at the given path, the reference of front and plan shapes.
 *  bun rest.ts <asset.json> */
import { restFrames } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const asset = (await Bun.file(process.argv[2]!).json()).asset;
const frames = restFrames(asset.rig);
console.log(JSON.stringify(Object.fromEntries(Object.entries(frames).map(([id, f]) => [id, { o: f.frame.o, u: f.frame.u, v: f.frame.v, w: f.frame.w }]))));
