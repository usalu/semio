import { readFileSync } from 'node:fs';
import assert from 'node:assert/strict';
import Ajv2020 from 'ajv/dist/2020';
import leb from '@webassemblyjs/leb128/lib/leb.js';
const root = process.cwd();
const { encodeAppFrame, decodeAppFrame } = await import(root + '/🧰️framework/🔨️modules/🧾️app-channel/🎞️frame-codec/🟦️.ts');
const base = root + '/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel';
const fixture = JSON.parse(readFileSync(base + '/🧪️tests/🧫️fixtures/🪪️document-identity-wire/🔣️.json','utf8'));
const validate = new Ajv2020().compile(JSON.parse(readFileSync(base + '/🧬️schema/🪪️document-identity/🔣️.json','utf8')));
const unsigned = (value: bigint):number[] => { const bytes=Buffer.alloc(8);bytes.writeBigUInt64LE(value);return [...leb.encodeUIntBuffer(bytes)]; };
for(const row of fixture.cases){
 assert.equal(validate(row.identity),true);
 const identity={app_instance_id:row.identity.appInstanceId,parent_document_id:row.identity.parentDocumentId,base_revision:BigInt(row.identity.baseRevision)};
 const frame={DocumentIdentity:{in_reply_to:BigInt(row.seq),identity}};
 const document=identity.parent_document_id===null?null:Buffer.from(identity.parent_document_id,'utf8');
 const oracle=[31,...unsigned(BigInt(row.seq)),...unsigned(BigInt(identity.app_instance_id)),...(document===null?[0]:[1,...unsigned(BigInt(document.length)),...document]),...unsigned(identity.base_revision)];
 assert.deepEqual(oracle,row.frameBytes);assert.deepEqual([...encodeAppFrame(frame)],oracle);assert.deepEqual(decodeAppFrame(Uint8Array.from(oracle)),frame);assert.throws(()=>decodeAppFrame(Uint8Array.from([...oracle,0])));
}
const boundary={DocumentIdentity:{in_reply_to:0xffff_ffff_ffff_ffffn,identity:{app_instance_id:0xffff_ffff,parent_document_id:'🎬'.repeat(512),base_revision:0xffff_ffff_ffff_ffffn}}};assert.equal(encodeAppFrame(boundary).length,2077);assert.deepEqual(decodeAppFrame(encodeAppFrame(boundary)),boundary);
for(const baseRevision of ['-1','00','18446744073709551616','+1'])assert.equal(validate({appInstanceId:1,parentDocumentId:'document',baseRevision}),false);
console.log('[DEBUG] identity3 shared vectors + independent Ajv/LEB128 oracle and canonical u64 refusals pass');
const {encodeAppCommand,decodeAppCommand}=await import(root+'/🧰️framework/🔨️modules/🧾️app-channel/🎛️command-codec/🟦️.ts');
const media=JSON.parse(readFileSync(base+'/🧪️tests/🧫️fixtures/🎬️media-export-wire/🔣️.json','utf8'));
const text=(value:string):number[]=>{const bytes=Buffer.from(value,'utf8');return [...unsigned(BigInt(bytes.length)),...bytes];};
for(const name of ['submit','submitCheckpoint']){const row=media.commands[name],f=row.fields;const command={SubmitMediaExport:{seq:BigInt(f.seq),port:f.port,expected_parent_document_id:f.expectedParentDocumentId,expected_base_revision:BigInt(f.expectedBaseRevision),checkpoint_id:f.checkpointId}};const oracle=[37,...unsigned(BigInt(f.seq)),...text(f.port),...text(f.expectedParentDocumentId),...unsigned(BigInt(f.expectedBaseRevision)),...(f.checkpointId===null?[0]:[1,...text(f.checkpointId)])];assert.deepEqual(oracle,row.bytes);assert.deepEqual([...encodeAppCommand(command)],oracle);assert.deepEqual(decodeAppCommand(Uint8Array.from(oracle)),command);}
const invalidSelector=Uint8Array.from(media.commands.submit.bytes);invalidSelector[invalidSelector.length-1]=2;assert.throws(()=>decodeAppCommand(invalidSelector));
console.log('[DEBUG] Head/checkpoint native export command vectors match independent LEB128 and exact selector refusal');
