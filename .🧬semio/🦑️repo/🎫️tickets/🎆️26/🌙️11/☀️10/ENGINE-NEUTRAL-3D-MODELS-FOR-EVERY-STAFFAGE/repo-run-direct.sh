#!/bin/sh
# usage: repo-run-direct.sh NAME RUNTIME(node|bun) CONFIG [vitest filter...]   direct vitest, logs under 🗑️generated/repo-run
R=/Users/ueli/Documents/semio
G="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE/🗑️generated/repo-run"
name=$1; rt=$2; config=$3; shift 3
export PATH="$HOME/.local/bin:$PATH"
[ "$rt" = bun ] && rt=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "${RUNCWD:-$R}"
date +%T > "$G/$name.start"
"$rt" "$R/node_modules/vitest/vitest.mjs" run --configLoader native --config "$config" --maxWorkers=2 "$@" > "$G/$name.log" 2>&1
echo "exit $?" >> "$G/$name.log"; date +%T > "$G/$name.end"
