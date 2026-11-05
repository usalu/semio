import fs from "node:fs";
import path from "node:path";
import assert from "node:assert/strict";
import {createRequire} from "node:module";
const require=createRequire(import.meta.url),toml=require("@iarna/toml"),smol=require("smol-toml"),Ajv=require("ajv");
const root=path.resolve("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg"),base=path.join(root,"🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline"),schema=path.join(base,"🧬️schema/📦️packages/🦀️rust"),apps=path.join(base,"✏️editor/📦️packages/🦀️rust");
const fixture=JSON.parse(fs.readFileSync(path.join(root,"🧪️tests/🧫️ownership/🔣️.json"),"utf8"));
assert(new Ajv().compile({type:"object",required:["schema","vocabulary","authoring"],properties:{schema:{const:"stdio.jpg.component-ownership/v1"},vocabulary:{type:"object"},authoring:{type:"object"}}})(fixture));
function manifest(dir){const text=fs.readFileSync(path.join(dir,"Cargo.toml"),"utf8");const value=toml.parse(text);assert.deepEqual(value,smol.parse(text));return value}
const vocabulary=manifest(schema),authoring=manifest(apps),pure=fs.readFileSync(path.join(schema,"🦀️.rs"),"utf8"),app=fs.readFileSync(path.join(apps,"🦀️.rs"),"utf8"),facade=fs.readFileSync(path.join(root,"🦀️.rs"),"utf8");
assert.equal(vocabulary.package.name,fixture.vocabulary.package);
assert.deepEqual(Object.keys(vocabulary.dependencies).sort(),fixture.vocabulary.runtimeOwners.slice().sort());
for(const name of fixture.vocabulary.forbiddenModules)assert(!new RegExp("pub mod "+name+"\\b").test(pure));
assert.equal(authoring.package.name,fixture.authoring.package);
assert(authoring.dependencies["semio-framework-ui-contract"]);
assert.equal((app.match(/pub mod main;/g)||[]).length,fixture.authoring.frameWindows);
assert(Buffer.byteLength(app)<=fixture.authoring.maximumSourceBytes);
assert(!facade.includes("semio_s_artifact_stdio_jpg_schema_2::editor::"));
assert(!facade.includes("semio_s_artifact_stdio_jpg_schema_2::viewer::"));
assert(!pure.includes("🪞️mirror/"));
assert(!app.includes("🪞️mirror/"));
let pathCount=0;
for(const dir of [schema,apps]){
  const source=fs.readFileSync(path.join(dir,"🦀️.rs"),"utf8");
  for(const match of source.matchAll(/#\[path = "([^"]+)"\]/g)){
    if(match[1]===".")continue;
    assert(fs.existsSync(path.resolve(dir,match[1])),match[1]);
    pathCount++;
  }
}
console.log(JSON.stringify({tomlIndependentParsers:2,pureRuntimeOwners:Object.keys(vocabulary.dependencies).length,ownedAppWindows:fixture.authoring.frameWindows,checkedMounts:pathCount,authoringSourceBytes:Buffer.byteLength(app),scope:"Physical owner/neutral contract and independent manifest parsing; Rust compiler remains required."}));

const {pathToFileURL}=await import("node:url");
const {readRustSourceClosureV1}=await import(pathToFileURL(path.resolve("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts")).href);
const closures=[];
for(const [owner,dir]of [["vocabulary",schema],["authoring",apps],["mutations",path.join(root,"🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📦️packages/🦀️rust")]]){
  for(const cfg of [[],["test"]]){
    const inputs=await readRustSourceClosureV1([path.join(dir,"🦀️.rs")],path.resolve("."),cfg,async p=>{try{return await fs.promises.readFile(p)}catch(error){if(error.code==="ENOENT")return null;throw error}},{manifestDirectory:dir,generated:()=>undefined});
    closures.push({owner,cfg,files:inputs.length,bytes:inputs.reduce((sum,input)=>sum+input.bytes,0),activeLines:inputs.reduce((sum,input)=>sum+input.activeLines,0)});
  }
}
console.log(JSON.stringify({sourceClosures:closures}));

const policy=JSON.parse(fs.readFileSync(path.resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"),"utf8")).storage.sourceBoundary;
assert.deepEqual({maxSourceFiles:policy.maxSourceFiles,maxSourceBytes:policy.maxSourceBytes,maxActiveLines:policy.maxActiveLines},fixture.ordinaryBoundary);
for(const row of closures){assert(row.files<=policy.maxSourceFiles, row.owner+" cfg "+row.cfg+" file boundary");assert(row.bytes<=policy.maxSourceBytes,row.owner+" byte boundary");assert(row.activeLines<=policy.maxActiveLines,row.owner+" line boundary");}
console.log(JSON.stringify({ordinaryPolicy:policy,measuredClosures:closures.length,withinAllBoundaries:true}));
