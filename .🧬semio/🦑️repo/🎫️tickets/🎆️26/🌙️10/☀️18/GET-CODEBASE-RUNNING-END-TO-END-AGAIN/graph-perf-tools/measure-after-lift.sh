#!/bin/bash
# usage: measure-after-lift.sh TARGET [cold]   TARGET = a cheap cached task such as <project>:<target>. Run ONLY after "PAUSE LIFTED", the staged set applied, `bun repo dependencies sync` green and the fleet below load 10.
# Measures (wall seconds, timestamped logs in $OUT): CLI boot, `show projects` (cold when `cold` is given and the implementation revision changed, then twice warm), `run TARGET` three times.
# Targets of the brief: show projects <= 300 s cold, <= 30 s warm; `run TARGET` overhead <= 3 s warm (wall of the second and third run minus the task's own time).
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"; ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"; cd "$ROOT" || exit 2
TARGET="${1:?target required}"; MODE="${2:-warm}"
OUT="$HERE/../🗑️generated/graph-perf-measure-$(date +%H%M%S)"; mkdir -p "$OUT"
stamp() { perl -MPOSIX -MTime::HiRes=time -e '$|=1; while(<STDIN>){ print POSIX::strftime("%H:%M:%S ",localtime), $_ }'; }
timed() { local name="$1"; shift; local s=$(perl -MTime::HiRes=time -e 'print time'); ( "$@" 2>&1 ) | stamp > "$OUT/$name.log"; local e=$(perl -MTime::HiRes=time -e 'print time'); printf '%-28s %8.1f s\n' "$name" "$(echo "$e - $s" | bc -l)" | tee -a "$OUT/summary.txt"; }
echo "load: $(uptime)" | tee "$OUT/summary.txt"
for i in 1 2 3; do timed "cli-help-$i" bun repo --help; done
if [ "$MODE" = cold ]; then SEMIO_BUILD_PROFILE_INPUTS=1 timed "show-projects-cold" bun repo show projects; fi
for i in 1 2; do SEMIO_BUILD_PROFILE_INPUTS=1 timed "show-projects-warm-$i" bun repo show projects; done
for i in 1 2 3; do SEMIO_BUILD_PROFILE_INPUTS=1 timed "run-$i" bun repo run "$TARGET"; done
{ echo; echo "graph reuse decisions:"; command grep -h "Graph observation\|Graph membership\|Bootstrap phases" "$OUT"/run-*.log | sed 's/^/  /'; } | tee -a "$OUT/summary.txt"
echo "logs: $OUT"
