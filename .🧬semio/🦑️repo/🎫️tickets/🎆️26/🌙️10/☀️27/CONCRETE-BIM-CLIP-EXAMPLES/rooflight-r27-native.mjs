/** ☀️ Independent native rooflight topology, material constituents and placed triangles. */
import {createRequire} from 'node:module';
import {join} from 'node:path';
import {verifyServiceSteps} from './native-services-r21.mjs';
/** 🧵 Decode actual native material and shape-aspect ownership, including localized names. */
export async function verifyRooflightSteps(config,samples){
 const result=await verifyServiceSteps(config,samples);
 const library=createRequire(import.meta.url)(config.modulePath),api=new library.IfcAPI();
 await api.Init(name=>join(config.wasmDirectory,name));
 try{for(const sample of samples){
  const id=api.OpenModel(new TextEncoder().encode(sample.text));
  try{
   const get=key=>api.GetLine(id,Number(sample.ids[key].slice(1)));
   const set=get('constituentset');
   const constituents=set.MaterialConstituents.map(ref=>api.GetLine(id,ref.value));
   const roles=constituents.map(c=>c.Name.value).sort();
   if(JSON.stringify(roles)!==JSON.stringify(sample.roles))throw Error('Constituent roles changed');
   const names=constituents.map(c=>api.GetLine(id,c.Material.value).Name.value);
   if(JSON.stringify([...new Set(names)].sort())!==JSON.stringify(sample.materials))throw Error('Localized constituent materials changed');
   const parts=Object.keys(sample.ids).filter(k=>/^aspect_representation[0-9]+$/.test(k)).flatMap(key=>get(key).Items.map(ref=>ref.value));
   if(new Set(parts).size!==sample.parts||parts.length!==sample.parts)throw Error('Aspect ownership is not a partition');
  }finally{api.CloseModel(id);}
 }}finally{api.Dispose();}
 return result;
}
