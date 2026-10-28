#!/bin/sh
# usage: birth-probe.sh <listfile of build-relative test files> <outfile> -> for each file the first genuine birth (setup, setup-sdk, orchestration, model) under which it passes within 150 s ("none" otherwise); tab separated; resumable
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
B=$(ls /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun | head -1)
G="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts"
M="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
mkdir -p /private/tmp/claude-501/rg/tmp "$C/🗑️generated/route-g/birth"
unset SEMIO_SCRIPT_STALL_MS
while read -r f; do
  [ -z "$f" ] && continue
  /usr/bin/grep -qF "$f	" "$2" 2>/dev/null && continue
  found=none
  for b in setup setup-sdk orchestration model; do
    log="$C/🗑️generated/route-g/birth/$(printf '%s' "$f" | md5 -q)-$b.log"
    TMPDIR=/private/tmp/claude-501/rg/tmp SEMIO_WORKSPACE_ROOT=/Users/ueli/Documents/semio nice -n 10 "$B" "$G" "--file=$M/$f" "--birth=$b" > "$log" 2>&1 &
    pid=$!; n=0
    while kill -0 $pid 2>/dev/null && [ $n -lt 150 ]; do sleep 1; n=$((n+1)); done
    if kill -0 $pid 2>/dev/null; then kill $pid 2>/dev/null; sleep 1; kill -9 $pid 2>/dev/null; echo "timeout" >> "$log"; wait $pid 2>/dev/null; continue; fi
    wait $pid; if [ $? -eq 0 ]; then found=$b; break; fi
  done
  printf '%s\t%s\n' "$f" "$found" >> "$2"
done < "$1"
