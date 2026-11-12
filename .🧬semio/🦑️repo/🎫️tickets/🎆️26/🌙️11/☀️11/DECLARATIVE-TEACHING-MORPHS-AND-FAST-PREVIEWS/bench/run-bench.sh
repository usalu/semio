#!/usr/bin/env bash
# Usage: run-bench.sh <root> <prefix> <script> <scene> <cold|warm> <l|h> <fps> [extra run_scene args...]
# <root> is the source tree root (live repo or a git-archive snapshot); outputs go to $T/🗑️generated/<prefix>-<q><fps>-<cold|warm>.
# cold = fresh output root; warm = second run reusing the output root of the cold run. Always a fresh process.
set -u
REPO=/c/git/semio
T="$REPO/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️11/DECLARATIVE-TEACHING-MORPHS-AND-FAST-PREVIEWS"
root=$1 prefix=$2 script=$3 scene=$4 temp=$5 q=$6 fps=$7
shift 7
name="$prefix-$q$fps"
out="$T/🗑️generated/$name"
label="$name-$temp"
if [ "$temp" = cold ]; then rm -rf "$out"; fi
mkdir -p "$out"
export PYTHONUTF8=1 PYTHONDONTWRITEBYTECODE=1 BENCH_ROOT="$root" SEMIO_TEACHING_CACHE_DIR="$REPO/.🧬semio/🎓️teaching/⚡️cache/fonts"
cd "$root"
{
  echo "[DEBUG] load-before $(powershell -NoProfile -c "(Get-Counter '\\Processor(_Total)\\% Processor Time' -SampleInterval 1 -MaxSamples 3 | % { [int]\$_.CounterSamples.CookedValue }) -join ','") free_mb=$(powershell -NoProfile -c "[int](Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory/1024")"
  "$REPO/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe" "$T/bench/run_scene.py" --script "$script" --scene "$scene" --out "$out" --label "$label" --quality "$q" --fps "$fps" "$@"
  echo "[DEBUG] load-after $(powershell -NoProfile -c "(Get-Counter '\\Processor(_Total)\\% Processor Time' -SampleInterval 1 -MaxSamples 3 | % { [int]\$_.CounterSamples.CookedValue }) -join ','")"
} > "$T/🗑️generated/$label.log" 2>&1
grep -E "load-|\[DEBUG\] bench" "$T/🗑️generated/$label.log" | cut -c1-400
