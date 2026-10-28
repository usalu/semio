#!/bin/sh
# 🕰 audit4 agedlight laws on <root> (a mirror holding the whole 🏛️sealed, 🚪️door and 🧷️format-stamps directories). RED_ON_BASE=1 first puts the base trampoline back beside the new law and requires exactly that law to fail.
root="${1:?usage: verify.sh <root>}"; here=$(cd "$(dirname "$0")" && pwd); S="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed"
python3 -I "$here/stage.py" apply --root "$root" >/dev/null || exit 2
cd "$root/$S" || exit 2
if [ "${RED_ON_BASE:-0}" = 1 ]; then
  cp "$here/base/e00" "$root/$S/🚦️trampoline/🟦️.ts"
  out=$(bun test "./🧪️tests/🚦️trampoline/🟦️.ts" --timeout 300000 2>&1); cp "$here/files/e00" "$root/$S/🚦️trampoline/🟦️.ts"
  echo "$out" | grep -E "^ *[0-9]+ (pass|fail)|^\(fail\)" | cut -c1-160
  echo "$out" | grep -qE "^ *1 fail" && echo "$out" | grep -q "^(fail) explicit promotion: a light command whose aged" && echo "RED ON BASE: OK" || { echo "VERIFY FAIL: not red on base"; exit 1; }
fi
status=0
for t in $(find 🧪️tests -name '🟦️.ts' ! -path '*🧫️fixtures*' | sort); do
  line=$(bun test "./$t" --timeout 300000 2>&1 | grep -E "^ *[0-9]+ (pass|fail)" | tr -s ' \n' ' '); echo "$t:$line"
  case "$line" in *" 0 fail"*) ;; *) status=1;; esac
done
[ $status = 0 ] && echo "VERIFY OK" || echo "VERIFY: failures above (compare with the same files on base)"
exit $status
