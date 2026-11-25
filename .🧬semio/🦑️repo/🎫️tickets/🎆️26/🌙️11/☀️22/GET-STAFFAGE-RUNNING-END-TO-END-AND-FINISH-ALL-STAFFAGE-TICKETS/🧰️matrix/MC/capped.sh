#!/bin/zsh
# 🧢️ Runs a command and keeps it inside its memory reservation: samples the resident memory of the whole process tree every two seconds, stops the tree when it is above the cap and appends `<time> | <label> | peak <MB> | cap <MB> | exit <n> | <s> s` to $GEN/log/peaks.txt, so every reservation of the batch rests on a measured peak: capped.sh <cap megabytes> <label> <command> <args…>
source "${0:h}/env.sh"
cap="$1"; label="$2"; shift 2
mkdir -p "$GEN/log"
tree() { ps -axo pid=,ppid=,rss= | awk -v root="$1" '{ parent[$1] = $2; rss[$1] = $3 } END { for (p in parent) { q = p; while (q != root && (q in parent) && q != 1) q = parent[q]; if (q == root) sum += rss[p] } print int(sum / 1024) }'; }
members() { ps -axo pid=,ppid= | awk -v root="$1" '{ parent[$1] = $2 } END { for (p in parent) { q = p; while (q != root && (q in parent) && q != 1) q = parent[q]; if (q == root) print p } }'; }
start=$(date +%s); peak=0; over=""
"$@" &
pid=$!
while kill -0 $pid 2>/dev/null; do
  now=$(tree $pid); (( now > peak )) && peak=$now
  if (( now > cap )); then over=" | STOPPED ABOVE THE CAP"; kill -TERM $(members $pid) 2>/dev/null; fi
  sleep 2
done
wait $pid; code=$?
echo "$(date +%H:%M) | $label | peak $peak MB | cap $cap MB | exit $code | $(( $(date +%s) - start )) s$over" >> "$GEN/log/peaks.txt"
exit $code
