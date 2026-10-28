import{createHash}from"node:crypto";
import{writeSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{repositoryCommandKernelV1}from"../../⌨️command/🧭️selection/🟦️.ts";
import{buildSourceClosureStreamLoaderV1}from"../../📥️source/🔏️closure/🟦️.ts";
import type{CapturedBuildCommandProofV1}from"../../📥️source/🔏️closure/🚪️command/🟦️.ts";
import{sealedPolicyV1}from"../🟦️.ts";

interface KernelV1{runCapturedBuildSourceV1(args:readonly string[],proof:CapturedBuildCommandProofV1):Promise<void>}
/** 🧭️ The five verb kernels, each a literal import so that the one snapshot carries all of them; the selection module decides which one a command line runs (a law keeps this table equal to its schema). */
export const sealedKernelsV1:Readonly<Record<"preflight"|"dependencies"|"cache"|"cache-ledger"|"runtime",()=>Promise<KernelV1>>>=Object.freeze({
 preflight:()=>import("../../⌨️command/🩺️preflight/🟦️.ts"),
 dependencies:()=>import("../../⌨️command/📦️dependencies/🟦️.ts"),
 cache:()=>import("../../⌨️command/💾️cache/🟦️.ts"),
 "cache-ledger":()=>import("../../⌨️command/💾️cache/📋️ledger/🟦️.ts"),
 runtime:()=>import("../../⌨️command/⚙️runtime/🟦️.ts")
});
/** 🚪️ The first argument that starts the front door instead of a verb kernel. */
export const sealedDoorRoleV1="--semio-role=door";
const checkV1=(proof:CapturedBuildCommandProofV1)=>sealedPolicyV1.check.argument+proof.sha256;

/** 🧪️ Answers the promotion check: imports the front door and every kernel (a load error anywhere fails the promotion) and reports whether the loader that started this snapshot is the loader it was built for; when it is not, it hands over the loader it needs. */
async function answerCheckV1(proof:CapturedBuildCommandProofV1):Promise<void>{
 await import("../../🚪️door/🏃️main/🟦️.ts");
 for(const load of Object.values(sealedKernelsV1))await load();
 const loaderSha256=createHash("sha256").update(buildSourceClosureStreamLoaderV1).digest("hex"),matches=loaderSha256===proof.loaderSha256;
 process.stdout.write(JSON.stringify({check:"loaded",matches,loaderSha256,...(matches?{}:{loader:buildSourceClosureStreamLoaderV1})})+"\n");
}
/** 🚪️ Runs the front door from the sealed snapshot: the entry script of the next restart is the one the user started, so a restart starts the next snapshot from the store; the supervisor of the door needs no other knowledge. */
async function runDoorV1(argv:readonly string[],proof:CapturedBuildCommandProofV1,ready:number):Promise<void>{
 try{writeSync(ready,"ready\n");}catch{}
 process.argv=[process.execPath,fileURLToPath(new URL("../../🚪️door/📜️script.ts",import.meta.url)),...argv];process.execArgv=[];
 const{runBuildDoorMainV1}=await import("../../🚪️door/🏃️main/🟦️.ts");
 await runBuildDoorMainV1(proof.root,argv,process.env);
}
/** 🏛️ The one entry of the sealed implementation: the front door, the promotion check, or the verb kernel the command line selects; the snapshot, the digest and the loader are the same for all of them. */
export async function runCapturedBuildSourceV1(args:readonly string[],proof:CapturedBuildCommandProofV1):Promise<void>{
 if(args.length===1&&args[0]===checkV1(proof))return answerCheckV1(proof);
 if(args[0]===sealedDoorRoleV1)return runDoorV1(args.slice(1),proof,sealedPolicyV1.readyDescriptor);
 return(await sealedKernelsV1[repositoryCommandKernelV1(args,process.env).kind]()).runCapturedBuildSourceV1(args,proof);
}
