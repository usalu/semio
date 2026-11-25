#!/bin/zsh
# 📮️ Puts one job into a lane of `worker.sh`, starts the lane's worker through the memory queue when none runs, and waits up to WAIT (default 160) seconds for the result: job.sh <lane> <MB> <name> <shell line>
BATCH=MA source "${0:A:h}/../MR/env.sh"
lane="$1"; mb="$2"; n="$3"; shift 3; j="$GEN/jobs-$lane"; mkdir -p "$j"
rm -f "$j/$n".(out|exit|done)(N); print -r -- "$*" > "$j/$n.job"
pgrep -f "MA/worker.sh $lane" > /dev/null || (cd "$R" && SEMIO_LANGUAGE=en nohup repo capacity run --memory="$mb" -- timeout 36000 "$W/worker.sh" "$lane" > "$j/worker.log" 2>&1 &)
for (( i = 0; i < ${WAIT:-160}; i += 2 )); do [[ -f "$j/$n.exit" ]] && break; sleep 2; done
[[ -f "$j/$n.exit" ]] && { echo "[$n] exit $(cat "$j/$n.exit")"; cat "$j/$n.out"; } || { echo "[$n] pending"; tail -1 "$j/worker.log" | cut -c1-160; }
