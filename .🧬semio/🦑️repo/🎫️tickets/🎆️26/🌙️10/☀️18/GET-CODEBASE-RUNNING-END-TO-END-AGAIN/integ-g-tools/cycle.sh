#!/bin/sh
# usage: cycle.sh <cycle-id> <product> ; one merged official Bazel run-many for one product; verdict row appended to generated/integ-g/cycles.tsv
# products: dash | print | course ; skips (exit 3) when >= 4 waiters hold the lease queue
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/integ-g"; mkdir -p "$O"
unset SEMIO_SCRIPT_STALL_MS
export SEMIO_LANGUAGE=en
id="$1"; prod="$2"
case "$prod" in
  dash) projects="@semio-tech/repo-dashboard-rs"; targets="${3:-build,test-unit,test-journeys}" ;;
  print) projects="@semio-tech/print-rs,@semio-tech/print,@semio-tech/presentation,@semio-tech/presentation-react,@semio-tech/mit-bestand-praesentation-projektetage"; targets="${3:-check,build,test}" ;;
  course) projects="@teaching/architecture-course,@semio-tech/ui-react"; targets="${3:-typecheck,test,build}" ;;
  *) exit 2 ;;
esac
w=$(bun repo leases status 2>/dev/null | /usr/bin/grep -c '^WAITING')
if [ "$w" -ge 4 ]; then exit 3; fi
log="$O/$id.$prod.log"; st=$(date +%H:%M:%S); s0=$(date +%s)
nice -n 10 bun repo --tool=bazel run-many --projects="$projects" --targets="$targets" --parallel=1 > "$log" 2>&1
rc=$?
printf '%s\t%s\trc=%s\twaiters=%s\t%s-%s\twall=%ss\t%s\n' "$id" "$prod" "$rc" "$w" "$st" "$(date +%H:%M:%S)" "$(( $(date +%s) - s0 ))" "$targets" >> "$O/cycles.tsv"
exit $rc
