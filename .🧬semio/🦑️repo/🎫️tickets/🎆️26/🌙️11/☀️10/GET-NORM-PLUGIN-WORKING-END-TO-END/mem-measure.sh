#!/usr/bin/env bash
# usage: mem-measure.sh <label> <manifest-dir> <build-crate> <touch-file> <rustc_crate_name>...   (one build at a time; touches the file so cargo recompiles that unit)
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
label="$1"; dir="$2"; crate="$3"; touchf="$4"; shift 4
G="$T/🗑️generated"; rm -f "$G/$label.stop"
python3 "$T/mem-sample.py" "$G/$label.stop" "$G/$label.peaks.json" "$@" &
sp=$!
[ -n "$touchf" ] && touch "$touchf"
"$T/mem-build.sh" "$dir" "$label" "$crate"
touch "$G/$label.stop"; wait $sp
echo "MEASURED $label $(cat "$G/$label.peaks.json")" >> "$G/$label.log"
