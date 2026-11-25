import os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"apply1.py"),encoding="utf8").read().split("PT=R")[0])
F=R+"🟦️.ts"
edit(F,[
("readyIdentityV1,readyConfirmProcessReplacementV1,","readyIdentityV1,readyPinStageRootV1,readyStageProcessVerdictV1,readyConfirmProcessReplacementV1,"),
("const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return(error as NodeJS.ErrnoException).code===\"EPERM\";}};\n\n/** 🧾️ The record of the stage in flight",
 "const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return(error as NodeJS.ErrnoException).code===\"EPERM\";}};\n\n/** 🧬️ Whether the process of a stage in flight is alive as that stage's process: its pid exists and the creation time of the process there is not another one than the stage was spawned with (a reused pid is another process and the stage is gone). */\nconst stageProcessAliveV1=(directory:string,row:Readonly<{stage:string;pid:number;startedMs:number}>,read:(pid:number)=>number|null=readyProcessStartSyncV1):boolean=>alive(row.pid)&&readyStageProcessVerdictV1(row,readyReadTreeV1(directory,row.stage)?.root,read(row.pid))==\"same\";\n\n/** 🧾️ The record of the stage in flight"),
(" if(record!==undefined&&!alive(record.pid)&&!existsSync(record.exitFile)){"," if(record!==undefined&&!stageProcessAliveV1(directory,record)&&!existsSync(record.exitFile)){"),
("return row!==undefined&&(alive(row.pid)||existsSync(row.exitFile))?row.stage:undefined;}","return row!==undefined&&(stageProcessAliveV1(directory,row)||existsSync(row.exitFile))?row.stage:undefined;}"),
("  const rootOf=():ReadyKnownV1=>({pid:inflight.pid,startMs:inflight.startedMs});","  let pinnedRoot=readyPinStageRootV1(inflight,[],readyReadTreeV1(directory,inflight.stage)?.root);\n  const rootOf=():ReadyKnownV1=>pinnedRoot;"),
("    const root=rootOf(),tolerance=treePolicy.toleranceMilliseconds,identity=readyIdentityV1(root,table,tolerance);","    pinnedRoot=readyPinStageRootV1(inflight,table,pinnedRoot);\n    const root=rootOf(),tolerance=treePolicy.toleranceMilliseconds,identity=readyIdentityV1(root,table,tolerance);"),
])
print("ok")
