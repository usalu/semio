#!/bin/zsh
# L-S4a: run one repo-lib test file with a wall-clock cap; usage: l-s4a-run.sh <tag> <test-name-folder> [extra bun test args…]
ROOT=/Users/ueli/Documents/semio
LIB="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
OUT="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/🗑️generated/launch-s4a"
tag="$1"; name="$2"; shift 2
mkdir -p "$OUT"
log="$OUT/$tag-$(print -r -- "$name" | LC_ALL=C sed 's/[^A-Za-z0-9-]//g').log"
cd "$ROOT"
perl -e 'alarm shift; exec @ARGV' "${CAP:-420}" bun test "$LIB/🧪️tests/$name/🟦️.ts" "$@" > "$log" 2>&1
code=$?
print -r -- "== $name exit=$code $(/usr/bin/grep -E '^ *[0-9]+ (pass|fail|skip|filtered out|todo)|^Ran ' "$log" | tr '\n' ' ')"
/usr/bin/grep -E '^\(fail\)' "$log" | cut -c1-200 | head -${FAILS:-12}
