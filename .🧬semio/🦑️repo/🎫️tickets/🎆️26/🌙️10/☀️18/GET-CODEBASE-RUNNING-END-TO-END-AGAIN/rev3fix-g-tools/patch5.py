import sys
p=sys.argv[1]+"/staged/rev3fix/verify.sh"
s=open(p,encoding="utf-8").read()
s=s.replace('nice -n 10 bun test "./$(basename "$1")" 2>&1','nice -n 10 bun test "./$(basename "$1")" --timeout 300000 2>&1')
s=s.replace('nice -n 10 bun test "./$f" 2>&1','nice -n 10 bun test "./$f" --timeout 300000 2>&1')
s=s.replace("# except 🧮️fact-index/🟦️.ts whose timing law is load dependent (red on the untouched tree under load too).","# (the timeout is raised because several of these laws are timing bound under the load of ~20 agents). Run on the real tree: the planner laws refuse a symlinked overlay.")
open(p,"w",encoding="utf-8").write(s)
