#!/bin/zsh
# 👷️ The second long grant (same as `worker.sh` with `jobs2.txt`, `job2-<n>.log`, `jobs2-done.txt`): one long grant that works through the job list of lane CL-T1, one job after the other: every line of `$GEN/jobs2.txt` is `<script of this folder> <args…>` (lines are appended while it runs); each job's output goes to `$GEN/log/job-<n>.log` and its exit code and seconds to `$GEN/log/jobs2-done.txt`. A line `stop` ends the worker; so do 20 minutes without a new line. worker2.sh
source "${0:A:h}/env.sh"
mkdir -p "$GEN/log"; touch "$GEN/jobs2.txt"; n=${$(cat "$GEN/log/jobs2-next.txt" 2>/dev/null):-1}; idle=0
while (( idle < 240 )); do
  line="$(sed -n "${n}p" "$GEN/jobs2.txt")"
  if [[ -z "$line" ]]; then sleep 5; idle=$((idle+1)); continue; fi
  idle=0; [[ "$line" == stop ]] && { echo $((n+1)) > "$GEN/log/jobs2-next.txt"; break; }
  s=$(date +%s); zsh -c "cd '$W' && zsh $line" > "$GEN/log/job2-$n.log" 2>&1; r=$?
  echo "$(date +%H:%M:%S) job $n exit $r ($(( $(date +%s)-s )) s): $line" | cut -c1-300 >> "$GEN/log/jobs2-done.txt"
  n=$((n+1)); echo $n > "$GEN/log/jobs2-next.txt"
done
