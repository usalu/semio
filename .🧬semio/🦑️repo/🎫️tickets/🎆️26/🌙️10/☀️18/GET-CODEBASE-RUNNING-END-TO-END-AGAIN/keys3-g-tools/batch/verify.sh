#!/bin/sh
# keys3-g verify.sh: the laws of the batch, graph-free, ONE file at a time (explicit ./ paths: a bare path is a Bun filter that scans the whole disk).
# before the batch landed: `sh verify.sh staged` runs the laws against patched COPIES (apply.py stage); after apply.py apply: `sh verify.sh`.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; B="$C/keys3-g-tools/batch"
python3 "$B/apply.py" check
if [ "$1" = "staged" ]; then
  rm -rf "$C/🗑️generated/keys3-g/applied" && python3 "$B/apply.py" stage "$C/🗑️generated/keys3-g/applied" >/dev/null
  KEYS3_ROOT="$PWD/$C/🗑️generated/keys3-g/applied"; export KEYS3_ROOT
fi
nice bun test "./$B/laws/tailwind-scan.test.ts" 2>&1 | tail -6
nice bun test "./$B/laws/cargo-resolution-token.test.ts" 2>&1 | tail -6
