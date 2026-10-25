#!/bin/sh
# usage: contended.sh <crate> <run-id> : a real `bun repo cargo ... check -p <crate>` that must QUEUE behind a custody held on its own lane build directory, then run to rc 0.
# Own agent session (unique client id) => its own lane, so the holder can name the exact build directory the real run will use.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/lockc-d"; ID="$2"; STOP="$G/$ID.stop"; rm -f "$STOP"
unset SEMIO_SCRIPT_STALL_MS
export CLAUDE_CODE_SESSION_ID="lockcd-$ID"
nice -n 10 bun "$C/lockc-d-tools/hold-custody.ts" 1500 "$STOP" > "$G/$ID.holder.log" 2>&1 &
HOLDER=$!
for i in $(seq 1 100); do /usr/bin/grep -q '"event":"held"' "$G/$ID.holder.log" && break; sleep 0.3; done
/usr/bin/grep -q '"event":"held"' "$G/$ID.holder.log" || { echo "holder did not take the custody"; kill $HOLDER; exit 2; }
echo "holder pid $HOLDER holds the custody since $(date +%T)"
start=$(date +%s)
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check -p "$1" > "$G/$ID.cargo.log" 2>&1 &
CARGO=$!
echo "cargo pid $CARGO started $(date +%T)"
for i in $(seq 1 2700); do
  /usr/bin/grep -q 'is held by process' "$G/$ID.cargo.log" && break
  kill -0 $CARGO 2>/dev/null || break
  sleep 1
done
if /usr/bin/grep -q 'is held by process' "$G/$ID.cargo.log"; then
  echo "WAITING-PROOF $(date +%T) $(/usr/bin/grep 'is held by process' "$G/$ID.cargo.log" | head -1 | cut -c1-300)"
  bun repo leases status 2>&1 | /usr/bin/grep -i 'custody' | cut -c1-400 > "$G/$ID.leases.txt"
  echo "leases status custody rows: $(wc -l < "$G/$ID.leases.txt")"
else
  echo "NO-WAIT-MESSAGE (cargo exited early?)"
fi
touch "$STOP"
echo "custody released $(date +%T)"
wait $CARGO; rc=$?
echo "cargo rc=$rc wall=$(( $(date +%s) - start ))s"
wait $HOLDER
