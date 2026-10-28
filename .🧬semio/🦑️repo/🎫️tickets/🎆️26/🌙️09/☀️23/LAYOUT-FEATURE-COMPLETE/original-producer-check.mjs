import fs from 'node:fs/promises';
import path from 'node:path';
import assert from 'node:assert/strict';
import {createRequire} from 'node:module';
export async function checkOriginalProducer(root){
 const require=createRequire(path.join(root,'package.json'));const Ajv2020=require('ajv/dist/2020').default;
 const directory=path.join(root,'🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/📤️producer');
 const schema=JSON.parse(await fs.readFile(path.join(directory,'🧬️schema/🔣️.json'),'utf8'));
 const fixture=JSON.parse(await fs.readFile(path.join(directory,'🧪️tests/🧫️fixtures/🔣️.json'),'utf8'));
 const validate=new Ajv2020({strict:false,allErrors:true}).compile(schema);assert.ok(validate(fixture),JSON.stringify(validate.errors));
 const source=await fs.readFile(path.join(directory,'🦀️.rs'),'utf8');const laws=await fs.readFile(path.join(directory,'🧪️tests/🦀️.rs'),'utf8');
 for(const method of ['original_push_demands','push_original','original_seal_demands','seal_original'])assert.ok(source.includes('pub fn '+method),'missing original API '+method);
 assert.ok(source.indexOf('cx.consume_retained(progress)?')<source.indexOf('source.as_ref()'),'source validation must follow original funded admission');
 assert.ok(source.indexOf('credit.state.writer.try_lock()')<source.indexOf('source.take()'),'credit authority must precede ownership movement');
 assert.ok(source.includes('*source=Some(original)'),'fixed queue rejection must restore the exact original');
 assert.ok(source.indexOf('state.chunks.push(source.take()')<source.indexOf('credit.state.bytes.store(next'),'credit must commit only after the no-allocation queue accepts');
 assert.ok(!source.includes('RetainedCloneGrant{'),'producer must never mint a caller grant');
 assert.ok(laws.includes('parent.borrow_context(')&&laws.includes('cx.with_retained_loan(')&&laws.includes('close_original_retained_step('),'native laws must use the original caller through transfer and close');
 assert.ok(laws.includes('JsonByteOracle::representation'),'native output must compare the serde_json byte oracle behind its first-party test interface');
 for(const candidate of ['🦀️.rs','🧬️schema/🔣️.json','🧪️tests/🦀️.rs','🧪️tests/🧫️fixtures/🔣️.json'])assert.ok(path.join(directory,candidate).length<=256);
 return {schemaValid:true,producerCases:fixture.cases.length,sealCases:fixture.sealCases.length,sourceContractAudit:true,nativeCompiled:false,nativeExecuted:false};
}
