#!/bin/sh
# 🧰 cbudget lane: the regression set of the budget clock, one bun test process per file; prints "<file-tail>: pass/fail lines" (usage: suite.sh [out-file])
here=$(cd "$(dirname "$0")" && pwd)
T="./🧰️framework/🔨️modules/🏃️process/🧪️testing"
out=${1:-/dev/stdout}
: > "$out"
for f in "$T/🎛️execution/🧪️tests/🟦️.ts" "$T/🧪️vitest/🧪️tests/🟦️.ts" "$T/🎚️budget/🧪️tests/🟦️.ts" "$T/🚦️protocol/🧪️tests/🟦️.ts" "$T/🟨️bun/🧯️ceiling/🧪️tests/🟦️.ts" "$T/🦀️cargo/🧪️tests/🟦️.ts" "$T/🦀️cargo/🧪️tests/🪜️order/🟦️.ts" "$T/👷️workers/🧪️tests/🟦️.ts" "$T/👷️workers/🧪️tests/🧱️capped-run/🟦️.ts"; do
  echo "== $f" >> "$out"
  sh "$here/mrun.sh" bun test "$f" 2>&1 | grep -E "^\((pass|fail)\)|^ *[0-9]+ (pass|fail)$" | sed -E 's/ \[[0-9.]+m?s\]//' >> "$out"
done
