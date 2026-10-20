#!/bin/sh
# usage: measure-after-lift.sh [cold]   AFTER the coordinator's GO and a quiet host: one cold-or-warm `show projects`, the per-phase rows, a concurrent reader probe and the descriptor count.
# Writes only under 🗑️generated/graph-speed/ of this ticket (kept small: filtered stderr). `cold` first removes nothing itself: run `bun repo cache prune inference` yourself when a true cold run is wanted.
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"
OUT="$HERE/../🗑️generated/graph-speed"; mkdir -p "$OUT"
cd "$ROOT" || exit 2
export SEMIO_GRAPH_PHASES=1
start=$(date +%s)
( bun repo show projects 2> "$OUT/stderr.log" > "$OUT/stdout.log" ) &
PID=$!
peak=0
while kill -0 "$PID" 2>/dev/null; do
  for child in $(pgrep -P "$PID" 2>/dev/null) "$PID"; do
    n=$(lsof -p "$child" 2>/dev/null | grep -c '\.pack')
    [ "$n" -gt "$peak" ] && peak=$n
  done
  sleep 5
done
wait "$PID"; rc=$?
end=$(date +%s)
echo "exit $rc, wall $((end-start)) s, peak open .pack descriptors $peak (limit 32 per process)"
grep '^\[graph-phase\]' "$OUT/stderr.log" | tail -40
echo "second run (warm, must hit the sealed graph without the writer lease):"
start=$(date +%s); bun repo show projects 2> "$OUT/stderr2.log" > /dev/null; echo "wall $(( $(date +%s)-start )) s"
grep -c 'repository-graph-facts' "$OUT/stderr2.log" | sed 's/^/lines naming the graph facts lease: /'
