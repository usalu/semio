import re,sys,os
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
run=sys.argv[1] if len(sys.argv)>1 else "env-pass2-run2"
summ=sys.argv[2] if len(sys.argv)>2 else "summary2.tsv"
rows=sorted(l.rstrip("\n").split("\t") for l in open(C+"/env-pass2-tools/"+summ,encoding="utf-8"))
sel=set(sys.argv[3:])
for r in rows:
    if sel and r[0] not in sel: continue
    t=open(C+"/🗑️generated/"+run+"/"+r[0]+".log",encoding="utf-8",errors="replace").read()
    short=r[4].replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/","L/").replace("🧰️framework/🔨️modules/","FM/")
    print("##",r[0],r[2],r[3],short)
    fails=re.findall(r"^\(fail\) (.*?)(?: \[[0-9.]+m?s\])?$",t,re.M)
    print("  fails:"," | ".join(f[:70] for f in fails[:6]))
    seen=[]
    for m in re.finditer(r"^((?:error|TypeError|ReferenceError|SyntaxError|AssertionError)[^\n]*)\n((?:.*\n){0,12}?)((?:\s+at [^\n]*\n)+)",t,re.M):
        e=m.group(1)[:140]
        if e.startswith("error: [script]") or e in seen: continue
        seen.append(e)
        fr=[x.strip() for x in m.group(3).splitlines() if "node_modules" not in x][:2]
        print("  E:",e); print("     ",*[re.sub(r"/Users/ueli/Documents/semio/🧰️framework/","FW/",f)[:200] for f in fr])
        if len(seen)>=3: break
