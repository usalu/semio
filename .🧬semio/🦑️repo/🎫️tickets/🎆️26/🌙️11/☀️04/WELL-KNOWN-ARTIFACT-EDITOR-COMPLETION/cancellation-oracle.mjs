import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import Decimal from "decimal.js";
const base="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🌱️initialization/🎟️registration/🧪️tests/";
const law=JSON.parse(readFileSync(base+"🛑️cancellation.json","utf8"));
const schema=JSON.parse(readFileSync(base+"🛑️schema.json","utf8"));
assert.equal(new Ajv().validate(schema,law),true);
for(const row of law.cases){
const retained=new Map([["finalAllocation",new Decimal(4096)],["originalSources",new Decimal(128)]]);
if(row.cancelAt!=="final-allocation-before-service-initialization")retained.set("childService",new Decimal(1024));
if(row.cancelAt==="after-tool-service-backing")retained.set("toolService",new Decimal(512));
const born=Decimal.sum(...retained.values());let released=new Decimal(0);let published=false;
for(const [owner,bytes]of retained){released=released.plus(bytes);retained.delete(owner);}
const actual={published,balanced:born.equals(released)&&retained.size===0};assert.deepEqual(actual,row.expected);
}console.log("[DEBUG] AJV and independent Decimal cleanup ledger passed",law.cases.length,"boundaries");
