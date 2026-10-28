#!/bin/sh
# 🧭️ [td-ucache] runs every unit-cache test file in one root, bare and with SEMIO_TEST_ARTIFACT_DIR under C/🗑️generated/tu, one verdict line per file and mode.
# usage: classify.sh <root> <label> [file-filter-regex]   (TU_ART=<dir> overrides the artifact parent, e.g. a directory inside the mirror checkout)
R="$1"; L="$2"; F="${3:-.}"
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
U="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache"
O="$C/🗑️generated/tu/$L"; mkdir -p "$O"; P="${TU_ART:-$O}/$L"; mkdir -p "$P"
cd "$R/$U" || exit 1
find . -name '🟦️.ts' | sort | while read -r f; do
 /usr/bin/grep -q '"bun:test"' "$f" || continue
 echo "$f" | /usr/bin/grep -Eq "$F" || continue
 for mode in bare art; do
  key=$(printf '%s' "$f" | shasum | cut -c1-10)
  if [ "$mode" = art ]; then A="$P/a-$key"; mkdir -p "$A"; out=$(SEMIO_TEST_ARTIFACT_DIR="$A" bun test "$f" 2>&1); else out=$(bun test "$f" 2>&1); fi
  printf '%s\n' "$out" > "$O/$mode-$key.txt"
  printf '%-4s %s :: %s\n' "$mode" "$f" "$(printf '%s\n' "$out" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail|skip)$' | tr -s ' \n' ' ')"
  printf '%s\n' "$out" | /usr/bin/grep -E '^\(fail\)' | cut -c1-200 | sed 's/^/     /'
 done
done
