#!/bin/zsh
# 📊️ The coverage matrix of the suite for assets one after the other (one job each, `T/📊️matrix.ts --fresh --assets <id>`) into $GEN/matrix/<id>, with the cells by status and the failing checks of each (`../MR/cells.py`) appended to $GEN/log/matrix.txt as each asset ends, so an interrupted chain keeps what it measured. The resident memory of the whole process tree (generator and asset process) is sampled every two seconds: its peak is part of the line, and a tree above MATRIX_CAP megabytes (default 2400, keep it below the reservation) is stopped, so a run never holds more than it reserved: matrix.sh <id>…   (run it through q.sh with the reservation the measured peak allows)
source "${0:h}/env.sh"
mkdir -p "$GEN/matrix" "$GEN/log"
cap=${MATRIX_CAP:-2400}
tree() { ps -axo pid=,ppid=,rss= | awk -v root="$1" '{ parent[$1] = $2; rss[$1] = $3 } END { for (p in parent) { q = p; while (q != root && (q in parent) && q != 1) q = parent[q]; if (q == root) sum += rss[p] } print int(sum / 1024) }'; }
for a in "$@"; do
  out="$GEN/matrix/$a"; rm -rf "$out"; mkdir -p "$out"; start=$(date +%s); peak=0; over=""
  (cd "$T" && exec timeout ${MATRIX_TIMEOUT:-5400} "$B" 📊️matrix.ts --language en --out "$out" --jobs 1 --fresh --assets "$a") > "$out/run.log" 2>&1 &
  pid=$!
  while kill -0 $pid 2>/dev/null; do
    now=$(tree $pid); (( now > peak )) && peak=$now
    if (( now > cap )); then over=" | STOPPED ABOVE THE CAP OF $cap MB"; kill -TERM $pid; fi
    sleep 2
  done
  wait $pid; code=$?
  echo "$a | exit $code | $(( $(date +%s) - start )) s | peak $peak MB$over | $(python3 "$M/cells.py" "$out" 2>&1 | tr '\n' ' ' | cut -c1-700)" | tee -a "$GEN/log/matrix.txt"
done
