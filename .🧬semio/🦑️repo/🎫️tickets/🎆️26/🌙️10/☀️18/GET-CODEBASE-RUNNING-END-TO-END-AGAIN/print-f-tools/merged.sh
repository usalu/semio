#!/bin/sh
# waits for the running p4 bun (pid $1), then runs p2+p5 as ONE run-many (each invocation pays the graph queue); max 2 attempts
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/print-f"
unset SEMIO_SCRIPT_STALL_MS
export SEMIO_LANGUAGE=en
while kill -0 "$1" 2>/dev/null; do sleep 10; done
n=1
while [ $n -le 2 ]; do
  log="$O/p25.c$n.log"; st=$(date +%H:%M:%S)
  nice -n 10 bun repo --tool=bazel run-many --projects=@semio-tech/print,@semio-tech/mit-bestand-praesentation-projektetage --targets=check,build,test --parallel=1 > "$log" 2>&1
  rc=$?
  printf 'p25c\tb%s\trc=%s\t%s-%s\tcheck,build,test\n' "$n" "$rc" "$st" "$(date +%H:%M:%S)" >> "$O/chain.tsv"
  [ $rc -eq 0 ] && break
  n=$((n+1))
done
