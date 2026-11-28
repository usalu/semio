import {createInterface} from "node:readline/promises";
import {resolveLanguageV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {writeResultLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";
import {collectFleetFactsV1,FLEET_READERS_V1,type FleetReadersV1} from "../📥️readers/🟦️.ts";
import {filterFleetReportV1,fleetTextV1,formatFleetReportV1,formatFleetSummaryV1,renderFleetJsonV1,selectFleetSectionsV1,summarizeFleetReportV1,summarizeFleetV1,validateFleetReportV1,validateFleetSelectionV1,validateFleetSummaryV1,type FleetLanguageV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 📝️ The closed arguments of `repo fleet status`: `detail` asks for the full report, `origin` and `section` select a part of it (and imply the detail view). */
export type FleetArgumentsV1=Readonly<{json:boolean;limit:number;detail:boolean;origin?:string;section?:string}>;
const DEFAULT_LIMIT=20,both=(key:string,values:Readonly<Record<string,string|number>>={}):string=>fleetTextV1("en",key,values)+"\n"+fleetTextV1("de",key,values);

/** 🧷️ Reads the options of `repo fleet status` (`--json`, `--detail`, `--origin=<id>`, `--section=<name>`, `--limit=<n>`); the global `--lang <en|de>` / `--lang=<en|de>` is consumed here as it is for the language resolution (`--language` is no flag of `repo`), every unknown or repeated option is refused in both languages. */
export function fleetArgumentsV1(args:readonly string[]):FleetArgumentsV1{
 if(args[0]!=="status")throw Error(both("error.syntax"));
 let json=false,detail=false,limit=DEFAULT_LIMIT,origin:string|undefined,section:string|undefined;const seen=new Set<string>(),rest=args.slice(1);
 for(let at=0;at<rest.length;at++)if(rest[at]==="--lang")rest.splice(at,2);else if(rest[at]!.startsWith("--lang="))rest.splice(at--,1);
 for(const argument of rest){
  const name=/^(--limit|--origin|--section)=/u.exec(argument)?.[1]??argument;
  if(seen.has(name)||!["--json","--detail","--limit","--origin","--section"].includes(name))throw Error(both("error.syntax"));
  seen.add(name);
  if(name==="--json")json=true;
  else if(name==="--detail")detail=true;
  else if(name==="--origin"){origin=argument.slice(9);if(origin==="")throw Error(both("error.syntax"));}
  else if(name==="--section"){section=argument.slice(10);const known=schema.definitions.Policy.const.summary.sections;if(!known.includes(section))throw Error(both("error.section",{section,list:known.join(", ")}));}
  else{const value=Number(argument.slice(8)),maximum=schema.definitions.Policy.const.runs.maximumRows;if(!Number.isSafeInteger(value)||value<1||value>maximum)throw Error(both("error.limit",{maximum}));limit=value;}
 }
 return Object.freeze({json,limit,detail,...origin===undefined?{}:{origin},...section===undefined?{}:{section}});
}

/** 🎛️ What the command touches besides the checkout: the readers, the clock and the output; tests replace them. */
export type FleetCommandPortsV1=Readonly<{readers?:FleetReadersV1;now?:()=>number;hostname?:Readonly<{interactive?:boolean;answer?:(question:string)=>Promise<string>;run?:(command:string,args:readonly string[])=>Promise<number>;platform?:string}>;refuse?:(text:string)=>void}>;

/** ▶️ Runs `repo fleet status [--json] [--detail] [--origin=<id>] [--section=<name>] [--limit=<n>]`: reads the existing status readers in parallel, aggregates, validates the report against its closed schema and prints by default the summary (under 16 KB) as compact text in the chosen language or as one JSON document; the full report is behind `--detail`, a part of it behind `--origin` and `--section`. It takes no lease, loads no graph and writes nothing. */
export async function runFleetCliV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv,write:(line:string)=>void=writeResultLineV1,ports:FleetCommandPortsV1={}):Promise<number>{
  if(args[0]==="broken"){const {runBrokenCliV1}=await import("../../💥️broken/⌨️command/🟦️.ts");return runBrokenCliV1(root,args.slice(1),environment,write);}
  if(args[0]==="hostname"){
  const language:FleetLanguageV1=resolveLanguageV1(["en","de"] as const,args,environment).language,rest=args.slice(1).filter((argument,at,all)=>argument!=="--lang"&&!argument.startsWith("--lang=")&&all[at-1]!=="--lang");
  if(rest.length!==1||rest[0]!=="apply")throw Error(both("error.syntax"));
  const {runTopologyCommandAsyncV1}=await import("../../../../🤝️shared-worktree/🗺️topology/⌨️command/🟦️.ts"),{fleetAddressTextV1}=await import("../../../../🤝️shared-worktree/🗺️topology/🧭️addresses/🟦️.ts"),own=ports.hostname??{};
  const refusal=fleetAddressTextV1("en","hostname.tty")+"\n"+fleetAddressTextV1("de","hostname.tty");
  const confirm=async(question:string):Promise<boolean>=>{
   if(!(own.interactive??Boolean(process.stdin.isTTY)))throw Error(refusal);
   if(own.answer!==undefined)return /^(y|yes|j|ja)$/iu.test((await own.answer(question)).trim());
   const lines=createInterface({input:process.stdin,output:process.stdout});
   try{return /^(y|yes|j|ja)$/iu.test((await lines.question(question)).trim());}finally{lines.close();}
  };
  const result=await runTopologyCommandAsyncV1(["hostname","--root",root,"--lang",language],{platform:own.platform??process.platform,signal:new AbortController().signal,progress:write,environment,confirm,...own.run===undefined?{}:{run:own.run}}).catch((error:unknown)=>{if(error instanceof Error&&error.message===refusal){(ports.refuse??console.error)(refusal);return {text:"",exitCode:2};}throw error;});
  if(result.text!=="")write(result.text);
  return result.exitCode;
 }
 const parsed=fleetArgumentsV1(args),language:FleetLanguageV1=resolveLanguageV1(["en","de"] as const,args,environment).language,now=ports.now??Date.now;
 const facts=await collectFleetFactsV1({root,environment,language,now},ports.readers??FLEET_READERS_V1),report=summarizeFleetV1(facts,language),problems=validateFleetReportV1(report);
 if(problems.length>0)throw Error(fleetTextV1(language,"error.invalid",{details:problems.slice(0,3).join("; ")}));
 if(!parsed.detail&&parsed.origin===undefined&&parsed.section===undefined){
  const summary=summarizeFleetReportV1(report,parsed.limit),invalid=validateFleetSummaryV1(summary);
  if(invalid.length>0)throw Error(fleetTextV1(language,"error.invalid",{details:invalid.slice(0,3).join("; ")}));
  if(parsed.json)write(renderFleetJsonV1(summary));
  else for(const line of formatFleetSummaryV1(language,summary))write(line);
  return 0;
 }
 const view=parsed.origin===undefined?report:filterFleetReportV1(report,parsed.origin),sections=parsed.section===undefined?undefined:new Set([parsed.section]);
 if(parsed.json&&parsed.origin===undefined&&parsed.section===undefined)write(JSON.stringify(report,null,2));
 else if(parsed.json){
  const selection=selectFleetSectionsV1(view,parsed.section===undefined?schema.definitions.Policy.const.summary.sections:[parsed.section],parsed.origin),invalid=validateFleetSelectionV1(selection);
  if(invalid.length>0)throw Error(fleetTextV1(language,"error.invalid",{details:invalid.slice(0,3).join("; ")}));
  write(JSON.stringify(selection,null,2));
 }
 else for(const line of formatFleetReportV1(language,view,parsed.limit,sections))write(line);
 return 0;
}
