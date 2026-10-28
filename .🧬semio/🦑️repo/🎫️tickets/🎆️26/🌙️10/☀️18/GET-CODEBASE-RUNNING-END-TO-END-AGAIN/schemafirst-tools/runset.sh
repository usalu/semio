#!/bin/sh
# usage: runset.sh <mirror> <out file> <repo-relative dir or file>...: runs every test file below the paths through the envelope runner (mrun env), one summary line per file plus the failing names
here=$(cd "$(dirname "$0")" && pwd)
c=$(cd "$here/.." && pwd)
m=$1; out=$2; shift 2
root=$(python3 -I "$c/mirror-tools/mirror.py" list | awk -v n="$m" '$1==n{print $NF}')
: > "$out"
for p in "$@"; do
  find "$root/$p" -type f \( -name '🟦️.ts' -o -name '*.test.ts' \) -path '*🧪️*' 2>/dev/null | sort | while read -r f; do
    rel=${f#$root/}
    sh "$c/red-g-tools/mrun.sh" "$m" env "$rel" 2>&1 | sed "s#^$m env:#${rel##*📚️library/}:#" >> "$out"
  done
done
wc -l "$out"
