// [mem2] Entry of the graph-free refresh harness snapshot (copied to <mirror>/fiharness/🟦️.ts; not a library file): the graph farm worker role goes through the shared verb kernel entry exactly as a production farm worker does; any other command line is the harness parent, which adopts its proof like a kernel and runs the production parent loop.
import{adoptCapturedBuildCommandV1}from"../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts";
import{runCapturedRepositoryKernelV1}from"../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/📥️entry/🟦️.ts";
import"../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts";
export async function runCapturedBuildSourceV1(args:readonly string[],proof:Parameters<typeof adoptCapturedBuildCommandV1>[0]):Promise<void>{
 if(args[0]==="--semio-role=graph-farm-worker")return runCapturedRepositoryKernelV1(args,proof,import.meta.semioCapturedImplementationIdentity,import.meta.semioCapturedLoaderIdentity,async()=>{throw Error("[DEBUG] harness worker reached a verb body");});
 adoptCapturedBuildCommandV1(proof);
 const{runHarnessV1}=await import("./run.ts");await runHarnessV1();
}
