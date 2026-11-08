import re,os,subprocess,collections,sys
root='/Users/ueli/Documents/semio'
os.chdir(root)
J='🧰️framework/🔨️modules/🧵️job/'
closure=["⏱️context/🦀️.rs","⏱️context/🎟️source/🦀️.rs","⏱️context/🎟️source/🏠️native/🦀️.rs","📬️outcome/⏸️continuation/🎟️source/🦀️.rs","📬️outcome/⏸️continuation/🎟️origin/🦀️.rs","⏱️context/🎟️source/👶️child/🦀️.rs","♻️retirement/🎟️context/🦀️.rs","⏱️context/📸️capture/🦀️.rs","📬️outcome/⏸️continuation/🎟️source/📤️header/🦀️.rs","⏱️context/🛬️owned/🦀️.rs","⏱️context/📦️return/🎟️place/🦀️.rs","📬️outcome/⏸️continuation/📦️capsule/🦀️.rs","📬️outcome/⏸️continuation/🎟️source/🛡️guard/🦀️.rs","📬️outcome/⏸️continuation/🎟️source/📮️exposure/🦀️.rs","⏱️context/📦️return/🪶️weak/🦀️.rs","⏱️context/📦️owner/🦀️.rs","⏱️context/🧪️testkit/🦀️.rs"]
cand={}
for c in closure:
    s=open(J+c).read()
    for m in re.finditer(r'(?m)^\s*(?:pub(?:\([a-z]+\))?\s+)?(?:const\s+)?(?:unsafe\s+)?(fn|struct|enum|trait|type|const|static)\s+([A-Za-z_0-9]+)',s):
        cand.setdefault(m.group(2),[]).append((c,m.group(1)))
names=set(cand)
tok=re.compile(r'[A-Za-z_][A-Za-z_0-9]*')
cnt=collections.Counter();tst=collections.Counter()
files=subprocess.run("git ls-files '*.rs'; git ls-files --others --exclude-standard '*.rs'",shell=True,capture_output=True,text=True).stdout.split('\n')
for p in set(files):
    if not p or not os.path.exists(p) or '/dist/' in p or '/target/' in p: continue
    try:s=open(p,errors='replace').read()
    except: continue
    for t in tok.findall(s):
        if t in names:
            cnt[t]+=1
            if '/tests/' in p or '🧪' in p: tst[t]+=1
own=collections.Counter()
for c in closure:
    s=open(J+c).read()
    for t in tok.findall(s):
        if t in names: own[t]+=1
out=[]
for n in names:
    out.append((cnt[n],n,cand[n][0]))
out.sort()
for c,n,d in out:
    if c<=2: print(c,own[n],n,d)
