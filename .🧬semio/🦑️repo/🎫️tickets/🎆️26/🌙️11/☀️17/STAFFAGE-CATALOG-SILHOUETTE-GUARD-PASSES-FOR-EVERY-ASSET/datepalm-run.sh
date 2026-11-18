#!/bin/bash
# usage: datepalm-run.sh '<ext json>' [thickness json] [plate factor]: restores s2.json, flattens fronds with the tip extensions, prints the axes
. "$(dirname "$0")/datepalm-env.sh"
TH="${2:-}"; [ -z "$TH" ] && TH='{"50":0.00855,"100":0.01729,"200":0.0165}'
cp "$G/s2.json" "$P"
python3 "$H/datepalm-flatten.py" "$TH" "${3:-0.5}" "$1"
cd "$TS" && $B "$H/datepalm-axes.ts"
