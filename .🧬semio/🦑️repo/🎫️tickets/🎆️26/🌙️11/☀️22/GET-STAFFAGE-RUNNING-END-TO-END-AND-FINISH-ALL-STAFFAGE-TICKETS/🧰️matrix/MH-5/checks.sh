#!/bin/zsh
# 🩺️ The check CLI and the model gate of trial variants: checks.sh <id> <scale> <ops.json>… applies each ops file to the backup into $GEN/try/<id>.<name>.json and prints the hard pairs of the model gate at the scale and the verdict of the check CLI on the copy (`--file`), inside the worker grant.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
a="$1"; scale="$2"; shift 2
for ops in "$@"; do
  name="${ops:t:r}"; out="$GEN/try/$a.$name.json"
  (cd "$T" && "$W/b.sh" "$W/fit.ts" "$GEN/backup/$a.json" --ops "$ops" --out "$out" | tail -1 | cut -c1-100; "$W/b.sh" "$W/mgate.ts" "$out" --scales "$scale" | grep HARD | cut -c1-200)
  echo "== $name check: $(cd "$S" && timeout 900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$out" 2>&1 | grep -E '^(✓|✗)|suitability-' | tr '\n' ' ' | cut -c1-240)"
done
