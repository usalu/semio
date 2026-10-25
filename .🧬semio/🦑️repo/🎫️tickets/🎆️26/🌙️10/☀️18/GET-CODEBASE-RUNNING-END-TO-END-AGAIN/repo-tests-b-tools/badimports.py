import os,re,sys
root=sys.argv[1]
pat=re.compile(r'''(?:from\s*|import\s*\(\s*|import\s+|require\(\s*)["'](\.{1,2}/[^"']*)["']''')
bad=0
for dp,dn,fn in os.walk(root):
  dn[:]=[d for d in dn if d not in("node_modules",".cache","target",".git")]
  for f in fn:
    if not f.endswith((".ts",".mjs",".tsx",".cjs",".js")): continue
    p=os.path.join(dp,f)
    try: s=open(p,encoding="utf8").read()
    except: continue
    for m in pat.finditer(s):
      spec=m.group(1); t=os.path.normpath(os.path.join(dp,spec))
      if "⚡️caching" in spec and not os.path.exists(t) and not os.path.exists(t+".ts"):
        print(p,"->",spec); bad+=1
print("bad",bad)
