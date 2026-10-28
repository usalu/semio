#!/bin/sh
# 🧪 capgrow product proof: a capacity-gated sleeper holds memory while the course builds with --skip-cache; the build must be admitted with fewer jobs and grow once the sleeper ends.
cd /Users/ueli/Documents/semio
free_mb=$(vm_stat | awk '/Pages free|Pages inactive|Pages speculative/ {s+=$NF} END {printf "%d", s*16384/1048576}')
hold=$((free_mb - 6000)); [ $hold -lt 2000 ] && hold=2000
echo "[capgrow-proof] holding $hold MB"
env -u CLAUDECODE bun repo capacity run --memory=$hold -- sleep 150 > "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gate-g-tools/run/g38-holder.log" 2>&1 &
sleep 15
python3 '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/actenv-tools/product_skip.py' g38-grow '@teaching/architecture-course:build' --skip-cache
wait
