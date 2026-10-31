/** 🛝️ Independently validate native exclusive flight material and original product-scope Reference. */
import assert from 'node:assert/strict';
import {readFile,writeFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {createRequire} from 'node:module';
import {dirname,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {setupNativeIfcEngine} from './native-ifc-setup-r18.mjs';
import {selectRepresentation,inspectMesh} from './native-mesh-diagnostic-r95.mjs';
const ticket=dirname(fileURLToPath(import.meta.url)),out=join(ticket,'🗑️generated/native-ramp-reference-r95'),path=join(out,'samples.json'),bytes=await readFile(path),input=JSON.parse(bytes),hash=value=>createHash('sha256').update(value).digest('hex'),validatorPaths=[fileURLToPath(import.meta.url),join(ticket,'native-mesh-diagnostic-r95.mjs'),join(ticket,'native-ifc-setup-r18.mjs')],validatorInputs=Object.fromEntries(await Promise.all(validatorPaths.map(async path=>[path,hash(await readFile(path))])));
for(const[name,value]of Object.entries(input.before))assert.equal(hash(await readFile(name)),value);
const config=await setupNativeIfcEngine(join(ticket,'🗑️generated/joint-ifc-r30')),library=createRequire(import.meta.url)(config.modulePath),api=new library.IfcAPI();await api.Init(name=>join(config.wasmDirectory,name));const rows=[];
try{for(const sample of input.samples)for(const representation of ['Body','Reference']){const model=api.OpenModel(new TextEncoder().encode(sample.text));try{
 const element=Number(sample.ids.element.slice(1));assert.equal(api.GetLine(model,element).type,library.IFCRAMPFLIGHT);selectRepresentation(api,model,element,representation);const mesh=inspectMesh(api,model,element),expected=representation==='Body'?sample.body_volume:sample.reference_volume;assert.ok(mesh.rows.length&&mesh.rows.every(row=>row.closed&&row.positive));assert.ok(mesh.float32_volume_interval[0]<=expected&&expected<=mesh.float32_volume_interval[1],JSON.stringify({representation,expected,mesh}));rows.push({language:sample.language,representation,expected,...mesh});
}finally{api.CloseModel(model);}}}finally{api.Dispose();}
for(const[name,value]of Object.entries({...input.before,...validatorInputs}))assert.equal(hash(await readFile(name)),value);assert.equal(hash(await readFile(path)),hash(bytes));await writeFile(join(out,'native-result.json'),JSON.stringify({success:true,input_sha256:hash(bytes),source_inputs:input.before,validator_inputs:validatorInputs,rows},null,2));console.log('[DEBUG] Native ramp Body/full Reference GREEN',rows.length,'separate representation instances');
