#!/bin/sh
# usage: run-prof.sh <tag> <repo args...>; like run-timed.sh plus a CPU profile (markdown) of every bun process into 🗑️generated/noop-g/prof-<tag>/
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/noop-g"; tag="$1"; shift
mkdir -p "$G/prof-$tag"; date +%s > "$G/$tag.start"
BUN_OPTIONS="--cpu-prof --cpu-prof-md --cpu-prof-dir=$PWD/$G/prof-$tag" SEMIO_REPO_PROFILE=1 nice -n 10 bun repo "$@" 2>&1 | python3 "$C/noop-g-tools/ts.py" > "$G/$tag.ts"
date +%s > "$G/$tag.end"
