#!/bin/sh
# usage: run.sh <asset> <builder-suffix|-> <render args...>
cd /c/git/semio || exit 1
export PYTHONIOENCODING=utf-8
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
ID="$1"; B="$2"; shift 2
OUT="$T/🗑️generated/M2"
mkdir -p "$OUT"
if [ "$B" != "-" ]; then python "$T/🧰️M2/build_$B.py" || exit 1; fi
repo tools exec bun -- "$T/🧰️M2/batch.ts" "$ID" check > "$OUT/check-$ID.txt" 2>&1
grep -v -E "^\[(capacity|repo|tools)\]" "$OUT/check-$ID.txt"
if [ $# -gt 0 ]; then
  repo tools exec bun -- "$T/🧰️M2/batch.ts" "$ID" render "$@" > "$OUT/render-$ID.txt" 2>&1
  grep -v -E "^\[(capacity|repo|tools)\]" "$OUT/render-$ID.txt" | tail -20
fi
