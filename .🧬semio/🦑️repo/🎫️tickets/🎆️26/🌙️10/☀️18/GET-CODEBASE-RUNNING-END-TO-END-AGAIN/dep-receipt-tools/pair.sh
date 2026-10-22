#!/bin/sh
# usage: pair.sh <outdir> <attempt-label>  (cwd = repo root): edit a harmless library file, then run two concurrent `bun repo show targets --json`
out=$1; tag=$2; here=$(cd "$(dirname "$0")" && pwd)
F="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟦️.ts"
printf '\n' >>"$F"
echo "$tag edit $(date '+%H:%M:%S') appended newline to $F" >>"$out/pairs.log"
rm -f "$out/$tag-A".* "$out/$tag-B".*
nice -n 10 sh "$here/run1.sh" "$tag-A" "$out" bun repo show targets --json &
nice -n 10 sh "$here/run1.sh" "$tag-B" "$out" bun repo show targets --json &
wait
for n in A B; do echo "$tag-$n start=$(cut -d' ' -f2 "$out/$tag-$n.start") rc=$(cat "$out/$tag-$n.rc") entry-proof=$(grep -c 'Authenticated captured build command entry' "$out/$tag-$n.err") build-wait=$(cat "$out/$tag-$n.err" "$out/$tag-$n.out" | grep -c 'resource build\|Another process is building\|Ein anderer Prozess')" >>"$out/pairs.log"; done
