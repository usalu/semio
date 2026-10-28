#!/bin/sh
# usage: apply.sh [--dry|--revert|--check] -> lands (or removes) the staged suite sweep module `🏗️build/🧾️ci-report/🗂️suites` (NEW files only, no existing file is edited: no base hashes; a destination that exists with other bytes refuses the whole batch)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S="$C/ci-g-tools/staged/files"
mode="${1:-apply}"
bad=0; n=0
list=$(cd "$S" && find . -type f -not -name '.DS_Store' | sort)
for f in $list; do
  src="$S/${f#./}"; dst="${f#./}"; n=$((n+1))
  case "$mode" in
    apply|--dry) if [ -e "$dst" ] && ! cmp -s "$src" "$dst"; then echo "REFUSE (exists, other bytes): $dst"; bad=1; fi;;
    --revert) if [ -e "$dst" ] && ! cmp -s "$src" "$dst"; then echo "REFUSE (changed since staging): $dst"; bad=1; fi;;
    --check) if [ ! -e "$dst" ]; then echo "MISSING: $dst"; bad=1; elif ! cmp -s "$src" "$dst"; then echo "CHANGED: $dst"; bad=1; fi;;
  esac
done
[ "$bad" = 0 ] || { echo "no file touched"; exit 1; }
case "$mode" in
  --dry) echo "dry run: $n files would land, 0 conflicts";;
  --check) echo "check: $n files identical to the staged copy";;
  --revert) for f in $list; do rm -f "${f#./}"; done; rmdir "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🔀️workflow/🧫️fixtures" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🔀️workflow" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🐍️oracle" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🧫️fixtures" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🧪️tests" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🧬️schema" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/📖️texts" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites" 2>/dev/null; echo "reverted $n files";;
  apply) for f in $list; do dst="${f#./}"; mkdir -p "$(dirname "$dst")"; cp "$S/${f#./}" "$dst"; done; echo "landed $n files";;
esac
