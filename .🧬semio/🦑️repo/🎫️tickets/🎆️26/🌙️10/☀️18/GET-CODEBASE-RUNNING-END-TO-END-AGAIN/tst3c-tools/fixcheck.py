import os,re,subprocess,sys,json
R="/Users/ueli/Documents/semio"; FX="🧫️fixtures"
plan=json.load(open("/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad/fixmove-plan.json",encoding="utf-8"))
olds=set()
for o in plan["moves"]: olds.add(o.split("/"+FX+"/")[0]+"/"+FX)
files=subprocess.run(["git","grep","-l","--untracked","-I",FX,"--","🧰️framework","🌎️hub","✏️s","🎓️teaching","♻️mit-bestand","🏢️semio-tech","📜️script.ts"],cwd=R,capture_output=True,text=True).stdout.splitlines()
lit=re.compile(r'''["'`]([^"'`\n]*'''+FX+r'''[^"'`\n]*)["'`]''')
bad=[]
for f in files:
    fa=R+"/"+f
    if not fa.endswith((".rs",".ts",".tsx",".json",".toml",".md",".mjs",".py")): continue
    try: txt=open(fa,encoding="utf-8").read()
    except Exception: continue
    for m in lit.finditer(txt):
        s=m.group(1)
        if "*" in s or "://" in s or "${" in s: continue
        c=os.path.normpath(os.path.join(os.path.dirname(fa),s)) if not s.startswith("/") else s
        if any(c==o or c.startswith(o+"/") for o in olds): bad.append((f,s))
print(len(bad))
for f,s in bad: print(f,"|",s)
