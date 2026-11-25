#!/bin/zsh
# 📬️ One queue grant of 1000 MB that serves a series of small runs, one process at a time (the job loop of MX-2, jobs.sh): runs every `*.job` file (a zsh script; B, R, S, T, U, W, M, GEN are set) of $GEN/lane in name order, writes its output to `<job>.log`, renames it to `<job>.done` and ends when no job arrived for 120 seconds: lane.sh   (LANE=<folder>, MB=<megabytes>)
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"
export LANE="${LANE:-lane}"; mkdir -p "$GEN/$LANE"
cd "$R" && SEMIO_LANGUAGE=en repo capacity run --memory="${MB:-1000}" -- timeout 14400 zsh -c '
idle=0
while [[ $idle -lt 24 ]]; do
  job="$(ls "$GEN/$LANE" | grep "\.job$" | head -1)"
  if [[ -z "$job" ]]; then idle=$((idle + 1)); sleep 5; continue; fi
  idle=0; echo "start $job $(date +%H:%M:%S)"
  zsh "$GEN/$LANE/$job" > "$GEN/$LANE/${job%.job}.log" 2>&1; code=$?
  mv "$GEN/$LANE/$job" "$GEN/$LANE/${job%.job}.done"; echo "end $job exit $code $(date +%H:%M:%S)"
done' >> "$GEN/$LANE.log" 2>&1
echo "lane closed $(date +%H:%M:%S)" >> "$GEN/$LANE.log"
