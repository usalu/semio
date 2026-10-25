import json,sys,os,time
snap=sys.argv[1];now=time.time()
d=json.load(open(snap))
rows=[]
for f in d['files']:
    p=f['path']
    try:m=os.stat(p).st_mtime
    except OSError:m=0
    rows.append((p,(now-m)/3600,len(f['base64'])*3//4))
rows.sort()
for p,a,s in rows:
    print(f"{a:7.1f}h {s:7d} {p}")
