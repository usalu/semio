#!/usr/bin/env python3
"""apply-script-v2.py <📜️script.ts> [<out>]: journal wiring of the plan reuse: the cursor is taken before the planner child starts, kept beside the record and offered to the check."""
import sys
src=sys.argv[1]; dst=sys.argv[2] if len(sys.argv)>2 else src
s=open(src,encoding="utf8").read()
def rep(old,new):
    global s
    assert s.count(old)==1,(old[:90],s.count(old))
    s=s.replace(old,new)
rep("import {bazelPlanFilesV1,bazelPlanKeyV1,bazelPlanOutputsV1,checkBazelPlanReuseV1,compareBazelPlanOutputsV1,recordBazelPlanV1,removeBazelPlanRecordV1,type BazelPlanOutputV1}","import {bazelPlanFilesV1,bazelPlanKeyV1,bazelPlanOutputsV1,checkBazelPlanReuseV1,compareBazelPlanOutputsV1,recordBazelPlanV1,removeBazelPlanRecordV1,takeBazelPlanCursorV1,type BazelPlanOutputV1}")
rep("record?:{key:string;ignore:ReadonlySet<string>;preload:string;out:string;proven?:readonly BazelPlanOutputV1[]}}","record?:{key:string;ignore:ReadonlySet<string>;preload:string;out:string;cursor?:string;proven?:readonly BazelPlanOutputV1[]}}")
rep("checkBazelPlanReuseV1({workspace,key,ignore,signal:phaseControl.signal,ports,","checkBazelPlanReuseV1({workspace,key,ignore,signal:phaseControl.signal,root,ports,")
rep("return {record:{key,ignore,preload:join(workspace,bazelPlanFilesV1.preload),out:join(workspace,bazelPlanFilesV1.witness),","const cursor=await takeBazelPlanCursorV1(root,phaseControl.signal);\n return {record:{key,ignore,preload:join(workspace,bazelPlanFilesV1.preload),out:join(workspace,bazelPlanFilesV1.witness),...(cursor===undefined?{}:{cursor}),")
rep("ignore:reuse.record.ignore,signal:phaseControl.signal,nowMs:Date.now()})","ignore:reuse.record.ignore,signal:phaseControl.signal,nowMs:Date.now(),root,...(reuse.record.cursor===undefined?{}:{cursor:reuse.record.cursor})})")
open(dst,"w",encoding="utf8").write(s)
print("patched",dst)
