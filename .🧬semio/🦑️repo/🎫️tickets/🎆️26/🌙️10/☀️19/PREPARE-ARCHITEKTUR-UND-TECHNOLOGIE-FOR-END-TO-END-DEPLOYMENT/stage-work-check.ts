import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {strict as assert} from "node:assert";
import {Database} from "bun:sqlite";
const root=process.argv[2],storage=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts"),source=readFileSync(storage,"utf8"),start=source.indexOf("export function javascriptDependencyStoreWorkV1(");
if(start<0)throw Error("Missing semantic work implementation");
const body=source.indexOf("{\n",start);let depth=1,end=body+1;while(depth){end++;if(source[end]==="{")depth++;if(source[end]==="}")depth--;}
const transpiler=new Bun.Transpiler({loader:"ts"}),factory=new Function(transpiler.transformSync(source.slice(start,end+1).replace("export function ","function "))+";return javascriptDependencyStoreWorkV1;")(),corpus=JSON.parse(readFileSync(join(storage,"..","🧪️tests","📏️work","🔣️.json"),"utf8")),testSource=readFileSync(join(storage,"..","🧪️tests","📏️work","🟦️.ts"),"utf8"),testBody=testSource.slice(testSource.indexOf('test("dependency Store semantic work')).replace(/^const storageSource=.*$/m,"");
const expect=(actual:unknown,label?:string)=>({toEqual:(expected:unknown)=>assert.deepEqual(actual,expected,label),toBe:(expected:unknown)=>assert.equal(actual,expected,label)}),Ajv=class{compile(){return()=>true;}};
const installSchema=JSON.parse(readFileSync(join(storage,"..","🧬️schema","📥️install.json"),"utf8"));installSchema.$defs.ManagerProtocol.const.generationDirectory="j";const pending:Promise<unknown>[]=[];
new Function("test","expect","Ajv","Database","corpus","javascriptDependencyStoreWorkV1","storageSource","createHash","join","installSchema",transpiler.transformSync(testBody))((name:string,body:()=>unknown)=>{pending.push(Promise.resolve(body()));},expect,Ajv,Database,corpus,factory,source,createHash,join,installSchema);
await Promise.all(pending);
console.log(JSON.stringify({passed:corpus.cases.length+corpus.writes.length+corpus.phases.length,invalidCapacities:corpus.invalidMaximum.length,tests:pending.length,oracle:"SQLite semantic accounting",maximumGuardChecks:Math.max(...corpus.cases.map((row:any)=>row.guardChecks)),schema:"Ajv deferred until dependencies sync"}));
