import json,sys,re,subprocess
F=sys.argv[1]; pats=sys.argv[2:]
r=json.load(open(F+"/tsp-tools/current.json"))
for x in r:
    if x["status"]=="pass" or not all(p in x["path"] for p in pats): continue
    t=open(x["log"],errors="replace").read()
    t=re.sub(r"/Users/ueli/Documents/semio/","",t)
    t=re.sub(r"✏️s/🔌️plugins/","P/",t)
    lines=t.split("\n")
    out=[]
    for i,l in enumerate(lines):
        if re.match(r"^(error|TypeError|ReferenceError|ENOENT|SyntaxError|Error)",l) :
            out.append(l[:360]); 
            out+= [m[:300] for m in lines[i+1:i+8] if m.strip() and not m.strip().startswith("at ")][:4]
            if len(out)>14: break
    fails=[l for l in lines if l.startswith("(fail)")][:3]
    print("=====",x["path"][-110:], "p%d f%d"%(x["pass_"],x["fail"]))
    print("\n".join(out[:14])); print("\n".join(f[:200] for f in fails))
