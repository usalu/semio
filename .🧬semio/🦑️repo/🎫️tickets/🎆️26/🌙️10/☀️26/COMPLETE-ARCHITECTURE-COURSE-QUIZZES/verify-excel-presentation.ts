import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
const q=JSON.parse(readFileSync("🎓️teaching/🏛️architecture/📊️excel/❓️quiz/🔣️.json","utf8"));
const f=JSON.parse(readFileSync("🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/📊️excel/🔣️.json","utf8")).presentation;
const items=q.tasks.flatMap((t:any)=>t.items);
const address=(m:any)=>m.columns[m.selection.column]+(m.selection.row+1);
const cell=(m:any,a:string,l:string)=>{const [,c,r]=/^([A-Z]+)(\d+)$/.exec(a)!; return m.rows[Number(r)-1][m.columns.indexOf(c)][l];};
for (const v of [...f.formulaCells,...f.pasteValueCells]) {
 const m=items.find((i:any)=>i.id===v.item).mockup;
 assert.equal(address(m),v.active,v.item+" active cell");
 for (const l of ["en","de"]) {
  assert.equal(m.formula[l],v.formula,v.item+" formula bar");
  assert.equal(Number(cell(m,v.active,l).replace(",",".")),v.result,v.item+" displayed result");
  for(const [a,n] of Object.entries(v.inputs??{})) assert.equal(Number(cell(m,a,l).replace(",",".")),n,v.item+" source "+a);
 }
}
for (const t of q.tasks) for (const i of t.items) for(const l of ["en","de"]) {
 const m=i.mockup;
 if(t.id==="copying-references") {
  assert.equal(m.formula[l],cell(m,address(m),l),i.id);
  assert.ok(m.caption[l].includes(l==="en"?"Formula display mode":"Formelanzeigemodus"),i.id);
 } else if(!["formula","reference","values"].includes(i.category)) assert.equal(m.formula[l],cell(m,address(m),l),i.id);
}
assert.equal(items.length,110);
console.log("[DEBUG] Passed independent Excel presentation cases and all 110 active-cell/formula-bar associations.");
