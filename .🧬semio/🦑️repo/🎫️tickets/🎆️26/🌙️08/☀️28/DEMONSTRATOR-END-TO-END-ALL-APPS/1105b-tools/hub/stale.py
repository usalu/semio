import subprocess,re,json,sys,collections
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
ren=json.load(open(C+"/🗑️generated/1105b/hub-renames.json"))
cnt=collections.Counter(ren.values())
ren={k:v for k,v in ren.items() if cnt[v]==1}
files=subprocess.run(["git","ls-files","--","*Cargo.toml",":!.🧬semio"],capture_output=True,text=True).stdout.split('\n')
rx=re.compile(r'^(\[dependencies\.|\[dev-dependencies\.|\[build-dependencies\.|\[workspace\.dependencies\.)?('+'|'.join(re.escape(k) for k in sorted(ren,key=len,reverse=True))+r')(\s*=|\.workspace|\])')
apply=len(sys.argv)>1 and sys.argv[1]=='apply'
tot=0;fl=0;by=collections.Counter()
for f in files:
    if not f:continue
    try:t=open(f).read()
    except Exception:continue
    L=t.split('\n');ch=False
    for i,l in enumerate(L):
        m=rx.match(l)
        if m and 'package =' not in l:
            by[m.group(2)]+=1;tot+=1;ch=True
            if apply:L[i]=l[:m.start(2)]+ren[m.group(2)]+l[m.end(2):]
    if ch:
        fl+=1
        if apply:open(f,'w').write('\n'.join(L))
print(tot,fl,by.most_common(8))
