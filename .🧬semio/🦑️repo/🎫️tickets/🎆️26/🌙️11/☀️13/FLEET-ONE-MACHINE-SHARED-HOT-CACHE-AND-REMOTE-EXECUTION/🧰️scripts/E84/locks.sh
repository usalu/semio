#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
mode=$1; shift
if [[ $mode == take ]]; then
  taken=()
  for a in "$@"; do
    if mkdir "$T/landing.lock.$a" 2>/dev/null; then echo "E84 $(date +%H:%M) tests-build-B-2" > "$T/landing.lock.$a/owner"; taken+=($a)
    else for t in $taken; do rm -r "$T/landing.lock.$t"; done; echo "blocked by $a: $(cat "$T/landing.lock.$a/owner" 2>/dev/null)"; exit 1; fi
  done; echo "taken: $taken"
else
  for a in "$@"; do grep -q '^E84' "$T/landing.lock.$a/owner" 2>/dev/null && rm -r "$T/landing.lock.$a"; done; echo released
fi
