#!/bin/zsh
# 🩺️ The proof of a group of plants of batch MV-3 after `make.sh`, meant for one grant of 2500 MB (`grant.sh 2500 <timeout> prove-group.sh <tag> <id>…`): per plant the gate over every subject (`M/gate.ts --soft`), the official scan and `M/measure.ts` (silhouette guard, `validateAsset`, three-views sweep; the backup first, then the catalog file), then the six real test files once each with the names of all plants of the group as filter (catalog-silhouette, three-views, catalog-physics, breathing-catalog, asset-validation, asset-power). One line per check is appended to `$GEN/proof/<id>.txt` (per plant) and `$GEN/proof/group-<tag>.txt` (tests). The check CLI (5000 MB) and the matrix (7000 MB) run apart: `check.sh`, `matrix.sh`.
export BATCH=MV-3; source "${0:A:h}/../MR/env.sh"
q() { if [[ -n "$MV3_GRANT" ]]; then local dir="$2" t="$3" r="$GEN/log/rss-$$.txt" code; shift 3; (cd "$dir" && /usr/bin/time -l -o "$r" timeout "$t" "$@"); code=$?; echo "$(( $(awk '/maximum resident set size/{print $1}' "$r") / 1048576 )) MB ${${@[2,5]}:t}" >> "$GEN/log/rss.txt"; rm -f "$r"; return $code; else "$W/q.sh" "$@"; fi }
tag="$1"; shift; d="$GEN/proof"; mkdir -p "$d"
for a in "$@"; do
  o="$d/$a.txt"; echo "proof of $(date +%F\ %H:%M), file $(wc -c < "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)" | tr -d ' ') bytes" > "$o"
  q 1000 "$T" 2400 "$B" "$M/gate.ts" "$a" --soft > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")" >> "$o"
  q 1000 "$T" 2400 "$B" 🔎️scan.ts --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)" >> "$o"
  q 2500 "$S/📦️packages/🟦️typescript" 2400 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$a" > "$d/$a-measure.log" 2>&1; grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log" >> "$o"
done
g="$d/group-$tag.txt"; echo "tests of $(date +%F\ %H:%M) for $*" > "$g"
"$W/vt.sh" "$tag-silhouette" "🧊️catalog-silhouette" "$@" >> "$g"
"$W/vt.sh" "$tag-three-views" "🧊️three-views" "$@" >> "$g"
"$W/vt.sh" "$tag-physics" "🌡️catalog-physics" "$@" >> "$g"
SUFFIX=":" "$W/vt.sh" "$tag-breathing" "🫁️breathing-catalog" "$@" >> "$g"
SUFFIX="" "$W/vt.sh" "$tag-validation" "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$@" >> "$g"
SUFFIX=":" "$W/vt.sh" "$tag-power" "⚡️asset-power" "$@" >> "$g"
echo "proof group $tag done"
