#!/bin/zsh
# 📊️ The coverage matrix of single assets into the record folder of the lane ($GEN/matrix-<kind>, plants in `trees`, the rest in `rigid`), one asset process at a time; an asset whose process tree grows above CAP megabytes is stopped and noted in $GEN/big.txt for a larger reservation; a current record is kept unless FRESH=1: matrix1.sh <id>…   (run it inside a grant)
source "${0:h}/env.sh"
tree() { ps -axo pid=,ppid=,rss= | awk -v root="$1" '{ parent[$1] = $2; rss[$1] = $3 } END { for (p in parent) { q = p; while (q != root && (q in parent) && q != 1) q = parent[q]; if (q == root) sum += rss[p] } print int(sum / 1024) }'; }
for a in "$@"; do
  kind=rigid; [[ -d "$S/🗂️catalog/🌳️vegetation/$a" ]] && kind=trees
  out="$GEN/matrix-$kind"; mkdir -p "$out" "$GEN/log"; rec="$out/cells/$a.json"
  [[ -z "$FRESH" && -f "$rec" && "$rec" -nt "$(asset "$a")" ]] && continue
  start=$(date +%s); peak=0; over=""
  (cd "$T" && exec timeout ${MATRIX_TIMEOUT:-14400} "$B" 📊️matrix.ts --language en --out "$out" --jobs 1 --fresh --shard 0/1 --assets "$a" --timeout-s ${MATRIX_TIMEOUT:-14400}) >> "$GEN/log/matrix1.log" 2>&1 &
  pid=$!
  while kill -0 $pid 2>/dev/null; do
    now=$(tree $pid); (( now > peak )) && peak=$now
    if (( now > ${CAP:-700} )); then over=" STOPPED ABOVE ${CAP:-700}"; kill -TERM $pid; sleep 3; fi
    sleep 2
  done
  wait $pid; code=$?
  [[ -n "$over" ]] && echo "$a" >> "$GEN/big.txt"
  echo "$(date +%H:%M) $a exit $code $(( $(date +%s) - start )) s peak $peak MB$over" >> "$GEN/log/matrix1-peaks.txt"
done
