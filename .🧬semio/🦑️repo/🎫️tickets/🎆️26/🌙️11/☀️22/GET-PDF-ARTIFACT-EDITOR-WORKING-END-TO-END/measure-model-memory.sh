#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
tag=${1:?tag}
shift
out="$here/🗑️generated/model-mem-$tag"
mkdir -p "$out"
: > "$out/top.log"
: > "$out/ps.log"
touch "$out/running"
(
  while [ -f "$out/running" ]; do
    for pid in $(pgrep -f 'crate-name semio_s_artifact_stdio_pdf_model' 2>/dev/null || true); do
      footprint=$(top -l 1 -pid "$pid" -stats pid,mem 2>/dev/null | awk -v pid="$pid" '$1==pid {print $2}')
      rss=$(ps -o rss= -p "$pid" 2>/dev/null | awk '{printf "%d", $1/1024}')
      echo "$(date +%T) $pid footprint=$footprint rss=${rss}M" >> "$out/ps.log"
    done
    if ! pgrep -f 'crate-name semio_s_artifact_stdio_pdf_model' >/dev/null 2>&1; then sleep 2; fi
    sleep 1
  done
) &
sampler=$!
status=0
sh "$here/native-cargo.sh" "$@" > "$out/cargo.log" 2>&1 || status=$?
rm -f "$out/running"
wait "$sampler" 2>/dev/null || true
echo "exit $status" >> "$out/cargo.log"
tail -1 "$out/ps.log" > "$out/peak.txt"
cat "$out/peak.txt"
