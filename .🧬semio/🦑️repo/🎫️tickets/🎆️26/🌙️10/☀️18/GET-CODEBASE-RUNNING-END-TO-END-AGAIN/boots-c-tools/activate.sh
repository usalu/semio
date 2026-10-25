#!/bin/sh
# usage: activate.sh <variant> [renderer=react] [profile=prototype]
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
V="$1"; R="${2:-react}"; P="${3:-prototype}"; T="${SEMIO_TOOL:-bazel}"
G="$C/🗑️generated/boots-c"
echo "[DEBUG] start $(date -u +%FT%TZ) $V $R $P"
nice -n 10 bun repo --tool="$T" run "@semio-tech/framework-os-dev:activate-$V-$R-$P" > "$G/activate-$V.out" 2>&1
echo "[DEBUG] exit=$? $(date -u +%FT%TZ)"
