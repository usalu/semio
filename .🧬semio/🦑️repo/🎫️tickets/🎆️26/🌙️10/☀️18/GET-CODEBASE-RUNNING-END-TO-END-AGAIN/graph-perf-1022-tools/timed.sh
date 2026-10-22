#!/bin/sh
# usage: GP_OUT=<dir> timed.sh <label> <cmd...>   (cwd = workspace root); writes <label>.{out,err(stamped),time,cpu.jsonl,rc} and appends timings.jsonl
label=$1; shift
out="$GP_OUT"; here=$(dirname "$0")
la=$(uptime | sed 's/.*averages: //')
mkfifo "$out/$label.fifo" 2>/dev/null
python3 "$here/stamp.py" <"$out/$label.fifo" >"$out/$label.err" &
stamper=$!
SEMIO_GRAPH_PHASES=1 /usr/bin/time -l -o "$out/$label.time" "$@" >"$out/$label.out" 2>"$out/$label.fifo" &
pid=$!
python3 "$here/treecpu.py" $pid "$out/$label.cpu.jsonl" 1 &
sampler=$!
wait $pid; rc=$?
wait $stamper; wait $sampler
rm -f "$out/$label.fifo"
echo $rc >"$out/$label.rc"
python3 - "$label" "$rc" "$la" "$out/$label.time" <<'PY' >>"$out/timings.jsonl"
import sys,json,re
l,rc,la,t=sys.argv[1:]
txt=open(t,errors="replace").read()
m=re.search(r"([\d.]+) real\s+([\d.]+) user\s+([\d.]+) sys",txt)
rss=re.search(r"(\d+)\s+maximum resident set size",txt)
print(json.dumps({"label":l,"rc":int(rc),"real":float(m[1]) if m else None,"user":float(m[2]) if m else None,"sys":float(m[3]) if m else None,"maxrssMB":round(int(rss[1])/1048576) if rss else None,"load":la}))
PY
