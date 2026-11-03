import { readFileSync } from "node:fs";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";
const [file] = process.argv.slice(2);
const a = JSON.parse(readFileSync(file, "utf8")).asset;
for (const p of a.poses) { try {
  const st: any = evaluate(a, { kind: "pose", pose: p.id }, { kind: "scale", scale: { denominator: 100 } } as any);
  const r: any = solveStructure(st);
  const reac = (r.reactions ?? []).map((x: any) => `${x.anchor ?? x.id}:${(x.fyN ?? 0).toFixed(0)}`).join(" ");
  const sum = (r.reactions ?? []).reduce((s: number, x: any) => s + (x.fyN ?? 0), 0);
  console.log(`${p.id}: sumFy ${sum.toFixed(0)} N | ${reac.slice(0, 260)} | keys ${Object.keys(r).join(",").slice(0, 100)}`);
} catch (e) { console.log(p.id, "ERR", (e as Error).message.slice(0, 160)); } }
