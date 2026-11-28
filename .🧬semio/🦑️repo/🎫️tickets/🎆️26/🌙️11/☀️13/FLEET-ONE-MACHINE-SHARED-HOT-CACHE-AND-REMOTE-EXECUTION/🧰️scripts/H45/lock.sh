#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
case "$1" in
 take) shift; for a in "$@"; do n=0; until mkdir "$T/landing.lock.$a" 2>/dev/null; do sleep 5; n=$((n+1)); [ $n -ge 60 ] && { echo "lock $a busy"; exit 1; }; done; echo "H45 $(date +%H:%M:%S)" > "$T/landing.lock.$a/owner"; echo "took $a"; done;;
 free) shift; for a in "$@"; do grep -q "^H45" "$T/landing.lock.$a/owner" 2>/dev/null && rm -rf "$T/landing.lock.$a" && echo "freed $a"; done;;
esac
