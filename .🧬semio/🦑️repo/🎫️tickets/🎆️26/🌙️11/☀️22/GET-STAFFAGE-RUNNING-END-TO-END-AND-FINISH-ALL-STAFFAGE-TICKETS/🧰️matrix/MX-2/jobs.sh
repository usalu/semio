#!/bin/zsh
# 📬️ The job loop of a queue grant that is already held (sourced at the end of chain-a.sh and chain-b.sh, so one grant serves a whole series of runs, one process at a time, as the queue rule asks): jobs.sh <lane> runs every `*.job` file of 🗑️generated/matrix/MX-2/jobs-<lane> in name order (a job is a zsh script; B, R, S, T, U, W, M, GEN are set), writes its output to `<job>.log`, renames the job to `<job>.done` with its exit code inside, and ends when no job arrived for 150 seconds or a file `stop` lies in the folder. Lane `a` holds 2500 MB (vitest suites, type check), lane `b` 1000 MB (scans, gates, guards, check CLI, matrix of one person or rigid asset).
lane="$1"; jobs="$GEN/jobs-$lane"; mkdir -p "$jobs"
idle=0
while [[ ! -f "$jobs/stop" && $idle -lt 30 ]]; do
  job="$(ls "$jobs"/*.job 2>/dev/null | head -1)"
  if [[ -z "$job" ]]; then idle=$((idle + 1)); sleep 5; continue; fi
  idle=0
  echo "[DEBUG] job ${job:t} start $(date +%H:%M:%S)" | tee -a "$jobs/lane.log"
  zsh "$job" > "${job%.job}.log" 2>&1; code=$?
  echo "exit $code $(date +%H:%M:%S)" > "${job%.job}.done"; rm -f "$job"
  echo "[DEBUG] job ${job:t} exit $code $(date +%H:%M:%S)" | tee -a "$jobs/lane.log"
done
echo "[DEBUG] lane $lane ends $(date +%H:%M:%S)" | tee -a "$jobs/lane.log"
