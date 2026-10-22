#!/bin/sh
# 🏊️ Applies the graph farm set to the live tree and runs its laws, niced, one process at a time. Dry run unless --write.
# usage: sh go.sh [--round 2] [--write]
#   (no round)  round 1: the farm, sealed records, the verified-block memo
#   --round 2   round 2 on top of round 1 (applies round 1 first when it is pending): worker-written leaves, batched accounting, early start, resident pool
# Order of the sets on the live tree: cache-gc apply3, apply4, then this set (round 1, round 2). run `bun repo tools setup nx` by hand afterwards, as the chain does.
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
ROUND=1
case " $* " in *" --round 2 "*) ROUND=2;; esac
ARGS=""
for argument in "$@"; do case "$argument" in --round|2) ;; *) ARGS="$ARGS $argument";; esac; done

python3 "$HERE/ops_gen.py"
python3 "$HERE/apply.py" check || true
if [ "$ROUND" = 1 ]; then
  python3 "$HERE/apply.py" apply $ARGS
else
  python3 "$HERE/rehearse2.py"
  case " $* " in *" --write "*) ;; *) echo "dry run: nothing written"; exit 0;; esac
  python3 "$HERE/apply.py" apply $ARGS
  python3 "$HERE/ops2_gen.py"
  python3 "$HERE/apply.py" check --spec round2 || true
  python3 "$HERE/apply.py" apply --spec round2 $ARGS
fi

case " $* " in *" --write "*) ;; *) echo "dry run: nothing written"; exit 0;; esac
ROOT=$(cd "$HERE" && git rev-parse --show-toplevel)
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B="$L/🏗️build"
cd "$ROOT"
if [ "$ROUND" = 1 ]; then
  LAWS="$B/🏊️farm/🧪️tests/🟦️.ts $B/👁️observation/🧪️tests/🛰️detached/🟦️.ts $B/📥️inputs/💾️storage/🧪️tests/🛰️staging/🟦️.ts $B/🌡️host-capacity/🧪️tests/🏊️farm/🟦️.ts $B/🕸️graph/🕰️generations/🧪️tests/🏊️farm-merge/🟦️.ts $B/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🏊️farm-dispatch/🟦️.ts $B/🕸️graph/📥️inference/🕸️dependency-edges/🧪️tests/🏊️module-farm/🟦️.ts"
else
  LAWS="$B/🏊️farm/🧪️tests/🟦️.ts $B/🏊️farm/🧰️pool/🧪️tests/🟦️.ts $B/📥️inputs/💾️storage/🏊️leaves/🧪️tests/🟦️.ts $B/📥️inputs/💾️storage/🏊️leaves/🧪️tests/🏊️duplicate/🟦️.ts $B/📥️inputs/🦀️resolution/🧪️tests/🏊️receipts/🟦️.ts $B/👁️observation/🧪️tests/🛰️detached/🟦️.ts $B/🌡️host-capacity/🧪️tests/🏊️farm/🟦️.ts $B/🕸️graph/🕰️generations/🧪️tests/🏊️farm-merge/🟦️.ts $B/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🏊️farm-dispatch/🟦️.ts $B/🕸️graph/📥️inference/🕸️dependency-edges/🧪️tests/🏊️module-farm/🟦️.ts"
fi
for law in $LAWS; do
  nice -n 10 bun test "./$law"
done
