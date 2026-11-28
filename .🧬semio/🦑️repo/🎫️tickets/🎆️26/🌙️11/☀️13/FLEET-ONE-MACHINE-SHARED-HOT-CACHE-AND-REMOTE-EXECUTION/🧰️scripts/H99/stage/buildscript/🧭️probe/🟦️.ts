import {createHash} from "node:crypto";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

type Environment=Readonly<Record<string,string|undefined>>;

/** 🧭️ The closed policy of what a build script may ask of the host and still run on another machine.
 * @see ./🧬️schema/🔣️.json */
export const BUILD_PROBE_POLICY_V1=schema.policy;
const policy=BUILD_PROBE_POLICY_V1,spellings=policy.spawn.compilerSpellings.map(row=>new RegExp(row,"u")),compileMarkers=policy.spawn.compileMarkers.map(row=>new RegExp(row,"u")),hostBound=policy.hostBound.map(row=>({...row,expression:new RegExp(row.pattern,"u")})),unsafe=new RegExp(policy.environment.unsafeValue,"u");
const ABSENT="\u0000absent",sha256=(value:string):string=>createHash("sha256").update(value).digest("hex");

/** 🧭️ What a script asks of the compiler: nothing, its version (answered from the toolchain identity of the run key), a compile (needs the identical toolchain) or a program that is not the compiler. */
export type ProbeClassV1="none"|"version"|"compile"|"foreign";

/** 🧭️ The verdict on one build script: portable (with the stand-in compiler and the environment it needs), needing a worker of the identical toolchain, or host-bound with its declared reason. */
export type ProbeVerdictV1=Readonly<{kind:"portable";probe:ProbeClassV1;standIn:boolean;carry:readonly (readonly [string,string])[];absent:readonly string[]}>|Readonly<{kind:"needs-toolchain";reason:"toolchain-compile-probe";detail:string}>|Readonly<{kind:"host-bound";reason:string;detail:string;en:string;de:string}>;

