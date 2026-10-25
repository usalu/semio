#!/bin/sh
# usage: chain.sh <batch-file>  lines `id|project|targets`; one official Bazel run-many per line (parallel=1, nice 10); retries on peer-churn races; verdicts in generated/dash-f/chain.tsv
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/dash-f"; mkdir -p "$O"
unset SEMIO_SCRIPT_STALL_MS
export SEMIO_LANGUAGE=en
while IFS='|' read -r id projects targets; do
  case "$id" in ''|'#'*) continue ;; esac
  n=1
  while [ $n -le 3 ]; do
    st=$(date +%H:%M:%S); log="$O/$id.b$n.$(date +%H%M%S).log"
    nice -n 10 bun repo --tool=bazel run-many --projects="$projects" --targets="$targets" --parallel=1 > "$log" 2>&1; rc=$?
    printf '%s\tb%s\trc=%s\t%s-%s\t%s\n' "$id" "$n" "$rc" "$st" "$(date +%H:%M:%S)" "$targets" >> "$O/chain.tsv"
    [ $rc -eq 0 ] && break
    if /usr/bin/grep -q "Stale loaded Source revision\|roster changed\|Source descriptor or path changed\|restart required\|Store Source fence changed\|Current shared Source-owner generation changed\|outdated implementation\|Uncaptured implementation import refused\|Undeclared compiler-captured physical Source read refused\|ENOENT: no such file or directory, lstat\|Another process kept editing\|lock file of dependency scope\|Bun refused the frozen" "$log"; then sleep 120; n=$((n+1)); continue; fi
    break
  done
done < "$1"
echo "done $(date +%H:%M:%S)" >> "$O/chain.tsv"
