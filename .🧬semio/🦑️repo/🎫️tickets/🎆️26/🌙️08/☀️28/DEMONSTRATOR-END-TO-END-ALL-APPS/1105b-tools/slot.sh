#!/bin/bash
# usage: slot.sh <label> <command...>  — FIFO fleet cargo slots: W- labels first, then arrival order; K normal slots (slots.k), W- may use K+3
# Hang guard: a cargo test binary (.../deps/ or .../out/<name>-<16 hex>) below this slot that adds no byte to the slot log for SLOT_HANG_SECS (default 180) is logged and killed (TERM, then KILL). Only that binary: never cargo, rustc or wrappers.
hang_guard() { local lim=${SLOT_HANG_SECS:-180} tick=${SLOT_HANG_TICK:-30} cur= prev= sz= last= t0=0 now
 while sleep "$tick" >/dev/null 2>&1; do
  cur=$(ps -axo pid=,ppid=,args= | awk -v r="$$" '{p[$1]=$2;a[$1]=$3} END{for(i in a){n=i;d=0;while(n!=""&&n!=0&&n!=1&&d++<64){if(n==r){if(match(a[i],/-[0-9a-f]+$/)&&RLENGTH==17&&a[i]~/\/(deps|out)\//)print i;break};n=p[n]}}}' | sort -n | tr '\n' ' ')
  sz=$(stat -L -f %z /dev/fd/1 2>/dev/null); now=$SECONDS
  if [ -z "$cur" ] || [ "$cur" != "$prev" ] || [ "$sz" != "$last" ]; then prev=$cur; last=$sz; t0=$now; continue; fi
  if [ $((now-t0)) -ge "$lim" ]; then
   echo "[slot] HANG GUARD: killing test binary pid(s) $cur of $L: no output for $((now-t0))s (limit ${lim}s) at $(date '+%T')" >&2
   kill -TERM $cur 2>/dev/null; sleep 10 >/dev/null 2>&1; kill -KILL $cur 2>/dev/null; t0=$SECONDS
  fi
 done
}
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
D="$C/🗑️generated/1105b/slots"; Q="$D/queue"; mkdir -p "$Q"; L="$1"; shift
case "$L" in W-*) P=0;; *) P=1;; esac
G="$C/🗑️generated/1105b/gate"; while [ -f "$G" ] && ! /usr/bin/grep -qx "$L" "$G" && ! case "$L" in renames*|W-*) true;; *) false;; esac; do sleep 15; done
TK="$Q/$P-$(perl -MTime::HiRes=time -e 'printf "%017.6f", time')-$$"; echo "$L" > "$TK"
trap 'rm -f "$TK"' EXIT
while :; do
 for t in "$Q"/*; do [ -e "$t" ] || continue; kill -0 "${t##*-}" 2>/dev/null || rm -f "$t"; done
 H=$(ls "$Q" | sort | head -1)
 if [ "$Q/$H" = "$TK" ]; then
  K=$(cat "$C/1105b-tools/slots.k" 2>/dev/null || echo 3); [ "$P" = 0 ] && K=$((K+$(cat "$C/1105b-tools/slots.wextra" 2>/dev/null || echo 3)))
  i=0; while [ "$i" -lt "$K" ]; do i=$((i+1));
   S="$D/s$i"
   if [ -d "$S" ]; then O=$(cat "$S/pid" 2>/dev/null); [ -n "$O" ] && kill -0 "$O" 2>/dev/null && continue; rm -rf "$S"; fi
   if mkdir "$S" 2>/dev/null; then echo $$ > "$S/pid"; echo "$L $(date '+%T')" > "$S/label"; rm -f "$TK"; trap 'kill $W 2>/dev/null; wait $W 2>/dev/null; rm -rf "$S"' EXIT; echo "[slot] $L holds s$i at $(date '+%T')"; hang_guard & W=$!; "$@"; exit $?; fi
  done
 fi
 sleep 10
done
