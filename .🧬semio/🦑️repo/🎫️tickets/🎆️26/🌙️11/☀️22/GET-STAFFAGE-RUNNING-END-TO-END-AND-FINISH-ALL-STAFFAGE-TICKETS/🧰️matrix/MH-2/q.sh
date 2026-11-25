#!/bin/zsh
# 🚏️ One tool process through the machine-wide memory queue (binding since 2026-11-24 23:00): q.sh <megabytes> <timeout-s> <working-dir> <command> <args…> runs, from the repo root, `repo capacity run --memory=<MB> -- timeout <s> …` with the command started in the working directory. The command waits in the queue until the host can carry it; the timeout counts from its start, and the queue's own `waiting` lines are kept out of the logs. A wait the queue ends by itself (memory starved or waited out, the command never started) is asked again, at most twelve times. A run of 2500 MB or more first takes the batch's own lock, so that this fixer never has two heavy runs at once, whatever chains are started side by side (NOLOCK=1 leaves the lock out).
mb="$1"; s="$2"; dir="$3"; shift 3
cd /Users/ueli/Documents/semio || exit 2
lock=(); (( mb >= 2500 )) && [[ -z "$NOLOCK" ]] && lock=(lockf -k "${TMPDIR:-/tmp}/semio-heavy-MH-2.lock")
err="$(mktemp)"; started="$(mktemp -u)"; trap 'rm -f "$err" "$started"' EXIT
for try in {1..12}; do
  $lock repo capacity run --lang=en --memory="$mb" -- timeout "$s" /bin/zsh -c ': > "$1" && cd "$2" && shift 2 && exec "$@"' _ "$started" "$dir" "$@" 2> "$err"; code=$?
  grep -v '^\[capacity\]' "$err" >&2
  [[ -e "$started" ]] && break
  grep -q 'The wait ends here' "$err" || break
done
exit $code
