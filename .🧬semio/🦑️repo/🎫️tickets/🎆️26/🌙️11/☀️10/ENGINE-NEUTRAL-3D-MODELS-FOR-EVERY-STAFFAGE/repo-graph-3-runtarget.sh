#!/bin/sh
# usage: repo-graph-3-runtarget.sh NAME TARGET   runs `repo run TARGET` from the workspace root, logs under 🗑️generated/repo-graph-3
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE/🗑️generated/repo-graph-3"
date +%T > "$G/$1.start"
/usr/bin/time -l repo run "$2" > "$G/$1.out" 2> "$G/$1.log"
echo "exit $?" >> "$G/$1.log"; date +%T > "$G/$1.end"
