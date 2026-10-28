/** 🧭️ Independent RFC6901 lookup and JSON Patch output behind the existing test-library interface. */
import patch from "fast-json-patch";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {strict as assert} from "node:assert";
interface PointerOracle {existing(path:string):boolean;insert(path:string):boolean;member(path:string):boolean;value(path:string):unknown}
const root=resolve(import.meta.dirname,"../../../../../../..");
const fixture=JSON.parse(readFileSync(resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/pointer.json"),"utf8"));
function segments(path:string):string[]|undefined{
 if(path==="")return[];
 if(!path.startsWith("/")||/~(?:[^01]|$)/u.test(path))return undefined;
 return path.slice(1).split("/").map(key=>key.replace(/~1/g,"/").replace(/~0/g,"~"));
}
function oracle(document:object):PointerOracle{
 const value=(path:string)=>patch.getValueByPointer(document,path);
 const existing=(path:string)=>{const fields=segments(path);if(!fields)return false;try{if(fields.length===0)return true;const parent=value(path.slice(0,path.lastIndexOf("/")));const key=fields.at(-1)!;if(Array.isArray(parent)&&! /^(0|[1-9][0-9]*)$/.test(key))return false;return patch.validate([{op:"remove",path}],document)===undefined;}catch{return false;}};
 return {value,existing,insert:path=>{const fields=segments(path);if(!fields?.length)return false;try{const parent=value(path.slice(0,path.lastIndexOf("/")));const key=fields.at(-1)!;if(Array.isArray(parent)&&key!=="-"&&!/^(0|[1-9][0-9]*)$/.test(key))return false;return patch.validate([{op:"add",path,value:null}],document)===undefined;}catch{return false;}},member:path=>{if(!existing(path)||path==="")return false;const parent=value(path.slice(0,path.lastIndexOf("/")));return typeof parent==="object"&&parent!==null&&!Array.isArray(parent);}};
}
const reference=oracle(fixture.document);
for(const row of fixture.paths){assert.equal(reference[row.mode as "existing"|"insert"|"member"](row.path),row.accepted,JSON.stringify(row));if(row.accepted&&row.mode==="existing")assert.notEqual(reference.value(row.path),undefined);}
for(const row of fixture.moves){const from=segments(row.from),path=segments(row.path);const accepted=Boolean(from?.length&&path?.length&&reference.existing(row.from)&&!(path.length>from.length&&from.every((key,index)=>key===path[index])));assert.equal(accepted,row.accepted);}
const updated=patch.applyPatch(fixture.document,[{op:"replace",path:"/a~1b/~0name/Grüße🧬/1",value:"changed"},{op:"add",path:"/array/-",value:3}],true,false).newDocument;
assert.equal(updated["a/b"]["~name"]["Grüße🧬"][1],"changed");assert.deepEqual(updated.array,[0,1,2,3]);assert.equal(updated.id,"18446744073709551615");assert.equal(fixture.document.array.length,3);assert.equal(fixture.nativeExecution,false);
console.log(JSON.stringify({oracle:"fast-json-patch 3.1.1 with explicit RFC6901/array grammar",paths:fixture.paths.length,moves:fixture.moves.length,utf8EscapedReplacement:true,appendOutput:true,exactU64Unchanged:true,nativeCursorExecution:false}));
