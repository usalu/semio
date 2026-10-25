#!/usr/bin/env python3
"""usage: orphan.py LISTFILE  Static E0117 detector: impl blocks in a crate whose trait and self type are both defined in no file of that crate."""
import os,re,sys
os.chdir("/Users/ueli/Documents/semio")
sys.argv=[sys.argv[0],"/dev/null"]+sys.argv[1:]
exec(open(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/plug-mz-c-tools/modscan2.py").read().split("tot=0")[0].replace("LIST=sys.argv[1]","LIST=sys.argv[2]"))
LISTF=sys.argv[2]
DEF=re.compile(r'\b(?:struct|enum|union|trait|type)\s+(\w+)')
IMPL=re.compile(r'\bimpl\b\s*(?:<[^{};]*?>\s*)?(!?)\s*([A-Za-z_][\w:]*)\s*(<[^{}]*?>)?\s+for\s+([&\w:\'<>\[\](), ]+?)\s*(?:where[^{]*)?\{')
STD={"Clone","Copy","Debug","Default","Display","PartialEq","Eq","Hash","PartialOrd","Ord","From","Into","Drop","Deref","DerefMut","Iterator","Send","Sync"}
for cf in [l.strip() for l in open(LISTF) if l.strip()]:
    try: t=open(cf,encoding="utf-8").read()
    except: continue
    if "[package]" not in t: continue
    seen=set()
    for kind,tp in targets(cf,t)[:1]: walk(tp,seen)
    texts={f:strip(open(f,encoding="utf-8").read()) for f in seen}
    defs=set()
    for s in texts.values(): defs|=set(DEF.findall(s))
    macros=any("macro_rules" in s for s in texts.values())
    out=[]
    for f,s in texts.items():
        for m in IMPL.finditer(s):
            tr=m.group(2).split("::")[-1]; targs=m.group(3) or ""; selfty=m.group(4)
            sn=re.sub(r"^[&\s']*(?:mut\s+)?","",selfty); sn=re.match(r"[\w:]+",sn); sn=sn.group(0).split("::")[-1] if sn else ""
            idents=set(re.findall(r"\w+",targs+" "+selfty))
            if tr in defs or sn in defs or (idents&defs): continue
            if len(sn)<=2 and sn.isupper(): continue
            out.append((tr,sn,f[-60:]))
    if out:
        print(cf[-90:])
        for tr,sn,f in sorted(set(out))[:12]: print("   impl",tr,"for",sn,"|",f)
