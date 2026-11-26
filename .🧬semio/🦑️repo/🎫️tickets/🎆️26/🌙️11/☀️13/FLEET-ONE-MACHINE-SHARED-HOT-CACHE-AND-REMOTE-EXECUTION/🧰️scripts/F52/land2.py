import os,sys
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/"
B=R+"📚️library/🏗️build/"
S=R+"🤝️shared-worktree/🔁️replica/🧷️sidecars/"
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        if s.count(old)!=1: sys.exit(f"anchor count {s.count(old)} in {path}: {old[:70]}")
        s=s.replace(old,new)
    t=path+".f52tmp";open(t,"w",encoding="utf8").write(s);os.replace(t,path)
edit(S+"🟦️.ts",[
("/** ✍️ Renews the lease of a sidecar for the process `pid`. */",
 "/** 🎫️ The lease file of one launcher: the base path of the record with the launcher's own pid (`<name>.<pid>.lease`), so a launcher that a newer one replaced never has its lease renewed and leaves by itself. */\nexport function sidecarLeaseOfV1(base:string,pid:number):string{return base.endsWith(\".lease\")?`${base.slice(0,-6)}.${pid}.lease`:`${base}.${pid}`;}\n\n/** ✍️ Renews the lease of a sidecar for the process `pid`. */"),
(" const renew=(slot:Slot):void=>{try{writeSidecarLeaseV1(sidecarFilesV1(options.facts.worktree,slot.spec.name).lease,process.pid,Date.now());}catch{}};",
 " const renew=(slot:Slot):void=>{if(slot.pid===null)return;try{writeSidecarLeaseV1(sidecarLeaseOfV1(sidecarFilesV1(options.facts.worktree,slot.spec.name).lease,slot.pid),process.pid,Date.now());}catch{}};\n const revoke=(slot:Slot,pid:number|null):void=>{const base=sidecarFilesV1(options.facts.worktree,slot.spec.name).lease;try{rmSync(base,{force:true});if(pid!==null)rmSync(sidecarLeaseOfV1(base,pid),{force:true});}catch{}};"),
("  const ran=now()-slot.startedMs;delete slot.proc;slot.pid=null;slot.lastExit=why;",
 "  const ran=now()-slot.startedMs,ended=slot.pid;delete slot.proc;slot.pid=null;slot.lastExit=why;revoke(slot,ended);"),
("   rmSync(files.record,{force:true});renew(slot);\n","   rmSync(files.record,{force:true});revoke(slot,null);\n"),
("   slot.proc=proc;slot.pid=proc.pid??null;slot.startedMs=now();slot.nextStartMs=null;set(slot,\"running\");",
 "   slot.proc=proc;slot.pid=proc.pid??null;slot.startedMs=now();slot.nextStartMs=null;set(slot,\"running\");renew(slot);"),
("  slot.pid=record.pid;slot.startedMs=record.startedMs;slot.nextStartMs=null;slot.lastExit=null;set(slot,\"running\");renew(slot);",
 "  slot.pid=record.pid;slot.startedMs=record.startedMs;slot.nextStartMs=null;slot.lastExit=null;set(slot,\"running\");revoke(slot,null);renew(slot);"),
])
edit(B+"🌐️remote-execution/🛰️sidecar/🟦️.ts",[
(" const lease=environment.SEMIO_FLEET_SIDECAR_LEASE,record=environment.SEMIO_FLEET_SIDECAR_RECORD,protocol=await import(\"../../../../🤝️shared-worktree/🔁️replica/🧷️sidecars/🟦️.ts\"),startedMs=Date.now();",
 " const base=environment.SEMIO_FLEET_SIDECAR_LEASE,record=environment.SEMIO_FLEET_SIDECAR_RECORD,protocol=await import(\"../../../../🤝️shared-worktree/🔁️replica/🧷️sidecars/🟦️.ts\"),startedMs=Date.now(),lease=base===undefined||base===\"\"?undefined:protocol.sidecarLeaseOfV1(base,process.pid);"),
])
edit(S+"🧬️schema/🔣️.json",[
("\"<name>.lease below .cache/f/sidecars: written by the daemon that owns the sidecar every five seconds (tmp + rename).",
 "\"<name>.<launcher pid>.lease below .cache/f/sidecars: written by the daemon that owns the sidecar for that one launcher every five seconds (tmp + rename), so a launcher that a newer one replaced is never renewed."),
])
edit(S+"🧪️tests/🟦️.ts",[
(" expect(JSON.parse(readFileSync(join(facts.worktree,\".cache\",\"f\",\"sidecars\",\"a.lease\"),\"utf8\"))).toMatchObject({version:1,pid:process.pid});",
 " expect(JSON.parse(readFileSync(join(facts.worktree,\".cache\",\"f\",\"sidecars\",`a.${supervisor.report()[0]!.pid}.lease`),\"utf8\"))).toMatchObject({version:1,pid:process.pid});\n expect(existsSync(join(facts.worktree,\".cache\",\"f\",\"sidecars\",\"a.lease\")),\"the shared lease of older launchers is gone: they leave by themselves\").toBe(false);"),
])
edit(B+"🌐️remote-execution/🛰️sidecar/🧪️tests/🟦️.ts",[
(" const lease=join(directory,\"agent.lease\"),write=():void=>writeFileSync(lease,JSON.stringify({version:1,pid:process.pid,wallMs:Date.now()}));\n write();\n const agent=Bun.spawn(",
 " const lease=join(directory,\"agent.lease\");let held=\"\";const write=():void=>{if(held!==\"\")writeFileSync(held,JSON.stringify({version:1,pid:process.pid,wallMs:Date.now()}));};\n const agent=Bun.spawn("),
(" for(let round=0;round<6;round++){await Bun.sleep(1000);write();}",
 " held=join(directory,`agent.${agent.pid}.lease`);write();\n for(let round=0;round<6;round++){await Bun.sleep(1000);write();}"),
])
print("ok")
