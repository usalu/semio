import {describe,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {countUnitV1,readBuildCountersV1,unitLedgerLinesV1,unitLedgerV1,type UnitCountKindV1} from "../🟦️.ts";

const python=`
import json,sys
d=json.load(sys.stdin)
b,a=d["before"],d["after"]
g=lambda k:max(0,a.get(k,0)-b.get(k,0))
l,f,c,s,k=g("localHit"),g("remoteHit"),g("miss"),g("stored"),g("skipped")
out=[] if l+f+c+k==0 else [
 "[units] %d restored local, %d restored fleet, %d compiled (%d stored, %d not shared)"%(l,f,c,s,k),
 "[units] %d lokal wiederhergestellt, %d aus der Flotte wiederhergestellt, %d kompiliert (%d gespeichert, %d nicht geteilt)"%(l,f,c,s,k)]
print(json.dumps(out))
`;
const zero:Record<UnitCountKindV1,number>={localHit:0,remoteHit:0,miss:0,stored:0,skipped:0};

describe("🧮️ unit ledger of a run",()=>{
 test("the ledger is the growth of the real counter files of a workspace between two readings",()=>{
  const root=mkdtempSync(join(tmpdir(),"unit-ledger-"));
  try{
   const grow=(kind:UnitCountKindV1,times:number)=>{for(let index=0;index<times;index++)countUnitV1(root,kind);};
   grow("localHit",2);grow("miss",1);
   const before=readBuildCountersV1(root).unit;
   grow("localHit",3);grow("remoteHit",688);grow("miss",5);grow("stored",5);grow("skipped",2);
   const ledger=unitLedgerV1(before,readBuildCountersV1(root).unit);
   expect(ledger).toEqual({restoredLocal:3,restoredFleet:688,compiled:5,stored:5,skipped:2});
   expect(unitLedgerLinesV1(ledger)[0]).toBe("[units] 3 restored local, 688 restored fleet, 5 compiled (5 stored, 2 not shared)");
  }finally{rmSync(root,{recursive:true,force:true});}
 });
 test("a run that touched no unit prints nothing and a emptied counter directory never counts backwards",()=>{
  expect(unitLedgerLinesV1(unitLedgerV1(zero,zero))).toEqual([]);
  expect(unitLedgerV1({...zero,miss:9},{...zero,miss:2})).toEqual({restoredLocal:0,restoredFleet:0,compiled:0,stored:0,skipped:0});
 });
 test("the English and German lines equal an independent python rendering over 200 seeded counter pairs",()=>{
  let seed=20261118;const next=()=>{seed=(seed*1103515245+12345)%2147483648;return seed%900;};
  for(let index=0;index<200;index++){
   const before={localHit:next(),remoteHit:next(),miss:next(),stored:next(),skipped:next()},after={localHit:next(),remoteHit:next(),miss:next(),stored:next(),skipped:next()};
   const run=spawnSync("python3",["-c",python],{input:JSON.stringify({before,after}),encoding:"utf8"});
   expect(run.status).toBe(0);
   expect(unitLedgerLinesV1(unitLedgerV1(before,after))).toEqual(JSON.parse(run.stdout));
  }
 });
});
