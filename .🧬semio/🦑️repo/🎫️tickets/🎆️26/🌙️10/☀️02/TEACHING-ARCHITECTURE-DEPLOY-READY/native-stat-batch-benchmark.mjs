import {lstat as callbackStat} from "node:fs";
import {lstat,writeFile,mkdtemp,mkdir,rm} from "node:fs/promises";
import {lstatSync} from "node:fs";
import {join,resolve} from "node:path";
import _ from "lodash";
const count=8192,batch=8,workers=16,area=await mkdtemp(join(resolve(process.argv[2]),"stat-bench-")),files=[];
try{
 for(let directory=0;directory<32;directory++){const folder=join(area,String(directory));await mkdir(folder);for(let i=0;i<count/32;i++)files.push(join(folder,String(i)));}
 let cursor=0;await Promise.all(Array.from({length:workers},async()=>{for(let i=cursor++;i<files.length;i=cursor++)await writeFile(files[i],"test-data");}));
 const baseline=paths=>Promise.all(paths.map(path=>lstat(path,{bigint:true})));
 const callbacks=paths=>new Promise((yes,no)=>{const results=Array(paths.length);let pending=paths.length,failed=false,failure;for(const[index,path]of paths.entries())callbackStat(path,{bigint:true},(error,value)=>{if(error&&!failed){failed=true;failure=error;}results[index]=value;if(--pending===0){if(failed)no(failure);else yes(results);}});});
 const samples=[];for(let turn=0;turn<4;turn++)for(const[name,stat]of turn%2?[["callbacks",callbacks],["promises",baseline]]:[["promises",baseline],["callbacks",callbacks]]){let next=0,bytes=0,entries=0;const start=performance.now();await Promise.all(Array.from({length:workers},async()=>{for(let i=next;i<files.length;i=next){next+=batch;for(const row of await stat(files.slice(i,i+batch))){bytes+=Number(row.blocks)*512;entries++;}}}));if(entries!==count)throw Error("Entry loss");samples.push({name,entries,bytes,wallMilliseconds:performance.now()-start});}
 const oracle=files.reduce((sum,path)=>sum+Number(lstatSync(path,{bigint:true}).blocks)*512,0);if(samples.some(row=>row.bytes!==oracle))throw Error("Independent physical block oracle mismatch");const summaries=["promises","callbacks"].map(name=>({name,medianMilliseconds:_.sortBy(samples.filter(row=>row.name===name),"wallMilliseconds")[2].wallMilliseconds}));console.log(JSON.stringify({version:1,scope:"bounded scratch stat-dispatch microbenchmark, not Native walk acceptance",runtime:process.versions,count,batch,workers,samples,summaries,oracle}));
}finally{await rm(area,{recursive:true,force:true});}
