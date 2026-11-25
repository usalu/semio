#!/bin/zsh
# 📬️ One queue grant that serves a series of short runs of the second takeover, one after the other: GRANT=1200 g.sh jobs.sh 20000   runs every `*.job` of $GEN/jobs (a grant below 1000 MB: $GEN/jobs-light) in name order (a job is a zsh script sourced with the paths of env.sh), writes its output to <job>.log and <job>.done, and gives the grant back when no job arrived for 150 seconds or a file `stop` lies in the folder.
touch "$GEN/lane.$MH4_GRANT"; d="$GEN/jobs"; (( MH4_GRANT < 1000 )) && d="$GEN/jobs-light"; mkdir -p "$d"; idle=0
while [[ ! -f "$d/stop" && $idle -lt 30 ]]; do
  job="$(ls "$d"/*.job 2>/dev/null | head -1)"
  if [[ -z "$job" ]]; then idle=$((idle + 1)); sleep 5; continue; fi
  idle=0
  mv "$job" "${job%.job}.run" 2>/dev/null || continue
  (source "${job%.job}.run") > "${job%.job}.log" 2>&1; code=$?
  mv "${job%.job}.run" "${job%.job}.done"; echo "[DEBUG] ${job:t} exit $code $(date +%H:%M:%S)" >> "$d/lane.log"
done
