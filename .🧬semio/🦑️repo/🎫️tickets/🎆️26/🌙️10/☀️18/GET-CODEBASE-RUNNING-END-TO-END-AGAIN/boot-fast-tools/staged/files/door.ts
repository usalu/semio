import {lstatSync,readFileSync} from "node:fs";
import {dirname,isAbsolute,join,parse,relative,resolve,sep} from "node:path";
import catalogue from "../⚙️operations/🔣️catalog.json" with {type:"json"};
import policy from "./🔣️.json" with {type:"json"};
import {buildOperationArgumentsV1,normalizeBuildOptionsV1} from "../⚙️operations/🔤️options/🟦️.ts";
import {buildHelpForArgumentsV1,resolveBuildCliLanguageV1} from "../⚙️operations/📖️help/🟦️.ts";
import {profilePhaseV1} from "../⏱️profile/🟨️.mjs";

/** 🛃️ The part of the neutral operations catalogue the front door reads: languages, global options with their allowed values, command and subcommand names. */
export type BuildDoorCatalogueV1=Readonly<{languages:readonly string[];flags:readonly Readonly<{name:string;aliases?:readonly string[];value:string;valueName?:string;operations?:readonly string[]}>[];operations:readonly Readonly<{name:string;requires:string;subcommands?:readonly Readonly<{name:string}>[]}>[]}>;

/** 🛅️ Which commands run without the build implementation, which subcommand sets are closed and the refusal texts. */
export type BuildDoorPolicyV1=Readonly<{version:1;passthrough:readonly string[];fullTargetPrefixes:readonly string[];environment:Readonly<Record<"--tool"|"--workers",string>>;light:readonly Readonly<{operation:string;export:string;call:"root-args-environment"|"args-options"}>[];closedSubcommands:readonly string[];refusedExitCode:number;texts:Readonly<Record<BuildDoorRefusalIdV1,Readonly<Record<string,string>>>>}>;

/** 🚧️ The refusals the front door can state before any storage, lease or graph exists. */
export type BuildDoorRefusalIdV1="unknownCommand"|"unknownSubcommand"|"invalidValue";

/** 🛣️ Where one invocation goes: help, a refusal, a light command that owns its closure, or the full implementation. */
export type BuildDoorRouteV1=Readonly<{kind:"full"}>|Readonly<{kind:"help";args:readonly string[]}>|Readonly<{kind:"refused";id:BuildDoorRefusalIdV1;values:Readonly<Record<string,string>>}>|Readonly<{kind:"light";operation:string;args:readonly string[]}>;

/** 🗺️ The language every later text uses and the route chosen once. */
export type BuildDoorPlanV1=Readonly<{language:string;route:BuildDoorRouteV1}>;

/** 🔌️ Replaceable sources, so a test can plan against another catalogue, policy or language rule. */
export type BuildDoorSourcesV1=Readonly<{catalogue?:BuildDoorCatalogueV1;policy?:BuildDoorPolicyV1;language?:(args:readonly string[],environment:NodeJS.ProcessEnv)=>string}>;

/** 🛄️ The only place a light command is named by module: each loader is a literal import, so bundlers and the closure laws see it. */
export const buildDoorLightLoadersV1:Readonly<Record<string,()=>Promise<Readonly<Record<string,unknown>>>>>=Object.freeze({
 leases:()=>import("../🔒️leases/⌨️command/🟦️.ts"),
 doctor:()=>import("../🩺️doctor/⌨️command/🟦️.ts"),
 capacity:()=>import("../🌡️host-capacity/⌨️command/🟦️.ts"),
});

const allowedValues=(source:BuildDoorCatalogueV1,option:string):readonly string[]|undefined=>{const name=source.flags.find(flag=>flag.name===option)?.valueName;return name!==undefined&&/^[A-Za-z0-9-]+(\|[A-Za-z0-9-]+)+$/u.test(name)?name.split("|"):undefined;};

