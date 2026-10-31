import {test,expect} from "bun:test";
import {AsyncLocalStorage} from "node:async_hooks";
import {mkdirSync,existsSync,symlinkSync,copyFileSync} from "node:fs";
import {join} from "node:path";
import {pathToFileURL} from "node:url";
const load=(path:string)=>import(pathToFileURL(join(process.cwd(),path)).href);
const processRoot="🧰️framework/🔨️modules/🏃️process/",library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const {withBuildTestInvocationV1}=await load(processRoot+"🧪️testing/📥️invocation/🟦️.ts");
const {createIssuedResourceControlV1,withScriptInvocationSignalV1,scriptInvocationEnvironment}=await load(processRoot+"🧭️routing/📥️invocation/🏃️process/🟦️.ts");
const {withAdmittedCompilerStoreV1}=await load(processRoot+"🧪️testing/🦀️cargo/🔌️provider/🟦️.ts");
const {readBuildMode}=await load(processRoot+"🏗️build/🎚️mode/🟦️.ts");
const {captureOwnedCommandResultV1}=await load(processRoot+"🎛️owned-execution/📥️capture/🟦️.ts");
const {privateRepositoryWorkspaceV1,testCheckoutRootV1}=await load(library+"🧪️testing/🧫️private-repository/🟦️.ts");
const repository=await privateRepositoryWorkspaceV1(),checkout=testCheckoutRootV1(),outside=AsyncLocalStorage.snapshot();
if(!existsSync(join(repository.root,"🧰️framework")))symlinkSync(join(checkout,"🧰️framework"),join(repository.root,"🧰️framework"),process.platform==="win32"?"junction":"dir");
copyFileSync(join(checkout,"rust-toolchain.toml"),join(repository.root,"rust-toolchain.toml"));
test("ordinary compiler generation publishes pinned Cargo test tools",()=>withBuildTestInvocationV1(original=>outside(()=>repository.invoke(privateOriginal=>withScriptInvocationSignalV1(privateOriginal,original.control.signal,actual=>withAdmittedCompilerStoreV1(actual.capabilities.workspaceRoot,readBuildMode(actual.capabilities.buildMode),createIssuedResourceControlV1(actual,"law:ordinary-cargo-tools",0),scriptInvocationEnvironment(actual,process.env),async(store,control)=>{
 await store.storage.recheck();
 expect(store.storage.allocation.kind).toBe("compiler-workspace");
 expect(store.environment.CARGO_HOME.startsWith(store.storage.backing.root)).toBe(true);
 const commands=[];
 for(const name of ["nextest","llvm-cov"]){
  const result=await captureOwnedCommandResultV1("cargo",[name,"--version"],actual.capabilities.workspaceRoot,{control,env:store.environment,output:{maximumBytes:8192,maximumLines:64}});
  commands.push({name,status:result.status,signal:result.signal,output:(result.stdout+result.stderr).slice(0,1024)});
 }
 console.log("[DEBUG] "+JSON.stringify({kind:"ordinary-compiler-generation",root:store.root,capture:store.captureDirectory,cargoHome:store.environment.CARGO_HOME,commands}));
 await store.storage.recheck();
 for(const command of commands){expect(command.status).toBe(0);expect(command.signal).toBeNull();}
})),{owner:"cargo-generation:probe",maximumElapsedMilliseconds:Math.floor(original.control.remainingMilliseconds()??290000)}))),270000);
