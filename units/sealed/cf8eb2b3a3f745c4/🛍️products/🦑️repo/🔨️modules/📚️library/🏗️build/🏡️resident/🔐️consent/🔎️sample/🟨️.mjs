/** 🔎️ The sample child of the service gate: `process <this file> <checkout>` reads the checkout the way a slice load does and sends `{place,error}` for the first place the system refuses, `{}` when nothing is refused; it runs as a forked, detached process of the service, the shape of the process that does the work, so a gate that passes proves the access of that process too. */
import {readdir} from "node:fs/promises";
import {sampleAccessV1} from "../🟨️.mjs";

const root=process.argv[2];
if(typeof root==="string"&&root.length>0){
 let found;
 try{found=await sampleAccessV1(root,await readdir(root));}
 catch(error){found=error?.code==="EPERM"||error?.code==="EACCES"?{place:root,error:error.code}:undefined;}
 process.send?.(found??{},()=>process.exit(0));
}
