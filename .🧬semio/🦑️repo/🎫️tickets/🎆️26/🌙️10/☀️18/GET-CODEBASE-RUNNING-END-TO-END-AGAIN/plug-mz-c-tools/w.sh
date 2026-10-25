#!/bin/sh
# usage: w.sh TAG   wasm32 check of the puzzle hub component closure (JSON errors)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/plug-mz-c"
cd /Users/ueli/Documents/semio || exit 1
rm -f "$G/$1.log" "$G/$1.log.rc" "$G/$1.log.attempts"
export SEMIO_SCRIPT_STALL_MS=14400000
python3 "$C/chain-tools/detach.py" "$G/$1.launch.log" sh "$C/plug-mz-c-tools/run2.sh" "$G/$1.log" check --manifest-path "🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust/Cargo.toml" --target wasm32-unknown-unknown --keep-going -p semio-hub-puzzle-component
