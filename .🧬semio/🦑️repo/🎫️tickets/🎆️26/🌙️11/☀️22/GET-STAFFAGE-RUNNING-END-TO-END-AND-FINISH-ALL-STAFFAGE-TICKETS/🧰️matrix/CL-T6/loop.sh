#!/bin/zsh
# 👷️ The job loop of lane CL-T6 WITHOUT a grant of its own: it works through `$GEN/<list>.txt` one line after the other (`<script of this folder> <args…>`, lines are appended while it runs) and asks the memory queue for every job by itself: `repo capacity run --memory=<MB>` around the job (2500 MB for a chain with its model-catalog run, 1000 MB for the measuring tools), so nothing is reserved while the loop waits or between jobs. Output of a job in `$GEN/log/<list>-<n>.log`, exit code and seconds in `$GEN/log/<list>-done.txt`. A line `stop` ends the loop; so do 30 minutes without a new line. loop.sh <list>
source "${0:A:h}/env.sh"
l="$1"; mkdir -p "$GEN/log"; touch "$GEN/$l.txt"; n=${$(cat "$GEN/log/$l-next.txt" 2>/dev/null):-1}; idle=0
while (( idle < 360 )); do
  line="$(sed -n "${n}p" "$GEN/$l.txt")"
  if [[ -z "$line" ]]; then sleep 5; idle=$((idle+1)); continue; fi
  idle=0; [[ "$line" == stop ]] && { echo $((n+1)) > "$GEN/log/$l-next.txt"; break; }
  mb=1000; [[ "$line" == chain* || "$line" == recat* || "$line" == try* ]] && mb=3600
  s=$(date +%s); (cd "$R" && repo capacity run --memory=$mb -- timeout 14400 zsh -c "cd '$W' && zsh $line") > "$GEN/log/$l-$n.log" 2>&1; r=$?
  echo "$(date +%H:%M:%S) job $n exit $r ($(( $(date +%s)-s )) s, $mb MB asked): $line" | cut -c1-300 >> "$GEN/log/$l-done.txt"
  n=$((n+1)); echo $n > "$GEN/log/$l-next.txt"
done
