#!/bin/zsh
# 📏️ Which op breaks a default-pose dimension: dims.sh <id> <scales> <ops.json>… applies each ops file to the backup into $GEN/try/<id>.<name>.json and prints the hard dimension and whole-figure lines of the real gate over the poses at the scales (inside the worker grant).
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
a="$1"; scales="$2"; shift 2; cd "$T"
for ops in "$@"; do
  name="${ops:t:r}"; out="$GEN/try/$a.$name.json"
  "$W/b.sh" "$W/fit.ts" "$GEN/backup/$a.json" --ops "$ops" --out "$out" | tail -1
  echo "== $name: $("$W/b.sh" "$M/gate.ts" "$out" --poses --scales "$scales" | grep -E ' HARD ' | cut -c1-200 | tr '\n' '|')"
done
