import json,sys,glob,collections,os,re
G=os.path.abspath(os.path.dirname(__file__)+"/../🗑️generated")
for f in sorted(glob.glob(G+"/tc2-pos-*-v3-result.json")):
    r=json.load(open(f))
    name=os.path.basename(f).split("-")[2]
    for cfg,s in r["summary"].items():
        c=collections.Counter()
        for x in s["new_list"]:
            msg=re.sub(r"`[^`]*`","`_`",x["msg"])
            if x["msg"]=="cannot find attribute `serde` in this scope": c["serde-attribute (oracle: testkit feature active)"]+=1; continue
            c[(x["code"],x["cat"],x["msg"][:100],os.path.basename(os.path.dirname(x["file"])))]+=1
        print(f"{name:9s} {cfg:5s} cur {s['cur_total']} base {s['base_total']} NEW {s['new']} GONE {s['gone']}")
        for k,v in c.most_common(10): print("      ",v,k)
