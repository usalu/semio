import { strict as assert } from "node:assert";
import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const folder=process.argv[2];
const section=(id,bytes)=>[id,bytes.length,...bytes];
const text=name=>[name.length,...Buffer.from(name)];
const code=body=>[body.length+2,0,...body,0x0b];
const input=join(folder,"fold-fixture.wasm"), output=join(folder,"fold-fixture-result.wasm");
writeFileSync(input,Uint8Array.from([0,97,115,109,1,0,0,0,
 ...section(1,[1,0x60,0,1,0x7f]),
 ...section(3,[5,0,0,0,0,0]),
 ...section(4,[1,0x70,0,2]),
 ...section(7,[2,...text("answer"),0,4,...text("table"),1,0]),
 ...section(9,[1,0,0x41,0,0x0b,2,2,3]),
 ...section(10,[5,...code([0x41,7]),...code([0x41,7]),...code([0x41,7]),...code([0x41,7]),...code([0x10,1])])]));
execFileSync(join(folder,"deduplicate-requester"),[input,output],{stdio:"inherit"});
for (const file of [input,output]) {
 const { instance }=await WebAssembly.instantiate(readFileSync(file));
 assert.equal(instance.exports.answer(),7);
 const t=instance.exports.table;
 assert.equal(t.get(0)(),7);
 assert.equal(t.get(1)(),7);
 assert.notEqual(t.get(0),t.get(1));
}
assert.ok(readFileSync(output).length<readFileSync(input).length);
