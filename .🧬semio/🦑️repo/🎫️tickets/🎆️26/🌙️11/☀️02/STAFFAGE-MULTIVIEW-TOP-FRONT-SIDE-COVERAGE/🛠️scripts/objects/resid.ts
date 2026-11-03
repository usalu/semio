const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { solveStructure } = await import(`${L}/🔨️modules/🏗️structure/🟦️.ts`);
const { queries } = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const { readFileSync } = await import("node:fs");
const [cat, slug] = process.argv.slice(2);
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
for (const { name, query } of queries(a)) {
  const r = solveStructure(evaluate(a, query, { kind: "scale", scale: { denominator: 50 } }));
  const m = Math.max(...Object.values(r.residual).map((v: any) => Math.abs(v)));
  if (m > 1e-6) console.log(name, m.toFixed(2), JSON.stringify(Object.fromEntries(Object.entries(r.residual).map(([k, v]: any) => [k, +v.toFixed(2)]))), r.reactions.map((x: any) => `${x.anchor}:${x.fyN.toFixed(0)}${x.active ? "" : "(off)"}`).join(" "));
}
