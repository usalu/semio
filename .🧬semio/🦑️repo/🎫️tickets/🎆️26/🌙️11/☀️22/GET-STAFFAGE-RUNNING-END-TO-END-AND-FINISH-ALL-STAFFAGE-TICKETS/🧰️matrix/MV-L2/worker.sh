#!/bin/zsh
# 👷️ Runs the jobs of round MV-L2 one after the other inside one grant: `q.sh <MB> $W <timeout> zsh worker.sh <MB> [lane]`. A job is a file `$GEN/jobs/<megabytes, four digits>-<name>.sh`; the worker takes the first one its grant can carry, renames it to `.run`, runs it (output to `$GEN/log/job-<name>.log`), renames it to `.done`, and ends when no job came for 180 s.
source "${0:A:h}/env.sh"
mkdir -p "$GEN/jobs" "$GEN/log"; idle=0
while (( idle < 18 )); do
  job=""; for f in "$GEN"/jobs/*.sh(N); do (( 10#${${f:t}%%-*} <= $1 )) && { job="$f"; break; }; done
  if [[ -z "$job" ]]; then sleep 10; (( idle++ )); continue; fi
  idle=0; mv "$job" "$job.run" 2>/dev/null || continue
  n="${${job:t}%.sh}"; echo "$(date +%H:%M:%S) $n start" >> "$GEN/log/jobs.txt"; /usr/bin/time -l -o "$GEN/log/rss-$n.txt" zsh "$job.run" > "$GEN/log/job-$n.log" 2>&1; echo "$(date +%H:%M:%S) $n exit $? $(( $(awk '/maximum resident set size/{print $1}' "$GEN/log/rss-$n.txt") / 1048576 )) MB" >> "$GEN/log/jobs.txt"; rm -f "$GEN/log/rss-$n.txt"; mv "$job.run" "$job.done"
done
