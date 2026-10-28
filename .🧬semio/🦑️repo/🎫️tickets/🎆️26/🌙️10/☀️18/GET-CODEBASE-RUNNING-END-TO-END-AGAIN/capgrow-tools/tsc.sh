#!/bin/sh
# 🔎 Strict tsc over the given mirror-relative files and their import closure; prints only errors in the given files plus a count: tsc.sh <mirror-root> <rel>...
M=$1; shift; cd "$M" || exit 2
printf '{"extends":"./tsconfig.json","compilerOptions":{"types":["bun"],"noEmit":true},"include":[],"files":[%s]}\n' "$(for f in "$@"; do printf '"%s",' "$f"; done | sed 's/,$//')" > tsconfig.cg.json
./node_modules/.bin/tsc -p tsconfig.cg.json --pretty false > .cg-tsc.out 2>&1; status=$?
total=$(grep -c "error TS" .cg-tsc.out); mine=0
for f in "$@"; do n=$(grep -F "$f(" .cg-tsc.out | grep -c "error TS"); mine=$((mine+n)); grep -F "$f(" .cg-tsc.out | grep "error TS" | cut -c1-400; done
echo "tsc exit $status: $total errors in closure, $mine in the named files"; rm -f tsconfig.cg.json
