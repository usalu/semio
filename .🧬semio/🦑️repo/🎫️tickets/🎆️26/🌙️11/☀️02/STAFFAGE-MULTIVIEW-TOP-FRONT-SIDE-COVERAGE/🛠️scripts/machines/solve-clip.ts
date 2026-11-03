import { readFileSync } from "node:fs";
import { fileOf } from "./lib.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";
const a = JSON.parse(readFileSync(fileOf("excavator"), "utf8")).asset;
for (const t of [0, 1.2, 2.4, 3.6]) { const st: any = evaluate(a, { kind: "clip", clip: "dump-and-stow", timeS: t }, { kind: "scale", scale: { denominator: 100 } } as any); const r: any = solveStructure(st);
  console.log(`t=${t}`, r.reactions.map((x: any) => `${x.anchor ?? x.id}:${x.fyN.toFixed(0)}`).join(" ")); }
