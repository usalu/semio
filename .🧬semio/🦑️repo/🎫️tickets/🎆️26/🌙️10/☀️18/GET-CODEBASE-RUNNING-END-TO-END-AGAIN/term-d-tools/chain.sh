#!/bin/sh
# usage: chain.sh <prefix> <target>...   official Bazel runs of @semio-tech/repo-dashboard-rs:<target>, one at a time, each with retry-run; verdicts in <prefix>.chain
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/term-d"
p=$1; shift
for t in "$@"; do
  rm -f "$O/$p-$t.done"
  sh "$C/term-d-tools/retry-run.sh" "$p-$t" 3 --tool=bazel run "@semio-tech/repo-dashboard-rs:$t"
  echo "$(date +%H:%M:%S) $t $(cat "$O/$p-$t.done" 2>/dev/null)" >> "$O/$p.chain"
done
echo done >> "$O/$p.chain"
