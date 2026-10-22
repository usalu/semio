#!/bin/sh
# 🏊️ Applies the graph farm set to the live tree and runs its laws, niced, one process at a time. Dry run unless --write.
# usage: sh go.sh [--write]     order: the set, then its laws; run `bun repo tools setup nx` by hand afterwards, as the chain does.
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
python3 "$HERE/ops_gen.py"
python3 "$HERE/apply.py" check || true
python3 "$HERE/apply.py" apply "$@"

case " $* " in *" --write "*) ;; *) echo "dry run: nothing written"; exit 0;; esac
ROOT=$(cd "$HERE" && git rev-parse --show-toplevel)
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B="$L/🏗️build"
cd "$ROOT"
for law in \
  "$B/🏊️farm/🧪️tests/🟦️.ts" \
  "$B/👁️observation/🧪️tests/🛰️detached/🟦️.ts" \
  "$B/📥️inputs/💾️storage/🧪️tests/🛰️staging/🟦️.ts" \
  "$B/🌡️host-capacity/🧪️tests/🏊️farm/🟦️.ts" \
  "$B/🕸️graph/🕰️generations/🧪️tests/🏊️farm-merge/🟦️.ts" \
  "$B/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🏊️farm-dispatch/🟦️.ts" \
  "$B/🕸️graph/📥️inference/🕸️dependency-edges/🧪️tests/🏊️module-farm/🟦️.ts"; do
  nice -n 10 bun test "./$law"
done
