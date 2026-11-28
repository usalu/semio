import {resolveLanguageV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {journalMachineV1} from "../../📜️change-journal/🟦️.ts";
import {brokenSweepOnceV1,brokenWatchV1,type BrokenSweepPortsV1} from "../🏃️sweep/🟦️.ts";
import {brokenTextV1} from "../📖️texts/🟦️.ts";

/** 🔌️ The seams of the command; tests replace them. */
export type BrokenCommandPortsV1=Readonly<{sweep?:BrokenSweepPortsV1;signal?:AbortSignal}>;

/** 🧹️ Runs `repo fleet broken sweep [--once|--watch] [--full]`: one sweep (default) or the resident sweep until cancelled; the check runs on a consistent source snapshot at the lowest priority and the board is published under this machine's origin in the fleet store, where `repo fleet status --section broken` merges it. */
export async function runBrokenCliV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv,write:(line:string)=>void,ports:BrokenCommandPortsV1={}):Promise<number>{
 const languages:readonly ("en"|"de")[]=[resolveLanguageV1(["en","de"] as const,args,environment).language];
 const rest=args.filter((argument,at,all)=>argument!=="--lang"&&!argument.startsWith("--lang=")&&all[at-1]!=="--lang");
 const flags=rest.slice(1),known=["--once","--watch","--full"];
 if(rest[0]!=="sweep"||flags.some(flag=>!known.includes(flag))||flags.includes("--once")&&flags.includes("--watch")){for(const language of languages)write(brokenTextV1(language,"usage"));return 2;}
 const machine=journalMachineV1(root,environment);
 if(machine===undefined){for(const language of languages)write(brokenTextV1(language,"machine.missing"));return 1;}
 const control=new AbortController();
 const abort=():void=>control.abort();
 process.on("SIGINT",abort);process.on("SIGTERM",abort);
 const snapshot:BrokenSweepPortsV1["snapshot"]=async live=>{
  const [{snapshotAcquireV1},{snapshotBaseV1}]=await Promise.all([import("../../📸️snapshot/⏩️advance/🟦️.ts"),import("../../📸️snapshot/🟦️.ts")]),base=await snapshotBaseV1(live,environment);
  const receipt=await snapshotAcquireV1({root:live,lane:"broken-sweep",mode:"root",base,build:true},{store:machine.store});
  return {root:receipt!.snapshotRoot,digest:receipt!.digest};
 };
 const sweepPorts:BrokenSweepPortsV1={snapshot,notice:write,languages,environment,signal:ports.signal??control.signal,...ports.sweep};
 try{
  const request={root,origin:machine.origin,store:machine.store};
  if(flags.includes("--watch"))await brokenWatchV1(request,sweepPorts);
  else await brokenSweepOnceV1({...request,full:flags.includes("--full")},sweepPorts);
  return 0;
 }finally{process.off("SIGINT",abort);process.off("SIGTERM",abort);}
}
