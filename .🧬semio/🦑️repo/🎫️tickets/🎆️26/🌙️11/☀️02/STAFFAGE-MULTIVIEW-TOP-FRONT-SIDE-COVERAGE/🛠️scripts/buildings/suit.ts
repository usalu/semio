import { readFileSync, existsSync } from "node:fs";
const R = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { resolveRepresentation, DEFAULT_PRINT_PROFILE } = await import(`${R}/🔨️modules/🖨️representation/🟦️.ts`);
for (const id of process.argv.slice(2)) {
  const f = ["🧱️components","🔌️appliances","👤️humans"].map(g=>`${R}/🗂️catalog/${g}/${id}/🔣️.json`).find(existsSync)!;
  const a = JSON.parse(readFileSync(f, "utf8")).asset;
  const row: string[] = [];
  for (const d of [500,200,100,50]) {
    const r = resolveRepresentation(a, { denominator: d }, DEFAULT_PRINT_PROFILE);
    row.push(`${d}:${r.kind}` + (r.kind==="unsuitable" ? `(${r.suitability.violations.length}: ${[...new Set(r.suitability.violations.map((v:any)=>v.check+"/"+v.shapes.join("+")))].slice(0,4).join(",")})` : ""));
  }
  console.log(id.padEnd(26), row.join("  "));
}
