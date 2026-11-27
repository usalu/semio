#!/bin/bash
# Samples the process tree under each slice worker (command naming slice-pool worker.ts) every 5 s: rss per direct child command class.
out="$1"; : > "$out"
while true; do
  ps -axo pid=,ppid=,rss=,command= > "$out.tmp"
  python3 - "$out.tmp" >> "$out" <<'PY'
import sys,time,collections
rows=[l.rstrip("\n").split(None,3) for l in open(sys.argv[1],encoding="utf8",errors="replace")]
rows=[(int(a),int(b),int(c),d) for a,b,c,d in (r+[""]*(4-len(r)) for r in rows) if a.isdigit()]
kids=collections.defaultdict(list);by={}
for p,pp,r,c in rows: kids[pp].append(p);by[p]=(r,c)
for p,(r,c) in by.items():
    if "slice-pool" in c and "worker.ts" in c:
        stack=[p];groups=collections.Counter();n=0
        while stack:
            q=stack.pop();rr,cc=by[q]
            kind="worker" if q==p else ("rustc" if "rustc" in cc else "cargo" if "cargo" in cc else "bun-child" if "bun" in cc else "tsc" if "tsc" in cc or "typescript" in cc else cc.split()[0].split("/")[-1][:20])
            groups[kind]+=rr;n+=1;stack.extend(kids[q])
        print(time.strftime("%H:%M:%S"),"worker",p,"procs",n,"MB",{k:round(v/1024) for k,v in groups.items()},"total",round(sum(groups.values())/1024))
PY
  sleep 5
done
