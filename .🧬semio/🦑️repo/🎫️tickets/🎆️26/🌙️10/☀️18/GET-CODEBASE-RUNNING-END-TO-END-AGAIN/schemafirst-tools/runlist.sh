#!/bin/sh
# usage: runlist.sh <mirror> <out> <listfile> <from> <to>: runs the test files of lines from..to of the list through the envelope runner, one summary line per file
here=$(cd "$(dirname "$0")" && pwd)
c=$(cd "$here/.." && pwd)
m=$1; out=$2; list=$3; from=$4; to=$5
: > "$out"
sed -n "${from},${to}p" "$list" | while read -r rel; do
  sh "$c/red-g-tools/mrun.sh" "$m" env "$rel" 2>&1 | sed "s#^$m env:#${rel##*📚️library/}:#" >> "$out"
done
wc -l "$out"
