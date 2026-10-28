#!/bin/sh
# usage: many.sh <tag> <tool> <listfile of target names> <chunk> -> official `repo` test targets via run-many chunks (one graph plan per chunk), sequential, resumable per chunk; refuses a chunk while 4 or more runs wait in the capacity queue
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$C/🗑️generated/route-g/official"; mkdir -p "$out"
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
tag="$1"; tool="$2"; list="$3"; n="$4"
split -l "$n" -a 3 "$list" "$out/$tag-chunk."
for c in "$out/$tag-chunk."*; do
  case "$c" in *.rc|*.log) continue;; esac
  b=$(basename "$c"); [ -f "$out/$b.rc" ] && continue
  while [ "$(bun repo leases status 2>/dev/null | /usr/bin/grep -c WAITING)" -ge 4 ]; do sleep 120; done
  targets=$(paste -sd, "$c"); s=$(date +%s)
  nice -n 10 bun repo --tool="$tool" run-many --projects=repo --targets="$targets" --parallel=1 --exclude-dependencies > "$out/$b.log" 2>&1
  rc=$?; echo $rc > "$out/$b.rc"
  printf '%s\t%s\t%s\t%ss\t%s\n' "$b" "$rc" "$(date +%T)" "$(( $(date +%s) - s ))" "$targets" >> "$out/$tag.tsv"
done
echo done > "$out/$tag.done"
