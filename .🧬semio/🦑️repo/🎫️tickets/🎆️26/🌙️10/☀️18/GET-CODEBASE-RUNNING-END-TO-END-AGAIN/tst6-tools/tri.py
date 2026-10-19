import json,re,sys,collections
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/tst6-tools/"
prev=json.load(open(C+"prev.json"))
PAT=[("DEP-PKG",r"Cannot find package '([^']+)'"),("DEP-EXPORT",r"Export named '([^']+)' not found in module '([^']+)'"),("DEP-MOD",r"Cannot find module '([^']+)'"),("DEP-RESOLVE",r"Could not resolve: \"([^\"]+)\""),("DEP-SUBPATH",r"Package subpath '([^']+)' is not defined"),("ENOENT",r"ENOENT[^\n]{0,200}")]
files=sys.argv[1:]
rows=[]
for fn in files:
  for l in open(C+fn,encoding="utf-8"):
    r=json.loads(l)
    if r["rc"]==0: continue
    txt=open(r["log"],encoding="utf-8",errors="replace").read() if r.get("log") else ""
    sig=[]
    for k,p in PAT:
        for m in re.finditer(p,txt):
            sig.append((k,"/".join(m.groups()) if m.groups() else m.group(0)[:150]))
    sigs=sorted(set(sig))[:6]
    m=re.search(r"^(error|SyntaxError|TypeError|ReferenceError)[^\n]{0,200}",txt,re.M)
    rows.append((r["file"],r["pass_"],r["fail"],sigs,(m.group(0) if m else "")[:200]))
for f,p,fl,s,e in rows:
    print(f[-100:],p,fl,s if s else "",e if not s else "")
