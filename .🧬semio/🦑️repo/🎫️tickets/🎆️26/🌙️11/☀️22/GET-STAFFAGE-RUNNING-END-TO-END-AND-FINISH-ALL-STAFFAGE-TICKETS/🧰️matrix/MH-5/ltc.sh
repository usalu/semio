#!/bin/zsh
# 🥖️ Loft variants with the silhouette guard and the check CLI: ltc.sh <id> <scales> <name>=<lofts.json>… is lt.sh followed by the verdict of `--file` for every variant.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
"$W/lt.sh" "$@" | grep -v 'lofts ('; a="$1"; shift 2
for pair in "$@"; do n="${pair%%=*}"; echo "$n check: $(cd "$S" && timeout 900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$GEN/try/$a.$n.json" 2>&1 | grep -E '^(✓|✗)|suitability-' | tr '\n' ' ' | cut -c1-300)"; done
