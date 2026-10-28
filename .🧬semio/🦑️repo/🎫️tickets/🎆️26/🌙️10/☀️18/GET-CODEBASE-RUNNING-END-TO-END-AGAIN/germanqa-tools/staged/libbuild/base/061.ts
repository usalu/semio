import {expect,test} from "bun:test";
import {cacheLedgerV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import type {MemberVerifyReportV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts";
import {cacheRunnerPortsV1} from "../../🏃️runner/🟦️.ts";
import {cacheArgumentsV1,runCacheLedgerCommandV1} from "../../⌨️command/🟦️.ts";
import {reclaimGraphOptionalV1,type GraphReclaimPortsV1} from "../../🚰️reclaim/🟦️.ts";

const GIB=1024**3,ledger=cacheLedgerV1(),report=(overrides:Partial<MemberVerifyReportV1>={}):MemberVerifyReportV1=>({groups:2,verified:10,bytes:2048,mismatched:[],mismatches:0,stale:1,unindexed:2,vanished:0,closures:3,closureMismatched:[],closureMismatches:0,cancelled:false,...overrides});
const ports=(controller=new AbortController(),stages:string[]=[])=>cacheRunnerPortsV1({signal:controller.signal,progress:async stage=>{stages.push(stage);},exclusive:async()=>undefined});

test("verify is read-only, needs --deep and accepts only --json and --lang, in both languages",()=>{
 expect(cacheArgumentsV1(["verify","--deep"],ledger)).toMatchObject({operation:"verify",deep:true,dryRun:false,json:false});
 expect(cacheArgumentsV1(["verify","--deep","--json","--lang=de"],ledger)).toMatchObject({operation:"verify",json:true,language:"de"});
 for(const args of[["verify"],["verify","--json"],["verify","--deep","--dry-run"],["verify","--deep","--area=graph-inference"],["gc","--deep"],["verify","--deep","--deep"]]){
  const thrown=(()=>{try{cacheArgumentsV1(args,ledger);}catch(error){return String((error as Error).message);}return"";})();
  expect(thrown,args.join(" ")).toContain("\n");expect(thrown.length,args.join(" ")).toBeGreaterThan(0);
 }
 expect(()=>cacheArgumentsV1(["verify"],ledger)).toThrow("cache verify needs --deep");expect(()=>cacheArgumentsV1(["verify"],ledger)).toThrow("cache verify braucht --deep");
});

test("verify prints what differs and a summary in the caller's language, exits 1 on a difference and 0 on none, and reports progress",async()=>{
 const stages:string[]=[],lines:string[]=[];
 const clean=await runCacheLedgerCommandV1(["verify","--deep","--lang=en"],{workspace:"/w",environment:{},write:line=>lines.push(line),verify:async control=>{await control.progress("selections/x",0,2);await control.progress("selections/y",1,2);return report();}},ports(new AbortController(),stages));
 expect(clean).toBe(0);expect(lines).toEqual(["Verifying 2 graph groups deeply (every indexed leaf is read and hashed again)","Verified 10 leaves (2.00 KiB) in 2 groups and 3 closure inventories: 0 differ, 0 closure inventories differ; 1 stale, 2 unindexed, 0 vanished"]);
 expect(stages).toEqual(["Verifying selections/x (0/2)","Verifying selections/y (1/2)"]);
 const german:string[]=[],code=await runCacheLedgerCommandV1(["verify","--deep","--lang=de"],{workspace:"/w",environment:{},write:line=>german.push(line),verify:async()=>report({mismatches:1,mismatched:["selections/x/np/a.json"],closureMismatches:1,closureMismatched:["build-input-closures/b.json"]})},ports());
 expect(code).toBe(1);expect(german).toEqual(["Hash weicht vom Index ab: selections/x/np/a.json","Closure-Inventar weicht von seinem Namen ab: build-input-closures/b.json","10 Blätter (2.00 KiB) in 2 Gruppen und 3 Closure-Inventare geprüft: 1 weichen ab, 1 Closure-Inventare weichen ab; 1 veraltet, 2 nicht indexiert, 0 verschwunden"]);
 const json:string[]=[];await runCacheLedgerCommandV1(["verify","--deep","--json"],{workspace:"/w",environment:{},write:line=>json.push(line),verify:async()=>report()},ports());
 expect(JSON.parse(json.join("\n"))).toMatchObject({version:1,groups:2,verified:10,mismatches:0});expect(json.length).toBe(1);
});

test("a cancelled verification says so and exits 130, and a host without the deep check says it is unavailable",async()=>{
 const lines:string[]=[],controller=new AbortController(),code=await runCacheLedgerCommandV1(["verify","--deep","--lang=en"],{workspace:"/w",environment:{},write:line=>lines.push(line),verify:async control=>{controller.abort();expect(control.cancelled()).toBe(true);return report({cancelled:true});}},ports(controller));
 expect(code).toBe(130);expect(lines).toEqual(["Cache collection cancelled"]);
 const none:string[]=[];expect(await runCacheLedgerCommandV1(["verify","--deep","--lang=de"],{workspace:"/w",environment:{},write:line=>none.push(line)},ports())).toBe(1);expect(none).toEqual(["Tiefenprüfung ist hier nicht verfügbar"]);
});

function fake(free:number[],log:string[],index?:GraphReclaimPortsV1["index"]):GraphReclaimPortsV1{
 let at=0;
 return{exclusive:async()=>({release:()=>{log.push("release");}}),open:async()=>({close:async()=>{log.push("close");}}),...(index?{index:async image=>{log.push("index");return index(image);}}:{}),free:async()=>free[Math.min(at++,free.length-1)]!,selected:async()=>{log.push("selected");},optional:async()=>{log.push("optional");}};
}
const target=ledger.areas.find(area=>area.id==="graph-inference")!.reclaim!.targetAvailableBytes;

test("the reclaim indexes first and reports it, even when there is nothing to reclaim, and never indexes in a dry run",async()=>{
 const log:string[]=[],work={groups:3,indexed:12,busy:1,refused:2,compacted:1,cancelled:false};
 expect(await reclaimGraphOptionalV1("/w",ledger,false,"en",fake([target+1],log,async()=>work))).toBe("Indexed 12 optional graph leaves in 3 groups (1 groups busy, 2 leaves refused, 1 indexes compacted)\nThe graph image has 2.00 GiB free, at least the target of 2.00 GiB; nothing to reclaim");
 expect(log).toEqual(["index","close","release"]);
 const planned:string[]=[];expect(await reclaimGraphOptionalV1("/w",ledger,true,"de",fake([GIB],planned,async()=>work))).toBe("Das Graph-Image hat 1.00 GiB frei bei einem Ziel von 2.00 GiB; würde optionale Caches zurückgewinnen");expect(planned).not.toContain("index");
 const quiet:string[]=[];expect(await reclaimGraphOptionalV1("/w",ledger,false,"en",fake([target+1],quiet,async()=>({groups:3,indexed:0,busy:0,refused:0,compacted:0,cancelled:false})))).toBe("The graph image has 2.00 GiB free, at least the target of 2.00 GiB; nothing to reclaim");
 const cancelled:string[]=[];expect(await reclaimGraphOptionalV1("/w",ledger,false,"en",fake([GIB],cancelled,async()=>({...work,cancelled:true})))).toBe("Indexed 12 optional graph leaves in 3 groups (1 groups busy, 2 leaves refused, 1 indexes compacted)\nCache collection cancelled");expect(cancelled).not.toContain("selected");
});
