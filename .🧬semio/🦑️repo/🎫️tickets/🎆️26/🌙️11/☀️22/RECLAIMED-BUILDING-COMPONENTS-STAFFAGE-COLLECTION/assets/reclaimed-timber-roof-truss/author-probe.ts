/** 🔬️ Prints contacts, reactions and residual of the structural solve at chosen instants of a clip of the candidate: `bun author-probe.ts <clip> <t>...`. */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";
const doc = JSON.parse(await Bun.file(`${import.meta.dir}/🔣️.json`).text());
const [clip, ...times] = process.argv.slice(2);
for (const t of times.map(Number)) {
  const s = evaluate(doc.asset, { kind: "clip", clip, timeS: t } as never);
  const r = solveStructure(s);
  console.log(t, "res", r.residual.fxN.toFixed(3), r.residual.fyN.toFixed(3), "com", r.centreOfMass.x.toFixed(4), r.reactions.filter((x) => x.active).map((x) => `${x.anchor}:${x.fxN.toFixed(1)},${x.fyN.toFixed(1)}`).join(" "));
}
