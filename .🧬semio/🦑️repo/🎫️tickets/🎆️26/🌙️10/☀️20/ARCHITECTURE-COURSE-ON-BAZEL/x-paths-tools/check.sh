#!/bin/bash
# usage: check.sh [workdir]
# Verifies the staged short-path patch without touching the checkout: builds an unpatched and a patched overlay
# (copies of the build library, symlinks for the rest), runs the new tests and the regression set on both, compares
# the aggregator's failing test names and type-checks the touched files. Set SEMIO_TEST_BAZEL_BINARY to also run the
# real-Bazel tests (the brief forbids that for executors; the coordinator may).
set -u
here="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$here" rev-parse --show-toplevel)"
work="${1:-${TMPDIR:-/tmp}/xp-check}"
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
BZ="$LIB/🏗️build/🔧️tools/🏗️bazel"
BS="$LIB/⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel"
mkdir -p "$work"
overlay() {
  python3 "$here/mkoverlay.py" "$root" "$work/$1" "$LIB" >/dev/null || exit 1
  if [ -d "$root/node_modules/ajv" ]; then rm -rf "$work/$1/node_modules"; ln -s "$root/node_modules" "$work/$1/node_modules"
  else python3 "$here/mknodemodules.py" "$root" "$work/$1" "$here/closure-packages.json" ajv fast-deep-equal fast-uri json-schema-traverse require-from-string iconv-lite safer-buffer minimatch brace-expansion balanced-match typescript=5.9.3 @iarna/toml >/dev/null; fi
}
overlay base; overlay patched
python3 "$here/apply.py" --root "$work/patched" | grep -v "^applied" ; python3 "$here/apply.py" --root "$work/patched" --check | tail -1
tests=("$BZ/🧪️tests/📥️paths/🟦️.ts" "$BZ/🧪️tests/📥️platform/🟦️.ts" "$BZ/🧪️tests/📥️workspace/🟦️.ts" "$BZ/🧪️tests/📥️sources/🟦️.ts" "$BZ/🧪️tests/📥️actions/🟦️.ts" "$BZ/🧪️tests/📥️inventory/🟦️.ts" "$BZ/🧪️tests/📥️fingerprint/🟦️.ts" "$BZ/🧪️tests/📥️delegation/🟦️.ts" "$BZ/🧪️tests/🔏️implementation/🟦️.ts" "$BZ/🧪️tests/📥️coverage/🟦️.ts" "$BZ/🧪️tests/📥️outputs/🟦️.ts" "$BZ/🧪️tests/📥️closure/🟦️.ts" "$BZ/🏃️execution/🧪️tests/🟦️.ts" "$BZ/🧭️commands/🧪️tests/🟦️.ts" "$BZ/🎯️affected/🧪️tests/🟦️.ts" "$BZ/💾️storage/🧪️tests/🟦️.ts" "$BS/🩺️readiness/🧪️tests/🟦️.ts")
status=0
for name in base patched; do
  : > "$work/$name.txt"
  for f in "${tests[@]}"; do
    [ -f "$work/$name/$f" ] || { echo "$f :: absent" >> "$work/$name.txt"; continue; }
    result=$(cd "$work/$name" && bun test "./$f" 2>&1 | grep -E "^ *[0-9]+ (pass|fail|skip)$" | tr -s ' ' | tr '\n' ' ')
    echo "$(basename "$(dirname "$(dirname "$f")")")/$(basename "$(dirname "$f")") ::$result" >> "$work/$name.txt"
  done
  (cd "$work/$name" && bun test "./$BZ/🧪️tests/🟦️.ts" > "$work/$name.aggregate.txt" 2>&1)
  grep "^(fail)" "$work/$name.aggregate.txt" | sed 's/ \[[0-9.]*ms\]//' | sort > "$work/$name.failures.txt"
done
echo "== regression counts (base | patched)"; diff <(sed 's/^\(.*\) ::/\1/' "$work/base.txt") <(sed 's/^\(.*\) ::/\1/' "$work/patched.txt") && echo "identical"
echo "== aggregator failing names: new failures in patched"; comm -13 "$work/base.failures.txt" "$work/patched.failures.txt" | tee "$work/new-failures.txt"
[ -s "$work/new-failures.txt" ] && status=1
echo "== aggregator tail (patched)"; tail -5 "$work/patched.aggregate.txt"
echo "== new tests"; grep "paths" "$work/patched.txt"; grep -q " 0 fail" <<<"$(grep paths "$work/patched.txt")" || status=1
echo "== type check (messages in the touched adapter files that the base does not have; missing global types appear in both)"
for name in base patched; do
  extra=""; [ "$name" = patched ] && extra=",\"./$BZ/🧪️tests/📥️paths/🟦️.ts\""
  printf '{"extends":"./tsconfig.json","include":[],"files":["./%s","./%s","./%s","./%s"%s]}\n' "$BZ/🌐️platform/🟦️.ts" "$BZ/📜️script.ts" "$BS/🩺️readiness/🟦️.ts" "$BZ/🧪️tests/📥️platform/🟦️.ts" "$extra" > "$work/$name/tsc-check.json"
  (cd "$work/$name" && bun node_modules/typescript/bin/tsc -p tsc-check.json > "$work/$name.tsc.txt" 2>&1 || true)
  grep -E "^($BZ|$BS)/.*error TS" "$work/$name.tsc.txt" | sed -E 's/^[^:]*\([0-9,]+\): //' | sort -u > "$work/$name.tsc.messages.txt"
  echo "$name: $(wc -l < "$work/$name.tsc.messages.txt" | tr -d ' ') distinct messages in the touched files"
done
comm -13 "$work/base.tsc.messages.txt" "$work/patched.tsc.messages.txt" | tee "$work/new-type-errors.txt"
[ -s "$work/new-type-errors.txt" ] && status=1
echo "check status: $status"
exit $status
