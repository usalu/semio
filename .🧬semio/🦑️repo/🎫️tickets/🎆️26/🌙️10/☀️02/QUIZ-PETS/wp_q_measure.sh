#!/usr/bin/env bash
# Work package Q: runs one verb of the pets core package (`test`, `test quick`, `typecheck`, …) and prints its exit code,
# its wall time, the processor load of the host just before it, and Vitest's own summary lines.
# Usage: bash wp_q_measure.sh <label> <package directory> <script arguments…>   (output: 🗑️generated/wp-q/<label>.txt)
set -u
ticket="$(cd "$(dirname "$0")" && pwd)"
label="$1"
package="$2"
shift 2
out="$ticket/🗑️generated/wp-q/$label.txt"
mkdir -p "$(dirname "$out")"
load="$(pwsh -NoProfile -Command "(Get-CimInstance Win32_PerfFormattedData_PerfOS_Processor -Filter \"Name='_Total'\").PercentProcessorTime")"
cd "$package" || exit 2
start=$(date +%s%N)
bun ./📜️script.ts "$@" > "$out" 2>&1
code=$?
wall=$(( ($(date +%s%N) - start) / 1000000 ))
echo "$label: exit=$code wall=${wall}ms load-before=${load}% $(grep -E 'Test Files|Tests |Duration|\[budget\]' "$out" | sed -E 's/ +/ /g' | cut -c1-150 | tr '\n' '|')"
