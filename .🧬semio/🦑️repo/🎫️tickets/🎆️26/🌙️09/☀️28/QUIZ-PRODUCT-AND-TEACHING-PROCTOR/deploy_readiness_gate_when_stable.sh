#!/bin/bash
# ⏳️ Deploy-readiness helper: other agents edit the proctor while the readiness gate has to run, so the tree compiles
# only at times. Waits until `cargo check -p teaching-proctor` passes, runs the gate at once, and repeats while the gate
# fails because the tree stopped compiling underneath it; any other failure, or a green gate, ends the loop.
#   bash deploy_readiness_gate_when_stable.sh <log directory> [attempts]
set -u
logs="$1"
attempts="${2:-6}"
export NX_PLUGIN_NO_TIMEOUTS=true RUSTC_WRAPPER=""
for attempt in $(seq 1 "$attempts"); do
  until cargo check -p teaching-proctor > /dev/null 2>&1; do sleep 30; done
  log="$logs/40-deploy-check-attempt-$attempt.log"
  date +"start %Y-%m-%d %H:%M:%S" > "$log"
  bun nx run @teaching/architecture-quiz:deploy-check >> "$log" 2>&1
  code=$?
  echo "exit=$code" >> "$log"
  date +"end %H:%M:%S" >> "$log"
  echo "[DEBUG] attempt $attempt exit=$code ($(grep -c "step [0-9]/" "$log") steps started)"
  [ "$code" -eq 0 ] && exit 0
  grep -q "could not compile\|cargo build of teaching-proctor failed\|error\[E[0-9]*\]" "$log" || exit "$code"
done
exit 1
