#!/usr/bin/env bash
# 🧪️ Run every Norm family's example crate tests one after another and print one verdict line each.
# Usage: test-family-examples.sh [family-dir ...]
set -u
T="$(cd "$(dirname "$0")" && pwd)"
G="$T/🗑️generated"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
A="$ROOT/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
mkdir -p "$G"
cd "$A"
families=("$@")
[ ${#families[@]} -eq 0 ] && families=(*/)
for d in "${families[@]}"; do
  d="${d%/}"
  family="${d#*️}"
  log="$G/examples-$family.log"
  (cd "$A/$d" && repo cargo --store=shared --configuration=prototype -- test -p "semio-s-artifact-norm-$family-examples" --offline) > "$log" 2>&1
  code=$?
  summary=$(grep -E "^test result:" "$log" | awk '{p+=$4; f+=$6} END {print p" passed, "f" failed"}')
  echo "[DEBUG] $family exit $code: $summary"
done
