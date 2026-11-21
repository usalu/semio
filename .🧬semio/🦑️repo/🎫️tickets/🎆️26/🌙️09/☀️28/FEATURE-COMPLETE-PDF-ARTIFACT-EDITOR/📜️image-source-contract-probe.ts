/** 🖼️ Runs the typed image carrier witness through the real retained browser reader. */
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
const {Builder,readers} = await import(resolve("🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts"));
const fixture=JSON.parse(readFileSync("🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️tests/🧫️fixtures/🧾️typed/🔣️.json","utf8"));
const project=(value:unknown)=>{
  const reader=readers.component(new Builder(),value);
  for(let row=reader.next();;row=reader.next())if(row.done)return row.value;
};
const rows=fixture.components.filter((row:{wire:{type:string}})=>row.wire.type==="image");
assert.equal(rows.length,2);
for(const row of rows)assert.deepEqual(project(row.wire),row.expected);
const feature=rows.find((row:{wire:{sourceBodyBytes?:number}})=>row.wire.sourceBodyBytes !== undefined);
for(const size of [-1,2.5,2**32])assert.throws(()=>project({...feature.wire,sourceBodyBytes:size}));
console.log("[DEBUG] Verified the inline and 65,537-byte retained image descriptors");
