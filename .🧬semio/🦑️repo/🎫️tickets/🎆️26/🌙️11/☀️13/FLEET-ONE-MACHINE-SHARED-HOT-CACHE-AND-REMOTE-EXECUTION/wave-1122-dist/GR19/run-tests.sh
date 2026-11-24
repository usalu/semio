#!/bin/zsh
# usage: run-tests.sh slug... ; one repo test-file per slug (first test file under the member), sequential
R=/Users/ueli/Documents/semio; B="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
cd $R
unsetopt nomatch
for slug in "$@"; do
  dir=$(find "$B/🕸️graph" "$B/🪪️graph-state" "$B/👁️observation" -maxdepth 9 -type d -name "*$slug" 2>/dev/null | while read d; do if [ -f "$d/🟦️.ts" ] || ls "$d"/*.test.ts >/dev/null 2>&1; then echo "$d"; break; fi; done | head -1)
  if [ -z "$dir" ]; then echo "$slug: NO DIRECTORY"; continue; fi
  file="$dir/🟦️.ts"; [ -f "$file" ] || file=$(ls "$dir"/*.test.ts | head -1)
  out=$(repo test-file "$file" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$|^(passed|FAILED) ")
  echo "$slug: $(echo $out | tr '\n' ' ' | sed -E 's# [^ ]*/🧪️tests[^ ]* # #' | cut -c1-120)"
done
