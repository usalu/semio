#!/bin/bash
# usage: slot.sh <label> <command...>  — FIFO fleet cargo slots: W- labels first, then arrival order; K normal slots (slots.k), W- may use K+3
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
D="$C/🗑️generated/1102/slots"; Q="$D/queue"; mkdir -p "$Q"; L="$1"; shift
case "$L" in W-*) P=0;; *) P=1;; esac
TK="$Q/$P-$(perl -MTime::HiRes=time -e 'printf "%017.6f", time')-$$"; echo "$L" > "$TK"
trap 'rm -f "$TK"' EXIT
while :; do
 for t in "$Q"/*; do [ -e "$t" ] || continue; kill -0 "${t##*-}" 2>/dev/null || rm -f "$t"; done
 H=$(ls "$Q" | sort | head -1)
 if [ "$Q/$H" = "$TK" ]; then
  K=$(cat "$C/1102-tools/slots.k" 2>/dev/null || echo 3); [ "$P" = 0 ] && K=$((K+$(cat "$C/1102-tools/slots.wextra" 2>/dev/null || echo 3)))
  for i in $(seq 1 "$K"); do
   S="$D/s$i"
   if [ -d "$S" ]; then O=$(cat "$S/pid" 2>/dev/null); [ -n "$O" ] && kill -0 "$O" 2>/dev/null && continue; rm -rf "$S"; fi
   if mkdir "$S" 2>/dev/null; then echo $$ > "$S/pid"; echo "$L $(date '+%T')" > "$S/label"; rm -f "$TK"; trap 'rm -rf "$S"' EXIT; echo "[slot] $L holds s$i at $(date '+%T')"; "$@"; exit $?; fi
  done
 fi
 sleep 10
done
