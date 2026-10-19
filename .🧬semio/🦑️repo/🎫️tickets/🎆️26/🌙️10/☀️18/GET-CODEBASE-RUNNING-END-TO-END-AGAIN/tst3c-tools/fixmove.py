import os,re,subprocess,sys,json
R="/Users/ueli/Documents/semio"
FX="🧫️fixtures"; TS="🧪️tests"
apply = len(sys.argv)>1 and sys.argv[1]=="apply"
offs=[l.strip() for l in open(sys.argv[2] if len(sys.argv)>2 else R+"/list-offenders.txt",encoding="utf-8") if l.strip()]
moves={}
for rel in offs:
    parts=rel.split("/"); i=parts.index(FX)
    if "🧬️schema" in parts[3:i]: print("SKIP schema-ancestor",rel); continue
    new="/".join(parts[:i]+[TS,FX]+parts[i+1:])
    moves[R+"/"+rel]=R+"/"+new
conf=[n for n in moves.values() if os.path.exists(n)]
if conf: print("CONFLICT",conf[:5]); 
dirmap={}
for o,n in moves.items():
    od=o.split("/"+FX+"/")[0]+"/"+FX; nd=n.split("/"+FX+"/")[0]+"/"+FX
    dirmap[od]=nd
print("moves",len(moves),"dirs",len(dirmap))
def mapped(abspath):
    if abspath in moves: return moves[abspath]
    for od,nd in dirmap.items():
        if abspath==od or abspath.startswith(od+"/"):
            return nd+abspath[len(od):]
    return None
files=subprocess.run(["git","grep","-l","--untracked","-I",FX,"--","🧰️framework","🌎️hub","✏️s","🎓️teaching","♻️mit-bestand","🏢️semio-tech","📜️script.ts","📋️build-owners.json"],cwd=R,capture_output=True,text=True).stdout.splitlines()
lit=re.compile(r'''["'`]([^"'`\n]*''' + FX + r'''[^"'`\n]*)["'`]''')
edits={}; unresolved=[]
for f in files:
    fa=R+"/"+f
    if not fa.endswith((".rs",".ts",".tsx",".json",".toml",".md",".mjs",".py")): continue
    if fa in moves: continue
    try: txt=open(fa,encoding="utf-8").read()
    except Exception: continue
    out=[];last=0;changed=False
    for m in lit.finditer(txt):
        s=m.group(1)
        base=os.path.dirname(moves.get(fa,fa)) if False else os.path.dirname(fa)
        if s.startswith("/") or "://" in s or "*" in s: 
            cand=[R+"/"+s.lstrip("/")] if not s.startswith(R) else [s]
        else:
            cand=[os.path.normpath(os.path.join(base,s))]
        hit=None
        for c in cand:
            n=mapped(c)
            if n: hit=(c,n);break
        if hit:
            c,n=hit
            if s.startswith("/"): ns=n
            else:
                ns=os.path.relpath(n,base)
                if s.startswith("./") and not ns.startswith("."): ns="./"+ns
                if not s.startswith("./") and not s.startswith("../") and not ns.startswith("."): pass
            if s.endswith("/") and not ns.endswith("/"): ns+="/"
            out.append((m.start(1),m.end(1),ns));changed=True
    if changed: edits[fa]=out
print("files to edit",len(edits),"total edits",sum(len(v) for v in edits.values()))
json.dump({"moves":moves,"edits":{k:v for k,v in edits.items()}},open("/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad/fixmove-plan.json","w",encoding="utf-8"),ensure_ascii=False)
if apply:
    for fa,es in edits.items():
        txt=open(fa,encoding="utf-8").read()
        for a,b,ns in sorted(es,reverse=True): txt=txt[:a]+ns+txt[b:]
        open(fa,"w",encoding="utf-8").write(txt)
    for o,n in moves.items():
        os.makedirs(os.path.dirname(n),exist_ok=True); os.rename(o,n)
    for od in dirmap:
        try:
            for dp,dn,fn in os.walk(od,topdown=False):
                if not os.listdir(dp): os.rmdir(dp)
        except Exception: pass
    print("applied")
