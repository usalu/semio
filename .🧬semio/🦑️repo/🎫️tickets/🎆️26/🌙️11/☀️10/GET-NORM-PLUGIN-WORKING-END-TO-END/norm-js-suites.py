import json,os,glob
p="/home/deck/semio/✏️s/🔌️plugins/📕️norm"
plan=json.load(open(p+"/🧪️tests/🧫️fixtures/🏃️package-runner/🔣️.json"))
out=[]
for a in sorted(os.listdir(p+"/🗿️artifacts")):
    s=f"{p}/🗿️artifacts/{a}/🏅️standards/🔖️1/🪆️subsets/✳️any"
    ex=f"{s}/📚️examples"
    if os.path.isdir(ex):
        out+=[f"{ex}/{e}/🧪️tests/🧩️example/🟦️.ts" for e in sorted(os.listdir(ex))]
    out.append(f"{s}/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts")
out=[x for x in out if os.path.exists(x)]
out+=[f"{p}/{x}" for x in plan["suites"]]
print("\n".join(out))
