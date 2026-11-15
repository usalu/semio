#!/bin/zsh
T="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
if /usr/bin/grep -q "claude-package-AC" "$T/landing.lock/owner" 2>/dev/null; then rm "$T/landing.lock/owner" && rmdir "$T/landing.lock" && echo released; else echo "not mine: $(cat $T/landing.lock/owner 2>&1)"; exit 1; fi
