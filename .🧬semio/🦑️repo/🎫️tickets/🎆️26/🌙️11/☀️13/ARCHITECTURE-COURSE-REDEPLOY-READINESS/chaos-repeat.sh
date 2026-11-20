#!/bin/sh
# 🔁️ Starts the chaos stack (dev topology unless STATIC=1) on PROCTOR_PORT/SITE_PORT and runs the chaos crowd spec CHAOS_RUNS times against it, one log per run and one result line per run in repeat.log. Env: CHAOS_RUNS CHAOS_WORK STATIC PROCTOR_PORT SITE_PORT
D="$(cd "$(dirname "$0")" && pwd)"
W="$D/${CHAOS_WORK:-🗑️generated/chaos}"
mkdir -p "$W"
export PATH=$HOME/.local/share/semio/node/v24.15.0/bin:$PATH CHAOS_DATA="${CHAOS_DATA:-/private/tmp/claude-501/-Users-ueli-Documents-semio/9d0eceba-b1b8-4814-b9cd-caae5bbd19aa/scratchpad/chaos-data-${PROCTOR_PORT:-8793}}"
node "$D/chaos-stack.mjs" > "$W/stack.log" 2>&1 &
STACK=$!
until grep -q "stack ready" "$W/stack.log" 2>/dev/null; do kill -0 $STACK 2>/dev/null || { echo "stack died" >> "$W/repeat.log"; exit 1; }; sleep 2; done
[ "${STATIC:-0}" = 1 ] && PROCTOR_URL="http://127.0.0.1:${PROCTOR_PORT:-8793}" node "$D/chaos-build.mjs" >> "$W/stack.log" 2>&1
for run in $(seq 1 "${CHAOS_RUNS:-3}"); do
  CHAOS_BROWSERS=${CHAOS_BROWSERS:-10} CHAOS_PROTOCOL=${CHAOS_PROTOCOL:-90} CHAOS_QUIZZES=${CHAOS_QUIZZES:-10} "$D/chaos-run.sh" > "$W/run-$run.log" 2>&1
  echo "run $run exit $? $(date +%H:%M) $(grep -E '[0-9]+ (passed|failed)' "$W/run-$run.log" | tr '\n' ' ')" >> "$W/repeat.log"
done
kill -TERM $STACK; wait $STACK
echo finished >> "$W/repeat.log"
