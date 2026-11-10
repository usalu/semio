import re,sys,collections
stage="start";rows=collections.OrderedDict()
for line in open(sys.argv[1],errors="replace"):
    m=re.search(r'"command":"([a-z:-]+)"',line)
    if m and not m.group(1).startswith("graph:inference-entry"):stage=m.group(1)
    if line.startswith("TICK "):
        import json;t=json.loads(line[5:]);r=rows.setdefault(stage,[0,0,0,0]);r[0]=max(r[0],t["rss"]);r[1]=max(r[1],t["heap"]);r[2]=max(r[2],t["extra"]);r[3]+=1
for k,v in rows.items():print(f"{k:34} peak_rss={v[0]:5} peak_heap={v[1]:5} peak_extra={v[2]:4} ticks={v[3]}")
