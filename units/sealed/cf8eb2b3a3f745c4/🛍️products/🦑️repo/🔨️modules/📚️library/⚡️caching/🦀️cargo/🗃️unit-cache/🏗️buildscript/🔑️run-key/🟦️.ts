import {randomBytes} from "node:crypto";
import {readFileSync,renameSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {buildKeyEnvironmentV1,sha256V1,type unitKeyV1} from "../../🔏️key/🟦️.ts";

type Environment=Readonly<Record<string,string|undefined>>;

/** 🪪️ The file that names the unit a build script executable was compiled from, beside the executable. */
export const buildScriptIdentityFileV1=(real:string):string=>real+".semio-run";

/** 🪪️ The identity a build script executable enters the key of its runs by: `unit:<digest>` of the portable key of the unit that compiled it (written beside the executable by the wrapper and trusted only while it names the executable's current bytes), else `bytes:<sha256>` of the executable. A linked executable may differ in its bytes between machines (embedded paths, linker identifiers) although it is the same function of the same inputs; keyed by those inputs, every machine computes the same run key. */
export function buildScriptIdentityV1(real:string,sha256:string):string{
 try{
  const row=JSON.parse(readFileSync(buildScriptIdentityFileV1(real),"utf8")) as {version?:number;sha256?:string;identity?:string};
  if(row.version===1&&row.sha256===sha256&&typeof row.identity==="string"&&/^[0-9a-f]{64}$/u.test(row.identity))return"unit:"+row.identity;
 }catch{}
 return"bytes:"+sha256;
}

/** 🪪️ Writes the identity file of a build script executable (best effort, atomic): the unit key it was compiled under and the real paths its bytes depend on (`sensitive`), so two machines share its runs only when those paths are equal. */
export function markBuildScriptIdentityV1(real:string,portableKey:string,sensitive:readonly string[],sha256:string|null,key?:string):void{
 if(sha256===null)return;
 try{
  const file=buildScriptIdentityFileV1(real),temporary=file+"."+randomBytes(3).toString("hex");
  writeFileSync(temporary,JSON.stringify({version:1,sha256,identity:sha256V1(JSON.stringify(["build-script-identity",portableKey,sensitive])),...key===undefined?{}:{key}}));renameSync(temporary,file);
 }catch{}
}

/** 🔑️ The inputs of the key of one build script run, in the shape of a unit key so it is tokenised and hashed exactly like one (`portableKeyV1`, `remoteKeyV1`): the compiler release, the executable's identity (marked `run:` so it can never equal the source digest of a compiled unit), the arguments Cargo passed and the run's environment with the lane, the compiler flags and the tool paths neutralised. */
export function buildRunKeyInputV1(input:Readonly<{toolchain:string;identity:string;args:readonly string[];environment:Environment;laneRoot:string|undefined}>):Parameters<typeof unitKeyV1>[0]{
 return{toolchain:input.toolchain,normalized:["--semio-build-script-run",...input.args],sourceSha256:"run:"+input.identity,externs:[],environment:buildKeyEnvironmentV1(input.environment,input.laneRoot)};
}

/** 🔗️ The local key of the entry that compiled a build script executable, as the identity file beside it records it, trusted only while it names the executable's current bytes; `undefined` otherwise. A run declares it as the unit it consumed. */
export function buildScriptKeyV1(real:string,sha256:string):string|undefined{
 try{const row=JSON.parse(readFileSync(buildScriptIdentityFileV1(real),"utf8")) as {sha256?:string;key?:string};return row.sha256===sha256&&typeof row.key==="string"&&/^[0-9a-f]{64}$/u.test(row.key)?row.key:undefined;}catch{return undefined;}
}

/** 🔗️ The file beside an OUT_DIR (in the build directory of the package) that names the build script run the directory belongs to. */
export const buildRunKeyFileV1=(outDir:string):string=>join(dirname(outDir),"semio-run-key.json");

/** 🔗️ Records which run (its local key) produced or restored an OUT_DIR (best effort, atomic), so the unit that reads the directory can declare it as a unit it consumed. */
export function markBuildRunKeyV1(outDir:string,key:string):void{
 try{const file=buildRunKeyFileV1(outDir),temporary=file+"."+randomBytes(3).toString("hex");writeFileSync(temporary,JSON.stringify({version:1,key}));renameSync(temporary,file);}catch{}
}

/** 🔗️ The local key of the run that produced an OUT_DIR, `undefined` when the directory is not one of a run. */
export function buildRunKeyV1(outDir:string|undefined):string|undefined{
 if(outDir===undefined||outDir==="")return undefined;
 try{const row=JSON.parse(readFileSync(buildRunKeyFileV1(outDir),"utf8")) as {version?:number;key?:string};return row.version===1&&typeof row.key==="string"&&/^[0-9a-f]{64}$/u.test(row.key)?row.key:undefined;}catch{return undefined;}
}
