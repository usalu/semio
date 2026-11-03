import json,re,sys
sys.path.insert(0,".")
exec(open("classify.py").read().split("print(\"total")[0])
out=[]
for cr,why,f,n in rows:
    if not why.startswith("Option owner"):continue
    L=open(R+"/"+f,encoding="utf8").read().split("\n")
    sig=None
    for i in range(n-1,-1,-1):
        ms=list(re.finditer(r'fn \w+[^{;]*',L[i]))
        if ms:sig=ms[-1].group(0);break
    ret=re.search(r'->\s*(.*)$',sig or "")
    ret=ret.group(1)[:90] if ret else "()"
    s=L[n-1]
    segs=[s[max(0,m.start()-45):m.end()+25] for m in re.finditer(r'\.unwrap\(\)|\.expect\("[^"]*"\)|panic!|unreachable!',s)]
    out.append((cr,f.split("/")[-2][:12]+"/"+f.split("/")[-1][:14],n,ret,segs[:2]))
for i,x in enumerate(out):print(i,x[0][-26:],x[1],x[2],"|",x[3],"|",x[4])
json.dump(out,open("rem_opt.json","w"),ensure_ascii=False)
