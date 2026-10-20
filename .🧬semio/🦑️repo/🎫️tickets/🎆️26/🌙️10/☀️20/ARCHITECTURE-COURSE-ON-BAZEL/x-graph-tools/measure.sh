#!/bin/sh
# usage: sh measure.sh [OUT_DIR] [TARGET]      default TARGET=@teaching/architecture-course:test
# Run only after the staged set is applied AND the coordinator says dry-runs are allowed again. Read-only, but each cold run takes minutes.
# Pass 1 (cold or partially warm) and pass 2 (warm, nothing changed) of `bun repo --tool=bazel run TARGET --dry-run`, timed per phase.
# Acceptance: pass 2 shows `owner observations computed` = 0 and a total of a few seconds beyond the ingress; pass 1 shows <= one derivation per owner.
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE" && cd "$(git rev-parse --show-toplevel)" && pwd)
OUT=${1:-$HERE/../🗑️generated/x-graph}
TARGET=${2:-@teaching/architecture-course:test}
mkdir -p "$OUT"
cd "$ROOT" || exit 2
for pass in 1 2; do
  echo "== pass $pass: bun repo --tool=bazel run $TARGET --dry-run"
  bun "$HERE/graphtime.ts" "$OUT/dry-graph-$pass.log" bun repo --tool=bazel run "$TARGET" --dry-run | tail -40
done
echo "== census"
for pass in 1 2; do python3 "$HERE/profile-log.py" "$OUT/dry-graph-$pass.log" | head -4; done
