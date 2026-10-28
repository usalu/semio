#!/bin/sh
# 🎚 Laws of batch `profilefresh`: sh verify.sh <root> [store]   (`store`, live root only, adds the 4 store-backed installation laws; mirror: python3 -I C/mirror-tools/mirror.py run a5 --seal-allow <C-relative 🗑️generated/audit5> -- sh verify.sh <mirror-root>)
# RED_ON_BASE=1 (mirror only) first runs the new law against the base installation schema and requires exactly 1 failing test, then applies the batch.
# Legs (graph-free Script envelope C/dashhot-tools/env-run.ts): the compiled-sources law (incl. the new profile test) and the store-free installation laws.
set -u
root=$(cd "${1:?usage: verify.sh <root>}" && pwd -P); here=$(cd "$(dirname "$0")" && pwd)
C=$(python3 -I -c 'import os,sys;h=sys.argv[1]
while os.path.basename(h)!="GET-CODEBASE-RUNNING-END-TO-END-AGAIN" and h!="/":h=os.path.dirname(h)
print(h)' "$here")
art="$C/🗑️generated/audit5/art"; mkdir -p "$art"; fail=0
D=./🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧊️execution
LAW="🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧊️execution/🧾️compiled/🟦️.ts"
envelope(){ (cd "$root" && bun "$C/dashhot-tools/env-run.ts" "$root" "@semio-tech/repo-dashboard-rs:test-source-freshness" "$art" -- test --timeout 300000 "$@" 2>&1); }
counts(){ echo "$1" | grep -E "^ *[0-9]+ (pass|fail)" | tr -s ' \n' ' '; }
if [ "${RED_ON_BASE:-0}" = 1 ]; then
  python3 -I "$here/stage.py" revert --root "$root" >/dev/null || exit 2
  file=$(python3 -I -c 'import json,sys;print(json.load(open(sys.argv[1]))[sys.argv[2]]["file"])' "$here/manifest.json" "$LAW")
  cp "$here/files/$file" "$root/$LAW"
  out=$(envelope "$D/🧾️compiled/🟦️.ts"); echo "base: $(counts "$out")"; echo "$out" | grep -E "^\(fail\)" | cut -c1-160
  echo "$out" | grep -qE "^ *1 fail" && echo "$out" | grep -q "(fail) an installation is stale once the Cargo profile policy" || { echo "VERIFY FAIL: the profile law is not red (exactly 1) on base"; cp "$here/base/$file" "$root/$LAW"; exit 1; }
  cp "$here/base/$file" "$root/$LAW"; echo "RED ON BASE: OK"
fi
python3 -I "$here/stage.py" apply --root "$root" >/dev/null || exit 2
out=$(envelope "$D/🧾️compiled/🟦️.ts"); echo "compiled: $(counts "$out")"; echo "$out" | grep -E "^\(fail\)" | cut -c1-160; echo "$out" | grep -qE "^ *0 fail" || fail=1
out=$(envelope -t "^(source digest|dashboard controls|dashboard bootstrap)" "$D/🟦️.ts"); echo "installation: $(counts "$out")"; echo "$out" | grep -E "^\(fail\)" | cut -c1-160; echo "$out" | grep -qE "^ *0 fail" || fail=1
if [ "${2:-}" = store ]; then for name in "dashboard immutable installation" "installation Source staleness" "installation receipt schema" "installed finite dashboard"; do out=$(envelope -t "^$name" "$D/🟦️.ts"); echo "store: $name: $(counts "$out")"; echo "$out" | grep -qE "^ *0 fail" || fail=1; done; fi
[ $fail -eq 0 ] && echo "VERIFY profilefresh: GREEN" || echo "VERIFY profilefresh: RED"
exit $fail
