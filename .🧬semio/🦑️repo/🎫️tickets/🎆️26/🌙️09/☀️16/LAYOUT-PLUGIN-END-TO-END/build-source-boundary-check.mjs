/** 🧭️ Verifies the actual bounded build source and independent declaration discovery. */
import assert from "node:assert/strict";
import {readFileSync,writeFileSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import ts from "typescript";
const ticket=dirname(fileURLToPath(import.meta.url)),root=resolve(ticket,"../../../../../../..");
const fixture=JSON.parse(readFileSync(join(ticket,"build-source-boundary-cases.json"),"utf8"));
const authorizer=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts"));
const control=new AbortController(),deadline=setTimeout(()=>control.abort(Error("bounded source oracle deadline")),120000);
try{
 const cliImports=ts.preProcessFile(readFileSync(join(root,fixture.cliEntryOwner),"utf8"),true,true).importedFiles.filter(row=>row.fileName.endsWith("/📜️script.ts"));
 assert.equal(cliImports.length,1,"one actual CLI frontdoor implementation import");
 const cliEntry=relative(root,resolve(dirname(join(root,fixture.cliEntryOwner)),cliImports[0].fileName)).replaceAll("\\","/");
 assert.equal(cliEntry,fixture.entry,"neutral CLI identity matches the actual frontend import");
 const selector=await import(join(root,fixture.runtimeSelection.owner));
 const selected=selector.repositoryCommandKernelV1(fixture.runtimeSelection.args,fixture.runtimeSelection.environment);
 assert.equal(selected.kind,"runtime");
 const results=[];
 for (const entry of [cliEntry,relative(root,selected.entry).replaceAll("\\","/")]) {
 const captured=await authorizer.captureNxAuthorizerSourcesV1(root,entry,control.signal,{installed:true});
 const bytes=authorizer.nxAuthorizerCapturedBytesV1(captured),total=[...bytes.values()].reduce((n,row)=>n+row.byteLength,0),paths=[...bytes.keys()];
 assert.ok(total<=fixture.maximumBytes);assert.ok(paths.length<=fixture.maximumFiles);
 for(const path of fixture.retained)assert.ok(bytes.has(path),"genuine source remains captured "+path);
 for(const path of fixture.excluded)assert.equal(bytes.has(path),false,"incidental broad barrel excluded "+path);
 const source=readFileSync(join(root,fixture.boundary),"utf8"),parsed=ts.createSourceFile(fixture.boundary,source,ts.ScriptTarget.Latest,true);
 const names=parsed.statements.filter(ts.isExportDeclaration).flatMap(row=>row.exportClause&&ts.isNamedExports(row.exportClause)?row.exportClause.elements.map(item=>item.name.text):[]).sort();
 assert.deepEqual(names,[...fixture.symbols].sort(),"exact domain interface");
 assert.deepEqual(new Bun.Transpiler({loader:"ts"}).scan(source).exports.sort(),names,"independent compiler export oracle");
 const imports=ts.preProcessFile(readFileSync(join(root,fixture.consumer),"utf8"),true,true).importedFiles.map(row=>row.fileName);
 assert.ok(imports.some(path=>resolve(dirname(join(root,fixture.consumer)),path)===join(root,fixture.boundary)),"consumer uses domain interface");
 if(entry!==fixture.entry)for(const path of fixture.runtimeExcluded)assert.equal(bytes.has(path),false,"runtime excludes CLI frontdoor "+path);
 const result={entry,files:paths.length,bytes:total,maximumBytes:fixture.maximumBytes,retained:fixture.retained,excluded:fixture.excluded,exports:names};
 results.push(result);
 console.log("[DEBUG] Actual compiler source closure admitted: "+paths.length+" files, "+total+" bytes; exact discovery interface and independent compiler exports passed");
 }
 writeFileSync(join(ticket,"🗑️generated/fleet-host-2026-11-04/build-source-boundary.json"),JSON.stringify(results,null,2)+"\n");
}finally{clearTimeout(deadline)}
