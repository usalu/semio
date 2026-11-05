import fs from 'node:fs';
import assert from 'node:assert/strict';
import {createRequire} from 'node:module';
import {OBJLoader} from 'three/addons/loaders/OBJLoader.js';
const require=createRequire(import.meta.url),Ajv=require('ajv');
const folder='✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔌️mounted/';
const fixture=JSON.parse(fs.readFileSync(folder+'import.json','utf8')),schema=JSON.parse(fs.readFileSync(folder+'import.schema.json','utf8'));
assert(new Ajv({strict:true}).validate(schema,fixture));
const raw=fixture.source+fixture.paddingLine.repeat(fixture.paddingLines);
for(const form of fixture.forms){
 const payload=form==='dataUrl'?'data:model/obj;base64,'+Buffer.from(raw).toString('base64'):raw;
 assert(Buffer.byteLength(payload)>16384&&Buffer.byteLength(payload)<2097152);
 const decoded=form==='dataUrl'?Buffer.from(payload.split(',')[1],'base64').toString('utf8'):payload;
 const meshes=new OBJLoader().parse(decoded).children;assert.equal(meshes.length,fixture.expected.objects);
 const geometry=meshes[0].geometry,positions=[];for(let i=0;i<geometry.attributes.position.count;i++)positions.push([geometry.attributes.position.getX(i),geometry.attributes.position.getY(i),geometry.attributes.position.getZ(i)]);
 assert.deepEqual(positions.sort(),fixture.expected.positions.toSorted());assert.equal(geometry.attributes.position.count/3,fixture.expected.faces);
 console.log(form+': '+Buffer.byteLength(payload)+' bytes, independent Three geometry matched');
}

const policyFolder='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/🎟️typed/🏃️runtime/';
const policy=JSON.parse(fs.readFileSync(policyFolder+'limit.json','utf8')),policySchema=JSON.parse(fs.readFileSync(policyFolder+'limit.schema.json','utf8'));assert(new Ajv({strict:true}).validate(policySchema,policy));const wire=Buffer.from(JSON.stringify({count:policy.count,kind:'k'.repeat(policy.payloadBytes)}));assert(wire.length>policy.ordinaryBytes&&wire.length<=policy.maximumBytes);console.log('generic producer limit fixture: independent JSON wire extent matched');
