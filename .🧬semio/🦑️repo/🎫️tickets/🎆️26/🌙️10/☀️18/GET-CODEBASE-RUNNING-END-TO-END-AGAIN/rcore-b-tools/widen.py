import json,sys,re,os,collections
log=sys.argv[1]; apply=len(sys.argv)>2 and sys.argv[2]=="apply"
only=set(sys.argv[3].split(",")) if len(sys.argv)>3 else None
targets=collections.defaultdict(dict)   # file -> {line_no: (text, kind)}
for l in open(log,errors="replace"):
    if not l.startswith("{"): continue
    try:r=json.loads(l)
    except: continue
    if r.get("reason")!="compiler-message": continue
    m=r["message"]
    if m["level"]!="error": continue
    pkg=r["package_id"].split("#")[-1].split("@")[0] if "#" in r["package_id"] else r["package_id"]
    if only and pkg not in only: continue
    code=(m.get("code") or {}).get("code","")
    if not (code in ("E0603","E0616","E0624") or "is private" in m["message"]): continue
    for ch in m.get("children",[]):
        if "defined here" not in ch["message"] and "private" not in ch["message"]: continue
        for sp in ch.get("spans",[]):
            f=sp["file_name"]
            if not os.path.exists(f) or "/.cache/" in f or f.startswith("/"): continue
            txt=sp["text"][0]["text"] if sp.get("text") else None
            if txt is None: continue
            targets[f][sp["line_start"]]=txt
item=re.compile(r"^(\s*)(pub(?:\([^)]*\))?\s+)?((?:async\s+|const\s+|unsafe\s+|extern\s+(?:\"[^\"]*\"\s+)?)*(?:fn|struct|enum|trait|type|const|static|union)\b)")
field=re.compile(r"^(\s*)(pub(?:\([^)]*\))?\s+)?([a-z_][A-Za-z0-9_]*)\s*:")
changed=0;skipped=0
for f,lines in targets.items():
    src=open(f).read().split("\n"); dirty=False
    for ln,txt in lines.items():
        if ln-1>=len(src) or src[ln-1]!=txt: skipped+=1; continue
        line=src[ln-1]
        m=item.match(line) 
        if m and m.group(2) is not None and m.group(2).strip()=="pub": continue
        if m:
            src[ln-1]=m.group(1)+"pub "+line[m.start(3):]; dirty=True; changed+=1; continue
        m=field.match(line)
        if m and m.group(2) is not None and m.group(2).strip()=="pub": continue
        if m:
            src[ln-1]=m.group(1)+"pub "+line[m.start(3):]; dirty=True; changed+=1; continue
        skipped+=1
    if dirty and apply: open(f,"w").write("\n".join(src))
print("files",len(targets),"widened",changed,"skipped",skipped)
