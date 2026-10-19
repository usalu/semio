#!/usr/bin/env python3
# usage: fuzz.py '<needle>' [scope-glob]  -> best matching current lines for the identifiers of a stale needle
import re,subprocess,sys,collections
R="/Users/ueli/Documents/semio/"
needle=sys.argv[1]; scope=sys.argv[2] if len(sys.argv)>2 else "🧰️framework/*.rs"
toks=[t for t in dict.fromkeys(re.findall(r"[A-Za-z_][A-Za-z0-9_]{3,}",needle))]
def grep(t):
    r=subprocess.run(["git","grep","-n","-F","-w","-e",t,"--",scope],cwd=R,capture_output=True,text=True)
    return [l for l in r.stdout.split("\n") if l and "🗑️generated" not in l]
cnt={}
hits={}
for t in toks:
    h=grep(t); cnt[t]=len(h); hits[t]=h
# use the rarest 3 tokens with >0 hits
cand=sorted([t for t in toks if cnt[t]>0],key=lambda t:cnt[t])[:3]
print("tokens",{t:cnt[t] for t in toks})
score=collections.Counter(); text={}
for t in cand:
    for l in hits[t][:3000]:
        k=l.split(":",2)[0]+":"+l.split(":",2)[1]
        score[k]+=1; text[k]=l
for k,v in score.most_common(4):
    if v>=max(1,len(cand)-1):
        l=text[k]; fp=k.split(":")[0]
        body=l.split(":",2)[2]
        i=min([body.find(t) for t in cand if body.find(t)>=0] or [0])
        print(" ",v,fp[-85:],":",k.split(":")[1],"|",body[max(0,i-60):i+170].strip())
