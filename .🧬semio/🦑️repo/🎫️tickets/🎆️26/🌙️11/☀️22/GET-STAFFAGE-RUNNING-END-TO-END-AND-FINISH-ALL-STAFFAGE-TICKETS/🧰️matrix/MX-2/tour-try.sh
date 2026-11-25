#!/bin/zsh
# 🧪️ Variants of the loft ops of the reference person, inside a grant that is already held: tour-try.sh <variant>… applies ops/tour-<variant>.json to the catalog copy 🗑️generated/matrix/MX-2/tour/base.json and measures the trial copy at 1:50: print check, silhouette guard, validateAsset; with GATE=1 also the gate over every subject.
export BATCH=MX-2; source "${0:A:h}/../MR/env.sh"; out="$GEN/tour"; P="$S/📦️packages/🟦️typescript"; MC="$U/🧰️matrix/MC"
for v in "$@"; do
  f="$out/try-$v.json"
  (cd "$P" && timeout 300 "$B" "$W/loft.ts" "$out/base.json" --ops "$W/ops/tour-$v.json" --out "$f") > "$out/try-$v.txt" 2>&1; echo "== $v: $(grep -E 'REFUSED' "$out/try-$v.txt" | cut -c1-200) $(tail -1 "$out/try-$v.txt" | cut -c1-120)"
  (cd "$T" && timeout 900 "$B" "$MC/prints.ts" "$f") 2>&1 | grep -v '^\[' | cut -c1-500
  (cd "$P" && timeout 1500 "$B" "$W/guard.ts" "$f" --scales 50 --tag "$v ") 2>&1 | grep -v '^\[' | cut -c1-300
  (cd "$P" && timeout 900 "$B" "$MC/validate.ts" "$f") 2>&1 | grep -v '^\[' | cut -c1-300
  [[ -n "$GATE" ]] && { (cd "$T" && timeout 3000 "$B" "$M/gate.ts" "$f" --scales 50 --twins) > "$out/gate-$v.txt" 2>&1; tail -1 "$out/gate-$v.txt"; grep -E ' HARD | reported ' "$out/gate-$v.txt" | cut -c1-230; }
done
