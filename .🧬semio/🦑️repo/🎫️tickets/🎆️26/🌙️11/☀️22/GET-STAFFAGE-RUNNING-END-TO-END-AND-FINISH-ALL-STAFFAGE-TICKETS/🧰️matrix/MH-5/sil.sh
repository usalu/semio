#!/bin/zsh
# 🌓️ The silhouette guard of trial variants, one process after the other: sil.sh <id> <scales> <name>=<ops.json>… applies each ops file to the catalog file into $GEN/try/<id>.<name>.json and prints the guard lines of measure.ts for the scales (comma separated); the name `base` measures the backup.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
a="$1"; scales="$2"; shift 2
for pair in "$@"; do
  name="${pair%%=*}"; ops="${pair#*=}"
  if [[ "$name" == base ]]; then file="$GEN/backup/$a.json"; else file="$GEN/try/$a.$name.json"; (cd "$T" && "$B" "$W/fit.ts" "$a" --ops "$ops" --out "$file" | grep -E "REFUSED|OVER" | cut -c1-240); fi
  (cd "$S/📦️packages/🟦️typescript" && "$W/q.sh" timeout 1200 "$B" "$M/measure.ts" "$file" --scales "$scales" 2>&1 | grep -E " 1:|validate|sweep" | sed "s/^/$name: /" | cut -c1-260)
done
