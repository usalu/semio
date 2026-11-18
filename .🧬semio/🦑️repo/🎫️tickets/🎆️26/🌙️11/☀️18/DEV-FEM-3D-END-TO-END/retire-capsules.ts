import{lstatSync,readFileSync,readdirSync,renameSync,statfsSync}from"node:fs";
import{join}from"node:path";
import{concurrentUnreadyCapsulePatternV1,concurrentUnreadyCapsuleRetirableV1}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🚦️concurrent/🟦️.ts";
import{readConcurrentPointersV1,tryConcurrentPinExclusiveV1}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🚦️concurrent/💾️storage/🟦️.ts";
import{removeJavascriptDependencyTreeV1}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts";
import{repositoryLeaseDirectoryV1}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";

const workspace="C:\\semio",root=join(workspace,".cache","b","d","m"),container=join(root,"j"),graceMs=6*3600*1000,free=()=>{const row=statfsSync("C:\\",{bigint:true});return Number(row.bavail*row.bsize)/2**30;};
const current=new Set(readConcurrentPointersV1(root).map(pointer=>pointer.generation.slice(0,20))),directory=repositoryLeaseDirectoryV1(workspace),signal=new AbortController().signal;
console.log("C: free GiB before",free().toFixed(2));
let removed=0,young=0,skipped=0;
for(const name of readdirSync(container).filter(name=>concurrentUnreadyCapsulePatternV1.test(name))){
 const path=join(container,name),record=(file:string)=>JSON.parse(readFileSync(join(path,file),"utf8"));
 let qualifies=false;
 try{qualifies=concurrentUnreadyCapsuleRetirableV1({container,name,roster:readdirSync(path),intent:record("i"),receipt:record("r"),current,payloadReady:lstatSync(join(path,"p","bindings.json"),{throwIfNoEntry:false})!==undefined});}catch{}
 if(!qualifies){skipped++;continue;}
 if(Date.now()-lstatSync(path).mtimeMs<graceMs){young++;continue;}
 const pin=await tryConcurrentPinExclusiveV1(directory,workspace,"0".repeat(12)+name.slice(2),undefined,signal);
 if(!pin){skipped++;continue;}
 try{const tombstone=join(container,".x"+name);renameSync(path,tombstone);await removeJavascriptDependencyTreeV1(tombstone,async()=>{});removed++;console.log("removed",name);}finally{pin.release();}
}
console.log(JSON.stringify({removed,young,skipped}));
console.log("C: free GiB after",free().toFixed(2));
