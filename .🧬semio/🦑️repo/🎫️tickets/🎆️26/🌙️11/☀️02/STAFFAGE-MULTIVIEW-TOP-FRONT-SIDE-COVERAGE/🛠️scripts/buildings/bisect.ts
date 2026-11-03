import { readFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const base = JSON.parse(readFileSync(`${L}/🗂️catalog/👤️humans/${process.argv[2]}/🔣️.json`, "utf8")).asset;
const den = Number(process.argv[3]); const clip = process.argv[4]; const t = Number(process.argv[5]);
const run = (a: any) => { try { evaluate(a, { kind: "clip", clip, timeS: t }, { kind: "scale", scale: { denominator: den } }); return true; } catch { return false; } };
console.log("base ok", run(base));
const rep = base.representations[String(den)];
const bad: string[] = [];
for (const v of ["elevation", "front", "plan"]) for (let i = 0; i < rep.geometry[v].length; i++) {
  const s = rep.geometry[v][i]; if (!s.solid) continue;
  const a = JSON.parse(JSON.stringify(base)); const c = a.representations[String(den)].geometry[v][i];
  delete c.solid; if (run(a)) bad.push(`${v}:${s.id}`);
}
console.log("removing the solid fixes:", bad.join(" "));
