#!/bin/sh
# usage: sh regress.sh MIRROR_DIR LOG_PREFIX
# Runs the existing graph suites that exercise the touched files, one explicit file at a time, and prints pass/fail counts plus failing test names.
set -u
MIRROR=$1
PREFIX=$2
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
SUITES="
$B/🕸️graph/🚪️admission/🧪️tests/🟦️.ts
$B/🧪️tests/🕸️graph/🔄️candidate/🟦️.ts
$B/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts
$B/🧪️tests/🕸️graph/🛂️admission/🟦️.ts
$B/🧪️tests/🕸️graph/📌️pinned-publication/🟦️.ts
$B/🧪️tests/🕸️graph/🔭️observed-facts/🟦️.ts
$B/🧪️tests/🕸️graph/🧊️optional-cache/🟦️.ts
"
cd "$MIRROR"
export SEMIO_TEST_ARTIFACT_DIR="${SEMIO_TEST_ARTIFACT_DIR:-${TMPDIR:-/tmp}/x-graph}"
echo "$SUITES" | while IFS= read -r suite; do
  [ -z "$suite" ] && continue
  name=$(echo "$suite" | awk -F/ '{print $(NF-1)}')
  bun test "./$suite" > "$PREFIX.$name.log" 2>&1
  printf '%s: ' "$name"
  grep -E '^ *[0-9]+ (pass|fail)$' "$PREFIX.$name.log" | tr '\n' ' '
  echo
  grep -E '^\(fail\)' "$PREFIX.$name.log" | sed 's/ \[[0-9.]*m\?s\]$//' | sort > "$PREFIX.$name.fails"
done
