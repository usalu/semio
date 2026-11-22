#!/bin/zsh
# usage: run-suite.sh <checkout> <outfile> <list-file>   (list: paths relative to checkout)
co="$1"; out="$2"; list="$3"
cd "$co" || exit 2
: > "$out"
while IFS= read -r f; do
  [ -z "$f" ] && continue
  start=$(date +%s)
  res=$(repo test-file --json "$f" 2>/dev/null)
  rc=$?
  end=$(date +%s)
  line=$(printf '%s' "$res" | python3 -c 'import sys,json
try:
  d=json.loads(sys.stdin.read()); r=d["files"][0]; print(r["status"], r.get("pass"), r.get("fail"), (r.get("error") or "")[:160].replace("\n"," "))
except Exception as e: print("noresult")' )
  echo "$((end-start))s rc=$rc $line :: ${f##*library/}" >> "$out"
done < "$list"
