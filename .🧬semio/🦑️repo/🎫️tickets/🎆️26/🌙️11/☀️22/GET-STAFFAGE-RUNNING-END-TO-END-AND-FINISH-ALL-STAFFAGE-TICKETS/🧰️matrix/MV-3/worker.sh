#!/bin/zsh
# 👷️ Runs the jobs of batch MV-3 one after the other inside one grant, so a search of many short steps waits in the crowded queue once: worker.sh <megabytes of the grant>. A job is a file `$GEN/jobs/<megabytes, four digits>-<name>.sh`; the worker takes the first one its grant can carry, renames it to `.run`, runs it (output to `$GEN/log/job-<name>.log`), renames it to `.done`, and ends when no job it can carry came for 240 s, so an idle grant goes back.
export BATCH=MV-3 MV3_GRANT=1; source "${0:A:h}/../MR/env.sh"
mkdir -p "$GEN/jobs"; idle=0
while (( idle < 24 )); do
  job=""; for f in "$GEN"/jobs/*.sh(N); do (( 10#${${f:t}%%-*} <= $1 )) && { job="$f"; break; }; done
  if [[ -z "$job" ]]; then sleep 10; (( idle++ )); continue; fi
  idle=0; mv "$job" "$job.run" 2>/dev/null || continue
  n="${${job:t}%.sh}"; echo "$(date +%H:%M) $n start" >> "$GEN/log/jobs.txt"; zsh "$job.run" > "$GEN/log/job-$n.log" 2>&1; echo "$(date +%H:%M) $n exit $?" >> "$GEN/log/jobs.txt"; mv "$job.run" "$job.done"
done
