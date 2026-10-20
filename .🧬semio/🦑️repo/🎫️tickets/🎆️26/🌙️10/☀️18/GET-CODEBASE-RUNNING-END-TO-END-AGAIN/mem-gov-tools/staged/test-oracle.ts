import {Database} from "bun:sqlite";
import type {HostCapacityPolicyV1,HostDecisionV1,HostEvaluationV1,HostGrantV1,HostSampleV1,HostWaiterV1} from "../../📏️decision/🟦️.ts";

/** 🔮️ One fixture case: everything the pure decision reads and the answer it must give. */
export type HostCaseV1=Readonly<{name:string;note?:string;host:HostSampleV1;ledger:readonly HostGrantV1[];queue:readonly HostWaiterV1[];nowMs:number;expect:Omit<HostEvaluationV1,"decisions">&Readonly<{decisions:readonly HostDecisionV1[]}>}>;

const HARD=new Set(["pressure-critical","swap","swap-activity"]);

/** 🗄️ Evaluates a case with SQLite alone: the ordered queue is a table and the whole rule is one recursive query that carries room, load room, the queue barrier and the admission count from waiter to waiter, with closed-form job counts where the implementation loops. */
export function sqliteHostOracleV1(policy:HostCapacityPolicyV1,item:Omit<HostCaseV1,"expect">):HostCaseV1["expect"]{
 const host=item.host,share=Math.floor(host.totalMegabytes*policy.memory.floorBasisPoints/10000),floor=Math.min(host.totalMegabytes,Math.max(policy.memory.floorMinimumMegabytes,Math.min(policy.memory.floorMaximumMegabytes,share))),cap=Math.max(0,host.totalMegabytes-floor);
 const database=new Database(":memory:");
 try{
  database.run("CREATE TABLE grants(reserved INTEGER, observed INTEGER, cpu INTEGER, admitted INTEGER)");
  database.run("CREATE TABLE waiters(id TEXT, priority TEXT, arrived INTEGER, minimum INTEGER, pref INTEGER, fixed INTEGER, perjob INTEGER, cpu INTEGER)");
  const insertGrant=database.prepare("INSERT INTO grants VALUES (?,?,?,?)"),insertWaiter=database.prepare("INSERT INTO waiters VALUES (?,?,?,?,?,?,?,?)");
  for(const grant of item.ledger)insertGrant.run(grant.reservedMegabytes,grant.observedMegabytes,grant.cpu,grant.admittedAtMs);
  for(const waiter of item.queue){const r=waiter.request;insertWaiter.run(r.id,r.priority,waiter.arrivedAtMs,r.minimumJobs,r.preferredJobs,r.fixedMegabytes,r.perJobMegabytes,r.cpuPerJob);}
  const totals=database.query(`SELECT COALESCE(SUM(CASE WHEN ${item.nowMs}-admitted<${policy.memory.rampMilliseconds} THEN MAX(reserved-COALESCE(observed,0),0) ELSE 0 END),0) AS unrealized, COALESCE(SUM(CASE WHEN ${item.nowMs}-admitted<${policy.memory.rampMilliseconds} THEN reserved ELSE COALESCE(observed,reserved) END),0) AS committed, COALESCE(SUM(cpu),0) AS cpu, COUNT(*) AS n FROM grants`).get() as {unrealized:number;committed:number;cpu:number;n:number};
  const observed=host.availableMegabytes-floor-totals.unrealized,budget=Math.floor(cap*policy.memory.budgetBasisPoints/10000)-totals.committed,room=Math.min(observed,budget);
  const ceiling=policy.load.perCoreCeiling*host.cores,unlimited=host.load===null&&totals.cpu===0,load=unlimited?null:ceiling-Math.max(host.load??0,totals.cpu);
  const swapCeiling=Math.floor(host.totalMegabytes*policy.swap.ceilingBasisPoints/10000),rate=host.swapOutMegabytesPerSecond;
  const hard=host.pressure==="critical"?"pressure-critical":host.swapUsedMegabytes!==null&&host.swapUsedMegabytes>swapCeiling&&(rate===null||rate>0)?"swap":rate!==null&&rate>policy.swap.outMegabytesPerSecond?"swap-activity":null;
  if(hard!==null&&!HARD.has(hard))throw Error("Unknown hard reason");
  const filesOver=host.openFiles!==null&&host.openFilesMaximum!==null&&host.openFiles/host.openFilesMaximum>policy.files.ceilingBasisPoints/10000,imagesOver=host.attachedImages!==null&&host.attachedImages>=policy.attaches.maximum;
  const softReason=host.pressure==="warn"?"pressure-warn":filesOver?"files":imagesOver?"attaches":null,soft=softReason===null?0:1,quoted=hard===null?"NULL":`'${hard}'`,empty=totals.n===0?1:0;
  const held=item.ledger.reduce((sum,grant)=>sum+(grant.observedMegabytes??0),0),drainedRoom=Math.min(Math.min(host.availableMegabytes+held,host.totalMegabytes)-floor,Math.floor(cap*policy.memory.budgetBasisPoints/10000));
  const memoryJobs="CASE WHEN s.room>="+cap+" THEN w.pref WHEN w.perjob=0 THEN CASE WHEN w.fixed<=s.room THEN w.pref ELSE 0 END ELSE MAX(0,MIN(w.pref,(s.room-w.fixed)/w.perjob)) END",loadJobs="CASE WHEN s.load IS NULL THEN w.pref ELSE MAX(0,MIN(w.pref,CAST(s.load/w.cpu AS INTEGER))) END";
  const count=`CASE WHEN MIN(${memoryJobs},${loadJobs})>=w.minimum THEN MIN(${memoryJobs},${loadJobs}) WHEN ${memoryJobs}>=w.minimum AND s.own+w.minimum*w.cpu<=${ceiling} THEN w.minimum ELSE 0 END`;
  const fitsMinimum=`(MIN(w.fixed+w.minimum*w.perjob,${cap})<=s.room)`,overrideDue=`(${empty}=1 AND s.admitted=0 AND (${item.nowMs}-w.arrived)>=${policy.load.overrideAfterMilliseconds})`,open=`(${quoted} IS NULL AND s.blocked=0)`;
  const normal=`(${open} AND ${soft}=0 AND ${count}>=w.minimum)`,forced=`(${open} AND NOT ${normal} AND ${fitsMinimum} AND ${overrideDue})`;
  const drains=`(${soft}=0 AND MIN(w.fixed+w.minimum*w.perjob,${cap})<=${drainedRoom} AND w.minimum*w.cpu<=${ceiling})`,bypassed="(SELECT COUNT(*) FROM grants g WHERE g.admitted>=w.arrived)",limit=policy.wait.maximumBypasses;
  const waiting=`(NOT (${normal} OR ${forced}) AND ${open} AND (NOT ${fitsMinimum} OR ${soft}=0) AND ${drains})`,patient=`(${item.nowMs}-w.arrived>=${policy.wait.bypassMilliseconds} OR ${bypassed}>=${limit})`;
  const jobs=`CASE WHEN ${normal} THEN ${count} WHEN ${forced} THEN w.minimum ELSE 0 END`,admit=`(${normal} OR ${forced})`;
  const sql=`WITH RECURSIVE
   w AS (SELECT row_number() OVER (ORDER BY CASE WHEN priority='interactive' OR ${item.nowMs}-arrived>=${policy.wait.agingMilliseconds} THEN 0 ELSE 1 END, arrived, id) AS ord, * FROM waiters),
   step(ord,id,verdict,jobs,reserved,reason,room,load,blocked,admitted,heads,own) AS (
    SELECT 0,'','',0,0,'',${room},${load===null?"NULL":load},0,0,NULL,${totals.cpu}
    UNION ALL
    SELECT w.ord,w.id,CASE WHEN ${admit} THEN 'admit' ELSE 'wait' END,${jobs},CASE WHEN ${admit} THEN MIN(w.fixed+(${jobs})*w.perjob,${cap}) ELSE 0 END,
     CASE WHEN ${quoted} IS NOT NULL THEN ${quoted} WHEN s.blocked=1 THEN 'queue-order' WHEN ${normal} THEN CASE WHEN ${count}<w.pref THEN 'reduced' ELSE 'fits' END WHEN NOT ${fitsMinimum} THEN 'memory' WHEN ${overrideDue} THEN 'override' WHEN ${soft}=1 THEN '${softReason}' ELSE 'load' END,
     s.room-CASE WHEN ${admit} THEN MIN(w.fixed+(${jobs})*w.perjob,${cap}) ELSE 0 END,
     CASE WHEN s.load IS NULL THEN NULL ELSE s.load-(${jobs})*w.cpu END,
     CASE WHEN s.blocked=1 THEN 1 WHEN ${admit} THEN CASE WHEN s.heads IS NOT NULL AND s.heads+1>=${limit} THEN 1 ELSE 0 END WHEN ${waiting} AND ${patient} THEN 1 ELSE 0 END,
     s.admitted+CASE WHEN ${admit} THEN 1 ELSE 0 END,
     CASE WHEN s.blocked=1 THEN s.heads WHEN ${admit} THEN CASE WHEN s.heads IS NULL THEN NULL ELSE s.heads+1 END WHEN ${waiting} AND NOT ${patient} THEN MAX(COALESCE(s.heads,-1),${bypassed}) ELSE s.heads END,
     s.own+(${jobs})*w.cpu
    FROM step s JOIN w ON w.ord=s.ord+1)
   SELECT id,verdict,jobs,reserved,reason FROM step WHERE ord>0 ORDER BY ord`;
  const decisions=(database.query(sql).all() as {id:string;verdict:"admit"|"wait";jobs:number;reserved:number;reason:HostDecisionV1["reason"]}[]).map(row=>({id:row.id,verdict:row.verdict,jobs:row.jobs,reservedMegabytes:row.reserved,reason:row.reason}));
  return {floorMegabytes:floor,unrealizedMegabytes:totals.unrealized,headroomMegabytes:room,binding:budget<observed?"budget":"available",loadRoom:load,decisions};
 }finally{database.close();}
}

