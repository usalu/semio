#!/bin/zsh
# 🌵️ The matrix of one slow asset of batch MV-3 under one grant (`grant.sh 700 <timeout> chain-s.sh <id> [seconds]`): `📊️matrix.ts --assets <id>` with a per-asset limit of [seconds] (default 43200), its peak memory, the result line appended to `$GEN/proof/<id>.txt` and `$GEN/log/chain-s.txt`.
export BATCH=MV-3 MV3_GRANT=1; source "${0:A:h}/../MR/env.sh"
a="$1"; lim="${2:-43200}"; d="$GEN/proof"; o="$GEN/log/chain-s.txt"; mkdir -p "$d" "$GEN/log"; echo "chain S $a $(date +%F\ %H:%M)" >> "$o"
rm -rf "$GEN/matrix-$a"; s=$(date +%s)
(cd "$T" && /usr/bin/time -l -o "$GEN/log/rss-matrix-$a.txt" timeout $(( lim + 300 )) "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --timeout-s "$lim" --assets "$a") > "$d/$a-matrix.log" 2>&1; code=$?
echo "matrix exit $code $(grep '\[matrix\].*cells:' "$d/$a-matrix.log" | tail -1) ($(( $(date +%s)-s )) s, $(date +%F\ %H:%M), generator peak $(( $(awk '/maximum resident set size/{print $1}' "$GEN/log/rss-matrix-$a.txt") / 1048576 )) MB) $(python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | tail -3 | tr '\n' ' ' | cut -c1-600)" | tee -a "$d/$a.txt" >> "$o"
echo "chain S $a done $(date +%H:%M)" >> "$o"
