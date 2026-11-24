#!/bin/zsh
# 🩺️ The proof of repaired assets, one process after the other, one line per proof appended to 🗑️generated/matrix/MS/proof/lines.txt: the authors' scan (`T/🔎️scan.ts`), the gate over clip instants and transitions (`MR/gate.ts`), the derived-view distances, the real `🧊️catalog-silhouette` test, `validateAsset` against the state before the repair, the check CLI and the asset's tests in `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `✅️asset-validation` and `⚡️asset-power`: prove.sh <id>…   (QUICK=1 leaves out the five other suites)
source "${0:A:h}/env.sh"
mkdir -p "$GEN/proof"
for a in "$@"; do
  {
    (cd "$T" && timeout 900 "$B" "🔎️scan.ts" --language en --quiet "$a") > "$GEN/proof/$a-scan.log" 2>&1; echo "$a | scan: exit $? $(tail -1 "$GEN/proof/$a-scan.log" | cut -c1-200)"
    (cd "$T" && timeout 1800 "$B" "$U/🧰️matrix/MR/gate.ts" "$a") > "$GEN/proof/$a-gate.log" 2>&1; echo "$a | gate (clips, transitions): exit $? $(tail -1 "$GEN/proof/$a-gate.log")"
    echo "$a | derived: $(cd "$T" && timeout 600 "$B" "$W/derived.ts" "$S/🧪️tests" "$a" 2>&1 | grep -c FLAG) flagged scale(s)"
    echo "$a | silhouette: $("$W/vt.sh" "sil-$a" 🧊️catalog-silhouette "$a") $(grep -m1 -E "AssertionError|Error:" "$GEN/vt/sil-$a.log" | cut -c1-170)"
    echo "$a | validateAsset: $(cd "$S/📦️packages/🟦️typescript" && timeout 900 "$B" "$W/validate.ts" "$a" --against "$GEN/backup" 2>&1 | tail -1 | cut -c1-200)"
    (cd "$S" && timeout 2400 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$GEN/proof/$a-check.log" 2>&1; echo "$a | check: $(grep -m1 -E '^(✓|✗)' "$GEN/proof/$a-check.log" | cut -c1-60); $(grep -c 'authored, suitable' "$GEN/proof/$a-check.log") authored scale(s) suitable"
    [[ -n "$QUICK" ]] && continue
    echo "$a | three-views: $("$W/vt.sh" "tv-$a" 🧊️three-views "$a")"
    echo "$a | physics: $("$W/vt.sh" "ph-$a" 🌡️catalog-physics "$a")"
    echo "$a | breathing: $(SUFFIX="" "$W/vt.sh" "br-$a" 🫁️breathing-catalog "$a")"
    echo "$a | asset-validation: $(SUFFIX="" "$W/vt.sh" "av-$a" ✅️asset-validation "$a")"
    echo "$a | asset-power: $(SUFFIX="" "$W/vt.sh" "ap-$a" ⚡️asset-power "$a")"
  } | tee -a "$GEN/proof/lines.txt"
done
