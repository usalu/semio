#!/bin/zsh
# 👷️ Runs the job files of one lane one after the other in ONE grant of the memory queue: worker.sh <lane>   A job is `$GEN/jobs-<lane>/<name>.job` (shell lines, run in this folder); its output is `<name>.out`, the job file is renamed to `<name>.done` with the exit code in `<name>.exit`. The worker ends after 1200 s without a job.
BATCH=MA source "${0:A:h}/../MR/env.sh"
j="$GEN/jobs-$1"; mkdir -p "$j"; idle=0
while (( idle < 1200 )); do
  next=($j/*.job(N))
  if (( ${#next} == 0 )); then sleep 2; (( idle += 2 )); continue; fi
  idle=0; f="$next[1]"; n="${f:t:r}"; mv "$f" "$j/$n.running"
  (cd "$W" && /bin/zsh "$j/$n.running" > "$j/$n.out" 2>&1); echo $? > "$j/$n.exit"; mv "$j/$n.running" "$j/$n.done"
done
