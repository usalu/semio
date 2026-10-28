/** 🔎 Audit the complete canonical continuation instruction surface without evaluating a graph. */
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {readFile,writeFile,mkdir} from "node:fs/promises";
import assert from "node:assert/strict";
const root=resolve(import.meta.dir,"../../../../../../.."),schema=resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema"),base=resolve(schema,"⚖️validation");
const {compileIfc4x3Express}=await import(pathToFileURL(resolve(base,"compiler.ts")).href),compiled=await compileIfc4x3Express();assert.equal(compiled.complete,true);
const p=compiled.program,catalog=JSON.parse(await readFile(resolve(schema,"📚️catalog/🔣️.json"),"utf8")),calls=new Map<string,number>(),expressions=new Map<string,number>(),statements=new Map<string,number>();
const bump=(map:Map<string,number>,name:string)=>map.set(name,(map.get(name)??0)+1);
function visit(value:any){if(!value||typeof value!=="object")return;if(Array.isArray(value)){for(const item of value)visit(item);return;}if(typeof value.kind==="string"){if("offset"in value){if(["assign","return","if","case","repeat","block","escape","skip","call","null"].includes(value.kind)&&("body"in value||"expressions"in value||["escape","skip","null"].includes(value.kind)))bump(statements,value.kind);else bump(expressions,value.kind);}if(value.kind==="call"&&Array.isArray(value.operands))bump(calls,value.operands[0]?.name??"?");}for(const item of Object.values(value))visit(item);}
visit(p);
const builtins=[...calls].filter(([name])=>!p.functions.some((fn:any)=>fn.name===name)&&!catalog.entities[name]&&!catalog.types[name]).sort(([a],[b])=>a.localeCompare(b));
const result={expressionKinds:Object.fromEntries(expressions),statementKinds:Object.fromEntries(statements),builtins:Object.fromEntries(builtins),calls:Object.fromEntries([...calls].sort()),nativeExecution:false};
const generated=resolve(import.meta.dir,"🗑️generated/retained-express");await mkdir(generated,{recursive:true});await writeFile(resolve(generated,"surface.json"),JSON.stringify(result,null,2));
console.log(JSON.stringify({expressions:result.expressionKinds,statements:result.statementKinds,builtins:result.builtins,nativeExecution:false}));
