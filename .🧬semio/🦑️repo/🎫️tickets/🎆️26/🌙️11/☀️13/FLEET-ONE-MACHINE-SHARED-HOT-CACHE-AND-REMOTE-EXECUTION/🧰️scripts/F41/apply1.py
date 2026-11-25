import os,sys,re
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/"
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,old[:60],s.count(old))
        s=s.replace(old,new)
    tmp=path+".f41.tmp"
    open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,path)
PT=R+"🌲️process-tree/🟦️.ts"
s=open(PT,encoding="utf8").read()
i=s.index("/** 🕰️ Reads one lease owner's absolute process birth")
j=s.index("/** 🖥️ The command that prints the process table")
oldfn=s[i:j]
newfn='''/** 🕰️ Reads one lease owner's absolute process birth, or null when it could not be observed: the shared process identity read (Windows `Win32_Process.CreationDate`, elsewhere `ps -o lstart=`).
 * @see ../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts */
export function readyProcessStartSyncV1(pid:number,platform:NodeJS.Platform=process.platform):number|null{
 const read=readProcessStartV1(pid,platform);
 return typeof read==="number"?read:null;
}

'''
pairs=[
('import schema from "./🧬️schema/🔣️.json" with {type:"json"};\n','import schema from "./🧬️schema/🔣️.json" with {type:"json"};\nimport {pinSpawnedProcessV1,processIdentityVerdictV1,readProcessStartV1}from"../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";\n'),
(oldfn,newfn),
(''' const row=table.find(each=>each.pid===known.pid);
 if(row===undefined)return"gone";
 return known.startMs===0||near(row.startMs,known.startMs,tolerance)?"same":"reused";
}
''',''' const verdict=processIdentityVerdictV1(known,table.find(each=>each.pid===known.pid)?.startMs??"gone",tolerance);
 return verdict==="same"||verdict==="reused"?verdict:"gone";
}

/** 📌️ The identity of the stage process spawned at `startedMs`: the recorded start of the pid in an earlier sample (`restored`, a start other than the spawn time), else the row of the table in the window a spawn can have created it (the clock is read before the system creates the process and a loaded Windows machine creates it late), else the spawn time itself, which then judges any row outside the window another process. The creation time is never taken from the clock before the spawn alone: that told a slow Windows spawn from a reused pid.
 * @see ../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts */
export function readyPinStageRootV1(flight:Readonly<{pid:number;startedMs:number}>,table:readonly ReadyProcessV1[],restored?:ReadyKnownV1|undefined):ReadyKnownV1{
 if(restored!==undefined&&restored.pid===flight.pid&&restored.startMs!==flight.startedMs&&pinSpawnedProcessV1({pid:flight.pid,spawnedMs:flight.startedMs},restored.startMs)!==undefined)return{pid:restored.pid,startMs:restored.startMs};
 const pinned=pinSpawnedProcessV1({pid:flight.pid,spawnedMs:flight.startedMs},table.find(row=>row.pid===flight.pid)?.startMs);
 return pinned===undefined?{pid:flight.pid,startMs:flight.startedMs}:pinned;
}

/** 🧭️ Whether the process now at the pid of a stage in flight is that stage's process (`same`) or another process that reused the pid (`reused`), from its creation time `seenMs` (`null`: not readable, never a verdict of reuse) and the pinned start of the tree record: a pinned start is compared exactly, an unpinned one by the window a spawn can have created it in.
 * @see ../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts */
export function readyStageProcessVerdictV1(flight:Readonly<{pid:number;startedMs:number}>,restored:ReadyKnownV1|undefined,seenMs:number|null):"same"|"reused"{
 if(seenMs===null)return"same";
 const root=readyPinStageRootV1(flight,[],restored);
 if(root.startMs!==flight.startedMs)return processIdentityVerdictV1(root,seenMs)==="same"?"same":"reused";
 return pinSpawnedProcessV1({pid:flight.pid,spawnedMs:flight.startedMs},seenMs)!==undefined?"same":"reused";
}
'''),
]
edit(PT,pairs)
print("ok")
