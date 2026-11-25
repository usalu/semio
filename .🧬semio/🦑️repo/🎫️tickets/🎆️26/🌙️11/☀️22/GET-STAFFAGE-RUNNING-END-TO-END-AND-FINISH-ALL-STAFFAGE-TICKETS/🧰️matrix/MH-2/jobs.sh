#!/bin/zsh
# 📮️ The single work stream of batch MH-2 in the memory queue (queue rule of 2026-11-25 09:50: at most two requests of a fixer at a time, the other one is the matrix chain): jobs.sh runs, one after the other, the job files $GEN/jobs/<name>.job in name order. The first line of a job is `# <megabytes> <timeout-s>`, the rest a zsh script; its output goes to $GEN/jobs/<name>.out, the peak of its resident size to $GEN/jobs/<name>.peak (`guard.sh`, held to the reservation), and the file moves to $GEN/jobs/done/. The runner waits for new jobs and ends when $GEN/jobs/stop exists.
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"
j="$GEN/jobs"; mkdir -p "$j/done"
while [[ ! -e "$j/stop" ]]; do
  f="$(ls "$j"/*.job 2>/dev/null | head -1)"
  [[ -z "$f" ]] && { sleep 5; continue; }
  n="${f:t:r}"; read -r _ mb s < "$f"
  NOLOCK=1 "$W/q.sh" "$mb" "$s" "$T" "$W/guard.sh" "$mb" "$j/$n.peak" /bin/zsh "$f" > "$j/$n.out" 2>&1
  echo "JOBDONE $n exit $? $(date +%H:%M) $(cat "$j/$n.peak" 2>/dev/null)" | tee -a "$j/$n.out" >> "$j/log.txt"
  mv "$f" "$j/done/"
done
