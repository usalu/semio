import json,sys,glob
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
pre=sys.argv[1]
R={}
for f in glob.glob(C+"/🗑️generated/repot2-out/%s-*.jsonl"%pre):
    for l in open(f,encoding="utf-8"):
        r=json.loads(l); R[r["file"]]=r
g=[r for r in R.values() if r["rc"]==0]
print("suites",len(R),"green",len(g),"pass",sum(r["pass_"] for r in R.values()),"fail",sum(r["fail"] for r in R.values()))
if len(sys.argv)>2:
    for r in sorted(R.values(),key=lambda r:r["file"]):
        if r["rc"]!=0: print(r["rc"],r["pass_"],r["fail"],r["secs"],r["file"].replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/",""))
