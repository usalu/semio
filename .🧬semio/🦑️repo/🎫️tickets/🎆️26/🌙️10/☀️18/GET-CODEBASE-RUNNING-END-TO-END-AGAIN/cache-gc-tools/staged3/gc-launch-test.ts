import {afterAll,beforeAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {cacheLedgerV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {GRAPH_COLLECTOR_INTERVAL_MS_V1,graphCollectorDecisionV1,graphCollectorEnvironmentV1,launchGraphOptionalCollectorV1,type GraphCollectorPortsV1} from "../../🚀️launch/🟦️.ts";
import {reclaimGraphOptionalV1,type GraphReclaimPortsV1} from "../../🚰️reclaim/🟦️.ts";

const GIB=1024**3,ledger=cacheLedgerV1(),reclaim=ledger.areas.find(area=>area.id==="graph-inference")!.reclaim!;
let workspace="";
beforeAll(()=>{workspace=mkdtempSync(join(tmpdir(),"cgl-"));});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});

test("the decision is enough above the soft threshold, the writer's own last resort below the floor, and one launch per interval in between",()=>{
 const base={softBytes:reclaim.softAvailableBytes,floorBytes:GIB,nowMs:10_000_000,lastLaunchMs:null,minimumIntervalMs:GRAPH_COLLECTOR_INTERVAL_MS_V1};
 expect(graphCollectorDecisionV1({...base,availableBytes:reclaim.softAvailableBytes})).toBe("enough");
 expect(graphCollectorDecisionV1({...base,availableBytes:reclaim.softAvailableBytes*4})).toBe("enough");
 expect(graphCollectorDecisionV1({...base,availableBytes:GIB-1})).toBe("inline-floor");
 expect(graphCollectorDecisionV1({...base,availableBytes:0})).toBe("inline-floor");
 expect(graphCollectorDecisionV1({...base,availableBytes:GIB})).toBe("launch");
 expect(graphCollectorDecisionV1({...base,availableBytes:reclaim.softAvailableBytes-1})).toBe("launch");
 expect(graphCollectorDecisionV1({...base,availableBytes:GIB*1.5,lastLaunchMs:base.nowMs-1000})).toBe("throttled");
 expect(graphCollectorDecisionV1({...base,availableBytes:GIB*1.5,lastLaunchMs:base.nowMs-GRAPH_COLLECTOR_INTERVAL_MS_V1})).toBe("launch");
});

test("the soft threshold lies between the hard floor and the target, and the target fits the headroom schema",()=>{
 expect(reclaim.softAvailableBytes).toBeGreaterThan(GIB);
 expect(reclaim.targetAvailableBytes).toBeGreaterThan(reclaim.softAvailableBytes);
 expect(reclaim.targetAvailableBytes).toBeLessThanOrEqual(2684354560);
});

test("the detached collector starts with a clean environment, never inherits the writer's invocation envelope",()=>{
 const env=graphCollectorEnvironmentV1({PATH:"/bin",HOME:"/h",LANG:"de_DE.UTF-8",SEMIO_SCRIPT_PROCESS_INVOCATION:"{}",SEMIO_SCRIPT_POLICY:"{}",SEMIO_BUILD_GRAPH_PATH:"/g",SEMIO_BUILD_INVOCATION_PID:"9",SECRET_TOKEN:"x"});
 expect(env).toEqual({SEMIO_CACHE_BACKGROUND:"1",PATH:"/bin",HOME:"/h",LANG:"de_DE.UTF-8"});
});

type Started=Array<{command:string;args:readonly string[];cwd:string;env:Readonly<Record<string,string>>;log:string}>;
function ports(state:{now:number;stamp:number|null;started:Started;fail?:boolean;statuses?:string[];reports?:string[];failStatus?:boolean}):Partial<GraphCollectorPortsV1>{
 return{now:()=>state.now,readStamp:async()=>state.stamp,writeStamp:async()=>{if(state.fail)throw Error("disk full");state.stamp=state.now;},start:(command,args,options)=>{if(state.fail)throw Error("no process");state.started.push({command,args:[...args],...options});},writeStatus:async(_path,text)=>{if(state.failStatus)throw Error("status volume gone");state.statuses?.push(text);},report:line=>{state.reports?.push(line);},environment:{PATH:"/bin",SEMIO_SCRIPT_PROCESS_INVOCATION:"{}"},execPath:"/bun",script:"/s/script.ts"};
}

