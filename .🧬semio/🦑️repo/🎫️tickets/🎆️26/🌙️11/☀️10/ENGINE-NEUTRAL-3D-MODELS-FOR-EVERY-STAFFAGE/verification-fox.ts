import { readFileSync } from "node:fs";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const a = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/red-fox/🔣️.json", "utf8")).asset;
const pose = a.poses.find((p: any) => p.id === "curled-sleeping");
console.log(JSON.stringify(pose.bones).slice(0, 1800));
const st: any = evaluate(a, { kind: "pose", pose: "curled-sleeping" }, { kind: "scale", scale: { denominator: 50 } });
for (const [id, b] of Object.entries<any>(st.bones)) console.log(id.padEnd(22), "o", [b.frame.o.x, b.frame.o.y, b.frame.o.z].map((v: number) => v.toFixed(3)).join(","), "tip", [b.tip.x, b.tip.y, b.tip.z].map((v: number) => v.toFixed(3)).join(","));
const solids = a.volume.solids.filter((s: any) => /ear|eye|fore|front|leg/.test(s.id)).map((s: any) => `${s.id}:${s.bone}`);
console.log(solids.join(" "));
