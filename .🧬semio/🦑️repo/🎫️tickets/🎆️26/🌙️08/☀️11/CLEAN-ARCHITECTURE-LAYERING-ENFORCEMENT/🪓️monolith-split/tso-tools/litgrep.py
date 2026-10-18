import re,sys,os,subprocess,pickle
R="/Users/ueli/Documents/semio/"
CACHE=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/tso-rscorpus.pkl"
def corpus():
    if os.path.exists(CACHE): return pickle.load(open(CACHE,"rb"))
    files=subprocess.run(["git","-c","core.quotepath=false","ls-files","--","🧰️framework/*.rs","🌎️hub/*.rs","✏️s/*.rs"],capture_output=True,text=True,cwd=R).stdout.split("\n")
    d={}
    for f in files:
        if not f or "/🧪️tests/" in f or "/🧪️testing/" in f: continue
        try: d[f]=open(R+f,encoding="utf8",errors="replace").read()
        except Exception: pass
    pickle.dump(d,open(CACHE,"wb")); return d
if __name__=="__main__":
    t=sys.argv[1]; src=open(t,encoding="utf8").read(); d=corpus()
    seen=set(); miss=[]; tot=0
    for m in re.finditer(r'(?:includes|toContain|indexOf)\(\s*(?:`([^`\n]*)`|"((?:[^"\\\n]|\\.)*)")',src):
        lit=m.group(1) if m.group(1) is not None else m.group(2)
        lit=lit.replace('\\"','"').replace('\\\\','\\').replace('\\n','\n')
        if lit in seen or len(lit)<5 or "${" in lit: continue
        seen.add(lit); tot+=1
        if not any(lit in x for x in d.values()): miss.append(lit)
    print("literals",tot,"missing-everywhere",len(miss))
    for l in miss: print("  MISSING",repr(l[:120]))
