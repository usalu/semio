#!/bin/sh
# usage: health-run.sh NAME TARGET [extra repo run flags via EXTRA]
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/🗑️generated/health"
mkdir -p "$G"
export PATH="$HOME/.local/bin:$PATH"
date +%T > "$G/$1.start"
/usr/bin/time -l repo run "$2" $EXTRA > "$G/$1.out" 2> "$G/$1.log"
echo "exit $?" >> "$G/$1.log"; date +%T > "$G/$1.end"
