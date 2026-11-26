/** 🔎️ Checks and, if absent, uploads one retained Draw compiler input through the normal authenticated CAS client. */
import {createHash} from "node:crypto";
import {readFile} from "node:fs/promises";
import {httpRemoteExecutionContentV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🗄️content/🟦️.ts";
import {remoteCacheFetchV1,resolveAuthorisedRemoteCacheV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";

const hash="d98cae06dde281ccdd338118cbcdc5cee3ea5680b78cc131437dafb19dff4165",path=`/Users/ueli/.cache/semio/cas/v1/o/${hash.slice(0,2)}/${hash.slice(2)}`,bytes=await readFile(path);
if(bytes.length>1048576||createHash("sha256").update(bytes).digest("hex")!==hash)throw Error("Retained CAS bytes do not match the bounded original digest");
const root=process.cwd(),env={...process.env,SEMIO_WORKSPACE_ROOT:root},found=await resolveAuthorisedRemoteCacheV1(env,{root}),resolution=found.resolution;
if(!found.authorised||!resolution)throw Error("Authenticated fleet cache is unavailable");
const content=httpRemoteExecutionContentV1({endpoint:resolution.contract.endpoint,token:resolution.token,fetch:remoteCacheFetchV1({...env,...resolution.environment},{root}),timeoutSeconds:50,retries:0}),instance=resolution.contract.instance,digest={hash,size:bytes.length},signal=AbortSignal.timeout(55000),result:{hash:string;size:number;present?:boolean;upload?:string;readback?:string;error?:string}={hash,size:bytes.length};
try{result.present=await content.has(instance,hash,signal);if(!result.present&&Bun.argv.includes("--read-only"))result.upload="not attempted";else if(!result.present){await content.write(instance,digest,bytes,signal);result.upload="accepted";}else result.upload="not needed";const returned=await content.read(instance,digest,signal);result.readback=returned===undefined?"absent":returned.length===bytes.length&&createHash("sha256").update(returned).digest("hex")===hash?"verified":"mismatch";if(result.readback!=="verified")process.exitCode=1;}catch(error){result.error=error instanceof Error?error.message:String(error);process.exitCode=1;}
console.log(JSON.stringify(result));
