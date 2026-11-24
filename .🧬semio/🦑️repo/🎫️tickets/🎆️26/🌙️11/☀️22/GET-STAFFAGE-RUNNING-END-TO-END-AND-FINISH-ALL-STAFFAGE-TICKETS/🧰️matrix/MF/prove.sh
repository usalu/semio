#!/bin/zsh
# 🩺️ The proof of the assets of batch MF after their repair, one process after the other, logs in 🗑️generated/matrix/MF/proof: the authors' gate of the suite (`T/🔎️scan.ts`) over all of them, `validateAsset`, whether every authored scale prints, the product check CLI per asset, and the real tests with the name filter (catalog-silhouette, three-views, catalog-physics, breathing-catalog, asset-validation with the manifest cases, asset-power): prove.sh <id>…
export BATCH=MF; source "${0:A:h}/../MR/env.sh"
d="$GEN/proof"; mkdir -p "$d"
(cd "$T" && timeout 3000 "$B" 🔎️scan.ts --language en --quiet --fresh --out "$d/scan.jsonl" --report "$d/scan.json" "$@") > "$d/scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/scan.log")"
(cd "$S/📦️packages/🟦️typescript" && timeout 1800 "$B" "$W/validate.ts" "$@") > "$d/validate.log" 2>&1; echo "validate: $(grep -c "	0 issues" "$d/validate.log") of $# with 0 issues"; grep -v "	0 issues" "$d/validate.log" | head -20
(cd "$T" && timeout 1800 "$B" "$W/print.ts" "$@") > "$d/print.log" 2>&1; echo "print: $(grep -c "every authored scale" "$d/print.log") of $# print"; grep -v "every authored scale" "$d/print.log" | head
for a in "$@"; do (cd "$S" && timeout 1800 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$d/check-$a.log" 2>&1; echo "check: $(grep -m1 -E '^(✓|✗)' "$d/check-$a.log" | cut -c1-160)"; done
for suite in 🧊️catalog-silhouette 🧊️three-views 🌡️catalog-physics 🫁️breathing-catalog ⚡️asset-power; do "$W/vt.sh" "$suite" "$@"; done
PAT="(^|[ >])(${(j:|:)@})([ :]|$)|manifest|accepts the sample figure" "$W/vt.sh" ✅️asset-validation "$@"
