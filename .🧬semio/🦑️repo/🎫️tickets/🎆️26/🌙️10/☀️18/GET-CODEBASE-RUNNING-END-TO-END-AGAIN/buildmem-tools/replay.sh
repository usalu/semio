#!/bin/sh
# [buildmem] Replay law of batch `childheap` (graph-free, read-only on live data): infers the 36 heavy projects of the 10-27 print refresh (refresh-roots.txt) in ONE process twice, with the JavaScriptCore RAM size of the batch (2 GiB) and of the old grant-sized child (8 GiB, an 8190 MB grant), then compares every owner's observation rows (kind, path, fact; per-owner sha256) and prints wall / peak RSS / footprint of both. Exit 1 when any owner differs. D1=1 applies the queued D1 batches in memory first.
# usage: sh replay.sh <label-prefix> [D1=1]
cd "$(dirname "$0")/.." || exit 1
G=$PWD; O=$G/🗑️generated/buildmem; p=$1; shift
R=$(cat "$G/buildmem-tools/refresh-roots.txt")
EXTRA=""; for a in "$@"; do [ "$a" = "D1=1" ] && EXTRA=1; done
for v in F:2147483648 P:8587837440; do
  l=$p${v%%:*}; ram=${v#*:}
  if [ -n "$EXTRA" ]; then
    sh buildmem-tools/run.sh "$l" PROBES=0 "ONLY_ROOTS=$R" SEMIO_GRAPH_WORKERS=0 'SEMIO_HOST_CAPACITY_PLAN={"jobs":8,"memoryMegabytes":8190}' BUN_JSC_forceRAMSize=$ram "$(cat buildmem-tools/d1.env)" PRELOADS=$G/walk-g-tools/staged/preload_generic.ts ROWS_DIGEST=1 ROWS_OUT=$O/$l.rows.json WALL_S=500 RSS_CAP_MB=16000 SEMIO_LANGUAGE=en >/dev/null 2>&1
  else
    sh buildmem-tools/run.sh "$l" PROBES=0 "ONLY_ROOTS=$R" SEMIO_GRAPH_WORKERS=0 'SEMIO_HOST_CAPACITY_PLAN={"jobs":8,"memoryMegabytes":8190}' BUN_JSC_forceRAMSize=$ram ROWS_DIGEST=1 ROWS_OUT=$O/$l.rows.json WALL_S=500 RSS_CAP_MB=16000 SEMIO_LANGUAGE=en >/dev/null 2>&1
  fi
done
python3 -I buildmem-tools/summary.py ${p}F ${p}P
python3 -I -c "
import json,sys
a=json.load(open('$O/${p}F.rows.json'));b=json.load(open('$O/${p}P.rows.json'))
d=sorted(k for k in set(a)|set(b) if a.get(k)!=b.get(k))
print('owners',len(a),'differing',len(d),d[:5])
sys.exit(1 if d else 0)"
