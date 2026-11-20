import json,sys,collections
for f in sys.argv[1:]:
    p=json.load(open(f)); nodes={n["id"]:n for n in p["nodes"]}; parent={}
    for n in p["nodes"]:
        for c in n.get("children",[]): parent[c]=n["id"]
    def path(i):
        out=[]
        while i is not None:
            n=nodes[i]; out.append(n["callFrame"]["functionName"] or "(anon)"); i=parent.get(i)
        return out[::-1]
    runs=[];cur=None
    for i,(s,d) in enumerate(zip(p["samples"],p["timeDeltas"])):
        name=nodes[s]["callFrame"]["functionName"]
        if name in ("(idle)","(program)","(garbage collector)") and name=="(idle)":
            if cur: runs.append(cur); cur=None
            continue
        if cur is None: cur={"ms":0,"by":collections.Counter()}
        cur["ms"]+=d/1000
        pa=path(s); key=" > ".join([x for x in pa if x not in("(root)","(program)")][:7])
        cur["by"][key]+=d/1000
    if cur: runs.append(cur)
    print("##",f.split("/")[-1])
    for r in runs:
        if r["ms"]>=int(__import__("os").environ.get("MIN","40")):
            print(f"  task ~{r['ms']:.0f}ms")
            for k,v in r["by"].most_common(4): print(f"     {v:.0f}ms {k}")
