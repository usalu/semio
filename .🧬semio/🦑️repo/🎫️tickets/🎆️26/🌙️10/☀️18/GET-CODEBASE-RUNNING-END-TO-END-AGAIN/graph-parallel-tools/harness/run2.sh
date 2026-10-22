#!/bin/sh
# usage: run2.sh WS_NAME WORKERS N TAG [VAR=value ...]     (HARNESS=nodes-run.mjs|deps-run.mjs, default nodes-run.mjs)
# One cold run of the real inference in a scratch cache, niced. A sampler (1 s) adds up the resident memory of the whole process tree, remembers the
# CPU time of every process and kills the tree above RSS_LIMIT_MB (default 3900). Output: the SUMMARY line of the run, then one ROLES line:
# wall_s, parent_cpu_s, workers_cpu_s, parent_peak_mb, worker_peak_mb (largest single worker), tree_peak_mb (largest sum at one instant), workers_seen.
HERE=$(cd "$(dirname "$0")" && pwd)
[ $# -ge 4 ] && [ -n "$4" ] || { echo "usage: run2.sh WS_NAME WORKERS N TAG" >&2; exit 2; }
WS=$1; WORKERS=$2; N=$3; TAG=$4; shift 4
BASE=${GP_OUTDIR:-/private/tmp/graph-parallel}
OUT=$BASE/$TAG
rm -rf "$OUT" "$OUT".*
LIMIT=${RSS_LIMIT_MB:-3900}
START=$(date +%s)
env BUN_JSC_forceRAMSize=${GP_RAM:-1073741824} BUN_JSC_numberOfGCMarkers=1 BUN_JSC_useConcurrentGC=0 SEMIO_HOST_CAPACITY=off GP_TOLERANT=1 SEMIO_GRAPH_WORKERS="$WORKERS" N="$N" GP_WORKSPACE="/private/tmp/graph-parallel/$WS" GP_SCRATCH="$OUT" OUT="$OUT" "$@" nice -n 10 bun $BUN_ARGS "$HERE/${HARNESS:-nodes-run.mjs}" > "$OUT.log" 2>&1 &
ROOT=$!
: > "$OUT.samples"
while kill -0 $ROOT 2>/dev/null; do
  NOW=$(date +%s)
  ps -axo pid=,ppid=,rss=,cputime= | awk -v root=$ROOT -v now=$NOW '
    function secs(t,  a, n) { n = split(t, a, ":"); return n == 3 ? a[1]*3600 + a[2]*60 + a[3] : a[1]*60 + a[2] }
    {ppid[$1]=$2; rss[$1]=$3; cpu[$1]=secs($4)}
    END {for (p in ppid) { q=p; while (q in ppid && q != root && q != 1) q=ppid[q]; if (q == root) print now, p, (p == root ? "P" : "W"), rss[p], cpu[p] } }' >> "$OUT.samples"
  LAST=$(awk -v now=$NOW '$1 == now {t += $4} END {print int(t/1024)}' "$OUT.samples")
  if [ "${LAST:-0}" -gt "$LIMIT" ]; then
    echo "RSS LIMIT $LAST MB > $LIMIT: killing tree" >> "$OUT.log"
    awk -v now=$NOW '$1 == now {print $2}' "$OUT.samples" | xargs kill 2>/dev/null
  fi
  sleep 1
done
wait $ROOT
END=$(date +%s)
grep -v "^      at" "$OUT.log" | grep -E "^SUMMARY|RSS LIMIT|rror" | tail -3 | cut -c1-${WIDTH:-300}
python3 - "$OUT.samples" "$((END-START))" <<'PY'
import sys, collections
rows = [l.split() for l in open(sys.argv[1]) if l.strip()]
byp = collections.defaultdict(lambda: {"role": "", "rss": 0, "cpu": 0.0})
instant = collections.defaultdict(int)
for now, pid, role, rss, cpu in ((r[0], r[1], r[2], int(r[3]), float(r[4])) for r in rows):
    d = byp[pid]
    d["role"] = role
    d["rss"] = max(d["rss"], rss)
    d["cpu"] = max(d["cpu"], cpu)
    instant[now] += rss
parent = [d for d in byp.values() if d["role"] == "P"]
workers = [d for d in byp.values() if d["role"] == "W"]
print("ROLES wall_s=%s parent_cpu_s=%.0f workers_cpu_s=%.0f parent_peak_mb=%d worker_peak_mb=%d tree_peak_mb=%d workers_seen=%d" % (
    sys.argv[2], sum(d["cpu"] for d in parent), sum(d["cpu"] for d in workers), max([d["rss"] for d in parent] or [0]) // 1024,
    max([d["rss"] for d in workers] or [0]) // 1024, max(instant.values() or [0]) // 1024, len(workers)))
PY
