#!/bin/zsh
# usage: plan-line.sh N LOG  -- runs line N of cargo-fix plan-commands.txt in lane rustsync (foreground)
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cmd=$(sed -n "${1}p" "$C/cargo-fix-tools/plan/plan-commands.txt" | sed 's/^ *//')
export SEMIO_CARGO_LANE=rustsync
eval "nice -n 10 $cmd" > "$2" 2>&1
echo "rc=$?" >> "$2"
