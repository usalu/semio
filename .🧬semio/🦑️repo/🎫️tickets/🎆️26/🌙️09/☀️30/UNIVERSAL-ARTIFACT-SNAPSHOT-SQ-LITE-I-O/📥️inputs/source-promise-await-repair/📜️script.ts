/** ⏳️ Awaits existing negative semantic assertions while retaining every test condition. */
import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const ticket="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O";
const census=JSON.parse(readFileSync(join(ticket,"🗑️generated/root-semio-source-negative-promise-assertion-census.json"),"utf8"));
const files=census.rows.map(row=>{
 if(row.positions.some(p=>!p.async))throw Error("rejection assertion is outside an async owning test");
 let after=row.before;
 for(const p of [...row.positions].sort((a,b)=>b.offset-a.offset)){if(after.slice(p.offset,p.offset+7)!=="expect(")throw Error("AST source offset changed");after=after.slice(0,p.offset)+"await "+after.slice(p.offset);}
 if(readFileSync(row.path,"utf8")!==row.before)throw Error("concurrent source change requires fresh inventory: "+row.path);
 return{path:row.path,before:row.before,after,insertions:row.positions.length};
});
writeFileSync(join(ticket,"📥️inputs/root-semio-source-negative-promise-await-pairs.json"),JSON.stringify({scope:"Test-only ordering repair",files},null,2)+"\n");
for(const file of files){if(readFileSync(file.path,"utf8")!==file.before)throw Error("concurrent source change before guarded write");writeFileSync(file.path,file.after);}
const report=join(ticket,"📓️semio-source-negative-promise-runtime-audit.md");
writeFileSync(report,readFileSync(report,"utf8")+"\n## Narrow Await Pair Mounted\n\n"+files.reduce((n,f)=>n+f.insertions,0)+" original negative matchers are now awaited in "+files.length+" existing async test leaves. Every original assertion condition and production provider is unchanged. Actual registered after replay is pending. Guarded original/new source pairs are retained under ticket inputs. The initial Nx inline-code inventory failed shell reparsing before AST execution; the saved ticket 📜️script.ts inventory completed successfully.\n");
console.log(JSON.stringify({files:files.length,insertions:files.reduce((n,f)=>n+f.insertions,0),scope:"test-only"}));
