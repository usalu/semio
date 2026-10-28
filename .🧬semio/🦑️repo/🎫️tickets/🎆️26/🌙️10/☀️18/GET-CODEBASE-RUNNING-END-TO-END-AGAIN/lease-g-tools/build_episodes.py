import collections,json,sys
src,out=sys.argv[1],sys.argv[2]
rows=[r for r in json.load(open(src)) if any(o[0].endswith(":shard:7") for o in r["observations"])]
seen=set();uniq=[]
for r in rows:
    key=json.dumps(r["observations"]);
    if key in seen:continue
    seen.add(key);uniq.append(r)
rows=uniq
edges=collections.defaultdict(list)
for li,r in enumerate(rows):
    used={}
    for i,o in enumerate(r["observations"]):
        if o[4] in used:continue
        used[o[4]]=15*i-o[6]
    for pid,c in used.items():
        edges[("L",li)].append((("P",pid),c));edges[("P",pid)].append((("L",li),-c))
value={};components=[]
for start in list(edges):
    if start in value:continue
    value[start]=0;queue=[start];members=[start]
    while queue:
        node=queue.pop()
        for nxt,c in edges[node]:
            if nxt not in value:
                value[nxt]=value[node]+c if node[0]=="L" else value[node]+c
                queue.append(nxt);members.append(nxt)
    components.append(members)
episodes=[]
for members in components:
    logs=[m[1] for m in members if m[0]=="L"];pids=[m[1] for m in members if m[0]=="P"]
    head={}
    for li in logs:
        r=rows[li];t0=value[("L",li)]
        for i,o in enumerate(r["observations"]):
            t=t0+15*i
            h=head.setdefault(o[4],[t,t]);h[0]=min(h[0],t);h[1]=max(h[1],t)
    arrivals={p:value[("P",p)] for p in pids}
    base=min(arrivals.values())
    jobs=[{"pid":p,"arrivalSeconds":arrivals[p]-base,"blockSeconds":head[p][1]-head[p][0]+15,"firstHeadSeconds":head[p][0]-base} for p in sorted(pids,key=lambda p:arrivals[p])]
    waiters=[{"log":rows[li]["log"],"startSeconds":value[("L",li)]-base,"observations":len(rows[li]["observations"])} for li in logs]
    episodes.append({"jobs":jobs,"waiters":waiters,"spanSeconds":max(j["firstHeadSeconds"]+j["blockSeconds"] for j in jobs)})
episodes.sort(key=lambda e:-e["spanSeconds"])
json.dump({"resource":"repository-graph-facts:<root>:shard:7","source":"tonight 10-25 19:00..10-26 07:30 logs, 15 s cadence","episodes":episodes},open(out,"w"))
for e in episodes:print(len(e["jobs"]),"jobs",len(e["waiters"]),"waiters span %.0f min"%(e["spanSeconds"]/60),"max block %.0f min"%(max(j["blockSeconds"] for j in e["jobs"])/60))
