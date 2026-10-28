#!/bin/sh
# 🧾 Laws of batch `goyaml` after apply: VERIFY_ROOT=<mirror root holding the applied batch> sh verify.sh   (the registry law is scoped to the yaml directory: the whole-tree walk reports unrelated findings in a partial mirror)
set -e
here=$(cd "$(dirname "$0")" && pwd)
tools=$(dirname "$(dirname "$here")")
: "${VERIFY_ROOT:?pass the mirror root that holds the applied batch}"
root=$VERIFY_ROOT
python3 -I "$here/stage.py" check --root "$root"
test "$(GOFIX_WRITE=0 bun "$tools/reconcile.ts" "$root" "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧾️yaml/📦️packages/🐹️go" | python3 -I -c 'import json,sys;d=json.load(sys.stdin);print(d["changed"],len(d["findings"]))')" = "False 0"
out=${GOFIX_OUT:-$tools/../🗑️generated/gofix}
mkdir -p "$out/plan" "$out/out" "$out/gocache" "$out/gomod"
bun "$tools/plan.ts" "$root" "$out/plan"
python3 -I "$tools/run.py" --mirror "$root" --plan "$out/plan" --out "$out/out" --cache "$out/gocache" --label v-yaml --scratch inside --steps vet,test --only "🧾️yaml/📦️packages/🐹️go"
python3 -I "$tools/summary.py" "$out/out/summary-v-yaml.json"
