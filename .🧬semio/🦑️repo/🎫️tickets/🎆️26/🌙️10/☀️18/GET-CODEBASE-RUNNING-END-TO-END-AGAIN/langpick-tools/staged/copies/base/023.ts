import {leaseLanguageV1} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts";
import {writeResultLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";
import {collectFleetFactsV1,FLEET_READERS_V1,type FleetReadersV1} from "../📥️readers/🟦️.ts";
import {fleetTextV1,formatFleetReportV1,summarizeFleetV1,validateFleetReportV1,type FleetLanguageV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 📝️ The closed arguments of `repo fleet status`. */
export type FleetArgumentsV1=Readonly<{json:boolean;limit:number}>;
const DEFAULT_LIMIT=20,both=(key:string,values:Readonly<Record<string,string|number>>={}):string=>fleetTextV1("en",key,values)+"\n"+fleetTextV1("de",key,values);

/** 🧷️ Reads the options of `repo fleet status`; every unknown or repeated option is refused in both languages. */
export function fleetArgumentsV1(args:readonly string[]):FleetArgumentsV1{
 if(args[0]!=="status")throw Error(both("error.syntax"));
 let json=false,limit=DEFAULT_LIMIT;const seen=new Set<string>();
 for(const argument of args.slice(1)){
  const name=argument.startsWith("--limit=")?"--limit":argument;
  if(seen.has(name)||name!=="--json"&&name!=="--limit")throw Error(both("error.syntax"));
  seen.add(name);
  if(name==="--json")json=true;
  else{const value=Number(argument.slice(8)),maximum=schema.definitions.Policy.const.runs.maximumRows;if(!Number.isSafeInteger(value)||value<1||value>maximum)throw Error(both("error.limit",{maximum}));limit=value;}
 }
 return Object.freeze({json,limit});
}

/** 🎛️ What the command touches besides the checkout: the readers, the clock and the output; tests replace them. */
export type FleetCommandPortsV1=Readonly<{readers?:FleetReadersV1;now?:()=>number}>;

/** ▶️ Runs `repo fleet status [--json] [--limit=<n>]`: reads the existing status readers in parallel, aggregates, validates the report against its closed schema and prints it as compact text in the chosen language or as one JSON document. It takes no lease, loads no graph and writes nothing. */
export async function runFleetCliV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv,write:(line:string)=>void=writeResultLineV1,ports:FleetCommandPortsV1={}):Promise<number>{
 const parsed=fleetArgumentsV1(args),language:FleetLanguageV1=leaseLanguageV1(environment),now=ports.now??Date.now;
 const facts=await collectFleetFactsV1({root,environment,language,now},ports.readers??FLEET_READERS_V1),report=summarizeFleetV1(facts,language),problems=validateFleetReportV1(report);
 if(problems.length>0)throw Error(fleetTextV1(language,"error.invalid",{details:problems.slice(0,3).join("; ")}));
 if(parsed.json)write(JSON.stringify(report,null,2));
 else for(const line of formatFleetReportV1(language,report,parsed.limit))write(line);
 return 0;
}
