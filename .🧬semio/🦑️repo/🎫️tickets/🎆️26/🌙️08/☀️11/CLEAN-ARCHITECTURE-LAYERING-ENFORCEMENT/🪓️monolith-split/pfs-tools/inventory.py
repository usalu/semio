import os, subprocess, sys
R="/Users/ueli/Documents/semio"
paths=[
 "🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance",
 "🧰️framework/🛍️products/🐾️pets/🧬️schema",
 "🧰️framework/🛍️products/🐾️pets/🔨️modules/🚶️locomotion",
 "🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript",
 "🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar",
 "🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/🔬️probes",
 "🧰️framework/🛍️products/❓️quiz/🧪️tests",
 "🎓️teaching/🛂️proctor/🧪️tests/🌐️end-to-end",
]
out=subprocess.run(["git","-c","core.fsmonitor=false","-c","core.quotepath=false","status","--short","--untracked-files=all","--"]+paths,cwd=R,capture_output=True,text=True).stdout.split("\n")
rows=[]
for l in out:
    if not l.strip(): continue
    st=l[:2]; p=l[3:]
    ap=os.path.normpath(os.path.join(R,p)) if not os.path.isabs(p) else p
    rows.append((st,p))
# normalise relative paths printed relative to cwd by running from R
for st,p in rows:
    f=os.path.join(R,p)
    n=sum(1 for _ in open(f,encoding="utf-8",errors="replace")) if os.path.isfile(f) else -1
    print(f"{st}\t{n}\t{len(p.encode())}\t{p}")
