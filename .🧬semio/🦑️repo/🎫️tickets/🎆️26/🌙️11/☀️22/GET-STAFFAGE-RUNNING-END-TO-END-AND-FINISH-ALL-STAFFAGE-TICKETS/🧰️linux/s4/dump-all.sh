#!/bin/sh
# 🔬️ Lists the finest coarse volume (1:200; 1:100 where 1:200 is missing) of every asset of assets.txt into 🗑️generated/linux-s4/dump, one bun process after the other inside one grant of the memory queue.
# usage: dump-all.sh [asset id ...]
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
R=$(cd "$HERE/../../../../../../../../.." && pwd)
OUT="$HERE/../../🗑️generated/linux-s4/dump"
mkdir -p "$OUT"
[ $# -gt 0 ] && IDS="$*" || IDS=$(cat "$HERE/assets.txt")
cd "$R"
for id in $IDS; do
  scale=200
  [ "$id" = used-microwave-oven ] && scale=100
  repo tools exec bun -- "$HERE/dump.ts" "$id" "$scale" > "$OUT/$id.txt" 2> "$OUT/$id.err" || echo "FAILED $id" >&2
  echo "dumped $id $(wc -l < "$OUT/$id.txt") lines"
done
