#!/bin/sh
# usage: chain.sh <batch-file>   lines: <id>|<projects comma list>|<targets comma list>; one official Bazel run-many per line (parallel=1); verdicts in $O/chain.tsv
# gate: no foreign native image mounted under .cache/b/*/m/tmp (alloc-f root cause); infra faults retried up to 6 times
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/course-f"
unset SEMIO_SCRIPT_STALL_MS
export SEMIO_LANGUAGE=en
while IFS='|' read -r id projects targets; do
  case "$id" in ''|'#'*) continue ;; esac
  n=1
  while [ $n -le 6 ]; do
    g=0
    while mount | grep -aq '/\.cache/b/./m/tmp/.*pc-' && [ $g -lt 240 ]; do sleep 15; g=$((g+1)); done
    log="$O/$id.a$n.log"
    st=$(date +%H:%M:%S)
    nice -n 10 bun repo --tool=bazel run-many --projects="$projects" --targets="$targets" --parallel=1 > "$log" 2>&1
    rc=$?
    printf '%s\ta%s\trc=%s\t%s-%s\t%s\n' "$id" "$n" "$rc" "$st" "$(date +%H:%M:%S)" "$targets" >> "$O/chain.tsv"
    [ $rc -eq 0 ] && break
    if grep -aq 'Exact original parent child allocation required\|Sole exact committed recomputable\|kept editing\|outdated implementation\|frozen\|Complete prerequisite request required\|Closed generation pointer required' "$log"; then sleep 120; n=$((n+1)); continue; fi
    break
  done
done < "$1"
echo "done $1 $(date +%H:%M:%S)" >> "$O/chain.tsv"
