import json,sys,glob,re,collections
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
pre=sys.argv[1]
R={}
for f in glob.glob(C+"/🗑️generated/repot-out/%s-*.jsonl"%pre):
    for l in open(f,encoding="utf-8"):
        r=json.loads(l); R[r["file"]]=r
cnt=collections.Counter()
for f,r in sorted(R.items()):
    if r["rc"]==0: continue
    t=open(r["log"],encoding="utf-8").read() if "log" in r else ""
    errs=[l.strip() for l in t.splitlines() if re.match(r"\s*(error|TypeError|ReferenceError|SyntaxError|ENOENT|Error)\b",l.strip()) or l.strip().startswith("error:")]
    key=errs[0][:160] if errs else "?"
    print(f.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/","")[-90:],"|",key)
