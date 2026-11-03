const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { solveStructure } = await import(`${L}/🔨️modules/🏗️structure/🟦️.ts`);
const { queries } = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const { readdirSync, readFileSync } = await import("node:fs");
const only = process.argv.slice(2);
let states = 0, failing = 0;
for (const cat of ["🪑️objects", "🛋️furniture"])
  for (const slug of readdirSync(`${L}/🗂️catalog/${cat}`)) {
    if (only.length && !only.includes(slug)) continue;
    const a = JSON.parse(readFileSync(`${L}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
    let supports = 0, nan = 0, resid = 0, share = 0, errors = 0, active = 0;
    for (const { query } of queries(a)) {
      states++;
      try {
        const st = evaluate(a, query, { kind: "scale", scale: { denominator: 50 } });
        const r = solveStructure(st);
        supports = Math.max(supports, r.reactions.length);
        active = Math.max(active, r.reactions.filter((x: any) => x.active).length);
        if (r.reactions.some((x: any) => ![x.fxN, x.fyN, x.fzN, x.mxNm, x.myNm, x.mzNm].every(Number.isFinite))) nan++;
        resid = Math.max(resid, ...Object.values(r.residual).map((v: any) => Math.abs(v)));
        const total = r.reactions.reduce((t: number, x: any) => t + Math.max(0, x.fyN), 0);
        if (total > 0) share = Math.max(share, Math.max(...r.reactions.map((x: any) => x.fyN)) / total);
      } catch (e) { errors++; failing++; console.log("   error", slug, (e as Error).message.slice(0, 120)); }
    }
    if (nan > 0) failing++;
    console.log(slug.padEnd(24), "reactions", supports, "max active", active, "non-finite", nan, "errors", errors, "worst residual", resid.toExponential(1), "largest share", share.toFixed(2));
  }
console.log("states", states, "failing", failing);
