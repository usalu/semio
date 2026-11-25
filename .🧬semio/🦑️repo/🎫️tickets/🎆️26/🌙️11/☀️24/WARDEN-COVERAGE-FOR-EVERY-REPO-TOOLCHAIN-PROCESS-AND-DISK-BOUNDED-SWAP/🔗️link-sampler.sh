#!/bin/sh
# 🔗️ Samples resident memory (KB) once per second of every compiler, driver and linker process (`rustc`, `cc`, `clang`, `ld`) and writes `<epoch> <pid> <ppid> <rssKB> <command>` lines to <out> until it is killed; used to find which process of a test-executable link peaks.
# Usage: link-sampler.sh <out-file>
out=$1
while :; do
  now=$(date +%s)
  ps -axo pid=,ppid=,rss=,comm= | awk -v now="$now" '$4 ~ /(rustc|\/ld|\/cc|clang|ld-prime|dsymutil)$/ && $3 > 100000 {print now, $1, $2, $3, $4}' >> "$out"
  sleep 1
done
