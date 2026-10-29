import { readFileSync } from "node:fs";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const asset: any = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const ids = process.argv[4]!.split(",");
for (const t of process.argv.slice(5).map(Number)) {
  const st = evaluate(asset, { kind: "clip", clip: process.argv[3], timeS: t });
  console.log(t.toFixed(3), "root", st.bones.pelvis.origin.x.toFixed(4), ids.map((i) => `${i}:(${st.anchors[i].at.x.toFixed(3)},${st.anchors[i].at.y.toFixed(3)})`).join(" "), st.contacts.join(","));
}
