#!/bin/bash
# usage: datepalm-all.sh [thickness json] [plate factor] [ext json]: restores the snapshot, flattens the fronds, drops plugs, fixes the rings, triages
. "$(dirname "$0")/datepalm-env.sh"
TH="${1:-}"; [ -z "$TH" ] && TH='{"50":0.00855,"100":0.01729,"200":0.0165}'
EX="${3:-}"; [ -z "$EX" ] && EX='{"100":{"fr-l4b":0.0182,"fr-r1b":0.028},"50":{"fr-l4b":0.014,"fr-l1b":0.0156},"200":{"fr-l4b":0.03,"fr-l1b":0.028}}'
cp "$G/s2.json" "$P"
python3 "$H/datepalm-flatten.py" "$TH" "${2:-0.5}" "$EX" && python3 "$H/datepalm-plugs.py" && python3 "$H/datepalm-rings.py"
cd "$TS" && $B "$H/datepalm-axes.ts" && $B "$R/silhouette-triage.ts" current "$G/fa.jsonl" date-palm >/dev/null 2>&1; cat "$G/fa.jsonl"
