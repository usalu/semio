import json,re,sys,collections
res=json.load(open(sys.argv[1]))
out=[]
for r in res:
    if r["status"] in("pass",): continue
    t=open(r["log"],errors="replace").read()
    lines=[l for l in t.split("\n") if re.search(r"error:|Error:|Cannot find|ENOENT|not found|Expected|SIGSEGV|panic|crash",l)]
    key=lines[0].strip()[:220] if lines else t.strip().split("\n")[-1][:220]
    key=re.sub(r"/Users/ueli/Documents/semio/","",key)
    out.append((r["status"],key,r["path"]))
c=collections.Counter(re.sub(r"[🏅-🪆].*?/","",k)[:140] for _,k,_ in out)
for k,n in c.most_common(): print(n,k)
json.dump(out,open(sys.argv[2],"w"),ensure_ascii=False,indent=0)
