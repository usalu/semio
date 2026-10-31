/** 🪵 Publish only a complete source-bound independent stock acceptance result. */
import {readFile,open,writeFile,access} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {dirname,join,relative,sep} from 'node:path';
import {fileURLToPath} from 'node:url';
import assert from 'node:assert/strict';
const root=process.cwd(),ticket=dirname(fileURLToPath(import.meta.url)),out=join(ticket,'🗑️generated/native-r92'),hash=value=>createHash('sha256').update(value).digest('hex');
try{await access(join(out,'skirting.cancel'));throw new Error('Skirting publication cancelled');}catch(error){if(error.code!=='ENOENT')throw error;}
const candidateBytes=await readFile(join(out,'skirting-candidate.json')),candidate=JSON.parse(candidateBytes),cert=JSON.parse(await readFile(join(out,'skirting-validation.json'),'utf8')),publicationBytes=await readFile(join(out,'skirting-publication-input.json')),publication=JSON.parse(publicationBytes);
const nativeBytes=await readFile(join(out,'skirting-candidate-native-r94.json')),native=JSON.parse(nativeBytes),samplesBytes=await readFile(join(out,'skirting-candidate-samples-r94.json')),samples=JSON.parse(samplesBytes);
assert.ok(native.success&&native.unpublished_candidate&&!native.partial_diagnosed&&!native.failures.length,'Complete bilingual strict native candidate proof is required');
assert.equal(Object.keys(native.validator_inputs).length,3);assert.equal(native.input_sha256,hash(samplesBytes));assert.ok(samples.stable&&samples.unpublished_candidate&&!samples.partial_diagnosed);
assert.equal(native.source[relative(root,join(out,'skirting-candidate.json')).split(sep).join('/')],hash(candidateBytes));
const expectedNative=new Set(candidate.elements.flatMap(obj=>['en','de'].map(language=>obj.tag+'|'+language)));assert.equal(native.rows.length,expectedNative.size);assert.deepEqual(new Set(native.rows.map(row=>row.tag+'|'+row.language)),expectedNative);
for(const[name,value]of Object.entries({...native.source,...native.validator_inputs}))assert.equal(hash(await readFile(join(root,name))),value,'native proof input changed: '+name);
assert.ok(candidate.stable&&!candidate.partial_inclined_only&&!candidate.diagnostic_cache&&cert.success);assert.equal(cert.candidate_sha256,hash(candidateBytes));assert.equal(cert.publication_sha256,hash(publicationBytes));assert.equal(cert.products,candidate.elements.length);
for(const [name,value]of Object.entries({...cert.source_inputs,...cert.validation_inputs}))assert.equal(hash(await readFile(join(root,name))),value,'validated input changed: '+name);
const sourcePath=join(root,'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json'),bytes=await readFile(sourcePath),source=JSON.parse(bytes),oldCount=source.elements.length,tags=candidate.elements.map(obj=>obj.tag),existing=new Set(source.elements.map(obj=>obj.tag));
assert.equal(new Set(tags).size,tags.length);assert.ok(tags.every(tag=>!existing.has(tag)));assert.ok(!source.materials.some(obj=>obj.tag===publication.material.tag));
source.materials.push(publication.material);source.elements.push(...candidate.elements);source.relations.push(publication.material_relation);
const architecture=source.models.find(model=>model.id==='architecture');architecture.element_tags.push(...tags);
for(const obj of candidate.elements){const relation=source.relations.find(row=>row.class==='IfcRelContainedInSpatialStructure'&&row.relating===obj.storey);assert.ok(relation);relation.related.push(obj.tag);}
const encoded=Buffer.from(JSON.stringify(source,null,2)+String.fromCharCode(10)),handle=await open(sourcePath,'r+');
try{const stat=await handle.stat();const payload=encoded.length<stat.size?Buffer.concat([encoded,Buffer.alloc(stat.size-encoded.length,32)]):encoded;await handle.write(payload,0,payload.length,0);await handle.sync();}finally{await handle.close();}
const actual=await readFile(sourcePath),check=JSON.parse(actual);assert.equal(check.elements.length,oldCount+tags.length);assert.ok(tags.every(tag=>check.elements.some(obj=>obj.tag===tag)));
const report={success:true,before:hash(bytes),after:hash(actual),products_added:tags.length,products:check.elements.length,source_tags:tags,material:publication.material.tag,certificate:hash(await readFile(join(out,'skirting-validation.json'))),native_certificate:hash(nativeBytes),native_capture:hash(samplesBytes),native_fragments:native.rows.length,native_cells:native.rows.reduce((sum,row)=>sum+row.physical_body_items,0),candidate:hash(candidateBytes)};
await writeFile(join(out,'skirting-publication.json'),JSON.stringify(report,null,2));console.log('[DEBUG] validated timber source publication',JSON.stringify({success:report.success,after:report.after,products_added:report.products_added,products:report.products}));
