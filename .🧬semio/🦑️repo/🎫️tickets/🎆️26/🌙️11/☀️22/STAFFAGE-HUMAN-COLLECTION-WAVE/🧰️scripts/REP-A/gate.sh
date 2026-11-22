#!/bin/zsh
# 🚦️ The gates of people after a repair, one line each: the silhouette guard as the test measures it (diag.ts), validateAsset and the staffage check CLI. gate.sh <slug>…  (VOL=1 adds checkVolumes of every rule)
source "${0:A:h}/env.sh"
for a in "$@"; do
  f="$CAT/$a/🔣️.json"
  (cd "$W" && $B diag.ts "$f") > "$GEN/[DEBUG]-diag-$a.log" 2>&1
  (cd "$T/🧰️scripts/HW-03" && $B validate.ts "$f") > "$GEN/[DEBUG]-validate-$a.log" 2>&1
  (cd "$S" && $B 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --file "$f" --samples 9) > "$GEN/[DEBUG]-check-$a.log" 2>&1
  v=""
  if [[ -n "$VOL" ]]; then (cd "$T/🧰️scripts/HW-03" && $B measure.ts "$f" --no-silhouette) > "$GEN/[DEBUG]-vol-$a.log" 2>&1; v=" | volumes: $(tail -1 "$GEN/[DEBUG]-vol-$a.log")"; fi
  echo "$(tail -1 "$GEN/[DEBUG]-diag-$a.log") | validate: $(head -1 "$GEN/[DEBUG]-validate-$a.log" | cut -f2) | check: $(grep -m1 -E '^(✓|✗)' "$GEN/[DEBUG]-check-$a.log" | cut -c1-60)$v"
done
