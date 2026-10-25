"""status.py <census dirs...> — latest verdict per test file across the dirs (later dir wins); prints failing files with counts"""
import sys,os
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/proc-d/"
latest={}
for d in sys.argv[1:]:
    p=G+d+"/summary.txt"
    if not os.path.exists(p): continue
    for l in open(p,encoding="utf8"):
        c=l.rstrip("\n").split("\t")
        if len(c)>=5: latest[c[0]]=(c[1],c[2],c[3],d)
bad={f:v for f,v in latest.items() if v[0]!="rc=0"}
print(len(latest),"files seen;",len(bad),"failing")
for f,v in sorted(bad.items()): print(v[3],v[0],v[1],v[2],f)
