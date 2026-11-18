import subprocess,sys,re,concurrent.futures as cf
R="/Users/ueli/Documents/semio";G=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=G+"🏗️build/🕸️graph/📥️inference/";SI=B+"🧭️script-inputs/";PC=SI+"🔌️provider-closures/🧪️tests/"
D=sys.argv[1];S=sys.argv[2]
extra=["🧷️closure-facts","🪢️walk-questions","🧭️walk-edits","✂️derived-walks","🔗️late-closures","🔗️workspace-closure","🧮️covered-inputs","🪤️take-no-derive","🧰️shared-prime","🧩️group-merge","🧮️module-memo","🧮️pass-verdicts","🪆️owner-child-closure","🪪️layer-ids","♻️owner-reuse","🦀️compiled-closure","🧺️bundle-rows"]
L=[PC+"🪡️tool-cover/🟦️.ts",PC+"🛃️preflight/🟦️.ts",SI+"🪆️heavy-questions/🧪️tests/🟦️.ts",SI+"🧪️tests/🫧️take-release/🟦️.ts",SI+"🧪️tests/🪁️program-reads/🟦️.ts",SI+"🧪️tests/closure-answer-refresh.test.ts",SI+"🧪️tests/walk-members-merge.test.ts",G+"🏗️build/🪪️graph-state/📦️project-records/🚚️entry-lane/🧪️tests/🟦️.ts",B+"📦️project-nodes/🧪️tests/📥️take/🟦️.ts",B+"📦️project-nodes/📥️take/🧪️tests/🟦️.ts"]+[B+"🧪️tests/"+e+"/🟦️.ts" for e in extra]
def run(i):
    f=L[i];r=subprocess.run([sys.executable,D+"/run.py",S+"/srun/%d"%i,f,"--existing","--staged",S+"/stage"],capture_output=True,text=True)
    t=r.stdout+r.stderr;p=re.findall(r"^ *(\d+) pass",t,re.M);fl=re.findall(r"^ *(\d+) fail",t,re.M);err=re.findall(r"^error: (.{0,110})",t,re.M)
    return "%s : %s pass %s fail %s"%(f.replace(G,""),p[-1] if p else "?",fl[-1] if fl else "?",err[:1])
with cf.ThreadPoolExecutor(8) as ex:
    for line in ex.map(run,range(len(L))):print(line)
