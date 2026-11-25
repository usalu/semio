import {expect,test} from "bun:test";
import {join} from "node:path";
import {UNIT_POLICY_V1} from "../../🧱️units/🟦️.ts";
import {buildHolderLiveV1,type BuildHolderEvidenceV1} from "../🟦️.ts";

const grace=UNIT_POLICY_V1.heldGraceMilliseconds,rows:BuildHolderEvidenceV1[]=[];
for(const lane of[false,true])for(const lease of[false,true])for(const lock of["held","free","missing","unsupported"] as const)for(const idleMilliseconds of[0,grace-1,grace,grace+1])rows.push({lane,lease,lock,idleMilliseconds});

test("every combination of lane marker, build lease, compiler lock and idle time gives the verdict of the independent Python oracle",()=>{
 const run=Bun.spawnSync(["python3","-I",join(import.meta.dir,"🐍️.py"),String(grace)],{stdin:new TextEncoder().encode(JSON.stringify(rows))});
 expect(run.exitCode,new TextDecoder().decode(run.stderr)).toBe(0);
 expect(rows.map(row=>buildHolderLiveV1(row,grace))).toEqual(JSON.parse(new TextDecoder().decode(run.stdout)));
});

test("any single proof of a holder keeps a directory however long it has been idle, and nothing but a young unsupported lock does without one",()=>{
 const idle=grace*100;
 expect(buildHolderLiveV1({lane:true,lease:false,lock:"free",idleMilliseconds:idle},grace)).toBe(true);
 expect(buildHolderLiveV1({lane:false,lease:true,lock:"free",idleMilliseconds:idle},grace)).toBe(true);
 expect(buildHolderLiveV1({lane:false,lease:false,lock:"held",idleMilliseconds:idle},grace)).toBe(true);
 expect(buildHolderLiveV1({lane:false,lease:false,lock:"unsupported",idleMilliseconds:idle},grace)).toBe(false);
 expect(buildHolderLiveV1({lane:false,lease:false,lock:"unsupported",idleMilliseconds:0},grace)).toBe(true);
 expect(buildHolderLiveV1({lane:false,lease:false,lock:"missing",idleMilliseconds:0},grace)).toBe(false);
});
