import os,re,sys,json
R="/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
ARTS=["⚖️en1990","🏋️en1991","🏛️en1992","🔩️en1993","🧩️en1994","🪵️en1995","🪨️en1996","🌍️en1997","🫨️en1998","🪶️en1999"]
SUB="🏅️standards/🔖️1/🪆️subsets/✳️any"
out={}
for a in ARTS:
    base=f"{R}/{a}/{SUB}"
    ex=f"{base}/📚️examples"
    lib=open(f"{ex}/🦀️.rs").read()
    mods=re.findall(r'#\[path = "([^"]+)"\]\s*pub mod (\w+);',lib)
    edl=open(f"{base}/✏️editor/🦀️.rs").read()
    m0=re.search(r'fn examples\(\)(.*?)\n    \}\n',edl,re.S)
    listed=re.findall(r'examples::(\w+)::source\(\)',m0.group(1)) if m0 else []
    exs=[]
    for p,m in mods:
        src=open(f"{ex}/{p}").read()
        inc=re.findall(r'include_(?:str|bytes)!\("([^"]+)"\)',src)
        lab=re.search(r'native\("([^"]*)",\s*"([^"]*)"',src)
        ts=p.replace("🦀️.rs","🟦️.ts")
        tsl=None
        if os.path.exists(f"{ex}/{ts}"):
            t=open(f"{ex}/{ts}").read()
            m2=re.search(r'en:\s*"([^"]*)",\s*de:\s*"([^"]*)"',t); tsl=m2.groups() if m2 else None
        exs.append(dict(mod=m,dir=os.path.dirname(p),inc=[os.path.normpath(os.path.join(ex,os.path.dirname(p),i)) for i in inc],label=lab.groups() if lab else None,ts=tsl,listed=m in listed))
    out[a]=exs
    print("==",a,"listed",listed)
    refd=set()
    for e in exs:
        print("  ",e['mod'],e['listed'],e['label'],e['ts'],[i.replace(base,'') for i in e['inc']])
        refd.update(e['inc'])
    for root,ds,fs in os.walk(f"{base}/🖼️assets"):
        for f in fs:
            fp=os.path.join(root,f)
            if f.endswith('.semio') or f.endswith('.json'):
                if fp not in refd: print("   UNREF",fp.replace(base,''))
json.dump(out,open("inventory.json","w"),ensure_ascii=False,indent=1)
