#!/bin/sh
# usage: numscan1.sh <repo-relative file...>: numeric literals >= 100 of the given files (read from SF_SRC or live), with 60 chars of context
here=$(cd "$(dirname "$0")" && pwd)
root=${SF_SRC:-/Users/ueli/Documents/semio}
list=$(mktemp)
for f in "$@"; do echo "$f" >> "$list"; done
out=$(mktemp)
bun "$here/numscan.ts" "$list" "$root" "$out"
python3 -I - "$out" <<'P'
import json,sys
for l in open(sys.argv[1]):
    d=json.loads(l)
    print("##",d["file"][-70:])
    for r in d["rows"]: print(" ",r["line"],r["value"],"|",r["ctx"][20:125])
P
rm -f "$list" "$out"
