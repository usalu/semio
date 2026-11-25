#!/bin/zsh
# 🚦️ Runs one tool process of batch MU through the machine-wide memory queue (`repo capacity run`, binding memory rule of the brief) and inside its reservation (`capped.sh`: the process tree is stopped 50 MB below the reserved megabytes and its peak is logged): q.sh <megabytes> <timeout seconds> <working directory> <command> <args…>. The command waits in the queue until the host can carry it and asks again when the queue gives the request up or a signal from outside ends the run (at most 30 times); the timeout counts the run only.
mb="$1"; t="$2"; dir="$3"; shift 3
log="$(mktemp)"; again=0
while :; do
  (cd /Users/ueli/Documents/semio && repo capacity run --memory="$mb" -- timeout "$t" /bin/zsh -c 'cd "$1" && cap="$2" && shift 2 && exec "$0" "$cap" "$*" "$@"' "${0:h}/capped.sh" "$dir" "$(( mb - 50 ))" "$@") > "$log" 2>&1
  code=$?
  [[ $code -ne 0 ]] && grep -q "The wait ends here" "$log" && continue
  if [[ $code -eq 130 || $code -eq 143 ]] && (( ++again <= 30 )); then sleep 20; continue; fi
  break
done
grep -v '^\[capacity\]' "$log"; rm -f "$log"; exit $code
