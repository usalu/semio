/** ⏳️ Awaits existing direct negative semantic assertions under guarded Source pairs. */
import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const ticket="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O";
const census=JSON.parse(readFileSync(join(ticket,"🗑️generated/root-all-artifact-source-negative-promise-assertion-census.json"),"utf8"));
const files=census.rows.map(row=>{
 if(row.positions.some(p=>!p.async||!p.statement))throw Error("rejection assertion requires independent readback");
 let after=row.before;
 for(const p of [...row.positions].sort((a,b)=>b.offset-a.offset)){if(after.slice(p.offset,p.offset+7)!=="expect(")throw Error("AST source offset changed");after=after.slice(0,p.offset)+"await "+after.slice(p.offset);}
 if(readFileSync(row.path,"utf8")!==row.before)throw Error("concurrent source change requires fresh inventory: "+row.path);
 return{path:row.path,before:row.before,after,insertions:row.positions.length};
});
writeFileSync(join(ticket,"📥️inputs/root-step-wfc-three-source-negative-promise-await-pairs.json"),JSON.stringify({scope:"Test-only ordering repair",files},null,2)+"\n");
for(const file of files){if(readFileSync(file.path,"utf8")!==file.before)throw Error("concurrent source change before guarded write");writeFileSync(file.path,file.after);}
const report=join(ticket,"📓️all-artifact-source-negative-promise-runtime-audit.md");
writeFileSync(report,readFileSync(report,"utf8")+"\n## Narrow Await Pair Mounted\n\n"+files.reduce((n,f)=>n+f.insertions,0)+" existing negative matchers are now awaited in "+files.length+" existing async Source leaves. Every original condition is retained. Actual after replay and broader after AST census are pending.\n");
console.log(JSON.stringify({files:files.length,insertions:files.reduce((n,f)=>n+f.insertions,0),scope:"test-only"}));
