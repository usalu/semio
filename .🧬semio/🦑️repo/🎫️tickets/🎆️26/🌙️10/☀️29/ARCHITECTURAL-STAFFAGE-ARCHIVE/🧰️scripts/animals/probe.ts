// [scratch] prints anchor world positions and bounds per pose / clip instant
import { readFileSync } from "node:fs";
import { parseArgs } from "node:util";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { values } = parseArgs({ options: { file: { type: "string" }, asset: { type: "string" }, clip: { type: "string" }, n: { type: "string", default: "8" }, anchors: { type: "string" } } });
const doc = JSON.parse(readFileSync(values.file!, "utf8"));
const asset = (doc.assets ?? [doc]).find((a: any) => a.id === values.asset);
const ids: string[] = values.anchors ? values.anchors.split(",") : asset.anchors.filter((a: any) => a.kind === "contact").map((a: any) => a.id);
const f = (v: number) => v.toFixed(3);
const qs: any[] = values.clip ? Array.from({ length: Number(values.n) }, (_, i) => { const c = asset.clips.find((c: any) => c.id === values.clip); return { kind: "clip", clip: c.id, timeS: (c.durationS * i) / Number(values.n) }; }) : asset.poses.map((p: any) => ({ kind: "pose", pose: p.id }));
for (const q of qs) {
  const s = evaluate(asset, q);
  const b = s.bounds;
  console.log(`${q.pose ?? q.clip + "@" + q.timeS.toFixed(3)} H=${f(b.maxY - b.minY)} W=${f(b.maxX - b.minX)} minY=${f(b.minY)} x:[${f(b.minX)},${f(b.maxX)}] contacts=${s.contacts.join(",")}`);
  console.log("   " + ids.map((id) => `${id}=(${f(s.anchors[id].at.x)},${f(s.anchors[id].at.y)})`).join(" "));
}
