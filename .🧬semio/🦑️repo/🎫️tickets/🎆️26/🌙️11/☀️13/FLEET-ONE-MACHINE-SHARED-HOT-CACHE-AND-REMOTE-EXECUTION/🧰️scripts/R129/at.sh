#!/bin/zsh
# usage: at.sh <MM>   waits (0.1 s steps) for minute MM UTC, then runs window.sh in the foreground
until [ "$(date -u +%M)" = "$1" ]; do sleep 0.1; done
exec /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R129/window.sh
