# Original Browser Roster Native Storage Refusal — 2026-10-14

SOURCE36192/session2749 naturally Nx0: strict TypeScript,7 laws/0 failures/118 assertions. Paired NATIVE36798/session22075 naturally Nx1 after the same source suite passed7/0/118. Cargo Job lib rmeta and replication object writes failed ENOSPC; nextest capture also failed ENOSPC. Selected renderer native laws never ran and actual collector/slot bodies remain uncompiled. No native/DOM/GPU or universal issuer qualification. No live owned heavy.

The exact original arguments=selection=launches[0], finite authority, caps, canonical Cargo directory and prototype placement were preserved. No peer stop/deletion or shared cache reset.

Complete fatal and capture blocks:

```text
error: failed to write /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/canonical-architecture/prototype/build/semio-framework-job/8ef8096284b059da/out/rustc7LypGG/lib.rmeta: No space left on device (os error 28)

warning: `semio-framework-job` (lib) generated 15 warnings
error: could not compile `semio-framework-job` (lib) due to 1 previous error; 15 warnings emitted
warning: build failed, waiting for other jobs to finish...
error: could not write output to /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/canonical-architecture/prototype/build/semio-framework-replication/ca3831dfba1f1dd2/out/protocol-ca3831dfba1f1dd2.b6u95at560m2jqg50oamy597w.15pifwa.rcgu.o: No space left on device

warning: `semio-framework-replication` (lib) generated 1 warning
error: could not compile `semio-framework-replication` (lib) due to 1 previous error; 1 warning emitted
error: command `/Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin/cargo test --no-run --message-format json-render-diagnostics --package semio-framework-os-renderer-wgpu --lib --profile prototype --offline` exited with code 101
168 |    const plan=cargoTestPlanV1(request,policy,metadata),identity=cargoTestBuildLeaseIdentityV1(request,policy),stop=startNativeProgress("cargo:compiler-lease");
169 |    try{lease=await acquireCargoBuildLeaseV1({directory:policy.leaseDirectory,buildDirectory:identity.buildDirectory,args:["--profile",identity.profile],signal:controller.signal});}finally{stop();}
170 |    for(const step of plan){
171 |     const stop=startNativeProgress(`cargo:${step.phase}`), decoder=new StringDecoder("utf8");let output="";
172 |     try {const remaining=request.remainingMilliseconds?.();if(remaining!==undefined&&remaining!==null&&(!Number.isFinite(remaining)||remaining<1))throw Error("Original Cargo deadline exhausted");const budgetMs=remaining===undefined||remaining===null?step.budgetMs:Math.floor(step.budgetMs>0?Math.min(step.budgetMs,remaining):remaining);await runBudgetedTestCommand(port.command,[...port.args,...step.args],{cwd:request.cwd,env,signal:controller.signal,budgetMs,throwOnFailure:true,...(step.capture?{captureStdout:{limitBytes:536870912,onChunk:(bytes:Uint8Array)=>{output+=decoder.write(Buffer.from(bytes));}}}:{})});if(step.capture)writeFileSync(metadata,output+decoder.end());}
173 |     catch(error){if(step.capture)writeFileSync(join(directory,`${step.phase}-failure.stdout.txt`),output+decoder.end());throw error;}
                                       ^
ENOSPC: no space left on device, open '/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/semio-nextest-da4e8B/build-failure.stdout.txt'
    path: "/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/semio-nextest-da4e8B/build-failure.stdout.txt",
 syscall: "open",
   errno: -28,
    code: "ENOSPC"

      at runCargoTestsV1 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts:173:34)

Bun v1.3.14 (macOS arm64)
```
