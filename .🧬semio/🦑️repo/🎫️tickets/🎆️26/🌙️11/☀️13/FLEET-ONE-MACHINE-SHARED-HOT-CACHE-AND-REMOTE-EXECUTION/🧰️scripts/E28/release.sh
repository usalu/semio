#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
for a in caching-bootstrap caching-graph caching-patches caching-tests testing tools; do d="$T/landing.lock.$a"; if [ "$(head -1 $d/owner 2>/dev/null | cut -d' ' -f1)" = "E28" ]; then rm -f "$d/owner"; rmdir "$d" && echo "released $a"; fi; done
