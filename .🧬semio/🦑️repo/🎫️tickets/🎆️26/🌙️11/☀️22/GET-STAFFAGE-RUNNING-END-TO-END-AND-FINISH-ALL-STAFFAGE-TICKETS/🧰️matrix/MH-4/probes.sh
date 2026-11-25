#!/bin/zsh
# 🔦️ Several subjects of one trial copy under the lamp, inside a held grant: probes.sh <id> <scale> <subject>…   runs probe.ts on $GEN/try/<id>.json per subject and appends to $GEN/try/<id>.probe.out.
source "${0:A:h}/env.sh"
a="$1"; scale="$2"; shift 2
for subject in "$@"; do
  echo "#### 1:$scale $subject"
  "$W/cap.sh" 800 600 "$T" "$B" "$W/probe.ts" "${FILE:-$GEN/try/$a.json}" --scale "$scale" --subject "$subject" 2>&1 | cut -c1-${COLS:-260} | head -${LINES_MAX:-60}
done >> "$GEN/try/$a.probe.out"
