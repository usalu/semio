import re,collections,os,json
G=os.environ["G"]
rows=[l.split("\t")[0] for l in open(G+"/cpf-debug-census.tsv",encoding="utf-8")]
cnt=collections.Counter(); out=[]
cache={}
for r in rows:
    p,n=r.rsplit(":",1); n=int(n)
    if p not in cache: cache[p]=open(p,encoding="utf-8",errors="replace").read().split("\n")
    line=cache[p][n-1]
    for m in re.finditer(re.escape("[DEBUG]"),line):
        pre=line[max(0,m.start()-80):m.start()]
        post=line[m.end():m.end()+60]
        emit=bool(re.search(r'(eprintln!|println!|console\.\w+|stderr\.write|stdout\.write|writeSync|eprint!|print!)\W*(format!\W*)?$',pre))
        asrt=bool(re.search(r'(panic!|assert|expect|Error|toThrow|toContain|toBe|message|fail|unreachable)',pre))
        k="emit" if emit else ("assert" if asrt else "other")
        cnt[k]+=1; out.append((k,p,n,pre,post))
print(cnt)
json.dump(out,open(G+"/cpf-debug-ctx.json","w"),ensure_ascii=False)
for k,p,n,pre,post in out:
    if k!="assert": print(k,"/".join(p.split("/")[-3:]),n,repr(pre[-55:]),repr(post[:30]))
