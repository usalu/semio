#!/bin/zsh
# 🚦️ Runs one tool process of batch MC through the machine-wide memory queue (`repo capacity run`, binding memory rule of the brief) and inside its reservation (`capped.sh`: the process tree is stopped 50 MB below the reserved megabytes and its peak is logged): q.sh <megabytes> <timeout seconds> <working directory> <command> <args…>. The command waits in the queue until the host can carry it; the timeout counts the run only.
mb="$1"; t="$2"; dir="$3"; shift 3
cd /Users/ueli/Documents/semio && exec repo capacity run --memory="$mb" -- timeout "$t" /bin/zsh -c 'cd "$1" && cap="$2" && shift 2 && exec "$0" "$cap" "$*" "$@"' "${0:h}/capped.sh" "$dir" "$(( mb - 50 ))" "$@"
