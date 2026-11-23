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
  pkgs=()
  while IFS= read -r pkg; do
    [ -n "$pkg" ] && pkgs+=("$pkg")
  done < <(find "$A/$d" -path '*examples*' -name Cargo.toml -print0 | xargs -0 grep -h '^name = "' | sed 's/.*"\([^"]*\)".*/\1/' | sort -u)
  [ ${#pkgs[@]} -eq 0 ] && pkgs=("semio-s-artifact-norm-$family-examples")
  : > "$log"
  code=0
  for pkg in "${pkgs[@]}"; do
    (cd "$A/$d" && repo cargo --store=shared --configuration=prototype -- test -p "$pkg" --offline) >> "$log" 2>&1
    step=$?
    [ "$step" -ne 0 ] && code=$step
  done
  summary=$(grep -E "^test result:" "$log" | awk '{p+=$4; f+=$6} END {print p" passed, "f" failed"}')
  echo "[DEBUG] $family exit $code: $summary"
done