/** 🎲️ A seeded generator (mulberry32) so a corpus is reproducible in any language. */
export function seededRandomV1(seed:number):()=>number{
 let state=seed>>>0;
 return ()=>{state=(state+0x6D2B79F5)>>>0;let value=Math.imul(state^(state>>>15),1|state);value=(value+Math.imul(value^(value>>>7),61|value))^value;return ((value^(value>>>14))>>>0)/4294967296;};
}

/** 🧪️ Generates one random case from the generator; values are quantised so every language computes identical arithmetic. */
export function randomHostCaseV1(random:()=>number,index:number):Omit<HostCaseV1,"expect">{
 const pick=<T>(rows:readonly T[]):T=>rows[Math.floor(random()*rows.length)]!,whole=(low:number,high:number)=>low+Math.floor(random()*(high-low+1));
 const cores=pick([4,8,10,16]),total=pick([16384,32768,65536]),pressure=pick(["normal","normal","normal","normal","normal","normal","warn","critical"] as const),swapped=pick([0,0,0,0,0,null,10,100] as const);
 const host:HostSampleV1={version:1,atMs:1_000_000,platform:"darwin",cores,totalMegabytes:total,availableMegabytes:whole(Math.floor(total/4),total),pressure,swapUsedMegabytes:pick([null,0,0,100,100,total/2] as const),swapOutMegabytesPerSecond:swapped,load:pick([null,0,2.5,cores,cores*2,cores*3] as const),openFiles:pick([null,5000,5000,5000,5000,5000,200000,240000] as const),openFilesMaximum:pick([null,245760,245760,245760,245760] as const),attachedImages:pick([null,3,3,3,3,10,31,32,60] as const)};
 const ledger:HostGrantV1[]=Array.from({length:whole(0,4)},(_,row)=>{const reserved=whole(512,8192);return {id:`grant${index}x${row}aa`,operation:"run",jobs:1,cpu:whole(1,6),reservedMegabytes:reserved,admittedAtMs:900_000,observedMegabytes:pick([null,0,Math.floor(reserved/2),reserved]),pid:1,bootAtMs:0};});
 const queue:HostWaiterV1[]=Array.from({length:whole(1,6)},(_,row)=>{const minimum=whole(1,3);return {request:{id:`req${index}x${row}aaaa`,operation:"run",priority:pick(["batch","batch","interactive"] as const),minimumJobs:minimum,preferredJobs:minimum+whole(0,9),fixedMegabytes:whole(0,3072),perJobMegabytes:pick([0,512,1024,1024,2048,4096]),cpuPerJob:whole(1,3)},arrivedAtMs:whole(0,1_000_000),pid:1};});
 return {name:`corpus-${index}`,host,ledger,queue,nowMs:pick([1_000_000,1_100_000,1_200_000,1_300_000])};
}
