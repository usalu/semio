"""🧬 tc2: keeps the retransform result except for re-added `[DEBUG]` logging that peers already stripped from the live files.
usage: merge_retr.py [--apply]"""
import json,glob,os,difflib,re,sys
R="/Users/ueli/Documents/semio/"; B="/Users/ueli/.cache/tc2/bak-pre-retr/"
apply="--apply" in sys.argv
DBG=re.compile(r'(?:e?println!\("\[DEBUG\][^\n]*?\);)')
for sp in sorted(glob.glob("specs/*.json")):
    s=json.load(open(sp))
    for f in s["files"]:
        p=f if f.startswith("/") else R+f
        a=open(B+os.path.relpath(p,R),encoding="utf-8").read().split("\n"); b=open(p,encoding="utf-8").read().split("\n")
        if a==b: continue
        sm=difflib.SequenceMatcher(None,a,b,autojunk=False)
        out=[]; review=[]
        for tag,i1,i2,j1,j2 in sm.get_opcodes():
            if tag=="equal": out+=a[i1:i2]; continue
            A=a[i1:i2]; Bn=b[j1:j2]
            if tag=="insert":
                if all(l.startswith("#[allow(unused_imports)]") for l in Bn): out+=Bn
                elif all("[DEBUG]" in l for l in Bn): out+=A
                else: out+=Bn; review.append((tag,A,Bn))
                continue
            stripped=[DBG.sub("",l) for l in Bn]
            if len(A)==len(Bn) and all(DBG.sub("",y).replace(" ","")==x.replace(" ","") or DBG.sub("",y)==x or y.replace(" ","")==x.replace(" ","") for x,y in zip(A,Bn)):
                out+=A; continue
            keepA=any("properties::selection_actions" in l for l in A)
            out+=A if keepA else Bn
            if not keepA: review.append((tag,A,Bn))
        for tag,A,Bn in review:
            print(p.split("/")[-3:-1],tag)
            for l in A: print("   -",l[:200])
            for l in Bn: print("   +",l[:200])
        if apply: open(p,"w",encoding="utf-8").write("\n".join(out))
