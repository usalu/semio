#!/bin/sh
# Before/after measurement of the churn fuzz harness (harness: the module tests `🔁️churn/🧪️tests/🎲️fuzz`).
# usage: sh measure.sh <before-lib-root> <after-lib-root> <scratch-dir> [seeds] [mode]
# Output: one JSON line per variant (outcome counts, mean wall seconds, restarted, retried) plus <scratch>/<variant>-<mode>.jsonl.
set -eu
cd "$(dirname "$0")/../../../../../../../.."
HARNESS="🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🎲️fuzz/🐍️.py"
SEEDS="${4:-0:200}"; MODE="${5:-sync}"
for variant in before after; do
  lib="$1"; [ "$variant" = after ] && lib="$2"
  nice -n 10 python3 "$HARNESS" run --lib "$lib" --variant "$variant" --seeds "$SEEDS" --mode "$MODE" --jobs 5 --out "$3" --timeout 60 --keep $( [ "$MODE" = timed ] && echo "--work-ms 250" )
done
