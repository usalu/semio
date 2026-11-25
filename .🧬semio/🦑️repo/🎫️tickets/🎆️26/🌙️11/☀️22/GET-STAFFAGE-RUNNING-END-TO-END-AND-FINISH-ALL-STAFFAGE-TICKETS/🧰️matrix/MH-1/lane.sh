#!/bin/zsh
# 📬️ The job loop of ONE queue grant (the pattern of `../MX-2/jobs.sh`): lane.sh <lane> [<MB>] takes every `*.job` file of $GEN/jobs-<lane> in name order (a job is a zsh script; B, R, S, T, U, W, M, GEN and INSIDE are set), one process at a time inside one grant of <MB> (default 1200), writes its output to <job>.log, renames the job to <job>.done with the exit code, and ends when no job arrived for 300 seconds or a file `stop` lies in the folder.
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"; lane="$1"; jobs="$GEN/jobs-$lane"; mkdir -p "$jobs"
if [[ -z "$INSIDE" ]]; then INSIDE=1 exec "$W/q.sh" ${2:-1200} 86400 "$W" "${0:A}" "$lane"; fi
export INSIDE=1 B R S T U W M GEN; idle=0
while [[ ! -f "$jobs/stop" && $idle -lt 60 ]]; do
  job="$(ls "$jobs" | grep '\.job$' | head -1)"; job="${job:+$jobs/$job}"
  if [[ -z "$job" ]]; then idle=$((idle + 1)); sleep 5; continue; fi
  idle=0; echo "job ${job:t} start $(date +%H:%M:%S)" >> "$jobs/lane.log"
  zsh "$job" > "${job%.job}.log" 2>&1; echo "exit $? $(date +%H:%M:%S)" > "${job%.job}.done"; rm -f "$job"
  echo "job ${job:t} $(cat "${job%.job}.done")" >> "$jobs/lane.log"
done
echo "lane $lane ends $(date +%H:%M:%S)" >> "$jobs/lane.log"
