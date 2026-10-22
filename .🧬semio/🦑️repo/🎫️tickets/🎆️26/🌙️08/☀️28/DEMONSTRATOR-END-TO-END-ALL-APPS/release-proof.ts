import {BundleScript,ScriptRouter,receiveScriptProcessInvocation,withIssuedResourceControlV1} from "@semio-tech/process/routing";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "@semio-tech/process/leases";
import {withViteArtifactV1} from "@semio-tech/repo-lib/dev";
import {parseStoreReceiptV1,storeTreeDigestV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️store-receipt/🟨️.mjs";
import receiptSchema from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️store-receipt/🧬️schema/🔣️.json" with {type:"json"};
import {demonstratorRuntimeAssetSources} from "../../../../../../../♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📦️assets/🟦️.ts";
import {createHash} from "node:crypto";
import {constants} from "node:fs";
import {open,lstat,mkdir,writeFile} from "node:fs/promises";
import {dirname,join,relative,sep} from "node:path";
import {fileURLToPath} from "node:url";

/** 📖️ Reads only a bounded unchanged ordinary receipt descriptor. */
async function receiptBytes(path:string,control:ResourceLeaseControl):Promise<Buffer>{
 const file=await open(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0));let failed=false;
 try{const before=await file.stat({bigint:true}),keys=["dev","ino","mode","nlink","size","mtimeNs","ctimeNs"] as const,same=(row:typeof before)=>keys.every(key=>row[key]===before[key]);if(!before.isFile()||before.nlink!==1n||before.size<0n||before.size>BigInt(receiptSchema.definitions.Policy.const.maximumReceiptBytes))throw Error("Bounded ordinary build receipt required");const bytes=Buffer.alloc(Number(before.size));let at=0;while(at<bytes.length){checkResourceLeaseControl(control);const read=await file.read(bytes,at,Math.min(65536,bytes.length-at),at);if(!read.bytesRead)throw Error("Complete build receipt required");at+=read.bytesRead;}if((await file.read(Buffer.alloc(1),0,1,at)).bytesRead||!same(await file.stat({bigint:true}))||!same(await lstat(path,{bigint:true})))throw Error("Original build receipt changed");return bytes;}catch(error){failed=true;throw error;}finally{try{await file.close();}catch(error){if(!failed)throw error;}}
}

/** 🎟️ Crosschecks release output and its task receipt under the existing artifact reader. */
class ReleaseProof extends BundleScript{
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Release proof accepts no arguments");
  const proof=await withIssuedResourceControlV1(this.invocation,"release-proof",120000,phase=>withViteArtifactV1(this.repoRoot,"release","demonstrator:site",this.invocation,phase,async artifact=>{
   await artifact.recheck();if(artifact.owner!=="demonstrator:site"||!artifact.files.length||!artifact.files.includes("index.html")||!artifact.files.some(name=>name.endsWith(".js"))||!artifact.files.some(name=>name.endsWith(".wasm")))throw Error("Complete release browser publication required");
   const destinations=demonstratorRuntimeAssetSources(this.repoRoot,"release").map(row=>row.destination);for(const destination of destinations)if(!artifact.files.some(name=>name.startsWith(destination+"/")))throw Error("Missing authored release runtime asset: "+destination);
   const receiptPath=join(this.repoRoot,"♻️mit-bestand/🧺️demonstrator/.🧾️store/build.json"),before=await receiptBytes(receiptPath,artifact.control),receipt=parseStoreReceiptV1(before.toString("utf8")),output=relative(this.repoRoot,artifact.path).split(sep).join("/");if(receipt.project!=="@semio-tech/mit-bestand-demonstrator"||receipt.target!=="build")throw Error("Exact build task receipt required");const entry=receipt.entries.find((row:{path:string})=>row.path===output);if(!entry||entry.kind!=="directory"||entry.files!==artifact.files.length+1)throw Error("Build receipt must name the live release output");checkResourceLeaseControl(artifact.control);const actual=await storeTreeDigestV1(artifact.path);if(actual.bytes!==entry.bytes||actual.files!==entry.files||actual.sha256!==entry.sha256)throw Error("Build receipt content differs from live release output");if(!before.equals(await receiptBytes(receiptPath,artifact.control)))throw Error("Build receipt changed during proof");await artifact.recheck();
   return {version:1,mode:"release",owner:artifact.owner,project:receipt.project,target:receipt.target,receiptSha256:createHash("sha256").update(before).digest("hex"),output:entry,files:artifact.files.length,runtimeDestinations:destinations,verifiedAt:new Date().toISOString()};
  }));
  const output=join(fileURLToPath(new URL("./🗑️generated/",import.meta.url)),"release-proof.json");await mkdir(dirname(output),{recursive:true});await writeFile(output,JSON.stringify(proof,null,2)+"\n");process.stdout.write("Release publication proof: "+output+"\n");
 }
}
const router=new ScriptRouter(fileURLToPath(new URL(".",import.meta.url))).register("verify",ReleaseProof);
await receiveScriptProcessInvocation(process.env,original=>router.run(process.argv.slice(2),original));
