#!/bin/sh
# Runs the diagnostic graph build once: infers every project, collects every per-project failure, never publishes. Usage: sh collect-run.sh [OUTPUT_DIR]; reads OUTPUT_DIR/report/{report.json,summary.en.txt,summary.de.txt} afterwards.
T="$(cd "$(dirname "$0")" && pwd)"; W="$(cd "$T" && git rev-parse --show-toplevel)"; cd "$W" || exit 1
G="${1:-$T/🗑️generated/collect}"; mkdir -p "$G"
SEMIO_GRAPH_COLLECT_ERRORS="$G/report" SEMIO_GRAPH_PHASES=1 nice -n 10 bun repo show projects > "$G/show.out" 2> "$G/show.err"
echo "exit $? report: $G/report"
