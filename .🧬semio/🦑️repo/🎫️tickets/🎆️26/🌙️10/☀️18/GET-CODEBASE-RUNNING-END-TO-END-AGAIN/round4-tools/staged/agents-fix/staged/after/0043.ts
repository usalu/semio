import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {createHash} from "node:crypto";
import Ajv from "ajv";
import {sha256Hex} from "@semio-tech/framework-hash";
/** 🪪️ Ajv and independent OpenSSL SHA256 validate every ordered original-source transcript. */
export function runCanonicalSourceIdentityChecks():number{
 const fixture=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8"));const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));const validate=new Ajv({strict:true}).compile(schema);assert.equal(validate(fixture),true,JSON.stringify(validate.errors));let checks=0;
 for(const row of fixture.cases){const count=Buffer.alloc(8);count.writeBigUInt64BE(BigInt(row.roots.length));const transcript=Buffer.concat([Buffer.from(fixture.domain,"utf8"),Buffer.from([0]),count,Buffer.from(JSON.stringify(row.roots),"utf8")]);assert.equal(transcript.toString("hex"),row.transcriptHex,row.id);assert.equal(createHash("sha256").update(transcript).digest("hex"),row.sha256,row.id);assert.equal(sha256Hex(transcript),row.sha256,row.id);checks++;}
 let deep:unknown="deep original";for(let level=0;level<fixture.birth.deepArrayLayers;level++)deep=[deep];const deepBytes=Buffer.from(JSON.stringify(deep));assert.equal(deepBytes.length,141);assert.equal(sha256Hex(deepBytes),createHash("sha256").update(deepBytes).digest("hex"));assert.equal(fixture.maximumSourceDepth,fixture.birth.frames);assert.equal(fixture.birth.assumeAndHandoffSeparate,true);const admittedDepth=new Ajv({strict:true}).compile({type:"integer",minimum:1,maximum:fixture.maximumSourceDepth});assert.equal(admittedDepth(fixture.birth.deepArrayLayers+1),true);assert.equal(admittedDepth(fixture.maximumSourceDepth+1),false);checks++;
 assert.notEqual(fixture.cases[0].sha256,fixture.cases[1].sha256);assert.notEqual(fixture.cases[0].sha256,fixture.cases[2].sha256);return checks;
}