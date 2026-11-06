import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {createRequire} from "node:module";
import {resolve} from "node:path";
const require=createRequire(resolve("package.json")),Ajv=require("ajv");
const Decimal=require("decimal.js");
const root="🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🛂️allocation/🧺️backing";
const fixture=JSON.parse(readFileSync(resolve(root,"🧪️tests/🔣️.json"),"utf8"));
const schema=JSON.parse(readFileSync(resolve(root,"🧬️schema/🔣️.json"),"utf8"));
const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
assert.ok(validate(fixture),JSON.stringify(validate.errors));
const width={byte:1,word:8,wide:64,zero:0};
for(const row of fixture.vectors){assert.equal(new Decimal(row.count).times(width[row.kind]).toNumber(),row.bytes,row.id);const actual=Buffer.allocUnsafeSlow(row.bytes);assert.equal(actual.buffer.byteLength,row.bytes,row.id);}
for(const bytes of fixture.strings){const actual=Buffer.allocUnsafeSlow(bytes);assert.equal(actual.buffer.byteLength,bytes);}
console.log(JSON.stringify({vectors:fixture.vectors.length,strings:fixture.strings.length,schema:true,oracle:"decimal.js + Node Buffer.allocUnsafeSlow",rustExecution:false}));
