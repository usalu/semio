import re,sys,os,subprocess
R="/Users/ueli/Documents/semio/"
OS=R+"🧰️framework/🛍️products/💻️os/"
def norm(p): return os.path.normpath(p)
def analyze(rel):
    path=OS+rel if not rel.startswith("/") else rel
    src=open(path,encoding="utf8",errors="replace").read()
    d=os.path.dirname(path)
    reads=[]
    for m in re.finditer(r'new URL\(\s*(["\'`])((?:\.\./|\./)[^"\'`]*)\1\s*,\s*(?:import\.meta\.url|(\w+))\s*\)',src):
        rp=m.group(2)
        if m.group(3) and m.group(3)!="import.meta.url":
            continue
        reads.append(norm(os.path.join(d,rp)))
    print("##",rel)
    existing=[r for r in reads if os.path.isfile(r)]
    for r in reads:
        print("  read",("OK  " if os.path.isfile(r) else "MISS"),r.replace(R,""))
    texts={r:open(r,encoding="utf8",errors="replace").read() for r in existing}
    lits=[]
    for m in re.finditer(r'(?:includes|toContain|indexOf|contains)\(\s*(?:(")((?:[^"\\\n]|\\.)*)"|(\')((?:[^\'\\\n]|\\.)*)\')',src):
        lit=m.group(2) if m.group(1) else m.group(4)
        lit=bytes(lit,"utf8").decode("unicode_escape",errors="ignore") if "\\" in lit and False else lit
        tail=src[m.end():m.end()+40]
        neg=bool(re.match(r'\s*\)\s*\)?\s*\.toBe\(false\)',tail)) or src[max(0,m.start()-12):m.start()].endswith("not.")
        lits.append((lit,neg))
    seen=set()
    for lit,neg in lits:
        if lit in seen or len(lit)<6: continue
        seen.add(lit)
        found=[os.path.basename(os.path.dirname(r)) for r,t in texts.items() if lit in t]
        if not found and not neg:
            try:
                out=subprocess.run(["git","-c","core.quotepath=false","grep","-l","-F",lit,"--","🧰️framework/🛍️products/💻️os","🧰️framework/🔨️modules","*.rs"],cwd=R,capture_output=True,text=True,timeout=120).stdout.split("\n")
            except Exception as e: out=[]
            out=[o for o in out if o and "🧪️tests" not in o][:3]
            print("  MISSING",repr(lit[:90]),"=>",[o.replace("🧰️framework/","")[-90:] for o in out])
if __name__=="__main__":
    for a in sys.argv[1:]: analyze(a)
