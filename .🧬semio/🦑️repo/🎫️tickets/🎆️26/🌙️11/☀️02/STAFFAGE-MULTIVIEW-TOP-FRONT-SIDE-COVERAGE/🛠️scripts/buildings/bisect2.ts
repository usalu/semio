import { readFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const base = JSON.parse(readFileSync(`${L}/🗂️catalog/👤️humans/skateboarder/🔣️.json`, "utf8")).asset;
const run = (a: any, view: string, den: number) => { try { evaluate(a, { kind: "clip", clip: "push-cycle", timeS: 1.998 }, { kind: "scale", scale: { denominator: den } }); return true; } catch { return false; } };
for (const den of [50, 100, 200, 500]) {
  const ok = run(base, "", den);
  const culprits: string[] = [];
  if (!ok) for (const v of ["elevation", "front", "plan"]) {
    const rep = base.representations[String(den)];
    for (let i = 0; i < rep.geometry[v].length; i++) {
      const a = JSON.parse(JSON.stringify(base)); const g = a.representations[String(den)].geometry[v];
      const only = g[i]; a.representations[String(den)].geometry[v] = [only];
      for (const w of ["elevation", "front", "plan"]) if (w !== v) a.representations[String(den)].geometry[w] = [base.representations[String(den)].geometry[w][0]];
      if (!run(a, v, den)) culprits.push(`${v}:${only.id}`);
    }
  }
  console.log(den, ok, culprits.join(" "));
}
