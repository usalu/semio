#!/bin/zsh
# 📊️ The matrix of plants of batch MV-3, one after the other through the machine-wide memory queue (7000 MB each): the queue gives a request up when the host cannot carry it for a long time, so every plant is asked again (up to TRIES times, default 6) until its matrix ran; the result line is appended to `$GEN/proof/<id>.txt` with the number of tries. matrix.sh <id>…
export BATCH=MV-3; source "${0:A:h}/../MR/env.sh"
q() { if [[ -n "$MV3_GRANT" ]]; then local dir="$2" t="$3" r="$GEN/log/rss-$$.txt" code; shift 3; (cd "$dir" && /usr/bin/time -l -o "$r" timeout "$t" "$@"); code=$?; echo "$(( $(awk '/maximum resident set size/{print $1}' "$r") / 1048576 )) MB ${${@[2,5]}:t}" >> "$GEN/log/rss.txt"; rm -f "$r"; return $code; else "$W/q.sh" "$@"; fi }
d="$GEN/proof"; mkdir -p "$d"
for a in "$@"; do
  for try in $(seq 1 ${TRIES:-6}); do
    rm -rf "$GEN/matrix-$a"; s=$(date +%s)
    q 7000 "$T" ${MATRIX_TIMEOUT:-5400} "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1; code=$?
    if [[ -f "$GEN/matrix-$a/matrix.json" ]]; then break; fi
    echo "$a matrix try $try: exit $code after $(( $(date +%s)-s )) s, $(grep -m1 -oE 'error: .{0,120}' "$d/$a-matrix.log")" >> "$GEN/matrix-tries.txt"
  done
  echo "matrix exit $code $(grep '\[matrix\].*cells:' "$d/$a-matrix.log" | tail -1) ($(( $(date +%s)-s )) s with the wait in the queue, try $try) $(python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | tail -3 | tr '\n' ' ' | cut -c1-600)" >> "$d/$a.txt"
done
