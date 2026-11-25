#!/bin/zsh
# 🧷️ Takeover chain C of batch MV-3, meant for one grant (`grant.sh <MB> <timeout> chain-c.sh <tag> <id>…`): writes the repair of the named plants into the catalog (`make-many.sh`: the chain, the gate, print suitability, a second run that must change no byte), then proves them (`prove-group.sh <tag>`: gate over every subject, official scan, silhouette guard, `validateAsset`, three-views sweep, the six real test files), then the check CLI and the matrix of each plant, one process after the other. Lines go to `$GEN/log/makes.txt`, `$GEN/proof/<id>.txt`, `$GEN/proof/group-<tag>.txt` and `$GEN/log/chain-c.txt`. A tag that starts with `again` leaves the writing and the group proof out (plants that are written and proven: the check CLI and the matrix once more).
export BATCH=MV-3 MV3_GRANT=1; source "${0:A:h}/../MR/env.sh"
q() { local dir="$2" t="$3" r="$GEN/log/rss-$$.txt" code; shift 3; (cd "$dir" && /usr/bin/time -l -o "$r" timeout "$t" "$@"); code=$?; echo "$(( $(awk '/maximum resident set size/{print $1}' "$r") / 1048576 )) MB ${${@[2,5]}:t}" >> "$GEN/log/rss.txt"; rm -f "$r"; return $code; }
o="$GEN/log/chain-c.txt"; tag="$1"; shift; [[ "$tag" == again* ]] && NOMAKE=1 NOPROVE=1; d="$GEN/proof"; echo "chain C $tag $* $(date +%F\ %H:%M)" >> "$o"
[[ -n "$NOMAKE" ]] || zsh "$W/make-many.sh" "$@" >> "$o" 2>&1
[[ -n "$NOPROVE" ]] || zsh "$W/prove-group.sh" "$tag" "$@" >> "$o" 2>&1
for a in "$@"; do
  [[ -n "$NOCHECK" ]] || { q 1000 "$S" 3000 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$d/$a-check.log" 2>&1; echo "check: exit $? $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160); $(grep -ciE 'prints|suitable' "$d/$a-check.log") suitability lines" | tee -a "$d/$a.txt" >> "$o"; }
  [[ -n "$NOMATRIX" ]] || { rm -rf "$GEN/matrix-$a"; s=$(date +%s); q 1200 "$T" ${MATRIX_TIMEOUT:-7200} "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1; echo "matrix exit $? $(grep '\[matrix\].*cells:' "$d/$a-matrix.log" | tail -1) ($(( $(date +%s)-s )) s, $(date +%F\ %H:%M)) $(python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | tail -3 | tr '\n' ' ' | cut -c1-600)" | tee -a "$d/$a.txt" >> "$o"; }
done
echo "chain C $tag done $(date +%H:%M)" >> "$o"
