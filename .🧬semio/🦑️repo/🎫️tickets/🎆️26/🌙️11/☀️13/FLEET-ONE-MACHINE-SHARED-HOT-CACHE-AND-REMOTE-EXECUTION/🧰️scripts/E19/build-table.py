import json,re,os,collections
G=os.path.dirname(os.path.abspath(__file__))+"/../../wave-1122/🗑️generated/E19/"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def jl(f):
    out={}
    if os.path.exists(G+f):
        for l in open(G+f):
            try: j=json.loads(l); out[j["file"]]=j["rc"]
            except Exception: pass
    return out
raw={}
for f in ("raw.jsonl","raw2.jsonl","raw3a.jsonl","raw3b.jsonl"): raw.update(jl(f))
root={}
for f in ("rootA.jsonl","rootB.jsonl","rootF.jsonl"): root.update(jl(f))
sf={}
st0=json.load(open(G+"sf-state.json"))
for f_,v in st0.items():
    if v=="PASS": sf[f_]=0
    elif v=="FAIL": sf[f_]=1
for f in ("sf3a.jsonl","sf3b.jsonl","sf4.jsonl","sf5.jsonl"): sf.update(jl(f))
dig={}
if os.path.exists(G+"sfdigest.txt"):
    for b in open(G+"sfdigest.txt").read().split("=== ")[1:]:
        h,_,body=b.partition("\n"); f=h.split(" rc=")[0]
        ls=[l for l in body.split("\n") if re.match(r"^(error|\[E19\])",l) and "suite-files-failed" not in l]
        dig[f]=ls[0][:100] if ls else ""
allf=[l.strip() for l in open(G+"all.txt") if l.strip()]
rows=[];cnt=collections.Counter()
for f in allf:
    r=raw.get(f); rt=root.get(f); s_=sf.get(f)
    if r==0: st="PASS raw (tests dir)"
    elif rt==0: st="PASS raw (checkout root)"
    elif s_==0: st="PASS faithful (sf.sh)"
    elif rt is not None and rt!=0 and s_ is not None and s_!=0: st="FAIL (raw root and faithful)"
    elif s_ is not None and s_!=0: st="FAIL (faithful)"
    elif rt is not None and rt!=0: st="FAIL raw root, faithful pending"
    elif r is not None and r!=0: st="FAIL raw tests dir, root/faithful pending"
    else: st="NOT RUN"
    cnt[st]+=1
    rows.append((f.replace(L,""),("pass" if r==0 else "fail" if r is not None else "-"),("pass" if rt==0 else "fail" if rt is not None else "-"),("pass" if s_==0 else "fail" if s_ is not None else "-"),st,dig.get(f,"")))
print(json.dumps(cnt,ensure_ascii=False))
open(G+"table.md","w").write("| file | raw tests dir | raw checkout root | faithful (sf) | status | first error |\n|---|---|---|---|---|---|\n"+"\n".join("| %s | %s | %s | %s | %s | %s |"%r for r in rows if not r[4].startswith("PASS raw"))+"\n")
