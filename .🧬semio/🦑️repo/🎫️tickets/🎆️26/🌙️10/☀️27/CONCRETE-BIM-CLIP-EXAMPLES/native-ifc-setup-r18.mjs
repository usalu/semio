/** 📥 Publish one immutable authenticated test-only IFC engine generation through portable Node APIs. */
import{readFile,writeFile,mkdir,rename,rm}from'node:fs/promises';
import{join,resolve,relative,isAbsolute}from'node:path';
import{createHash,randomUUID}from'node:crypto';
import{gunzipSync}from'node:zlib';
const version='0.0.78',integrity='sha512-LGKMNYFVNyz9vVERN0iz+zvPQJqT7MEGBxslzcuuhwDDOA116LmJLKkg8sZ5SEvQJSc5NRVMWgpRNkDmuoQoJg==';

/** 🧰 Verify pinned bytes and atomically publish a complete generation before any concurrent reader can import it. */
export async function setupNativeIfcEngine(directory){
 if(resolve(directory).length>256)throw Error('IFC engine path exceeds256 characters');
 await mkdir(directory,{recursive:true});const archivePath=join(directory,'package.tgz');if(resolve(archivePath).length>256)throw Error('IFC engine path exceeds256 characters');let archive;
 try{archive=await readFile(archivePath);}catch(error){if(error.code!=='ENOENT')throw error;const response=await fetch('https://registry.npmjs.org/web-ifc/-/web-ifc-'+version+'.tgz');if(!response.ok)throw Error('web-ifc archive HTTP '+response.status);archive=Buffer.from(await response.arrayBuffer());}
 if('sha512-'+createHash('sha512').update(archive).digest('base64')!==integrity)throw Error('web-ifc package integrity mismatch');
 try{await readFile(archivePath);}catch(error){if(error.code!=='ENOENT')throw error;const temporary=join(directory,'.archive-'+randomUUID()+'.tgz');if(resolve(temporary).length>256)throw Error('IFC engine path exceeds256 characters');await writeFile(temporary,archive,{flag:'wx'});try{await rename(temporary,archivePath);}catch(error){if(!['EEXIST','EPERM'].includes(error.code))throw error;const current=await readFile(archivePath);if(!current.equals(archive))throw Error('Concurrent native archive differs');}finally{await rm(temporary,{force:true});}}
 const tar=gunzipSync(archive),selected=new Set(['package/web-ifc-api-node.js','package/web-ifc-node.wasm','package/package.json']),files=new Map();
 for(let offset=0;offset+512<=tar.length;){const name=tar.subarray(offset,offset+100).toString().replace(/\0.*$/s,'');if(!name)break;const size=parseInt(tar.subarray(offset+124,offset+136).toString().replace(/\0.*$/s,'').trim(),8)||0;if(selected.has(name)){files.set(name.substring(8),tar.subarray(offset+512,offset+512+size));selected.delete(name);}offset+=512+Math.ceil(size/512)*512;}
 if(selected.size)throw Error('IFC engine archive lacks expected files');const generation=join(directory,'v078-'+createHash('sha256').update(integrity).digest('hex').slice(0,12));
 for(const name of files.keys())if(resolve(join(generation,name)).length>256)throw Error('IFC engine path exceeds256 characters');
 async function installed(){try{for(const[name,bytes]of files)if(!(await readFile(join(generation,name))).equals(bytes))throw Error('Authenticated native SDK generation was modified');return true;}catch(error){if(error.code==='ENOENT')return false;throw error;}}
 if(!await installed()){const stage=join(directory,'.stage-'+randomUUID());for(const name of files.keys())if(resolve(join(stage,name)).length>256)throw Error('IFC engine staging path exceeds256 characters');await mkdir(stage);try{await Promise.all([...files].map(([name,bytes])=>writeFile(join(stage,name),bytes,{flag:'wx'})));try{await rename(stage,generation);}catch(error){if(!['EEXIST','ENOTEMPTY','EPERM'].includes(error.code)||!await installed())throw error;}}finally{const target=resolve(stage),base=resolve(directory),rel=relative(base,target);if(!rel||rel.startsWith('..')||isAbsolute(rel))throw Error('Native SDK staging cleanup escaped its destination');await rm(target,{recursive:true,force:true});}}
 if(!await installed())throw Error('Native SDK generation is incomplete');return{modulePath:join(generation,'web-ifc-api-node.js'),wasmDirectory:generation,version,integrity};
}
