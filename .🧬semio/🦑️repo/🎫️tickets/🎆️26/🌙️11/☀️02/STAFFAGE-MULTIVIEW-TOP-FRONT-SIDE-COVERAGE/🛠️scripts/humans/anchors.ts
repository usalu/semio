import { readdirSync } from "node:fs";
import { L, load } from "./lib.ts";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const only = process.argv[2];
for (const slug of readdirSync(`${L}/🗂️catalog/👤️humans`).sort()) {
  if (only && slug !== only) continue;
  const a = load(slug).asset;
  const r = Object.values<any>(a.representations).find((x) => x.status === "authored" && x.denominator === 50)!;
  const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } });
  console.log(`== ${slug} (${a.presentation.defaultPose})`);
  for (const an of a.anchors) { const s = st.anchors[an.id]; console.log("  ", an.id.padEnd(20), an.kind.padEnd(10), an.bone.padEnd(14), `local(${an.x},${an.y},${an.z ?? 0}) world x=${s.at.x.toFixed(3)} y=${s.at.y.toFixed(3)} z=${s.z.toFixed(3)}`, an.support ?? ""); }
}
