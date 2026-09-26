#!/bin/zsh
# 🐍 LB: run one case's oracle role through the platform. oracle.sh <implementation> <capture-name> <case…>
cd /Users/ueli/Documents/semio || exit 2
impl="$1"; name="$2"; shift 2
out=".tmp-ticket/wp-lb/generated/$name.txt"; : > "$out"
export SEMIO_TEST_LEVEL=exhaustive NX_DAEMON=false PYTHONDONTWRITEBYTECODE=1 CARGO_INCREMENTAL=0
for c in "$@"; do
  s=$(date +%s)
  bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx run @semio-tech/repo-test-domain:test-oracle --outputStyle=stream -- --implementation "$impl" --case "$c" > "$out.$$.tmp" 2>&1
  code=$?
  echo "$c EXIT=$code $(( $(date +%s) - s ))s $(sed 's/\x1b\[[0-9;]*m//g' "$out.$$.tmp" | /usr/bin/grep -o 'executed=[0-9]* passed=[0-9]* failed=[0-9]* errored=[0-9]* parity=[0-9/]*' | tail -1)" >> "$out"
  sed 's/\x1b\[[0-9;]*m//g' "$out.$$.tmp" | /usr/bin/grep -E 'FAIL|✗|fail|error|Error' | head -40 >> "$out"
  mv "$out.$$.tmp" "${out%.txt}-$(echo $c | tr -cd "a-z0-9-").log"
done
echo "DONE $(date '+%H:%M:%S')" >> "$out"
