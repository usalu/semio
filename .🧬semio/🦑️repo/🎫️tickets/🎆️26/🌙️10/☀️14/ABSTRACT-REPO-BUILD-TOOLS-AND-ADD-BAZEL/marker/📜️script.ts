import assert from "node:assert/strict";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {dirname,join} from "node:path";
const root=process.env.SEMIO_WORKSPACE_ROOT!,output=process.env.SEMIO_TEST_ARTIFACT_DIR!;
if(!root||!output)throw Error("Explicit workspace and ticket artifacts required");
const {testArtifactPublication}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🧪️tests/🟦️.ts"));
await testArtifactPublication(output);
console.log("[DEBUG] Framework artifact publication and independent marker oracle passed");
const fixture=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));
const {stageArtifacts}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts"));
const browser=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle");
const {copyBrowserArtifacts}=await import(join(browser,"📦️distribution/🟦️.ts"));
const temporary=mkdtempSync(join(output,"marker-consumer-")),source=join(temporary,"published"),destination=join(temporary,"consumer"),files=new Map<string,string>();
try{
 for(const[name,text]of Object.entries(fixture.files)){const path=join(temporary,"input",name);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,String(text));files.set(name,path);}
 await stageArtifacts(source,fixture.owner,files,{leaseDirectory:join(temporary,"leases")});
 const marker=JSON.parse(readFileSync(join(source,fixture.markerFilename),"utf8")),require=createRequire(join(root,"package.json")),Ajv=require("ajv").default,schema=JSON.parse(readFileSync(join(browser,"🧬️schema/🔣️.json"),"utf8")),validator=new Ajv();validator.addSchema(schema);
 assert(validator.validate({$ref:schema.$id+"#/$defs/BrowserArtifactDistributionV1"},marker));
 assert.equal(validator.validate({$ref:schema.$id+"#/$defs/BrowserArtifactDistributionV1"},{...marker,files:[fixture.markerFilename]}),false);
 const copied=await copyBrowserArtifacts(destination,[{root:source,destination:"fixture",owner:fixture.owner}]);assert.equal(copied,files.size);
 const received=await require("fast-glob")("**/*",{cwd:join(destination,"fixture"),dot:true,onlyFiles:true});assert.deepEqual(received.sort(),Object.keys(fixture.files).sort());
 for(const[name,text]of Object.entries(fixture.files))assert.equal(readFileSync(join(destination,"fixture",name),"utf8"),text);
 await assert.rejects(()=>copyBrowserArtifacts(join(temporary,"foreign"),[{root:source,destination:"fixture",owner:"foreign"}]),/owner mismatch/);
 console.log(`[DEBUG] Actual browser consumer accepted ${fixture.markerFilename}, copied ${copied} declared files and excluded marker bytes; AJV and fast-glob agreed`);
}finally{rmSync(temporary,{recursive:true,force:true});}
