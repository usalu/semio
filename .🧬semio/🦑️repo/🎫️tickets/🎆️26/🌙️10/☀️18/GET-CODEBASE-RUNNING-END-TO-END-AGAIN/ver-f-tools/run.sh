#!/bin/sh
# usage: run.sh <tag> <repo args...> — one official run, log + verdict row under 🗑️generated/ver-f
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$C/🗑️generated/ver-f"; mkdir -p "$out"
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
tag="$1"; shift
s=$(date +%s)
nice -n 10 bun repo "$@" > "$out/$tag.log" 2>&1
rc=$?
printf '%s\t%s\t%s\t%ss\t%s\n' "$tag" "$rc" "$(date +%T)" "$(( $(date +%s) - s ))" "$*" >> "$out/verdicts.tsv"
echo "$rc" > "$out/$tag.done"
