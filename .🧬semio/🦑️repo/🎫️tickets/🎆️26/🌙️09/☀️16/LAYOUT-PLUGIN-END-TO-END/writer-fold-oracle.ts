/** 🧷️ Writer's native policy uses neutral records and independent JSON Patch text changes. */
import {readFile} from "node:fs/promises";
import {resolve} from "node:path";
import assert from "node:assert/strict";
import Ajv from "ajv/dist/2020";
import jsonpatch from "fast-json-patch";
const base="✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⌨️typing/🔗️fold";
const fixture=JSON.parse(await readFile(resolve(base,"🔣️.json"),"utf8")),schema=JSON.parse(await readFile(resolve(base,"📐️.json"),"utf8"));
const validate=new Ajv({strict:true,allErrors:true}).compile(schema);assert(validate(fixture),JSON.stringify(validate.errors));
const scene=JSON.parse(await readFile("🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/✂️text-splice/🔣️.json","utf8"));
const {composeTextSplicesV1}=await import(resolve("🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🟦️.ts"));
const apply=(text:string,splice:any):string=>{
 const chars=[...text],removed=[...splice.deleted];assert.deepEqual(chars.slice(splice.start,splice.start+removed.length),removed);
 const patches:any[]=[];for(let index=0;index<removed.length;index++)patches.push({op:"remove",path:"/"+splice.start});
 [...splice.insert].forEach((value,index)=>patches.push({op:"add",path:"/"+(splice.start+index),value}));
 return jsonpatch.applyPatch(chars,patches,true,false).newDocument.join("");
};
const expected=new Map<string,unknown>();
for(const row of fixture.cases){
 if(row.compositionCase){
  const source=scene.compositions.find((candidate:any)=>candidate.id===row.compositionCase);assert(source);
  const composition=composeTextSplicesV1(source.net,source.next,scene.contextScalars);assert.deepEqual(composition,source.composition);
  const sequential=apply(apply(source.text,source.net),source.next);
  if(composition.kind==="composed"){assert.equal(apply(source.text,composition.splice),sequential);assert.equal(row.expected,"net");expected.set(row.id,{kind:"net",net:[{mutation:"spliceText",...composition.splice}]});}
  else if(composition.kind==="cancelled"){assert.equal(sequential,source.text);assert.equal(row.expected,"net");expected.set(row.id,{kind:"net",net:[]});}
  else{assert.equal(row.expected,"split");expected.set(row.id,{kind:"split"});}
 }else{
  const replace=row.previous.length===1&&row.incoming.length===1&&row.previous[0].mutation==="editText"&&row.incoming[0].mutation==="editText";
  assert.equal(row.expected,replace?"replaceIncoming":"split");
  if(replace)assert.equal(jsonpatch.applyPatch({text:row.previous[0].text},[{op:"replace",path:"/text",value:row.incoming[0].text}],true,false).newDocument.text,row.incoming[0].text);
  expected.set(row.id,{kind:replace?"replaceIncoming":"split"});
 }
}
/** 🔐️ Require every actual Writer case exactly once with its independent result. */
export function compareWriterNativeRecords(text:string):number{
 const records=text.split("\n").filter(line=>line.startsWith("[writer-native-fold] ")).map(line=>JSON.parse(line.slice("[writer-native-fold] ".length)));
 assert.equal(records.length,expected.size);const seen=new Set<string>();
 for(const row of records){assert(!seen.has(row.name));seen.add(row.name);assert(expected.has(row.name));assert.deepEqual(row.result,expected.get(row.name));}
 assert.equal(seen.size,expected.size);return records.length;
}
const log=process.argv[2];let nativeCompared=false;
if(log){compareWriterNativeRecords(await readFile(resolve(log),"utf8"));nativeCompared=true;}
console.log(JSON.stringify({cases:expected.size,jsonPatch:true,nativeCompared}));
