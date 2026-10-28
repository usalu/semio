#!/bin/sh
# 🏃 Runs a dashboard test file in the lane mirror under the graph-free envelope: sh mrun.sh <log-name> <test-file> [bun test args...]
here=$(cd "$(dirname "$0")" && pwd); C=$(dirname "$here")
M=$(python3 -c "import os;print(os.path.realpath(os.path.expandvars('\$TMPDIR/sm/dhs')))")
rel="🗑️generated/dashhot"; A="$C/$rel/art"; mkdir -p "$A"; log="$C/$rel/$1.log"; file="$2"; shift 2
python3 -I "$C/mirror-tools/mirror.py" run dhs --seal-allow ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/$rel" -- bun "$here/env-run.ts" "$M" "@semio-tech/repo-dashboard-rs:test-source-freshness" "$A" -- test --timeout 300000 "$@" "$file" > "$log" 2>&1
grep -E "^\(pass\)|^\(fail\)|^ *[0-9]+ (pass|fail)|GUARD" "$log" | cut -c1-200