const withoutComments=(source:string):string=>source.split("\n").filter(line=>!/^\s*\/\//u.test(line)).join("\n");

/** 📎️ The source text of a script with its `include!("<path>")` files inlined (at most `depth` levels): the included file is part of the compiled script, so its text, not its path, is what the script can do. A file `read` cannot give leaves the macro as it is, which the host-path rule then names. */
export function expandedSourceV1(source:string,read:(path:string)=>string|undefined,depth=3):string{
 if(depth<=0)return source;
 return source.replace(/\binclude!\s*\(\s*"([^"]+)"\s*\)\s*;?/gu,(whole,path:string)=>{const text=read(path);return text===undefined?whole:expandedSourceV1(text,read,depth-1);});
}

/** 🔎️ Every `Command::new(<program>)` of a source text, the program as written. */
export function spawnedProgramsV1(source:string):readonly string[]{
 const code=withoutComments(source),rows:string[]=[];
 for(const match of code.matchAll(/\bCommand\s*::\s*new\s*\(/gu)){
  let depth=1,index=match.index!+match[0].length;const start=index;
  while(index<code.length&&depth>0){depth+=code[index]==="("?1:code[index]===")"?-1:0;index+=1;}
  rows.push(code.slice(start,index-1).trim());
 }
 return rows;
}

/** 🔎️ Classifies what a script asks of the compiler. A program named by a target guard (`freebsd-version` under `freebsd`) does not run on any other target OS, so it does not count there; with an unknown target OS it counts. */
export function buildScriptProbeV1(input:Readonly<{source:string;dependencies:readonly string[];targetOs:string|undefined}>):Readonly<{class:ProbeClassV1;foreign:readonly string[]}>{
 const programs=spawnedProgramsV1(input.source),foreign=programs.filter(program=>!spellings.some(row=>row.test(program))&&!policy.targetGuards.some(guard=>guard.program===program.replace(/^"|"$/gu,"")&&input.targetOs!==undefined&&input.targetOs!==guard.targetOs));
 if(foreign.length>0)return{class:"foreign",foreign};
 const libraries=policy.libraries.filter(row=>input.dependencies.includes(row.name)),compiler=programs.some(program=>spellings.some(row=>row.test(program)));
 if(libraries.some(row=>row.class==="compile")||compiler&&compileMarkers.some(row=>row.test(withoutComments(input.source))))return{class:"compile",foreign:[]};
 return{class:compiler||libraries.length>0?"version":"none",foreign:[]};
}

/** 🏷️ The declared reason a script stays on its host: the first matching row of the policy, else the program it starts. */
export function hostBoundReasonV1(source:string,foreign:readonly string[]):Readonly<{id:string;en:string;de:string}>{
 const code=withoutComments(source);
 for(const row of hostBound)if(row.expression.test(code))return{id:row.id,...row.text};
 const program=(foreign[0]??"unknown").replace(/^"|"$/gu,"").slice(0,40);
 return{id:"spawns-program",en:`starts the program ${program}`,de:`startet das Programm ${program}`};
}

/** 🪞️ The text of the stand-in compiler: it answers `--version` and `-vV` from the declared identity file beside it and refuses every other question (marker in `OUT_DIR`, status 97), so a script that asks more than the identity can answer runs here instead. */
export const rustcStandInTextV1=():string=>policy.standIn.lines.join("\n")+"\n";

/** 🪞️ The answer of the stand-in compiler to its arguments, `undefined` when it refuses; a model of the shell text above for tests. */
export function rustcStandInAnswerV1(identity:string,args:readonly string[]):string|undefined{
 const row=policy.standIn.answers.find(answer=>answer.arguments.join(" ")===args.join(" "));
 if(row===undefined)return undefined;
 return row.lines==="all"?identity:identity.split("\n")[0]+"\n";
}

/** 🧬️ The digest of the declared toolchain identity (the `-vV` text of the run key), as the stand-in's input and the worker's toolchain advertisement name it. */
export const toolchainDigestV1=(identity:string):string=>sha256(identity);

/** 🌿️ The environment names a script reads: a declared name or a compiler name is the command's own, an unset name is keyed as absent (a local run reads exactly the same), a set name the command can carry (`DEP_*`, `CARGO_MANIFEST_LINKS`) travels with its keyed value, any other set name stays with its host (`host-environment`); a carried value that names a path of the host is refused (`host-value`). */
export function environmentPlanV1(input:Readonly<{source:string;environment:Environment;declared:(name:string)=>boolean;probe:ProbeClassV1}>):Readonly<{carry:readonly (readonly [string,string])[];absent:readonly string[]}>|Readonly<{refusal:string}>{
 const code=withoutComments(input.source),names=new Set<string>(),compiler=new Set<string>([policy.compilerEnvironment.carried,...policy.compilerEnvironment.dropped]);
 for(const match of code.matchAll(/\b(?:var|var_os|cargo_env_var)\s*\(\s*"([^"]+)"/gu))names.add(match[1]!);
 const carried=(name:string):boolean=>policy.environment.carriedNames.includes(name)||policy.environment.carriedPrefixes.some(prefix=>name.startsWith(prefix));
 const carry:Array<readonly [string,string]>=[],absent:string[]=[];
 const take=(name:string,value:string):string|undefined=>{
  if(unsafe.test(value)||Buffer.byteLength(value)>policy.environment.maximumBytes)return"host-value:"+name.slice(0,40);
  carry.push([name,value]);return undefined;
 };
 for(const name of [...names].sort()){
  if(input.declared(name)||input.probe!=="none"&&compiler.has(name))continue;
  const value=input.environment[name];
  if(value===undefined){absent.push(name);continue;}
  if(!carried(name))return{refusal:"host-environment:"+name.slice(0,40)};
  const refusal=take(name,value);if(refusal!==undefined)return{refusal};
 }
 for(const name of Object.keys(input.environment).sort()){
  const value=input.environment[name];
  if(value===undefined||!carried(name)||carry.some(row=>row[0]===name))continue;
  const refusal=take(name,value);if(refusal!==undefined)return{refusal};
 }
 return{carry,absent};
}

/** 🔑️ The rows the run key takes for the plan, so a changed value, a newly set name or a changed toolchain gives another key on every machine alike: carried names with their value, absent names as absent, the toolchain by its digest. */
export function probeKeyRowsV1(plan:Readonly<{carry:readonly (readonly [string,string])[];absent:readonly string[]}>,identity:string):readonly (readonly [string,string])[]{
 return[...plan.carry,...plan.absent.map(name=>[name,ABSENT] as const),["semio-toolchain",toolchainDigestV1(identity)]].sort((left,right)=>left[0]<right[0]?-1:left[0]>right[0]?1:0);
}

/** 🔑️ The key environment of a probing run without the compiler wrappers: the stand-in answers whatever a wrapper would forward, so a host that sets `RUSTC_WRAPPER` and one that does not compute one key. */
export const probeKeyEnvironmentV1=(rows:readonly (readonly [string,string])[]):readonly (readonly [string,string])[]=>rows.filter(row=>!policy.compilerEnvironment.dropped.includes(row[0]));

/** ⚖️ The verdict on one script: hermetic or probing only through the declared toolchain and environment (portable), compiling probes (needs the identical toolchain on the worker), or host-bound with a declared reason. */
export function buildScriptVerdictV1(input:Readonly<{source:string;dependencies:readonly string[];environment:Environment;declared:(name:string)=>boolean;include?:(path:string)=>string|undefined}>):ProbeVerdictV1{
 const source=expandedSourceV1(input.source,input.include??(()=>undefined)),probe=buildScriptProbeV1({source,dependencies:input.dependencies,targetOs:input.environment.CARGO_CFG_TARGET_OS});
 const bound=probe.class==="foreign"||hostBound.some(row=>row.expression.test(withoutComments(source)));
 if(bound){const reason=hostBoundReasonV1(source,probe.foreign);return{kind:"host-bound",reason:"host-bound:"+reason.id,detail:reason.id,en:reason.en,de:reason.de};}
 const plan=environmentPlanV1({source,environment:input.environment,declared:input.declared,probe:probe.class});
 if("refusal" in plan){const id=plan.refusal.split(":")[0]!;return{kind:"host-bound",reason:"host-bound:"+id,detail:plan.refusal,...id==="host-value"?{en:"reads a value that names a path of the host",de:"liest einen Wert, der einen Pfad des Hosts nennt"}:{en:"reads an environment variable of the host that the command cannot carry",de:"liest eine Umgebungsvariable des Hosts, die das Kommando nicht mitgeben kann"}};}
 if(probe.class==="compile")return{kind:"needs-toolchain",reason:"toolchain-compile-probe",detail:"compiles probe programs with the compiler of the run key"};
 return{kind:"portable",probe:probe.class,standIn:probe.class==="version",carry:plan.carry,absent:plan.absent};
}
