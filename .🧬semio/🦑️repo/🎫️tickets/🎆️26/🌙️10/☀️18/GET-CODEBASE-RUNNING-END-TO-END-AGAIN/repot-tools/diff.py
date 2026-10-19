import json,sys,glob
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
def load(pre):
    R={}
    for f in glob.glob(C+"/🗑️generated/repot-out/%s-*.jsonl"%pre):
        for l in open(f,encoding="utf-8"):
            r=json.loads(l); R[r["file"]]=r
    return R
a,b=load(sys.argv[1]),load(sys.argv[2])
for f in sorted(a):
    x,y=a[f],b.get(f)
    if y and (x["rc"],x["pass_"],x["fail"])!=(y["rc"],y["pass_"],y["fail"]):
        print(x["rc"],x["pass_"],x["fail"],"->",y["rc"],y["pass_"],y["fail"],f.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/","")[-110:])
