#!/bin/sh
# 🔏 audit4 gatelock laws on <root> (a mirror; the batch applied). RED_ON_BASE=1 first swaps the 4 base files in, requires the new fence law to fail 3/3, then puts the batch back.
# Under `mirror.py run` (sealed) the real-process tests 'a grant is lent only while ...' (gate) and the 2 real handover processes fail on base too; run unsealed for 0 failures.
root="${1:?usage: verify.sh <root>}"; here=$(cd "$(dirname "$0")" && pwd); H="🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
python3 -I "$here/stage.py" check --root "$root" | grep -q drifted && { echo "VERIFY FAIL: batch not cleanly applied/base in $root"; exit 2; }
cd "$root/$H" || exit 2
if [ "${RED_ON_BASE:-0}" = 1 ]; then
  python3 -I "$here/stage.py" apply --root "$root" >/dev/null || exit 2
  for row in $(python3 -I -c 'import json,sys;[print(r["file"]+"|"+k) for k,r in json.load(open(sys.argv[1])).items() if r["base"]]' "$here/manifest.json"); do cp "$here/base/${row%%|*}" "$root/${row#*|}"; done
  out=$(bun test "./🧪️tests/🔏️fence/🟦️.ts" --timeout 60000 2>&1); echo "$out" | grep -E "^ *[0-9]+ (pass|fail)"
  echo "$out" | grep -qE "^ *3 fail" || { echo "VERIFY FAIL: the fence law is not red 3/3 on base"; python3 -I "$here/stage.py" apply --root "$root" >/dev/null; exit 1; }
  for row in $(python3 -I -c 'import json,sys;[print(r["file"]+"|"+k) for k,r in json.load(open(sys.argv[1])).items() if r["base"]]' "$here/manifest.json"); do cp "$here/files/${row%%|*}" "$root/${row#*|}"; done
  echo "RED ON BASE: OK (3/3)"
fi
python3 -I "$here/stage.py" apply --root "$root" >/dev/null || exit 2
status=0
for t in "🧪️tests/🔏️fence/🟦️.ts" "🧪️tests/🚪️gate/🟦️.ts" "🧪️tests/🤝️handover/🟦️.ts" "🧪️tests/🔼️upgrade/🟦️.ts" "🧪️tests/🔼️upgrade/🌱️grow/🟦️.ts" "🧪️tests/🔼️upgrade/🚪️gate/🟦️.ts" "🧪️tests/🔼️upgrade/🤝️mutual/🟦️.ts" "🧪️tests/🪜️phases/🚪️gate/🟦️.ts" "🧪️tests/🟦️.ts" "🧪️tests/⚓️boot-anchor/🟦️.ts"; do
  line=$(bun test "./$t" --timeout 120000 2>&1 | grep -E "^ *[0-9]+ (pass|fail)" | tr -s ' \n' ' '); echo "$t:$line"
  case "$line" in *" 0 fail"*) ;; *) status=1;; esac
done
[ $status = 0 ] && echo "VERIFY OK" || echo "VERIFY: failures above (sealed runs: compare with the baseline note in the header)"
exit $status
