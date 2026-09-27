#!/bin/zsh
# 🔐️ H13: run one hold script inside the fleet native lane (FIFO, one native cargo fleet-wide), waiting first while the
# machine runs more than 14 rustc (preamble rule 6). The lane log records queue + hold times.
# usage: h13-hold.sh <hold script> <label>
cd /Users/ueli/Documents/semio
HOLD=$1; LABEL=$2
LANE="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h13-logs/$LABEL-lane.txt"
until [ "$(ps -axo command | /usr/bin/grep -c '^[^ ]*rustc ')" -le 14 ]; do sleep 30; done
echo "=== queued $(date +%T) $LABEL" > "$LANE"
zsh "/Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh" native h13 -- zsh -c 'echo "=== hold $(date +%T)"; zsh "$1" "$2"; echo "=== release $(date +%T) rc $?"' h13 "$HOLD" "$LABEL" 2>&1 | tee -a "$LANE"
