#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
areas=(${=AREAS})
taken=()
for a in $areas; do
  if mkdir "$T/landing.lock.$a" 2>/dev/null; then
    echo "E26 $(date +%H:%M) test-file operation" > "$T/landing.lock.$a/owner"
    taken+=($a)
  else
    for t in $taken; do rm -r "$T/landing.lock.$t"; done
    echo "blocked by $a: $(cat "$T/landing.lock.$a/owner" 2>/dev/null | tr '\n' ' ')"
    exit 1
  fi
done
echo "all taken: $taken"
