#!/usr/bin/env python3
"""apply.py <📸️snapshot/🟦️.ts> [<out>]: a captured copy is reused only while the executable bit is also the one of the live file (chmod changes ctime, not mtime, ino or size)."""
import sys
src=sys.argv[1]; dst=sys.argv[2] if len(sys.argv)>2 else src
s=open(src,encoding="utf8").read()
old="if(row&&!row.racy&&row.liveMtimeNs===facts.mtimeNs.toString()&&row.liveIno===facts.ino.toString()&&BigInt(row.bytes)===facts.size&&await intact(aliasDestination(path),row.aliasMtimeNs,row.bytes))"
new="if(row&&!row.racy&&row.liveMtimeNs===facts.mtimeNs.toString()&&row.liveIno===facts.ino.toString()&&BigInt(row.bytes)===facts.size&&row.executable===((facts.mode&0o111n)!==0n)&&await intact(aliasDestination(path),row.aliasMtimeNs,row.bytes))"
assert s.count(old)==1
open(dst,"w",encoding="utf8").write(s.replace(old,new))