/** 🧮️ Names the operation arguments the way the invocation parser does (global selections consumed, the opaque tail kept) without loading it; a value outside the catalogue's enumeration is reported instead. */
export function selectBuildDoorArgumentsV1(args:readonly string[],environment:NodeJS.ProcessEnv,source:BuildDoorCatalogueV1=catalogue as BuildDoorCatalogueV1,rules:BuildDoorPolicyV1=policy as BuildDoorPolicyV1):Readonly<{args:readonly string[]}>|Readonly<{refused:Readonly<{option:string;value:string;allowed:string}>}>{
 const chosen:Record<"--tool"|"--workers",{value:string|undefined;from:string}>={"--tool":{value:environment[rules.environment["--tool"]],from:rules.environment["--tool"]},"--workers":{value:environment[rules.environment["--workers"]],from:rules.environment["--workers"]}},selected:string[]=[];
 for(let index=0;index<args.length;index++){
  const argument=args[index]!;
  if(argument==="--"){selected.push(...args.slice(index));break;}
  const option=argument==="--tool"||argument.startsWith("--tool=")?"--tool":argument==="--workers"||argument.startsWith("--workers=")?"--workers":undefined;
  if(option!==undefined){chosen[option]={value:argument===option?args[++index]??"":argument.slice(option.length+1),from:option};continue;}
  if(argument==="--graph-policy"){index++;continue;}
  if(argument.startsWith("--graph-policy="))continue;
  selected.push(argument);
 }
 for(const option of ["--tool","--workers"] as const){
  const {value,from}=chosen[option],allowed=allowedValues(source,option);
  if(value!==undefined&&(option==="--tool"||value!=="")&&allowed&&!allowed.includes(value))return {refused:{option:from,value,allowed:allowed.join("|")}};
 }
 return {args:buildOperationArgumentsV1(selected,source)};
}

/** 🆘️ Tells whether the operation arguments ask for help, as the invocation module does for a request before storage. */
export function buildDoorRequestsHelpV1(args:readonly string[]):boolean{
 const boundary=args.indexOf("--"),head=args.slice(0,boundary<0?undefined:boundary);
 return head.length===0||head.some(argument=>argument==="--help"||argument==="-h");
}

/** 🎯️ Chooses the language and the route of one invocation from the catalogue alone; nothing else is loaded, read or spawned. */
export function planBuildDoorV1(typed:readonly string[],environment:NodeJS.ProcessEnv,sources:BuildDoorSourcesV1={}):BuildDoorPlanV1{
 const source=sources.catalogue??(catalogue as BuildDoorCatalogueV1),rules=sources.policy??(policy as BuildDoorPolicyV1),args=normalizeBuildOptionsV1(typed,source),language=(sources.language??resolveBuildCliLanguageV1)(args,environment),full={language,route:{kind:"full"} as const};
 if(rules.passthrough.includes(args[0]??""))return full;
 const selection=selectBuildDoorArgumentsV1(args.slice(args[0]==="repo"?1:0),environment,source,rules);
 if("refused" in selection)return {language,route:{kind:"refused",id:"invalidValue",values:selection.refused}};
 const operation=selection.args,command=operation[0];
 if(command==="run"&&rules.fullTargetPrefixes.some(prefix=>operation[1]?.startsWith(prefix)))return full;
 if(buildDoorRequestsHelpV1(operation))return {language,route:{kind:"help",args:operation}};
 const row=command===undefined||command.startsWith("-")?undefined:source.operations.find(candidate=>candidate.name===command);
 if(command!==undefined&&!command.startsWith("-")&&!row)return {language,route:{kind:"refused",id:"unknownCommand",values:{command,known:source.operations.map(candidate=>candidate.name).join(", ")}}};
 const subcommand=operation[1];
 if(row&&rules.closedSubcommands.includes(row.name)&&subcommand!==undefined&&!subcommand.startsWith("-")&&!row.subcommands?.some(candidate=>candidate.name===subcommand))return {language,route:{kind:"refused",id:"unknownSubcommand",values:{command:row.name,subcommand,known:(row.subcommands??[]).map(candidate=>candidate.name).join(", ")}}};
 if(row&&rules.light.some(candidate=>candidate.operation===row.name))return {language,route:{kind:"light",operation:row.name,args:operation}};
 return full;
}

