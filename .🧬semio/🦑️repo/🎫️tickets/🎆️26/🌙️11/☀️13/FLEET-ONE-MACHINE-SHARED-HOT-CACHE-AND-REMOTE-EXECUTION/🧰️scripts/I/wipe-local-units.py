import json,os,shutil,sys,time
root,since,*crates=sys.argv[1:]
cut=time.mktime(time.strptime(since,"%Y-%m-%d %H:%M"))
gone=set()
for d,_,fs in os.walk(os.path.join(root,"e")):
    for f in fs:
        p=os.path.join(d,f)
        if os.path.getmtime(p)<cut: continue
        try: e=json.load(open(p))
        except Exception: continue
        if e.get("crate") in crates: gone.add(d)
for d in sorted(gone): print("remove",d); shutil.rmtree(d)
print(len(gone),"entries")
