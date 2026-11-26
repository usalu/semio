#!/bin/zsh
# 🧷️ Takeover chain E of batch MV-3, meant for one grant (`grant.sh 1000 <timeout> chain-e.sh <id>…`), one process at a time: writes the repair of each plant into the catalog (`make-many.sh`: the chain, the gate, print suitability, a second run that must change no byte), compares the drawings with the chain run on the file of before the first fixer (`same.py`: `body.ts --ops` on `$GEN/backup/<id>.json` when the plant has bodies), then proves it: gate over every subject (`--soft`), official scan, silhouette guard with `validateAsset` and the three-views sweep per scale (`M/measure.ts --scales <n>`), what shows beside the outlines in the wind poses (`visible.ts`, bodies and strands), check CLI. Lines go to `$GEN/log/makes.txt`, `$GEN/proof/<id>.txt` and `$GEN/log/chain-e.txt`.
export BATCH=MV-3 MV3_GRANT=1; source "${0:A:h}/../MR/env.sh"
q() { local dir="$2" t="$3" r="$GEN/log/rss-$$.txt" code; shift 3; (cd "$dir" && /usr/bin/time -l -o "$r" timeout "$t" "$@"); code=$?; echo "$(( $(awk '/maximum resident set size/{print $1}' "$r") / 1048576 )) MB ${${@[2,5]}:t}" >> "$GEN/log/rss.txt"; rm -f "$r"; return $code; }
o="$GEN/log/chain-e.txt"; d="$GEN/proof"; mkdir -p "$d"; echo "chain E $* $(date +%F\ %H:%M)" >> "$o"
for a in "$@"; do
  p="$d/$a.txt"; f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"
  zsh "$W/make-many.sh" "$a" >> "$o" 2>&1; sed -n "/^== $a /,\$p" "$GEN/log/makes.txt" | tail -12 >> "$o"
  echo "proof of $(date +%F\ %H:%M), file $(wc -c < "$f" | tr -d ' ') bytes, sha-256 $(shasum -a 256 < "$f" | cut -c1-16)" > "$p"
  if [[ -f "$W/body/$a.json" ]]; then q 1000 "$T" 900 "$B" "$W/body.ts" "$GEN/backup/$a.json" --ops "$W/body/$a.json" --out "$GEN/try/$a-from-backup.json" > "$GEN/log/$a-from-backup.log" 2>&1; echo "same: $(python3 "$W/same.py" "$GEN/try/$a-from-backup.json" "$f")" >> "$p"; fi
  q 1000 "$T" 3000 "$B" "$M/gate.ts" "$a" --soft > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")" >> "$p"
  q 1000 "$T" 3000 "$B" 🔎️scan.ts --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)" >> "$p"
  : > "$d/$a-measure.log"; for n in 50 100 200 500; do q 1000 "$S/📦️packages/🟦️typescript" 2400 "$B" "$M/measure.ts" "$a" --scales $n >> "$d/$a-measure.log" 2>&1; echo "measure 1:$n exit $? $(grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log" | tail -3 | tr '\n' ';')" >> "$p"; done
  grep -E "bbox" "$d/$a-measure.log" | cut -c1-260 >> "$p"
  q 1000 "$T" 1500 "$B" "$W/visible.ts" "$a" --prefix "${PREFIX:-mvb-}" > "$d/$a-visible.log" 2>&1; echo "visible exit $? $(tail -1 "$d/$a-visible.log" | cut -c1-260)" >> "$p"
  q 1000 "$S" 3000 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$d/$a-check.log" 2>&1; echo "check: exit $? $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160); $(grep -ciE 'prints|suitable' "$d/$a-check.log") suitability lines" >> "$p"
  cat "$p" >> "$o"
done
echo "chain E done $(date +%H:%M)" >> "$o"
