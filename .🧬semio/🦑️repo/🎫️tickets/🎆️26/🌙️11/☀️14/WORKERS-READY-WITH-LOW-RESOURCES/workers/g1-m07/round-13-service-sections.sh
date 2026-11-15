#!/bin/bash
# 🧾 Prints the service-log sections of the round 13 report: the lines ROUND-NOTE-P12 section 2 asks for, the stack of a reference, and the typed refusal with the 20 lines before it. Reads only. Usage: round-13-service-sections.sh <since UTC, %Y-%m-%dT%H:%M:%SZ> <reference>
T="$TMPDIR/semio-repo/$(id -u)"; since="$(date -j -u -f %Y-%m-%dT%H:%M:%SZ "$1" +%s)"
logs=(); for l in $(ls -tr "$T"/host-*.log); do [ "$(stat -f %m "$l")" -ge "$since" ] && logs+=("$l"); done
echo "## Service logs of this round, verbatim (restore line with A and S, plan stages, fleet, too heavy, closure-missing)"
for l in "${logs[@]}"; do echo "### $(basename "$l")"; echo '```'; grep -ahE 'restored|graph: plan stage|graph-plan-too-heavy|graph: refreshing|graph: [0-9]+ stale projects|taken from the fleet|inferred here|nothing to wait|\[graph-phase\]|closure-missing|generation [0-9]+ published|request-failed' "$l" | cut -c1-900; echo '```'; done
echo; echo "## Failure reference $2, stack"; echo '```'; for l in "${logs[@]}"; do grep -a -A7 "request-failed $2" "$l" | grep -av 'still running\|still working' | cut -c1-420; done; echo '```'
echo; echo "## The typed refusal with the 20 lines before it"; echo '```'; for l in "${logs[@]}"; do grep -a -B20 'graph-plan-too-heavy' "$l" | grep -av '^\[script\] {' | cut -c1-900 | tail -24; done; echo '```'
