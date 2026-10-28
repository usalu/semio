#!/bin/sh
# verify.sh: the laws of the batch, graph-free, one file at a time (explicit ./ paths: a bare path is a Bun filter that scans the whole disk). Run AFTER apply.py apply; the F1 law tests the live module.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; B="$C/undeclared-g-tools/batch"
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
python3 "$B/apply.py" check
UNDECLARED_G_MODULE="$L/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs" nice bun test "./$B/laws/resolution-configs.test.ts" 2>&1 | tail -6
nice bun test "./$B/laws/owner-extensions.test.ts" 2>&1 | tail -6
nice bun test "./$C/undeclared-g-tools/hybrid/tests/hybrid.test.ts" 2>&1 | tail -6
