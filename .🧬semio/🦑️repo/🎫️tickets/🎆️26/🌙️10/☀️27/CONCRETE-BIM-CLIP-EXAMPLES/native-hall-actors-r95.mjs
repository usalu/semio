/** 🏗️ Independently verify every changed hall wall and complete dependent door cell. */
import assert from 'node:assert/strict';
import {readFile,writeFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {createRequire} from 'node:module';
import {dirname,join,relative} from 'node:path';
import {fileURLToPath} from 'node:url';
import {verifyProducts,selectBody} from './native-products-r44.mjs';
import {setupNativeIfcEngine} from './native-ifc-setup-r18.mjs';
const root=process.cwd(),ticket=dirname(fileURLToPath(import.meta.url)),out=join(ticket,'🗑️generated/native-r95-hall'),path=join(out,'actor-samples.json'),bytes=await readFile(path),input=JSON.parse(bytes),hash=value=>createHash('sha256').update(value).digest('hex');
assert.ok(input.stable&&input.unpublished_candidate);assert.equal(input.samples.length,10);
const sources=[fileURLToPath(import.meta.url),join(ticket,'native-products-r44.mjs'),join(ticket,'native-ifc-setup-r18.mjs')],validatorInputs=Object.fromEntries(await Promise.all(sources.map(async p=>[relative(root,p),hash(await readFile(p))])));
for(const [name,value] of Object.entries(input.before))assert.equal(hash(await readFile(join(root,name))),value,'changed capture input '+name);
const volumes=await verifyProducts(ticket,input.samples),cfg=await setupNativeIfcEngine(join(ticket,'🗑️generated/joint-ifc-r30')),lib=createRequire(import.meta.url)(cfg.modulePath),api=new lib.IfcAPI();await api.Init(name=>join(cfg.wasmDirectory,name));const rows=[];
try{for(const sample of input.samples){const model=api.OpenModel(new TextEncoder().encode(sample.text));try{
 const element=Number(sample.ids.element.slice(1)),raw=api.GetRawLineData(model,element);assert.equal(raw.arguments.length,sample.entity==='IFCDOOR'?13:9);
 const material=api.GetLine(model,Number(sample.ids.rel_material.slice(1)));assert.ok(material.RelatedObjects.some(ref=>ref.value===element));assert.ok(api.GetLine(model,material.RelatingMaterial.value));
 selectBody(api,model,element);
 const mesh=api.GetFlatMesh(model,element);let cells=0;
 for(let i=0;i<mesh.geometries.size();i++){
  const part=mesh.geometries.get(i),geometry=api.GetGeometry(model,part.geometryExpressID),m=part.flatTransformation,vertices=api.GetVertexArray(geometry.GetVertexData(),geometry.GetVertexDataSize()),indices=api.GetIndexArray(geometry.GetIndexData(),geometry.GetIndexDataSize());
  const point=index=>{const x=vertices[6*index],y=vertices[6*index+1],z=vertices[6*index+2];return[m[0]*x+m[4]*y+m[8]*z+m[12],-(m[2]*x+m[6]*y+m[10]*z+m[14]),m[1]*x+m[5]*y+m[9]*z+m[13]];};
  const edges=new Map(),origin=point(indices[0]);let signed=0;assert.ok(indices.length);
  for(let j=0;j<indices.length;j+=3){const triangle=[0,1,2].map(k=>point(indices[j+k]));for(let e=0;e<3;e++){const a=triangle[e].join('|'),b=triangle[(e+1)%3].join('|');assert.notEqual(a,b,'collapsed native actor edge');const key=a<b?a+'>'+b:b+'>'+a;edges.set(key,(edges.get(key)||0)+1);}const[a,b,c]=triangle.map(p=>p.map((v,k)=>v-origin[k]));signed+=a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]);}
  assert.ok([...edges.values()].every(n=>n===2),'nonmanifold native actor cell');assert.ok(Number.isFinite(signed)&&Math.abs(signed/6)>0,'nonpositive native actor cell');cells++;geometry.delete();
 }
 rows.push({tag:sample.tag,language:sample.language,native_attributes:raw.arguments.length,strict_closed_positive_cells:cells});
}finally{api.CloseModel(model);}}}finally{api.Dispose();}
for(const [name,value] of Object.entries({...input.before,...validatorInputs}))assert.equal(hash(await readFile(join(root,name))),value,'changed native proof input '+name);assert.equal(hash(await readFile(path)),hash(bytes));
await writeFile(join(out,'actor-native.json'),JSON.stringify({success:true,unpublished_candidate:true,input_sha256:hash(bytes),source_inputs:input.before,validator_inputs:validatorInputs,volumes,rows},null,2));console.log('[DEBUG] Native coordinated hall actors GREEN',rows.length,'fragments',rows.reduce((sum,row)=>sum+row.strict_closed_positive_cells,0),'strict cells');