test("a writer below the soft threshold launches the collector once per interval, a writer above it or below the floor launches nothing, and a failing launch never fails the writer",async()=>{
 const state={now:50_000_000,stamp:null as number|null,started:[] as Started};
 expect(await launchGraphOptionalCollectorV1(workspace,3*GIB,ports(state),ledger)).toBe("enough");
 expect(await launchGraphOptionalCollectorV1(workspace,GIB/2,ports(state),ledger)).toBe("inline-floor");
 expect(state.started).toEqual([]);
 expect(await launchGraphOptionalCollectorV1(workspace,1.5*GIB,ports(state),ledger)).toBe("launch");
 expect(state.started).toEqual([{command:"/bun",args:["/s/script.ts","repo","cache","gc","--area=graph-inference"],cwd:workspace,env:{SEMIO_CACHE_BACKGROUND:"1",PATH:"/bin"},log:join(workspace,".🧬semio","🦑️repo","⚡️cache","build","graph-collector.log")}]);
 expect(state.stamp).toBe(state.now);
 expect(await launchGraphOptionalCollectorV1(workspace,1.5*GIB,ports(state),ledger)).toBe("throttled");
 state.now+=GRAPH_COLLECTOR_INTERVAL_MS_V1;
 expect(await launchGraphOptionalCollectorV1(workspace,1.5*GIB,ports(state),ledger)).toBe("launch");
 expect(state.started.length).toBe(2);
 const broken={now:1,stamp:null as number|null,started:[] as Started,fail:true,statuses:[] as string[],reports:[] as string[]};
 expect(await launchGraphOptionalCollectorV1(workspace,1.5*GIB,ports(broken),ledger)).toBe("failed");
 expect(JSON.parse(broken.statuses[0]!)).toEqual({version:1,atMs:1,state:"failed",message:"disk full"});
 expect(broken.reports).toEqual(["The graph cache collector could not be started: disk full\nDer Graph-Cache-Sammler konnte nicht gestartet werden: disk full"]);
 const blind={now:1,stamp:null as number|null,started:[] as Started,fail:true,failStatus:true,statuses:[] as string[],reports:[] as string[]};
 expect(await launchGraphOptionalCollectorV1(workspace,1.5*GIB,ports(blind),ledger)).toBe("failed");
 expect(blind.reports[0]).toBe("status: status volume gone");expect(blind.reports.length).toBe(2);
 const withoutArea={...ledger,areas:ledger.areas.filter(area=>area.id!=="graph-inference")};
 expect(await launchGraphOptionalCollectorV1(workspace,GIB,ports(state),withoutArea)).toBe("enough");
});

test("the stamp lives in the declared build area and is a real file after a real launch decision",async()=>{
 let started=0;
 const decision=await launchGraphOptionalCollectorV1(workspace,1.5*GIB,{start:()=>{started++;},execPath:"/bun",script:"/s",environment:{}},ledger);
 expect(decision).toBe("launch");
 expect(started).toBe(1);
 const stamp=join(workspace,".🧬semio","🦑️repo","⚡️cache","build","graph-collector.stamp");
 expect(existsSync(stamp)).toBe(true);
 expect(Date.now()-statSync(stamp).mtimeMs).toBeLessThan(60000);
 expect(await launchGraphOptionalCollectorV1(workspace,1.5*GIB,{start:()=>{started++;},execPath:"/bun",script:"/s",environment:{}},ledger)).toBe("throttled");
 expect(started).toBe(1);
});

