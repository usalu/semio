/** 🧪️ Tests for the budget sweep of the cache ledger. */
import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import budgetPolicy from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {CACHE_RUNTIME_V1,cacheAreaBudgetBytesV1,cacheLedgerV1,type CacheSizeLedgerV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {cacheArgumentsV1} from "../../⌨️command/🟦️.ts";
import {launchBudgetSweepIfDueV1,type GraphCollectorPortsV1} from "../../🚀️launch/🟦️.ts";
import {budgetSweepAreasV1,budgetSweepDecisionV1,overBudgetAreasV1,type BudgetSweepDecisionV1} from "../🟦️.ts";

const ledger=cacheLedgerV1(),GIB=1024**3,MINUTE=60000,runtime=CACHE_RUNTIME_V1;
const seeded=(seed:number)=>{let state=seed>>>0;return()=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return state/4294967296;};};
const python=`
import json,sys
v=json.load(sys.stdin);out=[]
for case in v["cases"]:
    age=float("inf") if case["last"] is None else case["now"]-case["last"]
    over=[a["id"] for a in v["areas"] if a["collect"] and a["budget"] is not None and a["id"] in case["records"] and case["records"][a["id"]]>a["budget"]]
    every=[a["id"] for a in v["areas"] if a["collect"]]+["retired","undeclared"]
    if age>=v["sweep"]: out.append({"decision":"launch","reason":"interval","areas":every})
    elif over and age>=v["over"]: out.append({"decision":"launch","reason":"over-budget","areas":over})
    else: out.append({"decision":"throttled" if over else "enough","reason":"none","areas":[]})
print(json.dumps(out))
`;
const areas=ledger.areas.map(area=>({id:area.id,collect:area.collect!==undefined,budget:cacheAreaBudgetBytesV1(ledger,area)}));

