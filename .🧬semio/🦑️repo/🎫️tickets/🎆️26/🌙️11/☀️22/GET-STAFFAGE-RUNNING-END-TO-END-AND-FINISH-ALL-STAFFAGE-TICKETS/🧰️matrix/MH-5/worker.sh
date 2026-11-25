#!/bin/zsh
# 👷️ The worker of this fixer: ONE grant of 2500 MB of the machine-wide queue (worker.sh start) inside which the queued jobs of $GEN/jobs run one after the other, each a zsh line that may only start what fits the reservation (fits, model gates, guards, measure, the real gate, the scan, the six tests, the search): the session of this fixer shares its memory budget with other fixers, a grant waits 30 to 80 minutes, so the steps of a landing cannot queue one by one. The check CLI (5000 MB) and the matrix (7000 MB) never run in here (step.sh refuses them inside a grant); they queue for themselves. worker.sh guarded is the same worker under a grant of ${WMEM:-800} MB whose process tree `../MM/guarded.sh` stops when it holds more resident memory than the grant (its log ends with the peak: the measurement that allows the smaller reservation). JOBS=<dir> names another job folder (a second lane, in place of the matrix lane: never more than two requests of this fixer in the queue). worker.sh add <name> <zsh line> queues a job (the output goes to $GEN/jobs/<n>-<name>.out, the last line is `exit <code>`); the worker ends after twenty minutes without a job or when $GEN/jobs/stop exists.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
jobs="${JOBS:-$GEN/jobs}"; mkdir -p "$jobs"
case "$1" in
  start) rm -f "$jobs/stop"; MEM=2500 "$W/q.sh" env MH5_GRANT=1 "$W/worker.sh" loop ;;
  guarded) rm -f "$jobs/stop"; MEM="${WMEM:-800}" "$W/q.sh" env MH5_GRANT=1 "$U/🧰️matrix/MM/guarded.sh" "${WMEM:-800}" "$jobs/guarded.log" "$W/worker.sh" loop ;;
  add) n=$(( $(ls "$jobs" | grep -cE '\.(job|running|done)$') + 1 )); print -r -- "$3" > "$jobs/$(printf '%03d' $n)-$2.job"; echo "$(printf '%03d' $n)-$2" ;;
  loop)
    echo "[DEBUG] worker admitted $(date +%H:%M)" > "$jobs/worker.log"; idle=0
    while true; do
      next="$(ls "$jobs"/*.job 2>/dev/null | head -1)"
      if [[ -z "$next" ]]; then [[ -f "$jobs/stop" || $idle -ge ${IDLE:-240} ]] && break; sleep 5; idle=$((idle + 1)); continue; fi
      idle=0; name="${next:t:r}"; mv "$next" "$jobs/$name.running"
      (cd "$W" && zsh "$jobs/$name.running") > "$jobs/$name.out" 2>&1; echo "exit $?" >> "$jobs/$name.out"; mv "$jobs/$name.running" "$jobs/$name.done"
      echo "[DEBUG] $(date +%H:%M) $name" >> "$jobs/worker.log"
    done
    echo "[DEBUG] worker ended $(date +%H:%M)" >> "$jobs/worker.log" ;;
esac
