#!/bin/zsh
# 🩺️ Proof of plants, one process after the other: the real `🧊️catalog-silhouette` test, `validateAsset` (now, and new against the state before this round), the three-views sweep, the print suitability, the product check CLI, then the suites `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `✅️asset-validation`, `⚡️asset-power` with the name filter; one line per proof, appended to 🗑️generated/silhouette/R4-P/proof.txt. prove.sh <id>…   (QUICK=1 leaves out the five other suites)
source "${0:A:h}/env.sh"
mkdir -p "$GEN/proof" "$GEN/trial"
for a in "$@"; do
  {
    echo "$a | $("$W/vt.sh" "sil-$a" 🧊️catalog-silhouette "$a" | tr '\n' ' ')"
    grep -E "^AssertionError" "$GEN/vt/sil-$a.log" | cut -c1-200 | head -3
    [[ -s "$GEN/trial/$a-validate-before.txt" ]] || "$W/t.sh" validate.ts "$GEN/backup/$a.json" "$GEN/trial/$a-validate-before.txt" > /dev/null
    echo "$a | validateAsset $("$W/t.sh" validate.ts "$a" "$GEN/proof/$a-validate.txt" | head -1 | cut -f2-), new against before: $(diff "$GEN/trial/$a-validate-before.txt" "$GEN/proof/$a-validate.txt" | grep -c '^>')"
    echo "$a | sweep $(N=3 "$W/t.sh" sweep.ts "$a" | cut -c1-240 | tr '\n' ' ')"
    echo "$a | print $("$W/t.sh" suit.ts "$a" | tail -1)"
    (cd "$S" && timeout 1500 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$GEN/proof/$a-check.log" 2>&1
    echo "$a | check CLI exit $?: $(grep -m1 -E '^(✓|✗)' "$GEN/proof/$a-check.log" | cut -c1-160)"
    [[ -n "$QUICK" ]] && continue
    for suite in 🧊️three-views 🌡️catalog-physics 🫁️breathing-catalog; do echo "$a | $("$W/vt.sh" "$suite-$a" "$suite" "$a" | tr '\n' ' ')"; done
    PAT="(^|[ >])($a)([ :]|\$)|manifest" "$W/vt.sh" "asset-validation-$a" ✅️asset-validation | tr '\n' ' ' | sed "s/^/$a | /"; echo
    PAT="(^|[ >])($a)([ :]|\$)|manifest|never negative|nameplate|every rating|engines|sorts the elements" "$W/vt.sh" "asset-power-$a" ⚡️asset-power | tr '\n' ' ' | sed "s/^/$a | /"; echo
  } 2>&1 | tee -a "$GEN/proof.txt"
done
