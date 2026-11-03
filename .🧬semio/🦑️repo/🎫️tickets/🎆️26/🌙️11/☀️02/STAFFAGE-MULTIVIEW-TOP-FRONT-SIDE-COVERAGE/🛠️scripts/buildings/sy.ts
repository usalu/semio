import { readFileSync, existsSync } from "node:fs";
const R="C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
export const path=(id:string)=>["🧱️components","🔌️appliances","👤️humans"].map(g=>`${R}/${g}/${id}/🔣️.json`).find(existsSync)!;
if (import.meta.main) {
const a = JSON.parse(readFileSync(path(process.argv[2]),"utf8")).asset;
for (const y of a.symbols) { console.log("SYM",y.id,y.semantics,y.units,y.denominators,JSON.stringify(y.title), JSON.stringify(y.description)); console.log(" dimNote",JSON.stringify(y.dimensionalNote)); console.log(" pens",JSON.stringify(y.pens),"hatches",JSON.stringify(y.hatches)); for (const v of ["elevation","front","plan"]) { const g=y.geometry[v]; console.log(" --",v, g===undefined?"ABSENT":g.length); for (const s of g??[]) console.log("   ",JSON.stringify(s).slice(0,300)); } }
}