test("the sweep rule equals an independent Python derivation on seeded states",()=>{
 const random=seeded(20261026),cases:Array<{now:number;last:number|null;records:Record<string,number>}>=[],sizes:CacheSizeLedgerV1[]=[];
 for(let round=0;round<300;round++){
  const now=2_000_000_000,records:Record<string,number>={};
  for(const area of areas)if(random()<0.6)records[area.id]=Math.floor((area.budget===null?GIB:area.budget)*(0.2+random()*1.5));
  cases.push({now,last:random()<0.15?null:now-Math.floor(random()<0.4?random()*runtime.overBudgetIntervalMs*1.5:random()*2*runtime.sweepIntervalMs),records});
  sizes.push({version:1,records:Object.fromEntries(Object.entries(records).map(([id,usedBytes])=>[id,{usedBytes,partial:false,atMs:now}]))});
 }
 const run=spawnSync(testPythonV1()??"python3",["-I","-c",python],{input:JSON.stringify({areas,cases,sweep:runtime.sweepIntervalMs,over:runtime.overBudgetIntervalMs}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 const expected=JSON.parse(run.stdout) as BudgetSweepDecisionV1[];
 cases.forEach((row,index)=>expect(budgetSweepDecisionV1({ledger,sizes:sizes[index]!,nowMs:row.now,lastLaunchMs:row.last}),"case "+index).toEqual(expected[index]!));
 expect(expected.filter(row=>row.reason==="over-budget").length).toBeGreaterThan(10);expect(expected.filter(row=>row.reason==="interval").length).toBeGreaterThan(10);expect(expected.filter(row=>row.decision==="throttled").length).toBeGreaterThan(10);
});

test("the areas of a sweep are valid collectable ids of the command, never owner-managed, and an excess of a compile cache is found",()=>{
 const ids=budgetSweepAreasV1(ledger);
 expect(()=>cacheArgumentsV1(["gc","--area="+ids.join(",")],ledger)).not.toThrow();
 for(const id of ids.filter(id=>id!=="retired"&&id!=="undeclared")){const area=ledger.areas.find(row=>row.id===id)!;expect(area.collect,id).toBeDefined();expect(area.managed,id).toBeUndefined();}
 expect(ids.slice(-2)).toEqual(["retired","undeclared"]);expect(ids).toContain("compile-cache-rust");expect(ids).not.toContain("unit-cache");
 const budget=cacheAreaBudgetBytesV1(ledger,ledger.areas.find(row=>row.id==="compile-cache-rust")!)!;
 const sizes:CacheSizeLedgerV1={version:1,records:{"compile-cache-rust":{usedBytes:budget+1,partial:false,atMs:1},"go":{usedBytes:1,partial:false,atMs:1}}};
 expect(overBudgetAreasV1(ledger,sizes,budgetPolicy.partitions)).toEqual(["compile-cache-rust"]);
 expect(overBudgetAreasV1(ledger,{version:1,records:{"compile-cache-rust":{usedBytes:budget,partial:false,atMs:1}}})).toEqual([]);
});

function harness(stamp:number|null,failure?:Error){
 const calls:Array<{command:string;args:readonly string[];log:string;env:Readonly<Record<string,string>>}>=[],reports:string[]=[],written:string[]=[],stamps:string[]=[];
 const ports:Partial<GraphCollectorPortsV1>={now:()=>10_000_000_000,readStamp:async()=>stamp,writeStamp:async path=>{stamps.push(path);},start:(command,args,options)=>{if(failure)throw failure;calls.push({command,args,log:options.log,env:options.env});},writeStatus:async(_,text)=>{written.push(text);},report:line=>{reports.push(line);},environment:{PATH:"/bin",SEMIO_BUILD_INVOCATION:"leak"},execPath:"/bun",script:"/script.ts"};
 return{ports,calls,reports,written,stamps};
}

test("the launcher starts one detached gc of the due areas, honours the stamp, strips the caller's envelope and reports a failed start in both languages",async()=>{
 const none:CacheSizeLedgerV1={version:1,records:{}},first=harness(null);
 expect(await launchBudgetSweepIfDueV1("/w",first.ports,ledger,none)).toBe("launch");
 expect(first.calls.length).toBe(1);expect(first.calls[0]!.args).toEqual(["/script.ts","repo","cache","gc","--area="+budgetSweepAreasV1(ledger).join(",")]);
 expect(first.calls[0]!.env.SEMIO_BUILD_INVOCATION).toBeUndefined();expect(first.calls[0]!.env.SEMIO_CACHE_BACKGROUND).toBe("1");expect(first.stamps.length).toBe(1);
 const recent=harness(10_000_000_000-MINUTE);expect(await launchBudgetSweepIfDueV1("/w",recent.ports,ledger,none)).toBe("enough");expect(recent.calls).toEqual([]);expect(recent.stamps).toEqual([]);
 const budget=cacheAreaBudgetBytesV1(ledger,ledger.areas.find(row=>row.id==="compile-cache-rust")!)!,over:CacheSizeLedgerV1={version:1,records:{"compile-cache-rust":{usedBytes:budget*2,partial:false,atMs:1}}};
 const soon=harness(10_000_000_000-MINUTE);expect(await launchBudgetSweepIfDueV1("/w",soon.ports,ledger,over)).toBe("throttled");expect(soon.calls).toEqual([]);
 const later=harness(10_000_000_000-runtime.overBudgetIntervalMs);expect(await launchBudgetSweepIfDueV1("/w",later.ports,ledger,over)).toBe("launch");expect(later.calls[0]!.args.at(-1)).toBe("--area=compile-cache-rust");
 const broken=harness(null,new Error("spawn denied"));expect(await launchBudgetSweepIfDueV1("/w",broken.ports,ledger,none)).toBe("failed");
 expect(broken.reports.join("\n")).toContain("The budget cache sweep could not be started: spawn denied");expect(broken.reports.join("\n")).toContain("Der Budget-Cache-Sammler konnte nicht gestartet werden: spawn denied");
 expect(JSON.parse(broken.written[0]!)).toMatchObject({version:1,state:"failed",message:"spawn denied"});
});

test("every heavy command checks the sweep right after the products collector and the intervals are authored",()=>{
 const text=readFileSync(new URL("../../../⌨️command/⚙️runtime/🟦️.ts",import.meta.url),"utf8");
 expect(text).toMatch(/profilePhaseV1\("products\.collect"[^\n]*\n\s*await profilePhaseV1\("budget\.sweep",async\(\)=>\{await\(await import\("\.\.\/\.\.\/🧹️cache-gc\/🚀️launch\/🟦️\.ts"\)\)\.launchBudgetSweepIfDueV1\(root\);\}\);/u);
 expect(runtime.sweepIntervalMs).toBeGreaterThan(runtime.overBudgetIntervalMs);expect(runtime.overBudgetIntervalMs).toBeGreaterThanOrEqual(MINUTE);
});
