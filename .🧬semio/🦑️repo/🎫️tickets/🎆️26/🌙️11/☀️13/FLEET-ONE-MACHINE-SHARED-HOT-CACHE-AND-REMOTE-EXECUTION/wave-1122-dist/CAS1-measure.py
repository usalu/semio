import os,sys,time,json,collections
root=os.path.expanduser("~/.cache/semio/cas/v1/o")
store="/Users/ueli/Documents/semio/.cache/b/p/m/fleet/cache-server/v1"
refs=set()
# referenced by sidecar metas (slotted entries)
for dp,dn,fn in os.walk(store):
    if "/.meta" in dp or dp.endswith(".meta"):
        for f in fn:
            try:
                m=json.load(open(os.path.join(dp,f)))
                for h in m.get("refs",[]):refs.add(h)
            except Exception:pass
pins=set()
pd=os.path.expanduser("~/.cache/semio/cas/v1/p")
for f in os.listdir(pd):
    try:
        p=json.load(open(os.path.join(pd,f)));pins.update(p.get("digests",[]))
    except Exception:pass
now=time.time()
tot=0;n=0
byhour=collections.Counter();kind=collections.Counter();kn=collections.Counter()
sizeb=collections.Counter()
for sh in sorted(os.listdir(root)):
    for name in os.listdir(os.path.join(root,sh)):
        try:st=os.lstat(os.path.join(root,sh,name))
        except OSError:continue
        h=sh+name
        n+=1;tot+=st.st_size
        age=(now-st.st_mtime)/3600
        byhour[min(int(age),48)]+=st.st_size
        k="pinned" if h in pins else ("meta-ref" if h in refs else "unreferenced")
        if st.st_nlink>1:k="linked"
        kind[k]+=st.st_size;kn[k]+=1
        b="<100K" if st.st_size<1e5 else "<1M" if st.st_size<1e6 else "<10M" if st.st_size<1e7 else "<100M" if st.st_size<1e8 else ">=100M"
        sizeb[(k,b)]+=st.st_size
g=1<<30
print("objects",n,"GiB",round(tot/g,2),"refs",len(refs),"pins",len(pins))
for k in kind:print(k,kn[k],round(kind[k]/g,2))
print("by age hour (GiB, last 12h):",[(a,round(byhour[a]/g,2)) for a in range(12)])
print("older 12-48h:",round(sum(v for a,v in byhour.items() if a>=12)/g,2))
for k,v in sorted(sizeb.items()):print(k,round(v/g,2))
