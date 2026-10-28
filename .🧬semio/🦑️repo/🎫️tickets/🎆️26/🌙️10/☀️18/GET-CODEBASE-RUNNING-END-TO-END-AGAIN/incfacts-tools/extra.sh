#!/bin/sh
# 🧪️ [incfacts] runs the second ring of suites that read the touched component sources (from the root, so suites that read root files work) and writes their failure sets to C/🗑️generated/incfacts/extra-<label>/. usage: sh extra.sh <root> <label>
R="$(cd "$1" && pwd -P)"; L="$2"; C="$(cd "$(dirname "$0")/.." && pwd)"; G="$C/🗑️generated/incfacts/extra-$L"; rm -rf "$G"; mkdir -p "$G" "$R/.tmp/ifn"
want=$(sed -n 's/.*"packageManager": "bun@\([^"]*\)".*/\1/p' "$R/package.json"); B="${BUN:-}"; [ -n "$B" ] || for b in "$HOME"/Library/Caches/semio/runtimes/bun/*/bun "$HOME"/.cache/semio/runtimes/bun/*/bun; do [ -x "$b" ] && [ "$("$b" --version)" = "$want" ] && B="$b" && break; done
L1="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
for t in "$L1/🏗️build/🧪️tests/🧂️hoisted-literals" "$L1/🏗️build/🕸️graph/🧪️tests/🧳️carried-closures" "$L1/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests" "$L1/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🧵️unit-cache-lanes" "$L1/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests" "$L1/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📇️formats"; do
  slug=$(printf '%s' "$t" | shasum | cut -c1-8); out="$G/$slug.txt"
  (cd "$R" && SEMIO_TEST_ARTIFACT_DIR="$R/.tmp/ifn" nice -n 10 "$B" test --timeout 300000 "./$t/🟦️.ts") > "$out" 2>&1
  grep -E '^\(fail\)' "$out" | sed -E 's/ \[[0-9.]+m?s\]$//' | sort > "$G/$slug.fails"
  printf '%s %s: %s\n' "$slug" "$(basename "$t")" "$(grep -E '^ *[0-9]+ (pass|fail)$' "$out" | tr -s ' \n' ' ')"
done
