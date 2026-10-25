#!/bin/sh
# Bun-bundled package :test (counted bun-bundle slot) while a cargo build of a larger crate runs; every line timestamped
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_SCRIPT_STALL_MS=14400000
(nice -n 10 bun repo --tool=semio exec --projects=@semio-tech/sequence-sequence -- repo ./📜️script.ts test 2>&1; echo "rc=$?") | python3 "$C/cap-c-tools/ts.py" > "$C/🗑️generated/cap-c/overlap-bun.log" &
until /usr/bin/grep -aq "executing\|Executing\|repo-exec" "$C/🗑️generated/cap-c/overlap-bun.log" 2>/dev/null; do sleep 2; done
(SEMIO_AGENT_ID=cap-c-g nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check --manifest-path Cargo.toml -p semio-framework-os-store 2>&1; echo "rc=$?") | python3 "$C/cap-c-tools/ts.py" > "$C/🗑️generated/cap-c/overlap-cargo.log"
wait
