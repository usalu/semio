#!/bin/sh
# 🧪 Runs every test file named in <list> inside mirror <name> under the runner envelope (red-g mrun.sh chain mode, artifact dir inside the mirror) and writes one line per file: index, pass/fail counts, and the sorted (fail) set to <out>/<label>.<index>.fails. usage: sh widesuites.sh <mirror-name> <list> <label> <out-dir> [from] [to]
NAME="$1"; LIST="$2"; LABEL="$3"; OUT="$4"; FROM="${5:-1}"; TO="${6:-100000}"; mkdir -p "$OUT"
C="$(cd "$(dirname "$0")/.." && pwd)"; n=0
while IFS= read -r s; do
  n=$((n+1)); [ $n -lt $FROM ] && continue; [ $n -gt $TO ] && break
  sh "$C/red-g-tools/mrun.sh" "$NAME" chain "$s" > "$OUT/$LABEL.$n.log" 2>&1
  grep -E "^\(fail\)" "$OUT/$LABEL.$n.log" | sed -E 's/ \[[0-9.]+m?s\]$//' | sort > "$OUT/$LABEL.$n.fails"
  echo "$n $(head -1 "$OUT/$LABEL.$n.log" | sed 's/^[^:]*: //' | cut -c1-40) $(echo "$s" | rev | cut -d/ -f2 | rev)"
done < "$LIST"