test("the real detached start writes the collector's output to the declared log and records a program that cannot start there",async()=>{
 const script=join(workspace,"collector.js"),build=join(workspace,".🧬semio","🦑️repo","⚡️cache","build","graph-collector.log"),wait=async(text:string)=>{for(let tries=0;tries<100&&!(existsSync(build)&&readFileSync(build,"utf8").includes(text));tries++)await new Promise(resolve=>setTimeout(resolve,100));return readFileSync(build,"utf8");};
 writeFileSync(script,'console.log("collected "+process.argv.slice(2).join(" "));');
 const ports={now:()=>1,readStamp:async()=>null,writeStamp:async()=>{},environment:{PATH:process.env.PATH},report:()=>{},writeStatus:async()=>{}};
 expect(await launchGraphOptionalCollectorV1(workspace,1.2*GIB,{...ports,execPath:process.execPath,script},ledger)).toBe("launch");
 expect(await wait("collected")).toContain("collected repo cache gc --area=graph-inference");
 expect(await launchGraphOptionalCollectorV1(workspace,1.2*GIB,{...ports,execPath:join(workspace,"no-such-program"),script},ledger)).toBe("launch");
 expect(await wait("launch failed")).toContain("launch failed: ");
});

function fake(free:number[],log:string[],options:{lease?:boolean;failOptional?:boolean}={}):GraphReclaimPortsV1{
 let index=0;
 return{
  exclusive:async resource=>{log.push("lease:"+resource);return options.lease===false?undefined:{release:()=>{log.push("release");}};},
  open:async()=>{log.push("open");return{close:async()=>{log.push("close");}};},
  free:async()=>{log.push("free");return free[Math.min(index++,free.length-1)]!;},
  selected:async(_image,request)=>{log.push("selected:"+request.minimumAvailableBytes+":"+request.wait);},
  optional:async(_image,request)=>{log.push("optional:"+request.minimumAvailableBytes);if(options.failOptional)throw Error("scope busy");}
 };
}

test("the reclaim takes a single-flight lease, uses only the two shared scoped recoveries, stops as soon as the target is met and always lets go",async()=>{
 const target=reclaim.targetAvailableBytes,log:string[]=[];
 const done=await reclaimGraphOptionalV1("/w",ledger,false,"en",fake([GIB,GIB,target+GIB],log));
 expect(log).toEqual(["lease:repository-graph-optional-collector:/w","open","free","selected:"+target+":false","free","optional:"+target,"free","close","release"]);
 expect(done).toBe("Reclaimed 2.00 GiB of optional graph caches; 3.00 GiB free, target 2.00 GiB");
 const sparing:string[]=[];
 await reclaimGraphOptionalV1("/w",ledger,false,"de",fake([GIB,target],sparing));
 expect(sparing).toEqual(["lease:repository-graph-optional-collector:/w","open","free","selected:"+target+":false","free","free","close","release"]);
 expect(sparing.some(line=>line.startsWith("optional:"))).toBe(false);
 const rich:string[]=[];
 expect(await reclaimGraphOptionalV1("/w",ledger,false,"en",fake([target+1],rich))).toBe("The graph image has 2.00 GiB free, at least the target of 2.00 GiB; nothing to reclaim");
 expect(rich.some(line=>line.startsWith("selected:")||line.startsWith("optional:"))).toBe(false);
 const planned:string[]=[];
 expect(await reclaimGraphOptionalV1("/w",ledger,true,"de",fake([GIB],planned))).toBe("Das Graph-Image hat 1.00 GiB frei bei einem Ziel von 2.00 GiB; würde optionale Caches zurückgewinnen");
 expect(planned.some(line=>line.startsWith("selected:")||line.startsWith("optional:"))).toBe(false);
 const taken:string[]=[];
 expect(await reclaimGraphOptionalV1("/w",ledger,false,"en",fake([GIB],taken,{lease:false}))).toBe("Another graph cache collector is running");
 expect(taken).toEqual(["lease:repository-graph-optional-collector:/w"]);
 const broken:string[]=[];
 await expect(reclaimGraphOptionalV1("/w",ledger,false,"en",fake([GIB],broken,{failOptional:true}))).rejects.toThrow("scope busy");
 expect(broken.slice(-2)).toEqual(["close","release"]);
 expect(await reclaimGraphOptionalV1("/w",{...ledger,areas:ledger.areas.filter(area=>area.id!=="graph-inference")},false,"en",fake([GIB],[]))).toBe("No reclaim threshold is declared for the graph image");
});
