#!/bin/zsh
# 🩺️ The proof of one plant of MV-2 on its catalog file, every process through the machine-wide memory queue (`repo capacity run`, from the repo root), one after the other, logs in `$GEN/proof` and `$GEN/vt`, result lines appended to `$GEN/proof/<id>.txt`: the gate over every pose, clip instant and transition (`../MR/gate.ts --twins`), the authors' scan (`T/🔎️scan.ts`), silhouette guard, `validateAsset` and three-views sweep (`../MR/measure.ts`), the product check CLI (CHECK_MB, default 1000 MB) and the real tests of the asset in catalog-silhouette, three-views, catalog-physics, breathing-catalog, asset-validation and asset-power (`../MR/vt.sh`, 2500 MB each). prove.sh <id>   (the matrix is `matrix.sh`)
BATCH=MV-2 source "${0:A:h}/../MR/env.sh"
a="$1"; d="$GEN/proof"; mkdir -p "$d" "$GEN/vt"; out="$d/$a.txt"; cd "$R"
q() { local mb="$1"; shift; repo capacity run --memory="$mb" -- "$W/timed.sh" "$@"; }
peak() { grep 'maximum resident' "$1" | awk '{printf "%d MB", $1/1048576}'; }
echo "===== $a $(date '+%F %T') file $(stat -f '%Sm %z' "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)")" >> "$out"
q 800 timeout 1800 zsh -c 'cd "$1" && shift && exec "$@"' _ "$T" "$B" "$M/gate.ts" "$a" --twins > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(grep 'hard findings' "$d/$a-gate.log") peak $(peak "$d/$a-gate.log")" >> "$out"
q 800 timeout 1800 zsh -c 'cd "$1" && shift && exec "$@"' _ "$T" "$B" 🔎️scan.ts --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(grep -E '^\[scan\] [0-9]+ assets' "$d/$a-scan.log" | cut -c1-200)" >> "$out"
q 800 timeout 900 zsh -c 'cd "$1" && shift && exec "$@"' _ "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" "$a" > "$d/$a-measure.log" 2>&1; grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log" >> "$out"
q ${CHECK_MB:-1000} timeout 3000 "$B" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$d/$a-check.log" 2>&1; echo "check exit $? $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160) peak $(peak "$d/$a-check.log"); scales: $(grep -cE 'authored, suitable' "$d/$a-check.log") suitable, $(grep -cE 'authored, (not|un)' "$d/$a-check.log") not" >> "$out"
vt() { q 2500 env BATCH=MV-2 SUFFIX="${SUFFIX-( |$)}" "$M/vt.sh" "$@" 2>/dev/null | grep -v '^\[capacity\]' >> "$out"; }
vt "$a-silhouette" "🧊️catalog-silhouette" "$a"
vt "$a-three-views" "🧊️three-views" "$a"
vt "$a-physics" "🌡️catalog-physics" "$a"
SUFFIX=":" vt "$a-breathing" "🫁️breathing-catalog" "$a"
SUFFIX="" vt "$a-validation" "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a"
SUFFIX=":" vt "$a-power" "⚡️asset-power" "$a"
echo "prove $a done $(date '+%T')" >> "$out"
