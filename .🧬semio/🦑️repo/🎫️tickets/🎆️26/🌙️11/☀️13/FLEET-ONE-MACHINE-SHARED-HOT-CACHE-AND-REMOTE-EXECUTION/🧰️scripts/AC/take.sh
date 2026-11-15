#!/bin/zsh
T="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
if mkdir "$T/landing.lock" 2>/dev/null; then echo "claude-package-AC $(date +%Y-%m-%dT%H:%M:%S)" > "$T/landing.lock/owner"; echo taken; else echo "held: $(cat $T/landing.lock/owner)"; exit 1; fi
