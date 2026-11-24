#!/bin/zsh
# 🔁 carve-C3-retry.sh: runs carve-C3-build.sh until the live workspace resolves and the final is built, at most 8 attempts, 10 minutes apart
# (the shared worktree changes under every build: a half-created crate of another developer fails cargo's planning within minutes).
cd /Users/ueli/Documents/semio
t=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️24/FLEET-MEMORY-WATCHDOG-ADMISSION-AND-UNIT-CARVING"
for attempt in 1 2 3 4 5 6 7 8; do
  [[ -f "$t/🗑️generated/thin.log" ]] && mv "$t/🗑️generated/thin.log" "$t/🗑️generated/thin-failed-$(date +%H%M%S).log"
  "$t/carve-C3-build.sh" thin thin | tail -1 | tee "$t/🗑️generated/last.tsv"
  grep -q "exit=0" "$t/🗑️generated/last.tsv" && { echo "built on attempt $attempt"; exit 0; }
  echo "attempt $attempt failed: $(grep -m1 -E '^error' "$t/🗑️generated/thin.log" | cut -c1-200)"
  sleep 600
done
echo "gave up after 8 attempts"; exit 1
