import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {buildLeaseReportV1} from "../../📋️status/🟦️.ts";

const resource="semio.compiler.host-capacity/v1",key=createHash("sha256").update(resource).digest("hex");
const entry=(pid:number)=>({version:1 as const,pid,startedAtMs:0,updatedAtMs:0,command:"c"+pid,directory:"/d",standing:{version:1 as const,priority:"agent" as const,implementation:{digest:"0123456789abcdef",loadedAtMs:0,components:{}},progressAtMs:0},holds:[{resource,mode:"shared" as const,sinceMs:1000,task:pid}],waits:[]});
const ticket=(pid:number,position:number,mode:"shared"|"exclusive")=>({hash:key,position,queued:5,arrivedAtMs:1000+position,pid,mode});

test("a holder's own arrival ticket behind an exclusive head is neither a wait nor a deadlock cycle",()=>{
 const report=buildLeaseReportV1({directory:"/d",atMs:5000,entries:[entry(1),entry(2),entry(3)],queues:[ticket(9,1,"exclusive"),ticket(1,2,"shared"),ticket(2,3,"shared"),ticket(3,4,"shared")],violations:[]});
 expect(report.waiters.filter(row=>[1,2,3].includes(row.pid))).toEqual([]);
 expect(report.cycles).toEqual([]);
 expect(report.holders.filter(row=>row.pid!==9).map(row=>row.pid).sort()).toEqual([1,2,3]);
});

test("a process that only queues shared behind an exclusive head still waits for it",()=>{
 const report=buildLeaseReportV1({directory:"/d",atMs:5000,entries:[entry(1)],queues:[ticket(9,1,"exclusive"),ticket(7,2,"shared"),ticket(1,3,"shared")],violations:[]});
 expect(report.waiters.map(row=>row.pid)).toEqual([7]);
 expect(report.waiters[0]!.blockers.length).toBeGreaterThan(0);
});
