#!/usr/bin/env bash
# usage: l1c-retry.sh <attempts> <name> <cargo test args...>; retries until the shared tree compiles
cd "$(dirname "$0")"
n=$1; name=$2; shift 2
rm -f "🗑️generated/l1c-$name.status"
for i in $(seq 1 "$n"); do
  log="🗑️generated/l1c-$name-$i.log"
  L1B_TMPFS=4g L1_LOCK=-q ./l1b-native.sh 3d test "$@" > "$log" 2>&1
  if ! grep -q "^error: could not compile\|^error: failed to" "$log"; then echo "ran attempt $i" > "🗑️generated/l1c-$name.status"; exit 0; fi
  echo "broken attempt $i: $(grep -m1 '^error: could not compile\|^error: failed to' "$log" | cut -c1-120)" >> "🗑️generated/l1c-$name.status"
  sleep 300
done
echo exhausted >> "🗑️generated/l1c-$name.status"
