import os,re,sys
R="/Users/ueli/Documents/semio"
skip={"node_modules",".git",".nx","target","🗑️generated",".🧬semio"}
rx=re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
bad=[]
for base in sys.argv[1:]:
    for dp,dn,fn in os.walk(R+"/"+base):
        dn[:]=[d for d in dn if d not in skip]
        for f in fn:
            if not f.endswith(".rs"): continue
            p=os.path.join(dp,f)
            try: t=open(p,encoding="utf-8").read()
            except Exception: continue
            for m in rx.finditer(t):
                tgt=os.path.normpath(os.path.join(dp,m.group(1)))
                if not os.path.exists(tgt): bad.append((p[len(R)+1:],m.group(1)))
for b in bad: print(b[0],"|",b[1])
print(len(bad))
