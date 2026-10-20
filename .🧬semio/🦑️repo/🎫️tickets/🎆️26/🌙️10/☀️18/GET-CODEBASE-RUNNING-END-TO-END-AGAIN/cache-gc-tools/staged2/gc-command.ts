import {cacheLedgerV1,cacheTextV1,formatCacheReportV1,resolveCacheLanguageV1,type CacheLanguageV1,type CacheLedgerV1,type CachePartitionBudgetsV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {cacheRunnerPortsV1,collectCacheV1,reportCacheLedgerV1,type CacheRunnerPortsV1} from "../🏃️runner/🟦️.ts";

/** 📝️ The closed arguments of `repo cache status|gc`. */
export type CacheArgumentsV1=Readonly<{operation:"status"|"gc";json:boolean;deep:boolean;dryRun:boolean;only:ReadonlySet<string>|undefined;language:string|undefined}>;
const both=(key:string,variables:Readonly<Record<string,string|number>>={}):string=>cacheTextV1("en",key,variables)+"\n"+cacheTextV1("de",key,variables);
const valued=Object.freeze(["--area","--lang"]),plain=Object.freeze(["--json","--dry-run","--deep"]);

/** 🧷️ Reads `status|gc` with its options; every unknown or repeated option is refused in both languages. */
export function cacheArgumentsV1(args:readonly string[],ledger:CacheLedgerV1=cacheLedgerV1()):CacheArgumentsV1{
 const operation=args[0];if(operation!=="status"&&operation!=="gc")throw Error(both("error.syntax"));
 const seen=new Set<string>(),values:Record<string,string>={};let json=false,dryRun=false,deep=false;
 for(let index=1;index<args.length;index++){
  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);
  if(seen.has(name))throw Error(both("error.repeated",{option:name}));
  seen.add(name);
  if(valued.includes(name)){const value=cut<0?args[++index]:argument.slice(cut+1);if(!value||value.startsWith("--"))throw Error(both("error.value",{option:name}));values[name]=value;}
  else if(plain.includes(name)&&cut<0&&(name!=="--dry-run"||operation==="gc")&&(name!=="--deep"||operation==="status")){if(name==="--json")json=true;else if(name==="--deep")deep=true;else dryRun=true;}
  else throw Error(both("error.option",{option:argument}));
 }
 const ids=new Set([...ledger.areas.map(area=>area.id),"retired","undeclared"]),only=values["--area"]?new Set(values["--area"].split(",").filter(Boolean)):undefined;
 for(const id of only??[])if(!ids.has(id))throw Error(both("error.area",{id}));
 return Object.freeze({operation,json,deep,dryRun,only,language:values["--lang"]});
}

/** 🎛️ What the command needs besides the runner ports: the workspace, the environment, the output and the ledger. */
export type CacheCommandContextV1=Readonly<{workspace:string;environment:Readonly<Record<string,string|undefined>>;write(line:string):void;ledger?:CacheLedgerV1;partitions?:CachePartitionBudgetsV1}>;

/** 🚀️ Runs `repo cache status|gc`: status reads the ledger without any lease and without walking a tree (statfs of the partitions plus the size ledger; `--deep` walks everything with progress and cancellation and records the sizes); gc collects retired roots, undeclared entries and over-budget areas under the lease-aware runner. Returns the process exit code. */
export async function runCacheLedgerCommandV1(args:readonly string[],context:CacheCommandContextV1,ports:CacheRunnerPortsV1):Promise<number>{
 const ledger=context.ledger??cacheLedgerV1(),parsed=cacheArgumentsV1(args,ledger),language:CacheLanguageV1=resolveCacheLanguageV1(parsed.language,context.environment);
 try{
  if(parsed.operation==="status"){
   const report=await reportCacheLedgerV1(context.workspace,ledger,ports,context.partitions,parsed.only,{deep:parsed.deep});
   if(parsed.json)context.write(JSON.stringify(report,null,2));else for(const line of formatCacheReportV1(language,report,ledger))context.write(line);
   return 0;
  }
  const result=await collectCacheV1(context.workspace,ledger,{dryRun:parsed.dryRun,only:parsed.only},ports,{language,write:parsed.json?()=>{}:context.write},context.partitions);
  if(parsed.json)context.write(JSON.stringify({version:1,dryRun:result.dryRun,freedBytes:result.freedBytes,kept:result.kept,removed:result.removed,failed:result.failed,delegated:result.delegated,plans:result.plans},null,2));
  else for(const failure of result.failed)context.write(failure.target+"/"+failure.name+": "+failure.message);
  return result.failed.length?1:0;
 }catch(error){
  if(ports.signal.aborted){context.write(cacheTextV1(language,"error.cancelled"));return 130;}
  throw error;
 }
}

/** 🏗️ Binds the command to the running build CLI: its invocation for progress and cancellation, and the lease directory of the leases partition for the area leases. */
export async function runCacheLedgerV1(root:string,args:readonly string[],invocation:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts").ResourceLeaseControl):Promise<void>{
 const routing=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),leases=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts"),opening=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts"),directory=opening.repositoryLeaseDirectoryV1(root),delegated=new Map<string,Promise<string|undefined>>();
 const ports=cacheRunnerPortsV1({
  signal:invocation.control.signal,
  progress:stage=>routing.advanceScriptInvocation(invocation,"cache:"+stage,"running"),
  exclusive:async resource=>{
   try{return await leases.acquireResourceLease({directory,resource,mode:"exclusive",signal:phaseControl.signal,control:phaseControl,timeoutMs:0});}
   catch(error){if(/timed out/iu.test(String((error as Error).message)))return undefined;throw error;}
  },
  delegate:(owner,_id,dryRun)=>{
   if(owner!=="test-evidence")return Promise.resolve(undefined);
   if(!delegated.has(owner))delegated.set(owner,import("../../../⚡️caching/🧹️pruning/🌐️workspace/🟦️.ts").then(module=>module.pruneTestEvidence(root,dryRun)).catch(error=>"failed: "+String((error as Error).message)));
   return delegated.get(owner)!;
  }
 });
 const code=await runCacheLedgerCommandV1(args,{workspace:root,environment:process.env,write:line=>console.log(line)},ports);
 if(code!==0)process.exitCode=code;
}
