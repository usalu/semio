#!/bin/sh
# usage: chain.sh <batch-file>   lines: <id>|<projects comma list>|<targets comma list>   one official Bazel run-many per line (parallel=1), retried while the image fault is first, then once; verdicts in $O/chain.tsv
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/fin-d"
unset SEMIO_SCRIPT_STALL_MS
export SEMIO_LANGUAGE=en
while IFS='|' read -r id projects targets; do
  case "$id" in ''|'#'*) continue ;; esac
  rcs=""; n=1
  while [ $n -le 4 ]; do
    log="$O/$id.a$n.log"
    st=$(date +%H:%M:%S)
    nice -n 10 bun repo --tool=bazel run-many --projects="$projects" --targets="$targets" --parallel=1 > "$log" 2>&1
    rc=$?
    rcs="$rcs $rc"
    printf '%s\ta%s\trc=%s\t%s-%s\t%s\n' "$id" "$n" "$rc" "$st" "$(date +%H:%M:%S)" "$targets" >> "$O/chain.tsv"
    [ $rc -eq 0 ] && break
    if grep -q 'Sole exact committed recomputable' "$log"; then sleep 120; n=$((n+1)); continue; fi
    [ $n -ge 2 ] && break
    n=$((n+1))
  done
done < "$1"
echo "done $1 $(date +%H:%M:%S)" >> "$O/chain.tsv"
