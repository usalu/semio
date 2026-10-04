import {mkdirSync,writeFileSync,rmSync} from "node:fs";
import {open} from "node:fs/promises";
import {join} from "node:path";
import {stageArtifacts} from "C:/git/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts";
const root=join(import.meta.dir,"../🗑️generated/publisher-eof-cancel-data");
mkdirSync(root,{recursive:true});
const source=join(root,"source");writeFileSync(source,"");
const target=join(root,"target"),leaseDirectory=join(root,"leases"),files=new Map([["empty",source]]);
await stageArtifacts(target,"audit-eof-cancel",files,{leaseDirectory});
const handle=await open(source,"r"),prototype=Object.getPrototypeOf(handle),original=prototype.read;
await handle.close();
const controller=new AbortController();let fired=false,outcome="";
prototype.read=async function(...args:unknown[]){const result=await original.apply(this,args);if(result.bytesRead===0){fired=true;controller.abort(new Error("audit EOF abort"));}return result;};
try{await stageArtifacts(target,"audit-eof-cancel",files,{leaseDirectory,signal:controller.signal});outcome="fulfilled";}catch(error){outcome="rejected: "+String(error);}finally{prototype.read=original;}
console.log("[DEBUG] "+JSON.stringify({fired,aborted:controller.signal.aborted,outcome}));
rmSync(root,{recursive:true,force:true});

