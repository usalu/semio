import json,re,statistics,sys
src,out=sys.argv[1],sys.argv[2]
episodes=json.load(open(src))["episodes"]
product=re.compile(r"^(dash|print|course|e2e|nx|exec)")
corpus=[]
for index,e in enumerate(episodes):
    w=sorted(e["waiters"],key=lambda x:x["startSeconds"])
    if len(w)<3:continue
    rows=[{"id":x["log"].split("/")[0][:24]+"#"+str(i),"lane":x["log"].split("/")[0],"arrival":round(x["startSeconds"]),"observedWait":x["observations"]*15} for i,x in enumerate(w)]
    for r in rows:r["start"]=r["arrival"]+r["observedWait"]
    order=sorted(rows,key=lambda r:r["start"])
    for i,r in enumerate(order):
        nxt=order[i+1]["start"]-r["start"] if i+1<len(order) else None
        r["service"]=max(15,nxt) if nxt is not None else None
    known=[r["service"] for r in order if r["service"] is not None]
    for r in order:
        if r["service"] is None:r["service"]=round(statistics.median(known))
        r["service"]=min(r["service"],8*3600)
        r["class"]="agent" if product.match(r["lane"]) else "background"
    base=min(r["arrival"] for r in rows)
    for r in rows:r["arrival"]-=base;r.pop("start")
    corpus.append({"id":f"tonight-{index}","resource":"repository-graph-facts:<root>:shard:7","jobs":rows,"spanSeconds":round(max(r["arrival"]+r["service"] for r in rows))})
json.dump({"version":1,"source":"streamed from C/generated logs 10-25 19:00..10-26 07:30; arrival=first queue notice, service=FIFO start difference","episodes":corpus},open(out,"w"),indent=0,ensure_ascii=False)
for c in corpus:print(c["id"],len(c["jobs"]),"jobs span %d min"%(c["spanSeconds"]/60),"service median %d s max %d s"%(statistics.median(r["service"] for r in c["jobs"]),max(r["service"] for r in c["jobs"])))
