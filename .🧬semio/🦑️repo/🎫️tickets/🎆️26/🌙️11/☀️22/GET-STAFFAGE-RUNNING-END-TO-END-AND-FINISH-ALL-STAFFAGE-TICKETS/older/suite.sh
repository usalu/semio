#!/usr/bin/env bash
# 🧪️ Runs staffage vitest suites by name filter with the pinned bun: suite.sh <output-name> <seconds> <filter…>; output in 🗑️generated/older/<output-name>.txt. Every run waits in the machine-wide memory queue (`repo capacity run`, SUITE_MEMORY_MB, default 2500).
set -u
here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../../../../../../../.." && pwd)"
bun="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun 2>/dev/null | head -1)"
[ -n "$bun" ] || bun="$(command -v bun)"
name="$1"; seconds="$2"; shift 2
out="$here/../🗑️generated/older"
mkdir -p "$out"
cd "$root" || exit 2
date "+start %H:%M:%S" > "$out/$name.txt"
repo capacity run --memory="${SUITE_MEMORY_MB:-2500}" -- timeout "$seconds" "$bun" node_modules/vitest/vitest.mjs run --configLoader native --reporter=verbose --config "🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎚️config/🟦️.ts" "$@" >> "$out/$name.txt" 2>&1
code=$?
echo "exit $code" >> "$out/$name.txt"
date "+end %H:%M:%S" >> "$out/$name.txt"
exit $code
