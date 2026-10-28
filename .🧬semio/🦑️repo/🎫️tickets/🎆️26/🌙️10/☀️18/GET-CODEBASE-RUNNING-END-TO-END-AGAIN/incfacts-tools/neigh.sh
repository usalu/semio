#!/bin/sh
# 🧭️ [incfacts] runs the neighbour suites of batch `libfacts` on a root, one bare `bun test` per suite under the pinned bun with an artifact dir inside the root, and writes the FAILURE SET per suite (names of failing tests) to C/🗑️generated/incfacts/neigh-<label>/; compare two labels with `diff -r`. usage: sh neigh.sh <root> <label>
R="$(cd "$1" && pwd -P)"; L="$2"; C="$(cd "$(dirname "$0")/.." && pwd)"; G="$C/🗑️generated/incfacts/neigh-$L"; rm -rf "$G"; mkdir -p "$G" "$R/.tmp/ifn"
want=$(sed -n 's/.*"packageManager": "bun@\([^"]*\)".*/\1/p' "$R/package.json"); B="${BUN:-}"; [ -n "$B" ] || for b in "$HOME"/Library/Caches/semio/runtimes/bun/*/bun "$HOME"/.cache/semio/runtimes/bun/*/bun; do [ -x "$b" ] && [ "$("$b" --version)" = "$want" ] && B="$b" && break; done
L1="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; I="$L1/🕸️graph/📥️inference"
for d in "$I/🧪️tests" "$I/🧪️tests/✂️derived-walks" "$I/🧪️tests/🍱️shared-setup" "$I/🧪️tests/🎓️course-inputs" "$I/🧪️tests/💾️cache" "$I/🧪️tests/📐️declared-inputs" "$I/🧪️tests/🔌️providers" "$I/🧪️tests/🔗️late-closures" "$I/🧪️tests/🔗️workspace-closure" "$I/🧪️tests/🗜️interned-closures" "$I/🧪️tests/🚧️withheld-rows" "$I/🧪️tests/🚶️repository-walks" "$I/🧪️tests/🦀️compiled-closure" "$I/🧪️tests/🧭️walk-edits" "$I/🧪️tests/🧮️covered-inputs" "$I/🧪️tests/🧷️closure-facts" "$I/🧪️tests/🪆️owner-child-closure" "$I/📦️project-nodes/🏊️project-farm/🧪️tests" "$I/🧭️script-inputs/🧪️tests" "$I/🧭️script-inputs/👓️walk-views/🧪️tests" "$I/🧭️script-inputs/🔌️provider-closures/🧪️tests" "$L1/📥️inputs/🧪️tests" "$L1/📥️inputs/🎯️precision/🧪️tests" "$L1/👁️observation/🧪️tests" "$L1/🕸️graph/🕰️generations/🧪️tests" "$L1/🧪️tests/🕸️graph/🎯️source-demand" "$I/🧪️tests/🖨️print-documents"; do
  [ -f "$d/🟦️.ts" ] || { echo "missing $d"; continue; }
  slug=$(printf '%s' "${d#$R/}" | shasum | cut -c1-8); out="$G/$slug.txt"
  (cd "$d" && SEMIO_TEST_ARTIFACT_DIR="$R/.tmp/ifn" nice -n 10 "$B" test --timeout 300000 ./🟦️.ts) > "$out" 2>&1
  grep -E '^\(fail\)' "$out" | sed -E 's/ \[[0-9.]+m?s\]$//' | sort > "$G/$slug.fails"
  printf '%s %s: %s\n' "$slug" "$(basename "$(dirname "$d")")/$(basename "$d")" "$(grep -E '^ *[0-9]+ (pass|fail)$' "$out" | tr -s ' \n' ' ')"
done
