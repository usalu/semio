#!/usr/bin/env bash
# run-laws.sh <mirror-name> <list> <outdir>: run each listed test file alone in the mirror (bare bun test, short artifact dir), one summary line per file
set -uo pipefail
C="$(cd "$(dirname "$0")/.." && pwd)"; name="$1"; list="$2"; out="$3"; mkdir -p "$out"
art="$C/🗑️generated/windows/a"; mkdir -p "$art"
index=0
while IFS= read -r file; do
  [ -z "$file" ] && continue; index=$((index + 1))
  log="$out/$index.log"
  SEMIO_TEST_ARTIFACT_DIR="$art" perl -e 'alarm 300; exec @ARGV' python3 -I "$C/mirror-tools/mirror.py" run "$name" --peers-ok --seal-allow "${art#/Users/ueli/Documents/semio/}" -- bun test "./$file" >"$log" 2>&1
  status=$?
  summary=$(grep -E '^ *[0-9]+ (pass|fail|skip)' "$log" | tr -s ' ' | tr '\n' ' ')
  printf '%s exit=%s %s %s\n' "$index" "$status" "$summary" "$file"
done <"$list"
