import {createHash} from "node:crypto";
import {mkdirSync,readFileSync,realpathSync,statSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {basename,isAbsolute,join,resolve} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {machineNeutralRemapV1} from "../🧭️remap/🟦️.ts";

/** 📐️ The authored unit-cache policy.
 * @see ../🧬️schema/🔣️.json */
export const UNIT_CACHE_POLICY_V1=schema.definitions.Policy.const;
const policy=UNIT_CACHE_POLICY_V1;

/** 🔗️ One `--extern` the compiler resolves from the dependency directory. */
export type RustcExternV1=Readonly<{flags:string;name:string;path:string}>;
/** 🧾️ The parts of one rustc command line that decide whether and how its result can be reused. */
export type RustcUnitV1=Readonly<{crateName:string;crateTypes:readonly string[];emit:readonly string[];outDir:string;source:string;target:string|undefined;externs:readonly RustcExternV1[];links:readonly string[];nativeSearch:boolean;incremental:string|undefined;artifacts:boolean}>;
/** 🚫️ Why a command line is never cached, `undefined` when it can be. */
export type RustcBypassV1=string|undefined;

const valueFlags=new Set(["--crate-name","--crate-type","--emit","--out-dir","--target","--edition","--extern","-L","-l","-C","--cfg","--check-cfg","--cap-lints","--error-format","--json","--diagnostic-width","--remap-path-prefix","-A","-W","-D","-F","--force-warn","-Z","--codegen","--sysroot","--print","--explain","-o","--color","--allow","--warn","--deny","--forbid","--force-warn","--emit-dep-info"]);

/** 📖️ Expands `@file` response files exactly as rustc does: one argument per line. */
export function expandRustcArgumentsV1(args:readonly string[],cwd:string):string[]{
 return args.flatMap(argument=>{if(!argument.startsWith("@")||argument.length<2)return[argument];try{return readFileSync(resolve(cwd,argument.slice(1)),"utf8").split(/\r?\n/u).filter(line=>line.length>0);}catch{return[argument];}});
}

type Option=Readonly<{index:number;span:number;flag:string;value:string}>;
function options(args:readonly string[]):{options:Option[];positional:string[]}{
 const found:Option[]=[],positional:string[]=[];
 for(let index=0;index<args.length;index++){
  const argument=args[index]!;
  if(argument==="-"||!argument.startsWith("-")){positional.push(argument);continue;}
  const equals=argument.startsWith("--")?argument.indexOf("="):-1;
  if(equals>0){found.push({index,span:1,flag:argument.slice(0,equals),value:argument.slice(equals+1)});continue;}
  if(valueFlags.has(argument)){found.push({index,span:2,flag:argument,value:args[index+1]??""});index++;continue;}
  const short=/^-([LlCAWDFZo])(.+)$/u.exec(argument);
  if(short){found.push({index,span:1,flag:"-"+short[1],value:short[2]!});continue;}
  found.push({index,span:1,flag:argument,value:""});
 }
 return{options:found,positional};
}

/** 🔬️ Reads the compilation-relevant options of one rustc command line. */
export function parseRustcUnitV1(args:readonly string[]):RustcUnitV1{
 const parsed=options(args),all=(flag:string)=>parsed.options.filter(row=>row.flag===flag).map(row=>row.value),first=(flag:string)=>all(flag)[0];
 const codegen=parsed.options.filter(row=>row.flag==="-C").map(row=>row.value),incremental=codegen.find(value=>value.startsWith("incremental="))?.slice("incremental=".length);
 const externs=all("--extern").map(value=>{const equals=value.indexOf("="),spec=equals<0?value:value.slice(0,equals),path=equals<0?"":value.slice(equals+1),colon=spec.lastIndexOf(":");return Object.freeze({flags:colon<0?"":spec.slice(0,colon+1),name:colon<0?spec:spec.slice(colon+1),path});});
 const json=all("--json").join(",");
 return Object.freeze({crateName:first("--crate-name")??"",crateTypes:Object.freeze(all("--crate-type").flatMap(value=>value.split(","))),emit:Object.freeze(all("--emit").flatMap(value=>value.split(",").map(part=>part.split("=")[0]!))),outDir:first("--out-dir")??"",source:parsed.positional[0]??"",target:first("--target"),externs:Object.freeze(externs),links:Object.freeze(all("-l")),nativeSearch:all("-L").some(value=>/^(native|all)=/u.test(value)),incremental,artifacts:json.includes("artifacts")});
}

/** ⚡️ Decides whether a command line can be served from the cache. */
export function rustcBypassV1(unit:RustcUnitV1,args:readonly string[]):RustcBypassV1{
 if(unit.source===""||unit.source==="-"||!unit.source.endsWith(".rs"))return"no-source-file";
 if(unit.crateName==="")return"no-crate-name";
 if(unit.outDir==="")return"no-out-dir";
 if(args.some(argument=>argument==="--print"||argument.startsWith("--print=")||argument==="-vV"||argument==="--version"||argument==="-V"||argument==="--explain"))return"query";
 if(!unit.artifacts)return"no-artifact-messages";
 if(!unit.emit.includes("link")&&!unit.emit.includes("metadata"))return"no-emit";
 const types=unit.crateTypes.length?unit.crateTypes:["bin"],allowed=policy.key.cacheableCrateTypes;
 if(types.some(type=>!allowed.includes(type)))return"crate-type:"+types.join(",");
 if(types.some(type=>policy.key.linkedCrateTypes.includes(type))&&unit.emit.includes("link")&&(unit.links.length>0||unit.nativeSearch))return"native-link";
 return undefined;
}

/** 🔤️ Hex SHA-256 of bytes or text. */
export function sha256V1(value:string|Uint8Array):string{return createHash("sha256").update(value).digest("hex");}

/** 🧭️ Replaces every occurrence of the lane root so equal compilations in different lanes produce equal text. */
export function laneNeutralV1(text:string,laneRoot:string|undefined):string{
 if(!laneRoot)return text;
 const forms=new Set([laneRoot,laneRoot.replaceAll("\\","/"),laneRoot.replaceAll("\\","\\\\")]);
 let result=text;for(const form of[...forms].sort((left,right)=>right.length-left.length))result=result.split(form).join(policy.key.laneToken);
 return result;
}

/** 🏠️ Restores the lane root into stored text. */
export function laneRestoreV1(text:string,laneRoot:string|undefined):string{
 if(!laneRoot)return text;
 return text.split(policy.key.laneToken).join(laneRoot);
}

const identityCodegenV1=/^(?:metadata|extra-filename)=/u;
const identitySpanV1=(args:readonly string[],index:number):number=>{
 const argument=args[index]!;
 if((argument==="-C"||argument==="--codegen")&&identityCodegenV1.test(args[index+1]??""))return 2;
 if(argument.startsWith("--codegen=")?identityCodegenV1.test(argument.slice(10)):argument.startsWith("-C")&&argument.length>2&&identityCodegenV1.test(argument.slice(2)))return 1;
 return 0;
};

/** 🆔️ The crate identity Cargo derives from the workspace (`-C metadata`, which seeds the stable crate id and every symbol hash) and the file-name suffix derived from it (`-C extra-filename`): both are left out of the key, because they differ between Cargo workspaces that compile the very same unit; the compiler is given {@link canonicalMetadataV1} instead. */
export function cargoIdentityArgumentsV1(args:readonly string[]):readonly string[]{
 const rows:string[]=[];
 for(let index=0;index<args.length;index++){const span=identitySpanV1(args,index);if(span>0){rows.push(args.slice(index,index+span).join(" ").replace(/^(?:-C|--codegen)[ =]?/u,""));index+=span-1;}}
 return rows;
}

/** 🆔️ The metadata every compilation of one unit gets, whoever compiles it, in whichever workspace and after whichever edit: a short hash of the slot-level identity of the unit (see `slotMetadataV1`), never of its content.
 * @see ../🛰️fleet/🪪️identity/🟦️.ts */
export function canonicalMetadataV1(identity:string):string{return sha256V1(JSON.stringify([policy.namespace,"metadata",identity])).slice(0,policy.key.metadataCharacters);}

/** 🔁️ The command line with the value of every `-C metadata=` replaced by `metadata` (in the spelling it had); `extra-filename` stays as Cargo gave it, because the file names Cargo looks for derive from it. */
export function withCanonicalMetadataV1(args:readonly string[],metadata:string):readonly string[]{
 return args.map((argument,index)=>{
  const previous=args[index-1];
  if((previous==="-C"||previous==="--codegen")&&argument.startsWith("metadata="))return"metadata="+metadata;
  if(argument.startsWith("-Cmetadata="))return"-Cmetadata="+metadata;
  if(argument.startsWith("--codegen=metadata="))return"--codegen=metadata="+metadata;
  return argument;
 });
}

/** 🏷️ The text with this unit's file-name suffix (`-C extra-filename`) replaced by its token, so an entry names its files the same in every workspace; ignored when the suffix is shorter than five characters. */
export function extraNeutralV1(text:string,extra:string):string{return extra.length<5?text:text.split(extra).join(policy.key.extraToken);}

/** 🏠️ Puts this unit's file-name suffix back where the token stands. */
export function extraRestoreV1(text:string,extra:string):string{return extra.length<5?text:text.split(policy.key.extraToken).join(extra);}

/** 🔖️ The file-name suffix (`-C extra-filename`) of a command line, `""` when it has none. */
export function extraFilenameV1(args:readonly string[]):string{
 for(const row of cargoIdentityArgumentsV1(args))if(row.startsWith("extra-filename="))return row.slice("extra-filename=".length);
 return"";
}

/** ✂️ Rewrites a command line without the parts that differ between lanes or between terminals: the output directory, the incremental directory, `-L dependency` search paths, extern paths, `--diagnostic-width` and `--color` and the rendering words of `--json` (`diagnostic-rendered-ansi`, `diagnostic-short`): Cargo sets them by terminal, colour and message format of the invocation, they change only the text of the compiler's rendered messages and nothing else, so each of their combinations would otherwise be a unit of its own on every machine that built it another way. */
export function normalizeRustcArgumentsV1(args:readonly string[],externDigests:ReadonlyMap<string,string>,laneRoot:string|undefined):string[]{
 const result:string[]=[];
 for(let index=0;index<args.length;index++){
  const argument=args[index]!;
  const identity=identitySpanV1(args,index);if(identity>0){index+=identity-1;continue;}
  const take=(flag:string):string|undefined=>{if(argument===flag){index++;return args[index]??"";}if(argument.startsWith(flag+"="))return argument.slice(flag.length+1);return undefined;};
  if(take("--out-dir")!==undefined)continue;
  if(argument==="-C"&&(args[index+1]??"").startsWith("incremental=")){index++;continue;}
  if(argument.startsWith("-Cincremental="))continue;
  if(argument==="-L"&&(args[index+1]??"").startsWith("dependency=")){index++;continue;}
  if(argument==="--diagnostic-width"){index++;continue;}
  if(argument.startsWith("--diagnostic-width="))continue;
  if(argument==="--color"){index++;continue;}
  if(argument.startsWith("--color="))continue;
  if(argument==="--json"||argument.startsWith("--json=")){
   const value=take("--json")!,words=value.split(",").filter(word=>word!=="diagnostic-rendered-ansi"&&word!=="diagnostic-short");
   result.push("--json="+words.join(","));continue;
  }
  if(argument.startsWith("-Ldependency="))continue;
  if(argument==="--extern"||argument.startsWith("--extern=")){const value=take("--extern")!,equals=value.indexOf("="),spec=equals<0?value:value.slice(0,equals),path=equals<0?"":value.slice(equals+1);result.push("--extern",path===""?spec:spec+"=#"+(externDigests.get(path)??"missing"));continue;}
  result.push(machineNeutralRemapV1(laneNeutralV1(argument,laneRoot)));
 }
 return result;
}

/** 🍎️ The identity of a macOS SDK as the key and the worker placement see it: the canonical name its `SDKSettings.json` declares (`macosx27.2`), else the version in the name of the SDK directory (`MacOSX15.4.sdk` gives `macosx15.4`, links resolved first, the given name second), never where it is installed or which link names it (Xcode's unversioned `MacOSX.sdk`, the command line tools or a task's own copy), so every machine with the same SDK version compiles and links alike. */
export function sdkIdentityV1(path:string):string{
 const trimmed=path.replace(/[\\/]+$/u,"");let real=trimmed;try{real=realpathSync(trimmed);}catch{}
 try{const name=(JSON.parse(readFileSync(join(real,"SDKSettings.json"),"utf8")) as {CanonicalName?:unknown}).CanonicalName;if(typeof name==="string"&&name!=="")return name.toLowerCase();}catch{}
 for(const candidate of [real,trimmed]){const found=/^(.*?)(\d+(?:\.\d+)*)\.sdk$/iu.exec(basename(candidate));if(found)return found[1]!.toLowerCase()+found[2];}
 return basename(real);
}

/** 🚩️ The Cargo-encoded compiler flags (separated by U+001F) with the machine-specific `FROM` of each path remapping replaced by the role it plays (see {@link machineNeutralRemapV1}), in the joined form and in the two-argument form. */
export function neutralRustflagsV1(encoded:string):string{
 const parts=encoded.split("\u001f");
 return parts.map((part,index)=>parts[index-1]==="--remap-path-prefix"?machineNeutralRemapV1("--remap-path-prefix="+part).slice("--remap-path-prefix=".length):machineNeutralRemapV1(part)).join("\u001f");
}

/** 🛠️ What a tool path of the environment (`RUSTC`, `RUSTDOC`, `RUSTC_LINKER`, `RUSTC_WRAPPER`, `RUSTC_WORKSPACE_WRAPPER`) is for the key: the file name of the program without its directory or extension (`rustc`, `cc`, `semio-unit-cache`), never where it is installed; the compiler build itself is the toolchain part of the key. */
export const toolIdentityV1=(path:string):string=>path.replace(/[\\/]+$/u,"").split(/[\\/]/u).at(-1)!.replace(/\.exe$/iu,"");

const TOOL_NAMES_V1=new Set(["RUSTC","RUSTDOC","RUSTC_LINKER","RUSTC_WRAPPER","RUSTC_WORKSPACE_WRAPPER"]);
const machineValueV1=(name:string,value:string):string=>name==="SDKROOT"?"⟨sdk⟩"+sdkIdentityV1(value):name==="CARGO_ENCODED_RUSTFLAGS"?neutralRustflagsV1(value):TOOL_NAMES_V1.has(name)?"⟨tool⟩"+toolIdentityV1(value):value;

/** 🚫️ The command line without its incremental session (`-C incremental=DIR` in the three spellings): an incremental compilation names its object files with a name that differs on every run, so its rlib has other bytes every time and other bytes than a compilation without a session, while the key of the unit leaves the session out. A unit that is shared through the fleet is compiled without one. */
export function withoutIncrementalV1(args:readonly string[]):readonly string[]{
 const kept:string[]=[];
 for(let index=0;index<args.length;index++){
  const argument=args[index]!;
  if(argument==="-C"&&(args[index+1]??"").startsWith("incremental=")){index++;continue;}
  if(argument.startsWith("-Cincremental=")||argument.startsWith("-C=incremental="))continue;
  kept.push(argument);
 }
 return kept;
}

/** 📦️ The OUT_DIR of a build script as a key value: `<anything>/build/<crate>-<hash>/out` (classic layout) or `<anything>/build/<crate>/<hash>/out` (`build-dir-new-layout`) is `⟨out⟩<crate>` (Cargo's hash of the unit and the directory the build lives in differ between checkouts, lanes and machines, the generated files are keyed by their bytes), any other path stays as it is, with the lane root replaced. */
export function outDirectoryIdentityV1(path:string,laneRoot:string|undefined):string{
 const text=laneNeutralV1(path,laneRoot),named=/(?:^|[\\/])build[\\/](?:([^\\/]+)-[0-9a-f]{16}|([^\\/]+)[\\/][0-9a-f]{16})[\\/]out[\\/]?$/u.exec(text);
 return named?"⟨out⟩"+(named[1]??named[2]):text;
}

/** 🌱️ The key form of an environment value the compiler's sources read with `env!`: the selected names as `keyEnvironmentV1` gives them, the OUT_DIR as `outDirectoryIdentityV1`, every other value with the lane root replaced. */
export function sourceValueV1(name:string,text:string,laneRoot:string|undefined):string{
 return name==="OUT_DIR"?outDirectoryIdentityV1(text,laneRoot):keyEnvironmentV1({[name]:text},laneRoot)[0]?.[1]??laneNeutralV1(text,laneRoot);
}

/** 📥️ The selected environment the compilation can observe without declaring it in its dependency file; values that name a place of this machine (the SDK, the remapped roots of the compiler flags) are reduced to what they are. */
export function keyEnvironmentV1(environment:Readonly<Record<string,string|undefined>>,laneRoot:string|undefined):Array<[string,string]>{
 const names=new Set(policy.key.environmentNames),rows:Array<[string,string]>=[];
 for(const name of Object.keys(environment).sort()){const value=environment[name];if(value===undefined)continue;if(names.has(name)||policy.key.environmentPrefixes.some(prefix=>name.startsWith(prefix)))rows.push([name,machineValueV1(name,laneNeutralV1(value,laneRoot))]);}
 return rows;
}

/** 🔑️ The first-level key: toolchain, normalized command line, root source bytes, dependency artifact bytes and selected environment. */
export function unitKeyV1(input:Readonly<{toolchain:string;normalized:readonly string[];sourceSha256:string;externs:readonly Readonly<{name:string;sha256:string}>[];environment:readonly (readonly [string,string])[]}>):string{
 return sha256V1(JSON.stringify([policy.namespace,input.toolchain,input.normalized,input.sourceSha256,input.externs.map(row=>[row.name,row.sha256]),input.environment]));
}

/** 🧩️ Names the parts of a first-level key by short digests (toolchain, arguments, source, externs, environment), so two compilations of one crate that disagree on the key are explained by the part that differs.
 * @see ./🧪️tests/🧩️parts/🟦️.ts */
export function unitKeyPartsV1(input:Parameters<typeof unitKeyV1>[0]):string{
 const part=(value:unknown)=>sha256V1(JSON.stringify(value)).slice(0,8);
 return["toolchain:"+part(input.toolchain),"args:"+part(input.normalized),"source:"+part(input.sourceSha256),"externs:"+part(input.externs.map(row=>[row.name,row.sha256])),"environment:"+part(input.environment)].join(" ");
}

function unescapeDepPath(text:string):string{return text.replace(/\\ /gu," ");}

/** 📜️ Reads a Makefile-style dependency file: the files the compilation read and the environment variables it observed. */
export function parseDepInfoV1(text:string):{dependencies:string[];environment:Array<{name:string;value:string|null}>}{
 const dependencies:string[]=[],environment:Array<{name:string;value:string|null}>=[];
 let ruleDone=false;
 for(const raw of text.split(/\r?\n/u)){
  if(raw.startsWith("# env-dep:")){const body=raw.slice("# env-dep:".length),equals=body.indexOf("=");environment.push(equals<0?{name:body,value:null}:{name:body.slice(0,equals),value:body.slice(equals+1).replace(/\\n/gu,"\n").replace(/\\r/gu,"\r").replace(/\\\\/gu,"\\")});continue;}
  if(raw.startsWith("#")||raw.trim()==="")continue;
  if(ruleDone)continue;
  const colon=raw.search(/(?<!\\): |(?<!\\):$/u);
  if(colon<0)continue;
  const rest=raw.slice(colon+1).trim();
  for(const token of rest.split(/(?<!\\) /u)){if(token!=="")dependencies.push(unescapeDepPath(token));}
  ruleDone=true;
 }
 return{dependencies,environment};
}

/** 🗂️ Resolves a dependency path against the compiler's working directory. */
export function resolveDependencyV1(cwd:string,path:string):string{return isAbsolute(path)?path:resolve(cwd,path);}

/** 🧬️ The identity of the compiler behind `inner`, from its `-vV` text: `SEMIO_UNIT_CACHE_TOOLCHAIN` when the launcher states it, else a probe memoised for one minute. */
export function toolchainIdentityV1(root:string,inner:readonly string[],environment:Readonly<Record<string,string|undefined>>):string{
 if(environment.SEMIO_UNIT_CACHE_TOOLCHAIN)return environment.SEMIO_UNIT_CACHE_TOOLCHAIN;
 const memo=join(root,"tc-"+sha256V1(inner.join("\0")+"\0"+(environment.RUSTUP_TOOLCHAIN??"")).slice(0,16));
 try{if(Date.now()-statSync(memo).mtimeMs<60000)return readFileSync(memo,"utf8");}catch{}
 const result=spawnSync(inner[0]!,[...inner.slice(1),"-vV"],{encoding:"utf8",env:environment as NodeJS.ProcessEnv});
 if(result.status!==0)throw Error("Compiler identity probe failed");
 mkdirSync(root,{recursive:true});writeFileSync(memo,result.stdout);return result.stdout;
}

/** 📍️ The sysroot of the compiler behind `inner` (`--print sysroot`), memoised for one minute like {@link toolchainIdentityV1}; undefined when the probe fails. */
export function rustSysrootV1(root:string,inner:readonly string[],environment:Readonly<Record<string,string|undefined>>):string|undefined{
 const memo=join(root,"sr-"+sha256V1(inner.join("\0")+"\0"+(environment.RUSTUP_TOOLCHAIN??"")).slice(0,16));
 try{if(Date.now()-statSync(memo).mtimeMs<60000)return readFileSync(memo,"utf8")||undefined;}catch{}
 const result=spawnSync(inner[0]!,[...inner.slice(1),"--print","sysroot"],{encoding:"utf8",env:environment as NodeJS.ProcessEnv});
 if(result.status!==0)return undefined;
 const sysroot=result.stdout.trim();mkdirSync(root,{recursive:true});writeFileSync(memo,sysroot);return sysroot||undefined;
}

/** 📨️ What the build broker is told about a unit: exactly the fields of the scheduler's `AdmitRequestV1` (the Rust `parse_unit` is the reference), `undefined` for probes. */
export type AdmitFactsV1=Readonly<{crate:string;package:string;version:string;target:string;crateTypes:readonly string[];outDir:string;extraFilename:string;source:string;manifestDir:string;primary:boolean;buildScriptOutDir:string|null;externs:readonly string[]}>;
export function admitFactsV1(args:readonly string[],environment:Readonly<Record<string,string|undefined>>,cwd:string):AdmitFactsV1|undefined{
 const unit=parseRustcUnitV1(args);
 if(unit.crateName===""||args.some(argument=>argument==="-vV"||argument==="-V"||argument==="--print"||argument.startsWith("--print=")||argument==="-"))return undefined;
 let extraFilename="";
 for(let index=0;index<args.length;index++){const argument=args[index]!;const value=argument==="-C"?args[index+1]:argument.startsWith("-C")?argument.slice(2):undefined;if(value?.startsWith("extra-filename="))extraFilename=value.slice("extra-filename=".length);}
 const absolute=(path:string)=>path===""||isAbsolute(path)?path:resolve(cwd,path);
 return Object.freeze({crate:unit.crateName,package:environment.CARGO_PKG_NAME??"",version:environment.CARGO_PKG_VERSION??"",target:unit.target??"host",crateTypes:Object.freeze(unit.crateTypes.length?[...unit.crateTypes]:["lib"]),outDir:absolute(unit.outDir),extraFilename,source:absolute(unit.source),manifestDir:environment.CARGO_MANIFEST_DIR??"",primary:environment.CARGO_PRIMARY_PACKAGE!==undefined,buildScriptOutDir:environment.OUT_DIR??null,externs:Object.freeze(unit.externs.filter(row=>row.path!=="").map(row=>absolute(row.path)))});
}

/** 🧱️ A path of a build directory as a key value: every `<anything>/build/<crate>-<hash>` (classic layout) or `<anything>/build/<crate>/<hash>` (`build-dir-new-layout`) inside a text is `⟨build⟩<crate>` (Cargo's hash of the unit and the directory the build lives in differ between lanes, checkouts and machines; the files below are what the key must not depend on by path). */
export function buildPathIdentityV1(text:string):string{
 return text.replace(/(?:[A-Za-z]:)?[\\/][^\s\u001f"';]*?[\\/]build[\\/](?:([^\\/\s]+)-[0-9a-f]{16}|([^\\/\s]+)[\\/][0-9a-f]{16})(?=[\\/\s\u001f"';]|$)/gu,(_,named:string|undefined,nested:string|undefined)=>"⟨build⟩"+(named??nested));
}

/** 🧭️ The directories of the build scripts of dependencies a run was told about (`DEP_<LINKS>_<KEY>` variables whose value is an absolute path in a build directory), each with the token its text carries in a stored run: longest path first, so a directory below another is replaced before it. A replay in another lane replaces the token with that lane's own directory. */
export function buildDependencyPathsV1(environment:Readonly<Record<string,string|undefined>>):Array<readonly [string,string]>{
 const rows:Array<readonly [string,string]>=[];
 for(const name of Object.keys(environment).sort()){const value=environment[name];if(value!==undefined&&name.startsWith("DEP_")&&/^[A-Z0-9_]+$/u.test(name)&&buildPathIdentityV1(value)!==value)rows.push(["@@semio-"+name.toLowerCase().replaceAll("_","-")+"@@",value]);}
 return rows.sort((left,right)=>right[1].length-left[1].length);
}

/** 🧪️ The environment a build script run is keyed by: Cargo's per-package, per-feature, per-cfg and dependency-metadata variables with the lane neutralised, the compiler flags with the machine-specific part of their path remappings reduced to its role and the compiler, wrapper and linker paths reduced to the program's name (the same neutralisation as {@link keyEnvironmentV1}); never `NUM_JOBS` or the job server. */
export function buildKeyEnvironmentV1(environment:Readonly<Record<string,string|undefined>>,laneRoot:string|undefined):Array<[string,string]>{
 const names=new Set(policy.buildScript.environmentNames),rows:Array<[string,string]>=[];
 for(const name of Object.keys(environment).sort()){const value=environment[name];if(value===undefined)continue;if(names.has(name)||policy.buildScript.environmentPrefixes.some(prefix=>name.startsWith(prefix)))rows.push([name,buildPathIdentityV1(machineValueV1(name,laneNeutralV1(value,laneRoot)))]);}
 return rows;
}

/** 📋️ The `cargo:rerun-if-*` directives of a build script's standard output. */
export function parseBuildDirectivesV1(stdout:string):{files:string[];variables:string[]}{
 const files:string[]=[],variables:string[]=[];
 for(const line of stdout.split(/\r?\n/u)){
  const match=/^cargo::?(rerun-if-changed|rerun-if-env-changed)=(.*)$/u.exec(line);if(!match)continue;
  (match[1]==="rerun-if-changed"?files:variables).push(match[2]!);
 }
 return{files,variables};
}

const argumentClassOf=(flag:string,value:string):string=>{
 if(flag==="--edition")return"edition";
 if(flag==="--crate-name"||flag==="--crate-type"||flag==="--emit")return"kind";
 if(flag==="--target")return"target";
 if(flag==="-C"||flag==="--codegen")return/^(?:metadata|extra-filename)=/u.test(value)?"metadata":"codegen";
 if(flag==="--cfg")return value.startsWith("feature=")?"features":"cfg";
 if(flag==="--check-cfg")return"cfg";
 if(["-A","-W","-D","-F","--cap-lints","--force-warn","--allow","--warn","--deny","--forbid"].includes(flag))return"lints";
 if(flag==="--remap-path-prefix"||flag==="--remap-path-scope")return"remap";
 if(["--error-format","--json","--color","--diagnostic-width"].includes(flag))return"diagnostics";
 if(flag==="-L"||flag==="-l")return"link";
 if(flag==="-Z")return"unstable";
 return"other";
};

/** 🧱️ Splits a normalized command line into its argument classes (`edition`, `kind`, `target`, `codegen`, `metadata`, `features`, `cfg`, `lints`, `remap`, `diagnostics`, `link`, `unstable`, `source`, `other`), each in command-line order. `--extern` is left out: every dependency is a component of its own. */
export function argumentClassesV1(normalized:readonly string[]):ReadonlyMap<string,readonly string[]>{
 const rows=new Map<string,string[]>(),add=(name:string,text:string)=>{const list=rows.get(name);if(list)list.push(text);else rows.set(name,[text]);};
 const parsed=options(normalized);
 for(const row of parsed.options){if(row.flag==="--extern")continue;add(argumentClassOf(row.flag,row.value),row.flag+(row.value===""?"":"="+row.value));}
 for(const text of parsed.positional)add("source",text);
 return rows;
}

const VALUE_ENVIRONMENT=new Set(["CARGO_ENCODED_RUSTFLAGS","RUSTFLAGS"]);
/** 🧾️ The key of a lookup as named components, each an individual digest, so two keys of one crate can be diffed: `toolchain`, `source`, `platform`, `roots`, one `extern:<name>` per dependency artifact, one `env:<NAME>` per observed variable and one `args.<class>` per argument class (see {@link argumentClassesV1}). `values` carries the text of the small, path-free classes (diagnostics, codegen, features, cfg, lints, unstable, kind, edition, target, remap) cut to a bounded length.
 * @see ./🧪️tests/🧬️components/🟦️.ts */
export function unitKeyComponentsV1(input:Parameters<typeof unitKeyV1>[0],extra:Readonly<{platform?:string;roots?:string}>={}):Readonly<{components:Record<string,string>;values:Record<string,string>}>{
 const width=policy.lookups.digestCharacters,part=(value:unknown)=>sha256V1(JSON.stringify(value)).slice(0,width),components:Record<string,string>={},values:Record<string,string>={};
 components.toolchain=part(input.toolchain);components.source=input.sourceSha256.slice(0,width);
 if(extra.platform!==undefined)components.platform=part(extra.platform);
 if(extra.roots!==undefined)components.roots=part(extra.roots);
 for(const row of input.externs)components["extern:"+row.name]=/^[0-9a-f]{16,}$/u.test(row.sha256)?row.sha256.slice(0,width):part(row.sha256);
 for(const [name,value] of input.environment){components["env:"+name]=part(value);if(VALUE_ENVIRONMENT.has(name)&&Object.keys(values).length<policy.lookups.maximumValues)values["env:"+name]=value.slice(0,policy.lookups.valueCharacters);}
 const shown=new Set(["diagnostics","codegen","features","cfg","lints","unstable","kind","edition","target","remap"]);
 for(const [name,list] of argumentClassesV1(input.normalized)){
  components["args."+name]=part(list);
  if(shown.has(name)&&Object.keys(values).length<policy.lookups.maximumValues)values["args."+name]=list.join(" ").slice(0,policy.lookups.valueCharacters);
 }
 return{components,values};
}

/** 🔀️ The components that are not equal in all of `rows` (absent in one counts as different), sorted. */
export function differingComponentsV1(rows:readonly Readonly<Record<string,string>>[]):string[]{
 const names=new Set(rows.flatMap(row=>Object.keys(row)));
 return[...names].filter(name=>new Set(rows.map(row=>row[name])).size>1).sort();
}
