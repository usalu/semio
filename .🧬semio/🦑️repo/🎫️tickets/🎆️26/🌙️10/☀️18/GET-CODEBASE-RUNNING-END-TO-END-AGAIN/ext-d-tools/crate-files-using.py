import json,re,os,sys
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/ext-d/"
crate_name=sys.argv[1]
pat=re.compile(r"(\bimage::|use image\b|extern crate image)")
pathmod=re.compile(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;')
def closure(lib):
    seen=set();stack=[lib]
    while stack:
        f=stack.pop()
        if f in seen or not os.path.isfile(f): continue
        seen.add(f)
        t=open(f,encoding="utf8",errors="replace").read()
        for m in pathmod.finditer(t):
            stack.append(os.path.normpath(os.path.join(os.path.dirname(f),m.group(1))))
        for m in re.finditer(r'(?:^|\n)\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;',t):
            for cand in (os.path.join(os.path.dirname(f),m.group(1)+".rs"),os.path.join(os.path.dirname(f),m.group(1),"mod.rs")):
                if os.path.isfile(cand): stack.append(cand)
    return seen
for u in json.load(open(G+"truth2.json"))["oracleConflicts"][0]["users"]:
    base=os.path.dirname(u);txt=open(u,encoding="utf8").read()
    m=re.search(r'\[lib\][^\[]*?path\s*=\s*"([^"]+)"',txt)
    lib=os.path.normpath(os.path.join(base,m.group(1))) if m else os.path.join(base,"src/lib.rs")
    files=closure(lib)
    hits=[f for f in files if pat.search(open(f,encoding="utf8",errors="replace").read())]
    print(u.split("/")[-4],len(files),"files; image hits:",[("T" if "🧪️tests" in h else "P")+":"+h.split("/")[-2] for h in hits])
