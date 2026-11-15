/** 🧽️ Contract: two plans answered by one service process leave the managed heap where the first one left it. Each plan receives a large transient slice graph (as the worker message of a real load); the service settles after the answer (collect, shrink, one bilingual line), so the heap after plan 2 stays within a tolerance of the heap after plan 1 and far below the plan's own footprint.
 * @see ../../🏛️service/🌐️host/🟨️.mjs */
import {afterAll,expect,test} from "bun:test";
import {copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {HERE} from "../🟦️.ts";
import {createHostServiceV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {requestPlanV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {settleGraphMemoryV1} from "../../../🧠️memory/🧽️settle/🟨️.mjs";

const MB=1048576,GRAPH=join(HERE,"🧫️fixtures/🛰️host-graph.json");
const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","pa-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const heapMb=()=>{Bun.gc(true);return process.memoryUsage().heapUsed/MB;};

test("plan 2 starts and ends at about the managed size of plan 1; the line names managed, resident and released",async()=>{
 const dir=join(base,"run");mkdirSync(dir,{recursive:true});
 const env={...process.env,XDG_RUNTIME_DIR:dir,TMPDIR:dir,LOCALAPPDATA:dir} as Record<string,string>,implementation=createHash("sha256").update("impl-answered").digest("hex"),runtime=hostEndpointsV1(implementation,env);
 const root=join(base,"w");mkdirSync(root,{recursive:true});copyFileSync(GRAPH,join(root,"graph.json"));const workspace=realpathSync(root);
 const inner=createGraphPlannerPortV1({identify:async()=>"k".repeat(64),loadGraph:async()=>JSON.parse(readFileSync(GRAPH,"utf8"))});
 let footprint=0;const planner={...inner,plan:async(request:any)=>{const slice=Array.from({length:3_000_000},(_,at)=>({at,text:"slice-graph-record-"+at}));footprint=Math.max(footprint,process.memoryUsage().heapUsed/MB);const answer=await inner.plan(request);return slice.length>0?answer:answer;}};
 const lines:string[]=[];
 const settle=()=>settleGraphMemoryV1({force:true,environment:{SEMIO_LANGUAGE:"en"},report:line=>lines.push(line)});
 const service=createHostServiceV1({runtime,implementation,ports:{planner,settle},policy:{residentCheckMilliseconds:50}});
 const ask=()=>requestPlanV1({root:workspace,implementation,targets:["app:test"],agent:"agent-1",language:"en",environment:env,spawnService:async()=>{await service.start();}});
 const start=heapMb();
 await ask();await Bun.sleep(300);const afterOne=heapMb();
 await ask();await Bun.sleep(300);const afterTwo=heapMb();
 console.log("[R2 measure] start "+start.toFixed(1)+" MB, plan footprint "+footprint.toFixed(1)+" MB, after plan 1 "+afterOne.toFixed(1)+" MB, after plan 2 "+afterTwo.toFixed(1)+" MB");
 expect(footprint-start).toBeGreaterThan(80);
 expect(afterOne-start).toBeLessThan(40);
 expect(Math.abs(afterTwo-afterOne)).toBeLessThan(25);
 expect(lines.length).toBe(2);
 expect(lines[1]).toMatch(/^graph: plan answered, managed \d+ MB, resident \d+ MB \(released \d+ MB\)$/u);
 await service.stop("test");
},60000);
