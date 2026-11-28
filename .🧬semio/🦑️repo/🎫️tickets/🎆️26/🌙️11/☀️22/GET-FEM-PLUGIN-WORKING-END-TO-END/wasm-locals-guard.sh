#!/usr/bin/env bash
# 🛡️ Guard law: no wasm function in the given rlibs may approach wasmparser's 50000 locals limit (default threshold 40000). Paths are scanned in batches (a single argv overflows Windows), any scanner error fails the guard, and an empty input fails too.
# Usage: wasm-locals-guard.sh [threshold] rlib...
T="$(dirname "$0")"; thr="${1:-40000}"; shift
[ $# -gt 0 ] || { echo "FAIL: no rlibs given"; exit 1; }
out=""; err=0
while [ $# -gt 0 ]; do
  batch=("${@:1:100}"); shift $(( $# < 100 ? $# : 100 ))
  o="$(python "$T/wasm_locals_scan.py" rlib "$thr" "${batch[@]}" 2>"$T/🗑️generated/locals-guard.err")" || err=1
  grep -q "^ERR\|Traceback" "$T/🗑️generated/locals-guard.err" && { cat "$T/🗑️generated/locals-guard.err"; err=1; }
  out="$out$o"
done
rm -f "$T/🗑️generated/locals-guard.err"
[ $err -eq 0 ] || { echo "FAIL: scanner error"; exit 1; }
[ -z "$out" ] && { echo "ok: no function >= $thr locals"; exit 0; }
echo "$out"; echo "FAIL: functions at or above $thr locals (limit 50000)"; exit 1
