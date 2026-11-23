import {lstatSync} from "node:fs";
import {basename,join,resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import type {RustcUnitV1} from "../🔏️key/🟦️.ts";

const policy=schema.definitions.Policy.const;
/** 📐️ The authored naming of the files a compilation leaves in its output directory.
 * @see ./🧬️schema/🔣️.json */
export const COMPLETE_POLICY_V1=policy;

const pick=(table:Readonly<Record<string,string>>,platform:string):string=>table[platform]??table.default!;
const codegenValue=(args:readonly string[],name:string):string|undefined=>args.flatMap((argument,index)=>argument==="-C"?[args[index+1]??""]:argument.startsWith("-C")&&argument.length>2?[argument.slice(2)]:[]).find(value=>value.startsWith(name+"="))?.slice(name.length+1);

/** 🧾️ The files in the output directory that a dependant (or Cargo) names for this compilation, from its command line alone: the dependency file, the `.rmeta` when metadata is emitted, the `.rlib` of a library, the dynamic library of a proc macro and the executable of a binary built for this machine. Whatever else the compiler writes (static and dynamic libraries of other kinds, cross compiled binaries) is covered by the compiler's own announcements ({@link announcedOutputsV1}). Empty when the command line names a single output file with `-o`.
 * @see https://doc.rust-lang.org/rustc/command-line-arguments.html#--emit-specify-the-types-of-output-files-to-generate */
export function expectedOutputsV1(unit:RustcUnitV1,args:readonly string[],platform:string=process.platform):readonly string[]{
 if(args.includes("-o")||args.some(argument=>argument.startsWith("-o")&&argument.length>2&&!argument.startsWith("--")))return[];
 const crate=unit.crateName,extra=codegenValue(args,"extra-filename")??"",fill=(template:string,more:Readonly<Record<string,string>>={})=>template.replace(/\{(\w+)\}/gu,(_,name:string)=>name==="crate"?crate:name==="extra"?extra:more[name]??"");
 const names:string[]=[],emit=new Set(unit.emit),types=unit.crateTypes.length?unit.crateTypes:["bin"],hostOnly=unit.target===undefined,templates=policy.templates;
 if(emit.has("dep-info"))names.push(fill(templates["dep-info"]));
 if(emit.has("metadata"))names.push(fill(templates.metadata));
 if(emit.has("link")){
  if(types.some(type=>(policy.rlibTypes as readonly string[]).includes(type)))names.push(fill(templates.rlib));
  if(types.includes("proc-macro"))names.push(fill(templates["proc-macro"],{prefix:pick(policy.dynamicPrefixes,platform),dynamic:pick(policy.dynamicExtensions,platform)}));
  if(hostOnly&&types.includes("bin"))names.push(fill(templates.bin,{executable:pick(policy.executableExtensions,platform)}));
 }
 return[...new Set(names)];
}

/** 🔔️ The artifact paths a compiler announced in its JSON messages (`link`, `metadata` and `dep-info`), in order of appearance without repeats; Cargo starts the dependants of a pipelined unit as soon as it reads the `.rmeta` announcement, so these are the files dependants will look for.
 * @see https://doc.rust-lang.org/rustc/json.html#artifact-notifications */
export function announcedPathsV1(text:string):string[]{
 const found:string[]=[];
 for(const line of text.split("\n")){if(!line.startsWith("{")||!line.includes("\"artifact\""))continue;try{const row=JSON.parse(line);if(typeof row.artifact==="string"&&["link","metadata","dep-info"].includes(row.emit))found.push(row.artifact);}catch{}}
 return[...new Set(found)];
}

/** 🧮️ The names among `need` that `have` lacks. */
export const uncoveredOutputsV1=(have:readonly string[],need:readonly string[]):string[]=>{const present=new Set(have);return[...new Set(need)].filter(name=>!present.has(name));};

/** 🔦️ The files a unit must have left in `directory` before it may be reported as done: the names its command line promises plus every path the compiler announced in `text`; returns the file names that are missing (empty when the unit is complete). A unit reported done without them lets Cargo start the dependants, whose compilers then fail with `extern location for <crate> does not exist`. */
export function missingOutputsV1(input:Readonly<{directory:string;cwd:string;expected:readonly string[];text:string}>):string[]{
 const wanted=[...input.expected.map(name=>join(input.directory,name)),...announcedPathsV1(input.text).map(path=>resolve(input.cwd,path))];
 return[...new Set(wanted)].filter(path=>{try{return!lstatSync(path).isFile();}catch{return true;}}).map(path=>basename(path));
}
