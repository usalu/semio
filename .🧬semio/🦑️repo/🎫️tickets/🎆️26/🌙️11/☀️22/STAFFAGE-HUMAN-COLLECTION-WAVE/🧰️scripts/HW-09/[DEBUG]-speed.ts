import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const a = (await Bun.file("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/cyclist/🔣️.json").json()).asset;
let t0 = performance.now();
for (let i = 0; i < 50; i++) evaluate(a, { kind: "pose", pose: "riding-upright" });
console.log("[DEBUG] ms/eval", (performance.now() - t0) / 50);
const s = evaluate(a, { kind: "pose", pose: "riding-upright" });
console.log(Object.keys(s.anchors).map((k) => `${k} ${s.anchors[k].at.x.toFixed(3)} ${s.anchors[k].at.y.toFixed(3)} ${s.anchors[k].z.toFixed(3)}`).join("\n"));
console.log(JSON.stringify(s.bones["hand-l"].frame), JSON.stringify(s.root));
