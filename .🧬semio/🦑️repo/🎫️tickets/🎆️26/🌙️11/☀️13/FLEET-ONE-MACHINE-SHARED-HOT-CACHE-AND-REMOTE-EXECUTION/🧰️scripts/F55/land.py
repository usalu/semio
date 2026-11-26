import os,sys,tempfile
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def edit(rel,pairs):
    p=B+rel;s=open(p,encoding="utf8").read()
    for a,b in pairs:
        assert s.count(a)==1,(rel,a[:60],s.count(a))
        s=s.replace(a,b)
    fd,t=tempfile.mkstemp(dir=os.path.dirname(p));os.write(fd,s.encode());os.close(fd);os.rename(t,p)
edit("🕸️graph/🕰️generations/🗃️row-log/🟨️.mjs",[(
"  /** 🔢️ How many distinct rows the log holds,",
"  /** 🧊️ Settles the log between waves: writes the buffered lines and trims the offset and length arrays to the rows held, so a log that only waits for its owners' ids to be read at the seal keeps no slack and no unwritten text. */\n  settle(){require();flush();if(offsets.length>count){offsets=offsets.slice(0,count);lengths=lengths.slice(0,count);}scratch=Buffer.allocUnsafe(0);},\n  /** 🔢️ How many distinct rows the log holds,")])
edit("🕸️graph/🕰️generations/🗃️row-log/🟨️.d.mts",[("close():void}","settle():void;close():void}")])
edit("🕸️graph/🧭️discovery/🟦️.ts",[(
"  return {...captured.value,rows:captured.observations,dispose:()=>rowLog.close()};\n}",
"  rowLog.settle();\n  return {...captured.value,rows:captured.observations,dispose:disposerOfRowLogV1(rowLog)};\n}\n\n/** 🧹️ The disposer of one part read, made outside the read so its closure holds the row log only and not the scope of the read (stubs of every owner, derived facts, the node rows), which every wave of a long read would otherwise keep until the seal. */\nfunction disposerOfRowLogV1(rowLog:{close():void}):()=>void{return()=>rowLog.close();}")])
edit("🧠️memory/🧽️settle/🟨️.mjs",[(" if(quiet(environment,limitBytes))return;\n const caches=stats()"," const caches=stats()")])
