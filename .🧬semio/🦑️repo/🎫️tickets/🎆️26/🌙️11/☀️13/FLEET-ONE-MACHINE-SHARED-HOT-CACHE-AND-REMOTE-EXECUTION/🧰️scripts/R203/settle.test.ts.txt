import {describe,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {nxTaskLedgerLineV1,nxTaskLedgerV1,settleNxSlotsV1} from "../🟦️.ts";
import type {NxTaskRowV1} from "../../../🪢️nx-slots/🟦️.ts";

const since=1_800_000_000_000;
const rows:NxTaskRowV1[]=[
 {hash:"7427260498132517366",project:"@semio-tech/norm-din4108-rs",target:"build",configuration:"prototype",createdAt:since+5000,accessedAt:since+5000},
 {hash:"15698478355623763392",project:"@semio-tech/dsl-derive-rs",target:"generate-schema-index",configuration:"prototype",createdAt:since-86_400_000,accessedAt:since+900},
 {hash:"111",project:"@semio-tech/old",target:"build",configuration:null,createdAt:since-86_400_000,accessedAt:since-86_000_000},
 {hash:"222",project:"@semio-tech/uncached",target:"build",configuration:null,createdAt:null,accessedAt:null},
];

describe("🧾️ per task hash lines of a run",()=>{
 test("a task stored during the run is stored, one read during the run is a hit, older and uncached tasks are not listed, the order is by task name",()=>{
  const ledger=nxTaskLedgerV1(rows,since);
  expect(ledger.map(row=>row.task)).toEqual(["@semio-tech/dsl-derive-rs:generate-schema-index:prototype","@semio-tech/norm-din4108-rs:build:prototype"]);
  expect(ledger.map(row=>row.outcome)).toEqual(["hit","stored"]);
  expect(ledger.map(nxTaskLedgerLineV1)).toEqual(["[nx-task] @semio-tech/dsl-derive-rs:generate-schema-index:prototype 15698478355623763392 hit","[nx-task] @semio-tech/norm-din4108-rs:build:prototype 7427260498132517366 stored"]);
 });
 test("the settlement of a run carries the ledger and still fails soft",async()=>{
  const root=mkdtempSync(join(tmpdir(),"nx-ledger-"));
  try{
   const data=join(root,"data");mkdirSync(data,{recursive:true});writeFileSync(join(data,"workspace.db"),"");
   const settled=await settleNxSlotsV1({root,environment:{NX_WORKSPACE_DATA_DIRECTORY:data},sinceMs:since,reader:async()=>rows,nowMs:since+60_000});
   expect(settled.tasks?.length).toBe(2);
   expect(await settleNxSlotsV1({root,environment:{},sinceMs:since,reader:async()=>rows})).toEqual({registered:undefined,local:undefined});
  }finally{rmSync(root,{recursive:true,force:true});}
 });
});
