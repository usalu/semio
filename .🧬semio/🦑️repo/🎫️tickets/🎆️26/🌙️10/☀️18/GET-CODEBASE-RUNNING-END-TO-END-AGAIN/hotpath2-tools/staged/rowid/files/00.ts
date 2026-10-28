import {createHash} from "node:crypto";
import {realpathSync,statSync} from "node:fs";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {findToolV1,resolveRegisteredToolV1,toolForCommandV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import {bazelKeyedCommandEnvironmentV1,type BazelCommandEnvironmentV1} from "../🟦️.ts";

const policy=schema.definitions.Policy.const;

/** 🔏️ The authored policy of runtime row identities. */
export const BAZEL_RUNTIME_IDENTITY_POLICY_V1=policy;
/** 🧭️ Where the executables of a row are found and what identifies each file; a test replaces it with fixtures. */
export interface BazelRuntimeHostV1{readonly resolve:(tool:string,environment:BazelCommandEnvironmentV1)=>{readonly command:string;readonly prefix:readonly string[];readonly source:string}|undefined;readonly stat:(path:string)=>string|undefined;readonly runtime:string}
/** 🧾️ The parts of a recorded `runtime` subject: the command text, the build mode and, when the row is listed and its executables are known, their identity. */
export interface BazelRuntimeSubjectV1{readonly runtime:string;readonly mode:string;readonly identity?:string}

/** 🔤️ Splits a row into words; only words and double quotes without substitution are understood, anything else is not a row this module may judge. */
export function bazelRuntimeWordsV1(text:string):readonly string[]|undefined{
 const words:string[]=[];let current:string|undefined,quoted=false;
 for(const character of text){
  if(quoted){if(character==='"'){quoted=false;continue;}if(policy.quoted.includes(character))return undefined;current+=character;continue;}
  if(character==='"'){quoted=true;current??="";continue;}
  if(/\s/u.test(character)){if(current!==undefined){words.push(current);current=undefined;}continue;}
  if(policy.unquoted.includes(character))return undefined;
  current=(current??"")+character;
 }
 if(quoted)return undefined;
 if(current!==undefined)words.push(current);
 return words;
}

/** 📋️ The tool and arguments of a listed row, or nothing when the row is not listed. */
export function bazelRuntimeToolRowV1(runtime:string):{readonly tool:string;readonly arguments:readonly string[]}|undefined{
 const words=bazelRuntimeWordsV1(runtime);if(words===undefined)return undefined;
 const head=policy.command.length;if(words.length<head+3||policy.command.some((word,index)=>words[index]!==word))return undefined;
 let index=head;if(words[index]===policy.light)index+=1;
 const tool=words[index],separator=words[index+1],args=words.slice(index+2);
 if(tool===undefined||separator!==policy.separator)return undefined;
 return policy.rows.some(row=>row.tool===tool&&row.arguments.length===args.length&&row.arguments.every((word,position)=>word===args[position]))?{tool,arguments:args}:undefined;
}

/** 🖥️ The real host: the tool registry resolves the program, `lstat`-free bigint stat fields of the real file identify it. */
export function systemBazelRuntimeHostV1(root:string):BazelRuntimeHostV1{
 return {
  runtime:process.execPath,
  resolve:(tool,environment)=>{const entry=findToolV1(tool)??toolForCommandV1(tool);if(entry===undefined)return undefined;const found=resolveRegisteredToolV1(entry,root,{env:environment as NodeJS.ProcessEnv,...entry.id===tool?{}:{command:tool}});return found.command===null?undefined:{command:found.command,prefix:found.prefix,source:found.source};},
  stat:path=>{try{const real=realpathSync(path),stats=statSync(real,{bigint:true});if(!stats.isFile())return undefined;return JSON.stringify([path,real,...policy.statFields.map(field=>String((stats as unknown as Record<string,bigint>)[field]))]);}catch{return undefined;}},
 };
}

/** 🪪️ The digest of everything that decides what a listed row prints: the host, the tool, how it was found and the identity of the runtime, the program and every script word of its prefix; nothing for a row that is not listed or whose executables cannot be identified. */
export function bazelRuntimeIdentityV1(root:string,runtime:string,environment:BazelCommandEnvironmentV1,host:BazelRuntimeHostV1=systemBazelRuntimeHostV1(root)):string|undefined{
 const row=bazelRuntimeToolRowV1(runtime);if(row===undefined)return undefined;
 const found=host.resolve(row.tool,bazelKeyedCommandEnvironmentV1(environment));if(found===undefined)return undefined;
 const files:string[]=[];
 for(const path of [host.runtime,found.command,...found.prefix.filter(word=>word.startsWith("/")||/^[A-Za-z]:[\\/]/u.test(word))]){const identity=host.stat(path);if(identity===undefined)return undefined;files.push(identity);}
 return createHash("sha256").update(JSON.stringify({version:policy.version,platform:process.platform,arch:process.arch,tool:row.tool,arguments:row.arguments,source:found.source,prefix:found.prefix,files})).digest("hex");
}

/** 🏷️ The subject a runtime observation is recorded under. */
export function bazelRuntimeSubjectV1(runtime:string,mode:string,identity?:string):string{return [runtime,mode,...identity===undefined?[]:[identity]].join(policy.subject);}
/** 🔎️ The parts of a recorded subject, nothing for a text that has none of the three shapes. */
export function parseBazelRuntimeSubjectV1(subject:string):BazelRuntimeSubjectV1|undefined{
 const [runtime,mode,identity,...rest]=subject.split(policy.subject);
 if(runtime===undefined||runtime===""||mode===undefined||rest.length>0)return undefined;
 return identity===undefined?{runtime,mode}:{runtime,mode,identity};
}

/** 🏃️ Runs a row and returns its standard output with the subject to record: the identity counts only when it is the same before and after the run. */
export async function measureBazelRuntimeRowV1(input:{readonly root:string;readonly runtime:string;readonly environment:BazelCommandEnvironmentV1;readonly run:(runtime:string)=>Promise<string>;readonly host?:BazelRuntimeHostV1}):Promise<{readonly stdout:string;readonly subject:string}>{
 const {root,runtime,environment}=input,host=input.host??systemBazelRuntimeHostV1(root),mode=String(environment.SEMIO_BUILD_MODE),before=bazelRuntimeIdentityV1(root,runtime,environment,host),stdout=await input.run(runtime),after=before===undefined?undefined:bazelRuntimeIdentityV1(root,runtime,environment,host);
 return {stdout,subject:bazelRuntimeSubjectV1(runtime,mode,before!==undefined&&before===after?before:undefined)};
}

/** ✅️ Decides one recorded runtime observation: the same mode and the same executable identity hold without a process, anything else runs the row again and compares the digest of its output. */
export async function checkBazelRuntimeRowV1(input:{readonly root:string;readonly subject:string;readonly expected:string;readonly environment:BazelCommandEnvironmentV1;readonly run:(runtime:string)=>Promise<string>;readonly digest:(stdout:string)=>string;readonly host?:BazelRuntimeHostV1}):Promise<boolean>{
 const row=parseBazelRuntimeSubjectV1(input.subject);if(row===undefined||String(input.environment.SEMIO_BUILD_MODE)!==row.mode)return false;
 if(row.identity!==undefined&&bazelRuntimeIdentityV1(input.root,row.runtime,input.environment,input.host)===row.identity)return true;
 return input.digest(await input.run(row.runtime))===input.expected;
}
