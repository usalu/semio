import os,re,subprocess,sys,json
R="/Users/ueli/Documents/semio"
G=sys.argv[2]
apply=sys.argv[1]=="apply"
FX="🧫️fixtures"
moves=json.load(open(G+"/plan.json",encoding="utf-8"))["moves"]
newset=set(moves.values())
dirmap={}
for o,n in moves.items():
    dirmap[o.split("/"+FX+"/")[0]+"/"+FX]=n.split("/"+FX+"/")[0]+"/"+FX
def mapped(a):
    if a in moves: return moves[a]
    for od,nd in dirmap.items():
        if a==od or a.startswith(od+"/"): return nd+a[len(od):]
    return None
files=subprocess.run(["git","grep","-l","--untracked","-I",FX,"--","🧰️framework","🌎️hub","✏️s","🎓️teaching","♻️mit-bestand","🏢️semio-tech","📜️script.ts","📋️build-owners.json"],cwd=R,capture_output=True,text=True).stdout.splitlines()
print("grep files",len(files))
lit=re.compile(r'''["'`]([^"'`\n]*'''+FX+r'''[^"'`\n]*)["'`]''')
edits={}
for f in files:
    fa=R+"/"+f
    if fa in newset or not fa.endswith((".rs",".ts",".tsx",".json",".toml",".md",".mjs",".py",".feature")): continue
    try: txt=open(fa,encoding="utf-8").read()
    except Exception: continue
    base=os.path.dirname(fa); out=[]
    for m in lit.finditer(txt):
        s=m.group(1)
        if "://" in s or "*" in s: continue
        c=(R+"/"+s.lstrip("/") if not s.startswith(R) else s) if s.startswith("/") else os.path.normpath(os.path.join(base,s))
        n=mapped(c)
        if not n: continue
        if s.startswith("/"): ns=n
        else:
            ns=os.path.relpath(n,base)
            if s.startswith("./") and not ns.startswith("."): ns="./"+ns
        if fa.endswith((".ts",".tsx",".mjs")) and not ns.startswith((".","/")): ns="./"+ns
        if s.endswith("/") and not ns.endswith("/"): ns+="/"
        if ns!=s: out.append((m.start(1),m.end(1),ns))
    if out: edits[fa]=out
print("files to edit",len(edits),"edits",sum(len(v) for v in edits.values()))
json.dump(edits,open(G+"/edits.json","w",encoding="utf-8"),ensure_ascii=False)
if apply:
    for fa,es in edits.items():
        txt=open(fa,encoding="utf-8").read()
        for a,b,ns in sorted(es,reverse=True): txt=txt[:a]+ns+txt[b:]
        open(fa,"w",encoding="utf-8").write(txt)
    print("applied")
