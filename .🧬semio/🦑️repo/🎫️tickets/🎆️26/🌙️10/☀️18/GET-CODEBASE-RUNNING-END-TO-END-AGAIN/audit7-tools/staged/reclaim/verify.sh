#!/bin/sh
# 🧪 Laws of batch `reclaim` (lane audit7): sh verify.sh <root>  (applies the batch first; \`contracts\` must be applied before; BUN=<bun> overrides the runtime)
# RED_ON_BASE=1 (mirror only): the core files go back to their base bytes while the new laws stay; exactly 1 law(s) must fail; then the batch is applied again.
set -u
root=$(cd "${1:?usage: verify.sh <root>}" && pwd -P); here=$(cd "$(dirname "$0")" && pwd); BUN=${BUN:-bun}; fail=0
I=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference; F=$I/📦️project-nodes/🏊️project-farm
counts(){ echo "$1" | grep -E "^ *[0-9]+ (pass|fail|skip)$" | tr -s ' \n' ' '; }
python3 -I "$here/stage.py" apply --root "$root" >/dev/null || exit 2
if [ "${RED_ON_BASE:-0}" = 1 ]; then
  rels=$(python3 -I -c 'import json,re,sys;m=json.load(open(sys.argv[1],encoding="utf8"));print("\n".join(r+"\t"+v["file"] for r,v in m.items() if re.search(sys.argv[2],r)))' "$here/manifest.json" '🏊️pool/🟨️.mjs|🏊️pool/🟨️.d.mts|🏊️pool/🧬️schema/🔣️.json|project-farm/🟨️.mjs')
  echo "$rels" | while IFS="$(printf '\t')" read -r rel file; do cp "$here/base/$file" "$root/$rel"; done
  out=$(cd "$root" && "$BUN" test "$F/🧪️tests/🔒️farm-close/🟦️.ts" -t "killed predecessor" 2>&1); echo "base core: $(counts "$out")"
  echo "$rels" | while IFS="$(printf '\t')" read -r rel file; do cp "$here/files/$file" "$root/$rel"; done
  echo "$out" | grep -qE "^ *1 fail$" || { echo "VERIFY FAIL: the new laws are not red on base"; exit 1; }
  echo "RED ON BASE: OK"
fi
out=$(cd "$root" && "$BUN" test "$F/🧪️tests/🟦️.ts" "$F/🧪️tests/🔒️farm-close/🟦️.ts" "$F/🧪️tests/🪆️captured/🟦️.ts" "$I/🧱️project-assembly/🧪️tests/🟦️.ts" "$I/🔖️stamps/🧪️tests/🟦️.ts" "$I/🧪️tests/🔖️output-stamps/🟦️.ts" ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🌱️native-scope/🟦️.ts 2>&1)
echo "farm + assembly + stamps + native-scope: $(counts "$out")"; echo "$out" | grep -E "^\(fail\)" | cut -c1-160; echo "$out" | grep -qE "^ *0 fail$" || fail=1
[ $fail -eq 0 ] && echo "VERIFY reclaim: GREEN" || echo "VERIFY reclaim: RED"
exit $fail
