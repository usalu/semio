import re,subprocess,sys,os
R="/Users/ueli/Documents/semio"
RUN={"RunScreen","TaskClock","TaskView","clockStage","usePassed","useRemaining","ClockStage"}
RES={"ResultsScreen","TaskResultView","deviation","tolerance"}
Q=R+"/🧰️framework/🛍️products/❓️quiz"
RUNP=Q+"/🎯️targets/⚛️react/🔨️modules/▶️run/🟦️.tsx"; RESP=Q+"/🎯️targets/⚛️react/🔨️modules/🏁️results/🟦️.tsx"
files=subprocess.run(["git","-c","core.fsmonitor=false","-c","core.quotepath=false","ls-files","--","*.ts","*.tsx"],cwd=R,capture_output=True,text=True).stdout.split("\n")
imp=re.compile(r'import\s+(type\s+)?\{([^}]*)\}\s*from\s*"@semio-tech/quiz-react"\s*;?',re.S)
write="--write" in sys.argv
for rel in files:
    if not rel or rel.startswith(".🧬semio"): continue
    p=R+"/"+rel
    if not os.path.exists(p): continue
    s=open(p,encoding="utf8").read()
    if "@semio-tech/quiz-react\"" not in s: continue
    out=s; changed=False
    for m in list(imp.finditer(s))[::-1]:
        whole_type=bool(m.group(1))
        specs=[x.strip() for x in m.group(2).split(",") if x.strip()]
        def base(x): return re.sub(r'^type\s+','',x).split(" as ")[0].strip()
        run=[x for x in specs if base(x) in RUN]; res=[x for x in specs if base(x) in RES]
        if not run and not res: continue
        keep=[x for x in specs if x not in run and x not in res]
        d=os.path.dirname(p)
        def line(items,target):
            t="type " if whole_type else ""
            return f'import {t}{{ {", ".join(items)} }} from "{os.path.relpath(target,d)}";'
        new=[]
        if keep: new.append(f'import {"type " if whole_type else ""}{{ {", ".join(keep)} }} from "@semio-tech/quiz-react";')
        if run: new.append(line(run,RUNP))
        if res: new.append(line(res,RESP))
        out=out[:m.start()]+"\n".join(new)+out[m.end():]; changed=True
    if changed:
        print(("WRITE " if write else "WOULD ")+rel)
        if write: open(p,"w",encoding="utf8").write(out)
