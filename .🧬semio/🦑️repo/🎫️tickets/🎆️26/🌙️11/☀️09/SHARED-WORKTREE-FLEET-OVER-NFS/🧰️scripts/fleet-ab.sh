#!/bin/bash
# usage: fleet-ab.sh LABEL RATE SECONDS [--faults] -- one reduced fleet run (FLEET_GROUPS x FLEET_PER_GROUP from the environment): rebuild, optional busy loops (FLEET_BURN=n, recorded by pid and stopped by pid), run, keep $R/run as $R/results/LABEL, print the summary. Env: S, FLEET_MODSRC, FLEET_RELAY_CONC, FLEET_AWAY.
set -u
here=$(cd "$(dirname "$0")" && pwd); cd "$here"
export R="${R:-$S/rehearsal}"; label=$1; rate=$2; seconds=$3; shift 3
mkdir -p "$R/results"; burn_pids=()
python3 fleet.py stop >/dev/null 2>&1
t0=$(date +%s); python3 fleet.py build > "$R/results/$label.build" 2>&1; echo "build $(( $(date +%s) - t0 )) s: $(tail -1 "$R/results/$label.build")"
for ((i=0;i<${FLEET_BURN:-0};i++)); do python3 -c "while True: pass" & burn_pids+=($!); done
[ ${#burn_pids[@]} -gt 0 ] && echo "${burn_pids[*]}" > "$R/burn.pids"
python3 fleet-run.py "$seconds" "$rate" "$@" > "$R/results/$label.log" 2>&1; status=$?
for p in ${burn_pids[@]+"${burn_pids[@]}"}; do kill "$p" 2>/dev/null; done; rm -f "$R/burn.pids"
python3 fleet.py stop >/dev/null 2>&1
rm -rf "$R/results/$label"; cp -R "$R/run" "$R/results/$label"; rm -rf "$R/results/$label/sample/../converge-cache"* 2>/dev/null
echo "exit $status"
python3 fleet-summary.py "$R/results/$label"
