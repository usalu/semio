#!/bin/zsh
# 🚏️ Runs job files one after the other inside ONE 800 MB grant, because the queue admits a request only every quarter of an hour: each $GEN/jobs/<name>.job is one command line of a script of this folder with `--inside` forms (`prove.sh …`, `sheets.sh …`, `look.sh …`) or any command; output in <name>.log, the job file is renamed to .done; ends after IDLE seconds (default 420) without a job: jobs.sh
me="${0:A}"; source "${0:A:h}/env.sh"; mkdir -p "$GEN/jobs"
if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory=800 -- timeout 14000 "$me" --inside; fi
setopt null_glob; idle=0
while (( idle < ${IDLE:-420} )); do
  j=("$GEN"/jobs/*.job)
  if (( ${#j} == 0 )); then sleep 3; idle=$((idle + 3)); continue; fi
  idle=0; f="$j[1]"; mv "$f" "${f:r}.run"; (cd "$R"; eval "$(cat "${f:r}.run")") > "${f:r}.log" 2>&1; echo "exit $?" >> "${f:r}.log"; mv "${f:r}.run" "${f:r}.done"
done
