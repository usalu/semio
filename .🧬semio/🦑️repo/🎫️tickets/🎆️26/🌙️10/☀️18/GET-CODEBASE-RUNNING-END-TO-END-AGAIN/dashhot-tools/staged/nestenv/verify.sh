#!/bin/sh
# 🎛️ Laws of batch `nestenv` after apply: sh verify.sh <root>  (witness suite incl. the extended ignored/authority law, plan-reuse suites)
set -u
root=$(cd "$1" && pwd -P); here=$(cd "$(dirname "$0")" && pwd); fail=0
python3 -I "$here/stage.py" check "$root" | grep -v "^applied"; python3 -I "$here/stage.py" check "$root" >/dev/null || fail=1
cd "$root"; B=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
for file in "$B/🔭️witness/🧪️tests/🟦️.ts" "$B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts" "$B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🧭️ignore-sources/🟦️.ts"; do
  out=$(bun test --timeout 300000 "$file" 2>&1); echo "${file##*/build/}: $(echo "$out" | grep -E "^ *[0-9]+ (pass|fail)" | tr '\n' ' ')"; echo "$out" | grep -E "^\(fail\)" | cut -c1-200
  echo "$out" | grep -qE "^ *[1-9][0-9]* fail" && fail=1; echo "$out" | grep -qE "^ *[1-9][0-9]* pass" || fail=1
done
[ $fail -eq 0 ] && echo "VERIFY nestenv: GREEN" || echo "VERIFY nestenv: RED"; exit $fail
