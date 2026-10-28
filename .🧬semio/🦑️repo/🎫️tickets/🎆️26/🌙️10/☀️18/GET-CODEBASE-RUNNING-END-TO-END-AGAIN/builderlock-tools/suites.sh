#!/bin/sh
# 👯 Failure-set runner of batch `multiwriter`: runs every neighbouring law file of the generation store on <root> one by one (no sub-filter) and writes `<out>/<label>.txt` = one line per failing test `<file> :: <test>` plus `FILE-ERROR <file>` when a file exits non-zero without a (fail) line.
# usage: sh suites.sh <root> <label> [<out dir>]   (default out: C/🗑️generated/builderlock)
set -u
root=$1; label=$2; here=$(cd "$(dirname "$0")" && pwd); out=${3:-$here/../🗑️generated/builderlock}
mkdir -p "$out"; result="$out/$label.txt"; : > "$result"
cd "$root" || exit 2
L=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
{
 find "$L/🕸️graph/🕰️generations" -name '🟦️.ts' -path '*🧪️tests*' ! -path '*🏃️world*' ! -path '*🧰️support*'
 find "$L/🕸️graph/🧪️tests" "$L/🧪️tests/🕸️graph" "$L/🕸️graph/🗂️generation-files/🧪️tests" -name '🟦️.ts' ! -path '*🧰️support*'
 echo "$L/📥️inputs/💾️storage/🧪️tests/🟦️.ts"
 echo "$L/🏡️resident/🧪️tests/🕰️graph-hooks/🟦️.ts"
 find "$L/🧭️invocation/🕸️graph/🧠️memo/🧪️tests" -name '🟦️.ts'
} | sort -u | while read -r file; do
 [ -f "$file" ] || continue
 log=$(bun test "$file" 2>&1); code=$?
 fails=$(printf '%s\n' "$log" | grep '^(fail)' | sed 's/ \[[0-9.]*m\{0,1\}s\]$//' | sed "s|^(fail) |$file :: |")
 if [ -n "$fails" ]; then printf '%s\n' "$fails" >> "$result"; elif [ $code -ne 0 ]; then echo "FILE-ERROR $file :: $(printf '%s\n' "$log" | grep -m1 -iE 'error|cannot' | cut -c1-200)" >> "$result"; fi
 printf '%s %s\n' "$code" "$file" >> "$out/$label.files"
done
sort -o "$result" "$result"
echo "[$label] $(wc -l < "$result") failing lines, $(wc -l < "$out/$label.files") files -> $result"
