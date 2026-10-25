import os,re,sys
bad="🧰️framework/🛍️products/🦑️repo/🔨️modules/⚡️caching"
rx=re.compile(r'["\'`](\.{1,2}/[^"\'`\n]*)["\'`]')
hits=0
for dp,dn,fn in os.walk("🧰️framework"):
    dn[:]=[d for d in dn if d not in("node_modules",".cache","target","dist")]
    for f in fn:
        if not f.endswith((".ts",".mjs",".js",".tsx",".mts")): continue
        p=os.path.join(dp,f)
        try: s=open(p,encoding="utf8").read()
        except Exception: continue
        for m in rx.finditer(s):
            r=os.path.normpath(os.path.join(dp,m.group(1)))
            if r.startswith(bad):
                print(p,m.group(1)); hits+=1
print("hits",hits)
