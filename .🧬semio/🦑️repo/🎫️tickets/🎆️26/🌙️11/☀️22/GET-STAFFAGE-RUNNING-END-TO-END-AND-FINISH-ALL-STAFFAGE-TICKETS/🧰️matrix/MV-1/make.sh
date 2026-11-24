#!/bin/zsh
# 🏭️ The multiview repair of one plant of batch MV-1, the last step of its chain (after its generator and after the silhouette ops of `🧰️silhouette/*/ops/<id>.json`), idempotent: `rebind.ts` when column 4 of `plants.tsv` starts with `rebind` (its further words are flags of `rebind.ts`) (the elevation shapes on the bone of their front and plan twins), the asset's own `fix-<id>.ts` if there is one, then `carry.ts` (depth plates below the flat front and plan outlines of the parts the wind poses turn; its flags are column 2 of `plants.tsv`), then `anchors.ts --ops ops/<id>.json` (the anchors on the drawing of their bone, absolute values). It writes the catalog file and prints the gate over every pose, clip instant and transition. make.sh <id>
BATCH=MV-1 source "${0:A:h}/../MR/env.sh"
a="$1"; cf=(${=$(awk -F'\t' -v a="$a" '$1==a{print $2}' "$W/plants.tsv")}); mkdir -p "$GEN/log"
cd "$T"
rb=(${=$(awk -F'\t' -v a="$a" '$1==a{print $4}' "$W/plants.tsv")}); if [[ "$rb[1]" == rebind ]]; then timeout 900 "$B" "$W/rebind.ts" "$a" --write $rb[2,-1] | tail -1; fi
if [[ -f "$W/fix-$a.ts" ]]; then timeout 600 "$B" "$W/fix-$a.ts" --write | tail -1; fi
timeout 3000 "$B" "$W/carry.ts" "$a" --write $cf > "$GEN/log/$a-make-carry.log" 2>&1; echo "carry exit $? $(tail -2 "$GEN/log/$a-make-carry.log" | cut -c1-120 | tr '\n' ';')"
timeout 3000 "$B" "$W/anchors.ts" "$a" --ops "$W/ops/$a.json" --write > "$GEN/log/$a-make-anchors.log" 2>&1; echo "anchors exit $? $(tail -2 "$GEN/log/$a-make-anchors.log" | cut -c1-140 | tr '\n' ';')"
timeout 1800 "$B" "$M/gate.ts" "$a" > "$GEN/log/$a-make-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$GEN/log/$a-make-gate.log")"
