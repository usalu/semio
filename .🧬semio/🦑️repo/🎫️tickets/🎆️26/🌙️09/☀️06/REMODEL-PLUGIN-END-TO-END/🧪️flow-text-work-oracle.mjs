import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require=createRequire(resolve("package.json"));
const Ajv=require("ajv");
const {takeWhile,drop}=require("lodash");
const base="🧰️framework/🔨️modules/🧵️job/📝️text/🧳️owned/🧪️tests";
const fixture=JSON.parse(readFileSync(resolve(base,"📁️text-work.json"),"utf8"));
const schema=JSON.parse(readFileSync(resolve(base,"📁️text-work.schema.json"),"utf8"));
const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
assert.ok(validate(fixture),JSON.stringify(validate.errors));
for(const row of fixture.cases){
 let pending=Array.from(row.source),chunks=[],output="";
 while(pending.length){
  let bytes=0;
  const part=takeWhile(pending,char=>{const width=Buffer.byteLength(char,"utf8");if(bytes+width>row.payloadBytes)return false;bytes+=width;return true;});
  assert.ok(part.length,row.id);
  chunks.push(bytes);output+=part.join("");pending=drop(pending,part.length);
 }
 assert.deepEqual(chunks,row.chunks,row.id);
 assert.equal(output,row.source,row.id);
 assert.equal(chunks.reduce((a,b)=>a+b,0),Buffer.byteLength(row.source,"utf8"),row.id);
}
console.log(JSON.stringify({cases:fixture.cases.length,schema:true,oracle:"lodash.takeWhile/drop + Node UTF-8 Buffer",rustExecution:false}));

