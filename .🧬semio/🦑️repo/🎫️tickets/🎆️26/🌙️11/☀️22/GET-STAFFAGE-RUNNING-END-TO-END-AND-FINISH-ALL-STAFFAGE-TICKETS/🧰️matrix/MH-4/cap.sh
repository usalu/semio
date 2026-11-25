#!/bin/zsh
# 🚦️ One bun or vitest run through the machine-wide memory queue of the repo (binding since 2026-11-24 23:00): cap.sh <MB> <timeout-s> <working-dir> <command…>   runs `repo capacity run --memory=<MB> -- timeout <s> …` from the repo root; the command itself starts in <working-dir>. It waits in the queue until the host can carry it; when the queue ends the wait itself ("The wait ends here", after about 40 minutes without the memory) the command was never started and is queued again, up to 30 times. The lines of the queue go to $GEN/capacity.log, not into the output. A run started inside a grant of q.sh that is at least as large (MH4_GRANT) runs directly under that grant and its peak resident memory is noted in $GEN/rss.log. Reserve 7000 for the matrix, 5000 for the check CLI, 2500 for gate, measure and every vitest run, 1000 for the scan and small scripts.
source "${0:A:h}/env.sh"
mb="$1"; s="$2"; dir="$3"; shift 3
if [[ -n "$MH4_GRANT" && "$mb" -le "$MH4_GRANT" ]]; then
  cd "$dir" || exit 1
  /usr/bin/time -l -o "$GEN/rss.$$" timeout "$s" "$@"; code=$?
  echo "$(date +%H:%M:%S) $(( $(awk '/maximum resident set size/ { print $1 }' "$GEN/rss.$$") / 1048576 )) MB peak, reserved $mb: ${${@[2]}:t} ${@[3]:t}" >> "$GEN/rss.log"; rm -f "$GEN/rss.$$"
  exit $code
fi
mkdir -p "$GEN"; tmp="$(mktemp "$GEN/cap.XXXXXX")"
for attempt in {1..30}; do
  (cd "$R" && repo capacity run --memory="$mb" -- timeout "$s" /bin/zsh -c 'cd "$1" && shift && exec "$@"' cap "$dir" "$@") > "$tmp" 2>&1
  code=$?
  if grep -q "The wait ends here instead of going on" "$tmp"; then echo "$(date +%H:%M:%S) queue gave up, attempt $attempt: $mb MB $*" | cut -c1-200 >> "$GEN/capacity.log"; sleep 45; continue; fi
  break
done
grep "^\[capacity\]" "$tmp" >> "$GEN/capacity.log"
grep -v "^\[capacity\]" "$tmp"
rm -f "$tmp"
exit $code
