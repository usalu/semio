/** 🔬 Compare original official ADD2 files through owned syntax, editing and semantic boundaries. */
import {readFile,writeFile} from "node:fs/promises";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
import assert from "node:assert/strict";
const root=resolve(import.meta.dir,"../../../../../../.."),base=resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any"),output=resolve(import.meta.dir,"🗑️generated/corpus");
const {parseIfc4x3Step,writeIfc4x3Step}=await import(pathToFileURL(resolve(base,"🚪️io/step.ts")).href);
const {validateIfc4x3Express}=await import(pathToFileURL(resolve(base,"🧬️schema/⚖️validation/population.ts")).href);
const {compileIfc4x3Express}=await import(pathToFileURL(resolve(base,"🧬️schema/⚖️validation/compiler.ts")).href);
const {applyIfcDiff}=await import(pathToFileURL(resolve(base,"🧬️schema/🔺️diff/🟦️.ts")).href);
const manifest=JSON.parse(await readFile(resolve(output,"downloaded.json"),"utf8")),compiled=await compileIfc4x3Express();
assert.ok(compiled.complete,JSON.stringify(compiled.diagnostics));
const rows:unknown[]=[];
for(const source of manifest.files){
 if(source.schema!=="IFC4X3_ADD2")continue;
 const row:any={path:source.path,sha256:source.sha256},start=performance.now();
 try{
  const graph=await parseIfc4x3Step(await readFile(resolve(output,source.file),"utf8"),{batchSize:4096});
  row.entities=graph.entities.length;
  assert.deepEqual(applyIfcDiff(graph,{}),graph);
  const text=await writeIfc4x3Step(graph,{batchSize:4096});
  assert.deepEqual(await parseIfc4x3Step(text,{batchSize:4096}),graph);
  row.roundtrip=true;
  const semantic=await validateIfc4x3Express(graph,{maximumWork:20_000_000,batchSize:4096},compiled.program);
  row.complete=semantic.complete;row.valid=semantic.valid;row.work=semantic.work;row.rules=semantic.results.length;row.diagnostics=semantic.diagnostics;
 }catch(error){row.error=String(error);}
 row.ms=Math.round(performance.now()-start);rows.push(row);
 await writeFile(resolve(output,"semio.json"),JSON.stringify({commit:manifest.commit,files:rows},(_,value)=>typeof value==="bigint"?value.toString():value,2)+"\n");
 process.stdout.write("[DEBUG] "+JSON.stringify({completed:rows.length,total:manifest.files.length,path:row.path,roundtrip:row.roundtrip,complete:row.complete,valid:row.valid,diagnostics:row.diagnostics?.length,error:row.error,ms:row.ms})+"\n");
}
