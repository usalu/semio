#!/bin/sh
# after the coordinator's GO and a green chain15: apply, light verification, then measure-after-go.sh
# usage: go.sh [--write]       (default: dry run of the apply and the test list)
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../../../../../../../.." && pwd)
python3 "$HERE/ops_gen.py"
python3 "$HERE/apply.py" check || true
[ "$1" = "--write" ] || { echo "dry run; use --write to apply"; exit 0; }
python3 "$HERE/apply.py" apply --write
export SEMIO_TEST_ARTIFACT_DIR=${SEMIO_TEST_ARTIFACT_DIR:-/private/tmp/conc-graph/artifacts}; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
G="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph"
for d in "$G/🕰️generations/🧪️tests" "$G/🕰️generations/🌍️environment/🧪️tests" "$G/🕰️generations/🔌️load/🧪️tests" "$G/🧪️tests/🕰️generation-protocol" "$G/📥️inference/🧪️tests/🧷️closure-facts" "$G/🧪️tests/🔓️sealed-reuse" "$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧾️record-header"; do
  (cd "$d" && nice -n 10 bun test ./🟦️.ts)
done
echo "next: bun repo tools setup nx (the loaded graph module changed), then measure-after-go.sh"
