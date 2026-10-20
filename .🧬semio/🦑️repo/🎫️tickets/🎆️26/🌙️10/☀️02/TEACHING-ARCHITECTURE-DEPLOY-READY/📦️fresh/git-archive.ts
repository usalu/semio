import {createHash} from "node:crypto";
import {paxFile} from "./core.ts";
import {revalidateGitSupplement,type GitReusePlan,type GitObservationPort} from "./git.ts";
import type {CaptureControl} from "./contract.ts";
const hash=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex"),maximum=536870912;
/** 📦️ Projects only freshly authenticated genuine Git metadata into one bounded transport payload. */
export async function archiveGitSupplement(plan:GitReusePlan,port:GitObservationPort,control:CaptureControl){
 const check=()=>{control.signal.throwIfAborted();if(control.remainingMilliseconds()<=0)throw Error("Genuine Git archive deadline exceeded");},proof=await revalidateGitSupplement(plan,port,control),parts:Uint8Array[]=[];let bytes=0,index=0;const retain=(chunk:Uint8Array)=>{check();if(bytes+chunk.length>maximum)throw Error("Bounded genuine Git metadata archive required");parts.push(chunk);bytes+=chunk.length;};
 for(const row of proof.files){check();const value=await port.read("retained",row.path,134217728);if(value.identity!==row.identity||value.bytes.length!==row.bytes||hash(value.bytes)!==row.sha256)throw Error("Genuine Git metadata changed before archive");for(const chunk of paxFile(row.path,value.bytes,index++))retain(chunk);await control.progress("capture:git-archive",{files:index,bytes});}
 const manifest=await port.read("retained","git-metadata.json",1048576);if(hash(manifest.bytes)!==plan.retainedManifestSha256)throw Error("Genuine Git manifest changed before archive");for(const chunk of paxFile(".semio-private/git.json",manifest.bytes,index))retain(chunk);retain(new Uint8Array(1024));const archive=Buffer.concat(parts,bytes);for(const row of proof.hostMetadata){check();const actual=await port.read("host",row.path,134217728);if(actual.identity!==row.identity||hash(actual.bytes)!==row.sha256)throw Error("Host Git metadata changed across archive");}check();return{archive,sha256:hash(archive),bytes,metadataManifestSha256:plan.retainedManifestSha256,head:proof.head,created:proof.created,sourceBlobs:0,files:proof.files.length,metadataBytes:proof.bytes,guestGitVerified:false};
}
