#!/bin/sh
# 🏁️ The full multiview matrix over the whole catalog, in lanes and resumable: STAFFAGE_LANGUAGE=en|de sh full.sh <run folder> [--lanes <n>] [--retry-failed] [--minutes <n>] [--assets a,b]
# `plan.ts` writes one line per request (memory, processes at a time, limit of one asset, asset ids; the heaviest first); `--lanes` workers (default 3) take the lines one by one, each line is ONE `repo capacity run` request of `📊️matrix.ts`, so at most `--lanes` requests of this run are in the queue at any time. A rerun skips every asset whose record is current: a cut (Ctrl-C, SIGTERM, a reboot, a killed lane) loses only the assets that were running and nothing else. SIGINT and SIGTERM stop all lanes and their requests.
# The rounds repeat until no asset is left or a round finishes no asset (an asset that fails by name keeps its record; `--retry-failed` runs those again, in the first round only). The merge runs at the end in a request of its own; its summary is `<run folder>/summary.json`, the merged cells `<run folder>/matrix.json`, the log of every request `<run folder>/lanes/<round>-<line>.log`.
# Do not start it before every batch is FINAL; do not start a second copy on the same run folder.
set -u
R=/Users/ueli/Documents/semio
T="$R/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage"
M="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🧰️matrix/MM"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
: "${STAFFAGE_LANGUAGE:?en or de}"
case "${1:-}" in "" | -*) echo "usage: STAFFAGE_LANGUAGE=en|de sh full.sh <run folder> [--lanes <n>] [--retry-failed] [--minutes <n>] [--assets a,b]" >&2; exit 2 ;; esac
mkdir -p "$1" || exit 1
out=$(cd "$1" && pwd); shift
cd "$R" || exit 1
retry=""; minutes=30; assets=""; lanes=3
while [ $# -gt 0 ]; do
  case "$1" in
    --retry-failed) retry="--retry-failed" ;;
    --minutes) minutes=$2; shift ;;
    --assets) assets=$2; shift ;;
    --lanes) lanes=$2; shift ;;
    *) echo "full: unknown option $1" >&2; exit 2 ;;
  esac
  shift
done
mkdir -p "$out/lanes" || exit 1
children=""
stop() { for pid in $children; do kill -TERM "$pid" 2>/dev/null; done; wait; exit 130; }
trap stop INT TERM

next() {
  until mkdir "$out/.lock" 2>/dev/null; do sleep 0.2; done
  n=$(( $(cat "$out/.next") + 1 ))
  echo "$n" > "$out/.next"
  rmdir "$out/.lock"
  echo "$n"
}

lane() {
  child=""
  trap '[ -n "$child" ] && kill -TERM "$child" 2>/dev/null; wait; exit 130' INT TERM
  while :; do
    at=$(next)
    [ "$at" -gt "$count" ] && return 0
    line=$(sed -n "${at}p" "$out/plan.txt")
    memory=${line%% *}; rest=${line#* }; jobs=${rest%% *}; rest=${rest#* }; limit=${rest%% *}; ids=${rest#* }
    echo "full: round $round request $at of $count, $memory MB, $jobs at a time, $ids" >&2
    repo capacity run --memory="$memory" -- timeout 86400 "$BUN" "$T/📊️matrix.ts" --language "$STAFFAGE_LANGUAGE" --out "$out" --jobs "$jobs" --timeout-s "$limit" --progress "$out/lanes/$round-$at.progress.json" --shard 0/1 $retry --assets "$ids" > "$out/lanes/$round-$at.log" 2>&1 &
    child=$!
    wait "$child" || echo "full: request $at of round $round ended with $?" >&2
    child=""
  done
}

round=0
before=-1
while :; do
  round=$((round + 1))
  repo capacity run --memory=800 -- timeout 900 "$BUN" "$M/plan.ts" --out "$out" --plan "$out/plan.txt" --minutes "$minutes" $retry ${assets:+--assets "$assets"} || { echo "full: the plan failed" >&2; exit 1; }
  left=$(awk '{ n += split($4, ids, ",") } END { print n + 0 }' "$out/plan.txt")
  [ "$left" -eq 0 ] && break
  [ "$left" -eq "$before" ] && { echo "full: the last round finished no asset, $left left" >&2; break; }
  before=$left
  count=$(wc -l < "$out/plan.txt" | tr -d ' ')
  echo 0 > "$out/.next"
  children=""
  i=0
  while [ "$i" -lt "$lanes" ]; do
    lane &
    children="$children $!"
    i=$((i + 1))
  done
  for pid in $children; do wait "$pid"; done
  children=""
  retry=""
done
repo capacity run --memory=700 -- timeout 3600 "$BUN" "$T/📊️matrix.ts" --language "$STAFFAGE_LANGUAGE" --out "$out" --merge-only ${assets:+--assets "$assets"}
