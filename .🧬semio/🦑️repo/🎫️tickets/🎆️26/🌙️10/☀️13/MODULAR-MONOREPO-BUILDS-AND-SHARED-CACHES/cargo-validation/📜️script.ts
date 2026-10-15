import {resolve,join} from "node:path";
import {readFileSync} from "node:fs";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,scriptInvocationEnvironment} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {runRepositoryCargoCommandV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import {cargoTargetDirectory} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗂️directories/🟦️.ts";
const root=resolve(import.meta.dir,"../../../../../../../.."),[command,mode]=process.argv.slice(2),commands=["locks","locks-remaining","locks-last","error","crypto","worker-public","core-remaining","types","history-contract","recensus-contract"];
if(process.argv.length!==4||!commands.includes(command??"")||!["prototype","release"].includes(mode??""))throw Error("Exact Cargo qualification prerequisite required");
const routes:Readonly<Record<string,string>>={locks:"job-downstream-locks","locks-remaining":"job-downstream-locks","locks-last":"job-downstream-locks",error:"crate-frontier-replication-error-tests",crypto:"crate-frontier-replication-crypto-tests","worker-public":"job-worker-contract","core-remaining":"job-core-contracts",types:"job-owners-types","history-contract":"cargo-retirement","recensus-contract":"cargo-retirement"},selection=["nx","run","repo-private-cache:"+routes[command!]!,"--configuration="+mode],policy={version:1 as const,owner:"repository:cargo-validation-bootstrap",maximumElapsedMilliseconds:600000};
await withScriptProcessEnvelope(createScriptProcessEnvelope(policy,{workspaceRoot:root,buildMode:mode,arguments:selection,selection,launches:[selection],native:{artifactDirectory:cargoTargetDirectory(root),transport:{maximumBytes:134217728,maximumLines:65536},child:{maximumElapsedMilliseconds:240000},network:{offline:false}}},Date.now()),async original=>{
 const environment={...process.env,SEMIO_BUILD_MODE:mode};
 if(command==="history-contract"||command==="recensus-contract"){process.stdout.write(await captureOwnedCommandV1(process.execPath,["test",join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/📜️history/🧪️tests/"+(command==="recensus-contract"?"📏️recensus/🟦️.ts":"🟦️.ts"))],root,{control:scriptCompilerAdmissionControlV1(original,"cargo:historical-custody-contract",120000),env:scriptInvocationEnvironment(original,environment)}));return;}
 if(command==="types"){process.stdout.write(await captureOwnedCommandV1(process.execPath,[Bun.resolveSync("typescript/bin/tsc",root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--types","bun",import.meta.path,join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/🟦️.ts"),join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/📜️history/🧪️tests/🟦️.ts"),join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/📜️history/🧪️tests/📏️recensus/🟦️.ts"),join(import.meta.dir,"../rust-headroom/📜️script.ts"),join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/📜️history/🧪️tests/📜️script.ts")],root,{control:scriptCompilerAdmissionControlV1(original,"cargo:validation-types",120000),env:scriptInvocationEnvironment(original,environment)}));console.log("[DEBUG] Actual Cargo validation prerequisite strict source qualification complete");return;}
 if(command==="locks"||command==="locks-remaining"||command==="locks-last"){
  const manifests=JSON.parse(readFileSync(join(import.meta.dir,"../job-owners/📋️workspaces.json"),"utf8")) as string[];
  if(manifests.length!==20||new Set(manifests).size!==20||manifests[0]!=="🧰️framework/Cargo.toml"||manifests.some(path=>join(root,path).length>256||path.startsWith("/")||path.split("/").includes("..")||!path.endsWith("Cargo.toml")))throw Error("Closed twenty actual workspace selections required");
  if(command==="locks-remaining"&&manifests[14]!=="✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/Cargo.toml")throw Error("Exact terminal continuation boundary required");
  if(command==="locks-last"&&manifests[19]!=="✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/Cargo.toml")throw Error("Exact final workspace boundary required");
  for(const manifest of command==="locks-last"?manifests.slice(19):command==="locks-remaining"?manifests.slice(14):manifests){
   original.control.signal.throwIfAborted();
   console.log("[DEBUG] Actual same-Original lock refresh starting "+manifest);
   await runRepositoryCargoCommandV1(original,root,["generate-lockfile","--manifest-path",join(root,manifest)],root,environment,240000);
   await runRepositoryCargoCommandV1(original,root,["fetch","--locked","--manifest-path",join(root,manifest)],root,environment,240000);
   console.log("[DEBUG] Actual same-Original lock refresh and locked fetch complete "+manifest);
  }
  return;
 }
 const packages={error:"semio-framework-replication-error",crypto:"semio-framework-replication-crypto","worker-public":"semio-framework-job-worker","core-remaining":"semio-framework-job"},targets=command==="worker-public"?["--test","original-payload","--test","original-normal-failure","--test","original-staged-metadata","--test","original-worker-turn","--test","original-native-preparation","--features","testkit"]:command==="core-remaining"?["--test","original-turn-failure","--test","original-close-context","--test","original-weak-return","--test","original-fixture-caller","--features","testkit"]:["--lib"];
 await runRepositoryCargoCommandV1(original,root,["test","--manifest-path",join(root,"🧰️framework/Cargo.toml"),"--package",packages[command as keyof typeof packages],...targets,"--locked","--","--nocapture"],root,environment,240000);
 console.log("[DEBUG] Actual Original native qualification complete "+JSON.stringify({command,mode}));
});
