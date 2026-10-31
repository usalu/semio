/** 🧿 Independently read exact translated analytic axes and source bore placements through native IFC. */
import assert from 'node:assert/strict';
import {readFile,writeFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {createRequire} from 'node:module';
import {dirname,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {setupNativeIfcEngine} from './native-ifc-setup-r18.mjs';
const ticket=dirname(fileURLToPath(import.meta.url)),out=join(ticket,'🗑️generated/analytic-axis-r95'),path=join(out,'samples.json'),bytes=await readFile(path),input=JSON.parse(bytes),hash=value=>createHash('sha256').update(value).digest('hex');
const validatorPaths=[fileURLToPath(import.meta.url),join(ticket,'native-ifc-setup-r18.mjs')],validatorInputs=Object.fromEntries(await Promise.all(validatorPaths.map(async path=>[path,hash(await readFile(path))])));
for(const[name,value]of Object.entries(input.before))assert.equal(hash(await readFile(name)),value);
const config=await setupNativeIfcEngine(join(ticket,'🗑️generated/joint-ifc-r30')),library=createRequire(import.meta.url)(config.modulePath),api=new library.IfcAPI();await api.Init(name=>join(config.wasmDirectory,name));const rows=[];
try{for(const sample of input.samples){const model=api.OpenModel(new TextEncoder().encode(sample.text));try{
const element=api.GetLine(model,Number(sample.ids.element.slice(1)));assert.equal(element.type,library.IFCCOLUMN);let points=0;
for(const[ref,expected]of Object.entries(sample.points)){const point=api.GetLine(model,Number(ref.slice(1)));assert.equal(point.type,library.IFCCARTESIANPOINT);assert.deepEqual(point.Coordinates.map(item=>item.value),expected);points++;}
assert.ok(points>0);rows.push({case:sample.case,language:sample.language,exact_native_axis_points:points});
}finally{api.CloseModel(model);}}}finally{api.Dispose();}
for(const[name,value]of Object.entries({...input.before,...validatorInputs}))assert.equal(hash(await readFile(name)),value);assert.equal(hash(await readFile(path)),hash(bytes));await writeFile(join(out,'native-result.json'),JSON.stringify({success:true,input_sha256:hash(bytes),source_inputs:input.before,validator_inputs:validatorInputs,rows},null,2));console.log('[DEBUG] Native exact analytic axes GREEN',rows.length,'fragments',rows.reduce((sum,row)=>sum+row.exact_native_axis_points,0),'exact points');
