#!/bin/sh
# usage: sweep-env.sh <outfile> <artifact dir> <file>...   same as sweep.sh but under the lease-proof process envelope (SEMIO_TEST_ARTIFACT_DIR set)
cd /Users/ueli/Documents/semio || exit 1
out="$1"; art="$2"; shift 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
for f in "$@"; do
  log=$(nice -n 10 perl -e 'alarm 300; exec @ARGV' -- bun "$C/lease-proof-tools/run-test.ts" "./$f" "" "$art" 2>&1)
  rc=$?
  p=$(printf '%s\n' "$log" | sed -n 's/^ *\([0-9][0-9]*\) pass$/\1/p' | tail -1)
  fl=$(printf '%s\n' "$log" | sed -n 's/^ *\([0-9][0-9]*\) fail$/\1/p' | tail -1)
  s=$(printf '%s\n' "$log" | sed -n 's/^ *\([0-9][0-9]*\) skip$/\1/p' | tail -1)
  printf '%s\t%s\t%s\t%s\t%s\n' "$f" "${p:-0}" "${fl:-0}" "${s:-0}" "$rc" >> "$out"
done
echo DONE >> "$out"
