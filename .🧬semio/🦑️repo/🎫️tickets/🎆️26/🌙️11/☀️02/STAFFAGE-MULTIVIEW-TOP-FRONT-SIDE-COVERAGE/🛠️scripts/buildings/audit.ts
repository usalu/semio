import { readFileSync } from "node:fs";
const R = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { sweep, consistency, authoredScales } = await import(`${R}/🧪️tests/🧊️three-views/🧰️.ts`);
const ids = process.argv.slice(2);
const cat: Record<string,string> = {};
for (const g of ["🧱️components","🔌️appliances","👤️humans"]) for (const id of ids) { try { readFileSync(`${R}/🗂️catalog/${g}/${id}/🔣️.json`); cat[id]=`${R}/🗂️catalog/${g}/${id}/🔣️.json`; } catch {} }
for (const id of ids) {
  const a = JSON.parse(readFileSync(cat[id], "utf8")).asset;
  const out: string[] = [];
  for (const s of authoredScales(a)) {
    for (const f of [...sweep(a, s), ...consistency(a, s)]) out.push(`${f.scale} ${f.check}: ${f.message}`);
  }
  const syms = (a.symbols ?? []).map((y: any) => `${y.id}[${Object.entries(y.geometry).filter(([,v]:any)=>v&&v.length).map(([k])=>k).join("+")}]`);
  console.log(`## ${id} dims=${JSON.stringify(a.dimensions)} symbols=${syms.join(",")}`);
  for (const l of out) console.log("  " + l.slice(0, 400));
  if (!out.length) console.log("  OK");
}
