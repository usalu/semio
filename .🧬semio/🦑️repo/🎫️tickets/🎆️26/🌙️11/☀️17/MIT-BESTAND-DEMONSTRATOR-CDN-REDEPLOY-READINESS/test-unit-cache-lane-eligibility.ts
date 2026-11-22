import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { join } from 'node:path';
import { lookupUnitV1, writeEntryV1 } from '../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts';
const dir=mkdtempSync(join(import.meta.dir,'🗑️generated','unit-lane-')), root=join(dir,'unit'), a=join(dir,'lane-a'), b=join(dir,'lane-b'),key='a'.repeat(64),sha=createHash('sha256').update('same-resource').digest('hex');
const old='semio-compiler-resources/100-11-0/observation.json', newer='semio-compiler-resources/200-22-0/observation.json';
const put=(p:string,data:string)=>{mkdirSync(join(p,'..'),{recursive:true});writeFileSync(p,data)};
try{
 process.env.SEMIO_CAS_ROOT=join(dir,'cas');
 put(join(a,old),'same-resource');put(join(b,newer),'same-resource');
 writeEntryV1(root,{version:1,key,createdMs:Date.now(),crate:'fixture',dependencies:[{path:'@@semio-lane@@/'+old,sha256:sha}],environment:[{name:'SEMIO_COMPILER_RESOURCE_ROOT',value:'@@semio-lane@@/semio-compiler-resources'}],outputs:[],stdout:null,stderr:null} as never);
 const lookup=(lane:string)=>lookupUnitV1(root,key,{cwd:dir,laneRoot:lane,environment:{SEMIO_COMPILER_RESOURCE_ROOT:join(lane,'semio-compiler-resources')},stampDirectory:undefined});
 if(!lookup(a)||lookup(b))throw Error('expected first lane hit and second lane miss despite identical key');
 put(join(b,old),'same-resource');if(!lookup(b))throw Error('same old relative path and bytes should be eligible');
 put(join(b,old),'changed-resource');if(lookup(b))throw Error('changed resource must not be eligible');
 console.log('real lookup: same key hits first lane, misses second with equivalent new-path resource; old path+bytes hits, changed bytes refuses');
}finally{rmSync(dir,{recursive:true,force:true});}
