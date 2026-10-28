#!/bin/sh
# 🧵 [cmdid] runs law files beneath the genuine `repo-lib:test-build-command-line` parent envelope (wire.ts) on <root>, artifact dir under the ticket 🗑️generated; prints "<pass>/<fail> <file>" plus failing names. usage: sh parent-run.sh <root> <repo-relative file>...
R="${1:?root}"; shift; here="$(cd "$(dirname "$0")" && pwd)"; ART="$here/../🗑️generated/cmdid/art"; mkdir -p "$ART"
wire=$(cd "$R" && bun "$here/wire.ts") || exit 1
for f in "$@"; do out=$(cd "$R" && SEMIO_TEST_ARTIFACT_DIR="$ART" SEMIO_SCRIPT_PROCESS_INVOCATION="$wire" bun test --timeout 170000 "./$f" 2>&1 </dev/null)
 echo "$(printf '%s\n' "$out" | grep -E '^ *[0-9]+ pass$' | tail -1 | tr -dc 0-9)/$(printf '%s\n' "$out" | grep -E '^ *[0-9]+ fail$' | tail -1 | tr -dc 0-9) $f"
 printf '%s\n' "$out" | grep -E '^\(fail\)|^error:' | sed 's/ \[[0-9.]*ms\]$//' | sort -u | cut -c1-200; done
