#!/bin/sh
# usage: health-direct.sh NAME CONFIG [vitest filter...]   direct vitest under bun, logs under 🗑️generated/health
R=/Users/ueli/Documents/semio
G="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/🗑️generated/health"
name=$1; config=$2; shift 2
rt=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "$R"
date +%T > "$G/$name.start"
/usr/bin/time -l "$rt" "$R/node_modules/vitest/vitest.mjs" run --configLoader native --config "$config" --maxWorkers=2 "$@" > "$G/$name.log" 2>&1
echo "exit $?" >> "$G/$name.log"; date +%T > "$G/$name.end"
