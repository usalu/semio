/** 🔬️ Real-process oracle of finding (a): `bun <this> <root>` runs the real `repo tools exec --light bun -- --version` runtime input of every task under three profile settings and prints the digests the plan witness would record. `merged` is the base behaviour (stdout and stderr of the child), `keyed` the fixed one (stdout alone). The recorded `x` digest of the live plan receipt (a run without profile) is the third, independent witness. */
import {createHash} from "node:crypto";
import {join} from "node:path";
import {pathToFileURL} from "node:url";

const root=process.argv[2]!,load=(rel:string)=>import(pathToFileURL(join(root,rel)).href);
const build="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",process_="🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript";
const[{captureOwnedCommandV1},{createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1},{createNativeBuildCallerCapabilitiesV1},environment]=await Promise.all([load(`${process_}/🎛️execution/🟦️.ts`),load(`${process_}/🧭️routing/🟦️.ts`),load(`${build}/📥️caller/🟦️.ts`),load(`${build}/🔧️tools/🏗️bazel/🌿️environment/🟦️.ts`).catch(()=>undefined)]);
const hash=(value:string)=>createHash("sha256").update(JSON.stringify(value)).digest("hex"),command="repo tools exec --light bun -- --version",settings:[string,Record<string,string>][]=[["off",{}],["json",{SEMIO_REPO_PROFILE:"json"}],["json again",{SEMIO_REPO_PROFILE:"json"}],["table",{SEMIO_REPO_PROFILE:"1"}],["agent",{CLAUDE_CODE_SESSION_ID:"envleak",XPC_FLAGS:"0x1"}]];
const rows:Record<string,string>[]=[];
const capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","fixture:build"],"bazel",{});
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:envleak-oracle",maximumElapsedMilliseconds:300000},capabilities,Date.now()),async original=>{
 const control=createIssuedResourceControlV1(original,"bazel:envleak-oracle",0);
 for(const[name,extra]of settings){
  const env={...process.env,...extra} as NodeJS.ProcessEnv;
  const merged=hash(await captureOwnedCommandV1(process.execPath,["exec",command],root,{env,control}));
  const keyed=environment?hash(await environment.runBazelRuntimeCommandV1(root,command,env,control)):"-";
  rows.push({setting:name,merged:merged.slice(0,12),keyed:keyed.slice(0,12)});
 }
});
console.table(rows);
const mergedAll=new Set(rows.map(row=>row.merged)),keyedAll=new Set(rows.map(row=>row.keyed));
console.log(JSON.stringify({mergedDistinct:mergedAll.size,keyedDistinct:keyedAll.size,keyedDigest:[...keyedAll][0]}));
