import json,os,re,resource,sys
src,out=sys.argv[1],sys.argv[2]
LIMIT=500*1024*1024
pat=re.compile(r"^(?P<file>.+?):(?:\[script\] )?.*?Waiting for access to (?P<res>\S+) \((?P<mode>shared|exclusive)\): position (?P<pos>\d+) of (?P<q>\d+), behind the request of process (?P<pid>\d+) \((?P<hm>\w+), queued (?P<s>\d+) s ago\)$")
logs={}
for n,line in enumerate(open(src,errors="replace")):
    if n%2000==0 and resource.getrusage(resource.RUSAGE_SELF).ru_maxrss>(LIMIT if sys.platform=="darwin" else LIMIT//1024): sys.exit("rss cap exceeded")
    m=pat.match(line.rstrip("\n"))
    if not m: continue
    logs.setdefault(m["file"],[]).append([m["res"],m["mode"],int(m["pos"]),int(m["q"]),int(m["pid"]),m["hm"],int(m["s"])])
rows=[]
for path,obs in logs.items():
    try: mtime=os.path.getmtime(path)
    except OSError: mtime=0
    rows.append({"log":path.split("/🗑️generated/")[-1],"mtime":mtime,"observations":obs})
rows.sort(key=lambda r:r["mtime"])
json.dump(rows,open(out,"w"))
print(len(rows),"logs",sum(len(r["observations"]) for r in rows),"observations")
