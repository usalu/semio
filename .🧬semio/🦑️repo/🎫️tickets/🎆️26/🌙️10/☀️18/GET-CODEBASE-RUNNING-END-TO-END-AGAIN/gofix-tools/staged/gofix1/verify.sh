#!/bin/sh
# 🐹 Laws of batch `gofix1` after apply: VERIFY_ROOT=<mirror root holding the applied batch> sh verify.sh   (never the live root: tests run in a mirror)
set -e
here=$(cd "$(dirname "$0")" && pwd)
tools=$(dirname "$(dirname "$here")")
: "${VERIFY_ROOT:?pass the mirror root that holds the applied batch}"
root=$VERIFY_ROOT
python3 -I "$here/stage.py" check --root "$root"
G=$(python3 -I -c 'import sys;print(sys.argv[1])' "${GOFIX_GOROOT:?GOFIX_GOROOT = the pinned Go root, see result-gofix.md}")
out=${GOFIX_OUT:-$tools/../🗑️generated/gofix}
mkdir -p "$out/plan" "$out/out" "$out/gocache" "$out/gomod"
bun "$tools/plan.ts" "$root" "$out/plan"
for scratch in inside outside; do
  python3 -I "$tools/run.py" --mirror "$root" --plan "$out/plan" --out "$out/out" --cache "$out/gocache" --label "v-$scratch" --scratch "$scratch" --steps vet,test --jobs 2 --only "📜️statutes/📦️packages/🐹️go,🚀️launcher/📦️packages/🐹️go,🔗️graphql/📦️packages/🐹️go,💻️client/🔌️mcp,⌨️cli/📦️packages/🐹️go"
done
python3 -I "$tools/run.py" --mirror "$root" --plan "$out/plan" --out "$out/out" --cache "$out/gocache" --label v-client --scratch inside --steps vet,test --jobs 1 --only "💻️client/⌨️cli" --timeout 60m --skip '^TestCanonicalGoTestDispatcher'
python3 -I "$tools/summary.py" "$out/out/summary-v-inside.json" "$out/out/summary-v-outside.json" "$out/out/summary-v-client.json"
