/** 📇 Compile exact immutable owner scheduling indices without changing canonical AST ordering. */
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {readFile,writeFile} from "node:fs/promises";
import {createHash} from "node:crypto";
import assert from "node:assert/strict";
const root=resolve(import.meta.dir,"../../../../../../.."),schema=resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema"),base=resolve(schema,"⚖️validation");
const {compileIfc4x3Express}=await import(pathToFileURL(resolve(base,"compiler.ts")).href),compiled=await compileIfc4x3Express();
assert.equal(compiled.complete,true,JSON.stringify(compiled.diagnostics));
const program=compiled.program,digest=createHash("sha256").update(JSON.stringify(program)).digest("hex"),native=await readFile(resolve(base,"program.rs"),"utf8");
assert.equal(native.match(/EXPRESS_PROGRAM_SHA256: &str = "([a-f0-9]+)"/)?.[1],digest,"Canonical native program must match current compiled source before indexing");
const catalog=JSON.parse(await readFile(resolve(schema,"📚️catalog/🔣️.json"),"utf8"));
const kinds=["rules","derived","inverse","bounds","unique"] as const;
const owners=[...new Set(kinds.flatMap(kind=>program[kind].map((row:any)=>row.owner)))].sort();
const subtype=(actual:string,expected:string)=>{for(let i=0;i<32;i++){if(actual===expected)return true;const parent=catalog.entities[actual]?.supertype;if(!parent)return false;actual=parent;}throw Error("Released hierarchy exceeds static admission");};
const indices=(kind:typeof kinds[number],accept:(owner:string)=>boolean)=>program[kind].flatMap((row:any,index:number)=>accept(row.owner)?[index]:[]);
const emit=(owner:string,inherited:boolean)=>`OwnerIndex{owner:${JSON.stringify(owner)},${kinds.map(kind=>`${kind}:&[${indices(kind,key=>inherited?subtype(owner,key):owner===key).join(",")}]`).join(",")}}`;
for(const kind of kinds){const all=owners.flatMap(owner=>indices(kind,key=>owner===key)).sort((a,b)=>a-b);assert.deepEqual(all,program[kind].map((_:any,index:number)=>index));}
const output=`/// 📇 Immutable owner indices of the exact canonical native program.\npub const PROGRAM_SHA256:&str=${JSON.stringify(digest)};\npub static OWNERS:&[OwnerIndex]=&[${owners.map(owner=>emit(owner,false)).join(",\n")}];\npub static ENTITIES:&[OwnerIndex]=&[${Object.keys(catalog.entities).sort().map(owner=>emit(owner,true)).join(",\n")}];\n`;
const path=resolve(base,"program-index-data.rs");assert.ok(path.length<=256);
if(process.argv.includes("--check"))assert.equal(await readFile(path,"utf8"),output);else await writeFile(path,output);
console.log(JSON.stringify({sha256:digest,owners:owners.length,entities:Object.keys(catalog.entities).length,counts:Object.fromEntries(kinds.map(kind=>[kind,program[kind].length])),checked:process.argv.includes("--check")}));
