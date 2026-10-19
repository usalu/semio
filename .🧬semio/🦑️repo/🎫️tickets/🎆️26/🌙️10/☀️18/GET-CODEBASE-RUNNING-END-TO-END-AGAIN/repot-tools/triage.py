import json,sys,glob,re,os
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
pre=sys.argv[1]; flt=sys.argv[2:] 
R={}
for f in glob.glob(C+"/🗑️generated/repot-out/%s-*.jsonl"%pre):
    for l in open(f,encoding="utf-8"):
        r=json.loads(l); R[r["file"]]=r
for f,r in sorted(R.items()):
    if r["rc"]==0: continue
    short=f.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/","")
    if flt and not any(x in short for x in flt): continue
    t=open(r["log"],encoding="utf-8").read() if "log" in r else ""
    lines=t.splitlines()
    fails=[l for l in lines if l.startswith("(fail)")]
    print("###",short,"pass",r["pass_"],"fail",r["fail"])
    for l in fails[:3]: print("   ",l[:150])
    shown=0
    for i,l in enumerate(lines):
        if re.match(r"\s*(error|TypeError|ReferenceError|SyntaxError|Error)\b",l) and not l.startswith("(fail)"):
            print("   E:"," | ".join(x.strip()[:200] for x in lines[i:i+4] if x.strip()))
            shown+=1
            if shown>=2: break
    for l in lines:
        m=re.search(r"at <anonymous> \((/[^)]+\.tsx?):(\d+):(\d+)\)",l)
        if m:
            try:
                L=open(m.group(1),encoding="utf-8").read().split("\n")[int(m.group(2))-1]; c=int(m.group(3))
                print("   @",m.group(1).split("/")[-2][:20],m.group(2),m.group(3),L[max(0,c-int(os.environ.get("W","200"))):c+90])
            except Exception as e: pass
            break
