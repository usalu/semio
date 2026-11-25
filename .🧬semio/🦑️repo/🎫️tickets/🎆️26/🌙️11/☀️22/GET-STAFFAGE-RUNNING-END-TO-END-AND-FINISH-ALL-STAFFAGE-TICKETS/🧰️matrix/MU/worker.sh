#!/bin/zsh
# 👷️ One queue grant for the whole lane, one tool process at a time: runs the job files `$GEN/jobs/<name>.sh` in name order as they appear (log `$GEN/jobs/done/<name>.log`, the file is moved there when it ended), and while none waits the next filler line of `$GEN/filler.txt` (a command line of this folder, removed when it ended); it ends when there has been no work for two minutes: worker.sh   (run it through q.sh)
source "${0:h}/env.sh"
mkdir -p "$GEN/jobs/done"; idle=0
while (( idle < 24 )); do
  job=($GEN/jobs/*.sh(N)); line=""
  [[ -s "$GEN/filler.txt" ]] && line="$(head -1 "$GEN/filler.txt")"
  if (( ${#job} > 0 )); then
    idle=0; n="${job[1]:t:r}"; mv "$job[1]" "$GEN/jobs/done/$n.sh"
    zsh "$GEN/jobs/done/$n.sh" > "$GEN/jobs/done/$n.log.part" 2>&1; echo "[DEBUG] exit $?" >> "$GEN/jobs/done/$n.log.part"; mv "$GEN/jobs/done/$n.log.part" "$GEN/jobs/done/$n.log"
  elif [[ -n "$line" ]]; then
    idle=0; eval "\"$W\"/$line" >> "$GEN/filler.log" 2>&1
    sed -i '' 1d "$GEN/filler.txt"
  else idle=$((idle + 1)); sleep 5; fi
done
