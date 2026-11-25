#!/bin/zsh
# 👷️ Runs the jobs of batch ML-3 one after the other inside ONE grant of the machine-wide memory queue, so a chain of steps waits in the crowded queue once: `q.sh <MB> <dir> <timeout> zsh worker.sh <MB>`. A job is a file `$GEN/jobs/<megabytes, four digits>-<name>.sh`; the worker takes the first one its grant can carry, renames it to `.run`, runs it (output `$GEN/log/job-<name>.log`), renames it to `.done`, and ends when no job came for 420 s, so an idle grant goes back. Jobs call the steps of `steps.sh`.
source "${0:A:h}/env.sh"
mkdir -p "$GEN/jobs" "$GEN/log"; idle=0
while (( idle < 42 )); do
  job=""; for f in "$GEN"/jobs/*.sh(N); do (( 10#${${f:t}%%-*} <= $1 && 10#${${f:t}%%-*} >= ${2:-0} )) && { job="$f"; break; }; done
  if [[ -z "$job" ]]; then sleep 10; (( idle++ )); continue; fi
  idle=0; mv "$job" "$job.run" 2>/dev/null || continue
  n="${${job:t}%.sh}"; echo "$(date +%H:%M:%S) $n start" >> "$GEN/log/jobs.txt"; zsh "$job.run" > "$GEN/log/job-$n.log" 2>&1; echo "$(date +%H:%M:%S) $n exit $?" >> "$GEN/log/jobs.txt"; mv "$job.run" "$job.done"
done
echo "$(date +%H:%M:%S) worker $1 MB ends idle" >> "$GEN/log/jobs.txt"