/** 🚫️ Renders one refusal in the chosen language from the policy texts; every placeholder is named in the route values. */
export function renderBuildDoorRefusalV1(route:Extract<BuildDoorRouteV1,{kind:"refused"}>,language:string,rules:BuildDoorPolicyV1=policy as BuildDoorPolicyV1):string{
 const text=rules.texts[route.id][language];
 if(text===undefined)throw Error(`No ${route.id} text for ${language}`);
 return text.replace(/\{([A-Za-z]+)\}/gu,(_match,name:string)=>route.values[name]??`{${name}}`);
}

/** 🪛️ What the front door needs from its process: output, exit code and the loader of one light command. */
export type BuildDoorPortsV1=Readonly<{write:(line:string)=>void;error:(line:string)=>void;exit:(code:number)=>void;load:(operation:string)=>Promise<Readonly<Record<string,unknown>>>}>;

/** 🖥️ The ports of the real process. */
export function buildDoorProcessPortsV1():BuildDoorPortsV1{
 return {write:line=>console.log(line),error:line=>console.error(line),exit:code=>{process.exitCode=code;},load:operation=>{const loader=buildDoorLightLoadersV1[operation];if(!loader)throw Error(`No light loader for ${operation}`);return loader();}};
}

/** 🛎️ Serves help, refusals and light commands; returns `full` only when the build implementation has to start, so a command that needs none never loads it. */
export async function runBuildDoorV1(root:string,typed:readonly string[],environment:NodeJS.ProcessEnv,ports:BuildDoorPortsV1=buildDoorProcessPortsV1(),sources:BuildDoorSourcesV1={}):Promise<"handled"|"full">{
 const rules=sources.policy??(policy as BuildDoorPolicyV1);
 let plan:BuildDoorPlanV1;
 try{plan=await profilePhaseV1("door.plan",()=>planBuildDoorV1(typed,environment,sources));}catch(error){ports.error(error instanceof Error?error.message:String(error));ports.exit(rules.refusedExitCode);return "handled";}
 if(environment===process.env)process.env.SEMIO_LANGUAGE=plan.language;
 const route=plan.route,localized={...environment,SEMIO_LANGUAGE:plan.language};
 if(route.kind==="full")return "full";
 if(route.kind==="help"){ports.write(buildHelpForArgumentsV1(route.args,localized));return "handled";}
 if(route.kind==="refused"){ports.error(renderBuildDoorRefusalV1(route,plan.language,rules));ports.exit(rules.refusedExitCode);return "handled";}
 const row=rules.light.find(candidate=>candidate.operation===route.operation)!,run=(await profilePhaseV1("door.light-load",()=>ports.load(route.operation)))[row.export] as (...parameters:unknown[])=>Promise<number>;
 ports.exit(row.call==="args-options"?await run(route.args.slice(1),{environment:localized,write:ports.write}):await run(root,route.args.slice(1),localized,ports.write));
 return "handled";
}

/** 🛬️ Resolves the nearest workspace with both build and package markers like the routing module does, without loading it. */
export function findBuildDoorRootV1(start:string,environment:NodeJS.ProcessEnv=process.env):string{
 let directory=resolve(start.trim()||environment.SEMIO_WORKSPACE_ROOT?.trim()||environment.REPO_ROOT?.trim()||process.cwd());
 const boundary=parse(directory).root,distance=relative(boundary,directory);
 if(isAbsolute(distance)||distance===".."||distance.startsWith(`..${sep}`))throw Error("workspace root boundary must contain its start directory");
 for(let depth=0;depth<32;depth++){
  if(["📋️project.json","package.json"].every(filename=>lstatSync(join(directory,filename),{throwIfNoEntry:false})?.isFile())){
   const {workspaces}=JSON.parse(readFileSync(join(directory,"package.json"),"utf8"));
   if(Array.isArray(workspaces)||workspaces!==null&&typeof workspaces==="object")return directory;
  }
  const parent=dirname(directory);
  if(parent===directory)break;
  directory=parent;
 }
 throw Error("workspace root with paired build and package markers was not found");
}
