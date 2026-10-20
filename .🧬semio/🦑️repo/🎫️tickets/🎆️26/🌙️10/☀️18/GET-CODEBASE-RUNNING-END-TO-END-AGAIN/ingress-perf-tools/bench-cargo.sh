#!/bin/sh
# 📊 After "PAUSE LIFTED": N timed `bun repo cargo` calls with phase marks, spawn census and optional cpu profile. Usage: bench-cargo.sh <label> <runs> [profile]
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$T/🗑️generated/ingress-perf-bench"; mkdir -p "$G"
label="$1"; runs="${2:-3}"; profile="$3"
i=1
while [ "$i" -le "$runs" ]; do
  log="$G/$label-$i.log"; census="$G/$label-$i.census.jsonl"
  start=$(perl -MTime::HiRes=time -e 'printf "%.3f", time')
  if [ -n "$profile" ]; then prof="--cpu-prof --cpu-prof-dir=$G/$label-$i-prof"; else prof=""; fi
  SEMIO_CARGO_TIMING=1 SEMIO_CENSUS_OUT="$census" BUN_OPTIONS="--preload $PWD/$T/ingress-perf-tools/census-preload.ts" \
    perl -e 'alarm 590; exec @ARGV' -- bun $prof repo cargo --store=shared --configuration=prototype -- check -p semio-framework-hash --lib --manifest-path 🧰️framework/Cargo.toml > "$log" 2>&1
  rc=$?
  end=$(perl -MTime::HiRes=time -e 'printf "%.3f", time')
  echo "$label run $i rc=$rc wall=$(perl -e "printf '%.2f', $end-$start") s log=$log"
  bun "$T/ingress-perf-tools/phase-table.ts" < "$log"
  grep "^\[census\]" "$log" | head -12
  i=$((i+1))
done
