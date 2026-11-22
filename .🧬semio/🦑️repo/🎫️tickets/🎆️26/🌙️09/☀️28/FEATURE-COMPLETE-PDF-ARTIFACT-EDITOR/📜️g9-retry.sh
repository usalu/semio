#!/bin/sh
# usage: g9-retry.sh NAME repo-args... ; retries while the build tool itself is mid-edit
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR/🗑️generated/g9/r2"; n=$1; shift
for a in $(seq 1 30); do
  SEMIO_LANGUAGE=en repo "$@" --output=static > "$D/$n.$a.log" 2>&1; c=$?
  echo "attempt $a exit $c" >> "$D/$n.status"
  [ $c -eq 0 ] && break
  grep -q "slice worker ended\|Export named\|graph service could not\|stamps moved\|Closed host capacity Policy\|plan cannot continue\|Closed cache ledger\|resident service met\|ENOENT\|Cannot find module\|SyntaxError" "$D/$n.$a.log" || break
  /bin/sleep 60
done
echo "final $c" >> "$D/$n.status"
