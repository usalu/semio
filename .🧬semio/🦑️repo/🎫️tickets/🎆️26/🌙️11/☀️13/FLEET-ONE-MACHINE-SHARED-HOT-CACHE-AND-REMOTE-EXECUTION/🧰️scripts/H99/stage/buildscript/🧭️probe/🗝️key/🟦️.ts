import {readFileSync} from "node:fs";
import {join,relative} from "node:path";
import offloadPolicy from "../../☁️offload/🧬️schema/🔣️.json" with {type:"json"};
import {buildScriptVerdictV1,probeKeyEnvironmentV1,probeKeyRowsV1,type ProbeVerdictV1} from "../🟦️.ts";

type Environment=Readonly<Record<string,string|undefined>>;
const environmentPolicy=offloadPolicy.policy.environment;

/** 📜️ The source file of a package's build script, from its `Cargo.toml` (`build = "path"`, `build = false`), `build.rs` by default; `undefined` when the package has none. */
export function buildScriptSourcePathV1(manifest:string):string|undefined{
 const row=/^\s*build\s*=\s*(?:"([^"]+)"|(false))/mu.exec(manifest);
 if(row?.[2]!==undefined)return undefined;
 return row?.[1]??"build.rs";
}

/** 📦️ The names of the build dependencies a manifest text declares (dashes as underscores), from its `[build-dependencies]` tables. */
export function buildDependencyNamesV1(manifest:string):readonly string[]{
 const tables=[...manifest.matchAll(/^\s*\[(?:[\w.-]+\.)?build-dependencies\][^[]*/gmu)].map(match=>match[0]).join("\n");
 return[...tables.matchAll(/^\s*"?([A-Za-z0-9_-]+)"?\s*[=.]/gmu)].map(match=>match[1]!.replaceAll("-","_"));
}

/** 🏷️ Whether the command policy itself carries an environment name of a build script run. */
export const buildScriptDeclaredNameV1=(name:string):boolean=>name==="OUT_DIR"||name==="CARGO_MANIFEST_DIR"||(environmentPolicy.names as readonly string[]).includes(name)||environmentPolicy.prefixes.some(prefix=>name.startsWith(prefix));

/** 📜️ The verdict on the build script of the package at `CARGO_MANIFEST_DIR`, read with the files it includes; `undefined` when the package cannot be read. */
export function buildScriptVerdictOfV1(environment:Environment):ProbeVerdictV1|undefined{
 const manifestDir=environment.CARGO_MANIFEST_DIR;if(manifestDir===undefined||manifestDir==="")return undefined;
 try{
  const manifest=readFileSync(join(manifestDir,"Cargo.toml"),"utf8"),path=buildScriptSourcePathV1(manifest);
  if(path===undefined||relative(manifestDir,join(manifestDir,path)).startsWith(".."))return undefined;
  const include=(name:string):string|undefined=>{try{return readFileSync(join(manifestDir,name),"utf8");}catch{return undefined;}};
  return buildScriptVerdictV1({source:readFileSync(join(manifestDir,path),"utf8"),dependencies:buildDependencyNamesV1(manifest),environment,declared:buildScriptDeclaredNameV1,include});
 }catch{return undefined;}
}

/** 🔑️ The key environment of a build script run whose verdict is portable or needs the identical toolchain: the key environment without compiler wrappers plus the carried and absent names and the toolchain digest, so every machine computes one key; `undefined` for any other script (its key stays as it is). */
export function buildScriptProbeKeyV1(input:Readonly<{environment:Environment;toolchain:string;keyEnvironment:readonly (readonly [string,string])[]}>):readonly (readonly [string,string])[]|undefined{
 const verdict=buildScriptVerdictOfV1(input.environment);
 if(verdict===undefined||verdict.kind==="host-bound")return undefined;
 if(verdict.kind==="needs-toolchain")return[...probeKeyEnvironmentV1(input.keyEnvironment),...probeKeyRowsV1({carry:[],absent:[]},input.toolchain)];
 return[...probeKeyEnvironmentV1(input.keyEnvironment),...probeKeyRowsV1(verdict,input.toolchain)];
}
