#!/bin/zsh
O=/Users/Shared/semio-store/v1/origins
date -u
for o in $O/g1-*; do n=${o:t}
 f=$o/ready-log-workspace-failed.txt
 printf "%s failedlog=%s | " $n "$(stat -f %Sm -t %H:%M $f 2>/dev/null)"
 grep -h "fleet delivered" $f 2>/dev/null | tail -1 | cut -c1-260 | tr '\n' ' '
 python3 - $o <<'P'
import json,sys
o=sys.argv[1]
try:
  d=json.load(open(o+"/pool.json")); print("| pool",d["state"],d.get("reason"),"children",d.get("children"),"claimed",d["claimed"],"pub",d["published"],"wave",str(d.get("wave"))[:8],"total",d.get("totalMegabytes"),"grant",d.get("grantMegabytes"))
except Exception as e: print("| pool ?")
try:
  r=json.load(open(o+"/ready.json")); print("   ready",r["ready"],r["stage"],r.get("reason",{}).get("code"),(r.get("tail") or [""])[-1][:100])
except Exception: pass
P
done
