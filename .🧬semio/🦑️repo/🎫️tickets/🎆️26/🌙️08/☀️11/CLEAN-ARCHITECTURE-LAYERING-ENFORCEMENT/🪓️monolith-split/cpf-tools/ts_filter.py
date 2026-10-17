import os,sys,json,collections
sys.path.insert(0,os.environ["T"])
import rs_body
G=os.environ["G"]
H=rs_body.load_h()
data=json.load(open(G+"/cpf-ts-extract.json",encoding="utf-8"))
plan=[]; stats=collections.Counter(); show=[]
for f in data:
    if "cs" not in f: continue
    cs=f["cs"]; runs=[]; cur=None
    for c in cs:
        if cur and not c["block"] and not cur[-1]["block"] and c["own"] and cur[-1]["own"] and c["line"]==cur[-1]["line"]+1 and c["fn"]==cur[-1]["fn"]:
            cur.append(c)
        else:
            cur=[c]; runs.append(cur)
    dele=[]
    for r in runs:
        hs=[rs_body.hsh(l) for c in r for l in rs_body.comment_lines(c)]
        hs=[h for h in hs if h]
        stats["runs"]+=1
        if not hs: stats["empty"]+=1; continue
        if any(h in H for h in hs): stats["inH"]+=1; continue
        stats["new"]+=1
        for c in r: dele.append([c["start"],c["end"]])
        show.append((f["path"][-60:],r[0]["line"]+1,len(r),r[0]["text"][:80].replace("\n"," ")))
    if dele: plan.append({"path":f["path"],"del":dele})
json.dump(plan,open(G+"/cpf-ts-plan.json","w"),ensure_ascii=False)
print(dict(stats),len(plan))
for s in show[:60]: print(s)
