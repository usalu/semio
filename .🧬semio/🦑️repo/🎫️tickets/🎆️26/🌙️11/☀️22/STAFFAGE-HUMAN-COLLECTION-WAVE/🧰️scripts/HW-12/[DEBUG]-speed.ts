import { evaluate, solveRig } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const asset = (await Bun.file(`/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/standing-adult/🔣️.json`).json()).asset;
let t0 = performance.now();
for (let i = 0; i < 50; i++) evaluate({ ...asset, poses: [{ ...asset.poses[0], bones: { ...asset.poses[0].bones, "arm-r": { angleDeg: i } } }] }, { kind: "pose", pose: asset.poses[0].id });
console.log("[DEBUG] evaluate ms", (performance.now() - t0) / 50);
console.log(solveRig.toString().slice(0, 300));
